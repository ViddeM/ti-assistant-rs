#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(clippy::single_match)]

use std::{net::SocketAddr, sync::Arc};

use clap::Parser;
use db::DbPool;
use diesel::{delete, insert_into, ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncConnection, RunQueryDsl};
use eyre::{bail, Context};
use gameplay::event::Event;
use lobby::GameId;
use tokio::{
    net::{TcpListener, TcpStream},
    select, spawn,
    sync::RwLock,
};
use ws_message::WsMessageOut;

use crate::{
    gameplay::game::Game,
    lobby::{generate_game_name, Lobbies, Lobby},
    websocket_client::WsClient,
    ws_message::WsMessageIn,
};

pub mod data;
pub mod db;
pub mod example_game;
pub mod game_options;
pub mod gameplay;
pub mod gc;
pub mod lobby;
pub mod schema;
pub mod websocket_client;
pub mod ws_message;

#[derive(Parser)]
pub struct Opt {
    #[clap(long, env = "BIND_HOST", default_value = "0.0.0.0")]
    host: String,

    #[clap(long, env = "BIND_PORT", default_value = "5555")]
    port: u16,

    /// Postgres URI
    #[clap(long = "db", env = "DATABASE_URL")]
    database_url: Option<String>,

    /// Automatically run database migrations, if needed.
    #[clap(short, long, env = "MIGRATE_DB", requires("database_url"))]
    migrate: bool,

    /// Cron string that defines when to unload inactive games from memory.
    ///
    /// Format: "sec min hour day_of_month month day_of_week year"
    #[clap(long, env = "MEM_GC_CRON")]
    mem_gc_cron: Option<String>,
}

/// Global shared state
pub struct Shared {
    /// Program arguments
    pub opt: Opt,

    /// Index of running and loaded games
    pub lobbies: Lobbies,

    /// Database pool (if any)
    pub db_pool: Option<DbPool>,
}

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let opt = Opt::parse();

    color_eyre::install()?;
    pretty_env_logger::init();

    let lobbies = Lobbies::default();
    let mut db_pool = None;

    if let Some(db_url) = &opt.database_url {
        if opt.migrate {
            db::run_migrations(db_url).wrap_err("failed to run migrations")?;
        }

        db_pool = Some(
            db::setup_pool(db_url)
                .await
                .wrap_err("failed to set up database pool")?,
        );
    }

    let shared = Arc::new(Shared {
        opt,
        lobbies,
        db_pool,
    });

    gc::setup_game_gc(&shared)?;

    let opt = &shared.opt;
    let server = TcpListener::bind((opt.host.as_str(), opt.port))
        .await
        .wrap_err_with(|| format!("Failed to listen on {}:{}", opt.host, opt.port))?;
    log::info!("Listening on {}:{}", opt.host, opt.port);

    loop {
        // TODO: figure out if this error should really be fatal
        let (stream, from) = server.accept().await.wrap_err("Failed to accept client")?;
        spawn(handle_client(Arc::clone(&shared), stream, from));
    }
}

pub async fn handle_client(shared: Arc<Shared>, stream: TcpStream, from: SocketAddr) {
    async fn inner(shared: &Shared, stream: TcpStream, from: SocketAddr) -> eyre::Result<()> {
        let Shared {
            lobbies, db_pool, ..
        } = shared;

        let mut ws_client = WsClient::accept(stream).await;

        let message = ws_client.receive_message::<WsMessageIn>().await?;

        let (id, lobby) = match message {
            WsMessageIn::NewGame => {
                let id = GameId::random();
                let name = generate_game_name(id);

                if let Some(db_pool) = &db_pool {
                    log::info!("persisting new game {id:?} in database");

                    let mut db = db_pool.get().await?;

                    use schema::game::dsl::game;
                    insert_into(game)
                        .values(&db::Game { id, name })
                        .execute(&mut db)
                        .await
                        .wrap_err_with(|| format!("insert new game {id:?} in db"))?;
                }

                let lobby = Lobby::new(Game::default());
                let mut lobbies = lobbies.list.write().await;

                if lobbies.contains_key(&id) {
                    bail!("new game id collision: {id:?}");
                }
                lobbies.insert(id, Arc::clone(&lobby));

                log::info!("created new game {id:?}");
                (id, lobby)
            }
            WsMessageIn::JoinGame(id) => {
                let mut list = shared.lobbies.list.write().await;

                if let Some(lobby) = list.get(&id) {
                    (id, Arc::clone(lobby))
                } else if let Some(db_pool) = &db_pool {
                    let mut db = db_pool.get().await?;

                    log::info!("loading game {id:?} from db");
                    use schema::game::dsl;
                    let _game: db::Game =
                        dsl::game.filter(dsl::id.eq(id)).get_result(&mut db).await?;

                    use schema::game_event::dsl::{game_event, game_id};
                    let events: Vec<db::GameEvent> = game_event
                        .filter(game_id.eq(id))
                        .load(&mut db)
                        .await
                        .wrap_err_with(|| format!("error querying game events ({id:?})"))?;

                    log::info!("replaying {} events for game {id:?}", events.len());
                    let mut game = Game::default();
                    for event in events {
                        let event = serde_json::from_value(event.event)?;
                        game.apply(event);
                    }

                    log::info!("loaded game {id:?}");
                    let lobby = Lobby::new(game);
                    list.insert(id, Arc::clone(&lobby));

                    (id, lobby)
                } else {
                    bail!("no lobby with id {id:?}");
                }
            }
            _ => {
                bail!("got unexpected initial message: {message:?}")
            }
        };

        ws_client
            .send_message(&WsMessageOut::game_options())
            .await?;
        ws_client.send_message(&WsMessageOut::join_game(id)).await?;

        let mut state_updates = {
            let lobby = lobby.read().await;
            ws_client
                .send_message(&WsMessageOut::GameState(lobby.game.current.clone()))
                .await?;

            // make sure we subscribe while we are holding the game state lock to avoid silly races
            lobby.state_updates.subscribe()
        };

        loop {
            select! {
                update = state_updates.recv() => {
                    log::debug!("sending state update to {from:?}");
                    ws_client.send_message(&WsMessageOut::GameState(update?)).await?;
                }
                message = ws_client.receive_message::<WsMessageIn>() => {
                    let message = message?;

                    match message {
                        WsMessageIn::Undo => handle_undo(shared, id, &lobby).await?,
                        WsMessageIn::Event(event) => handle_event(shared, id, &lobby, event).await?,

                        _ => bail!("got unexpected event: {message:?}"),
                    };

                }
            }
        }
    }

    if let Err(e) = inner(&shared, stream, from).await {
        log::warn!("disconnecting {from} because of error: {e:#}");
    } else {
        log::info!("{from} disconnected");
    }
}

async fn handle_event(
    shared: &Shared,
    id: GameId,
    lobby: &RwLock<Lobby>,
    event: Event,
) -> eyre::Result<()> {
    log::debug!("applying event {event:?}");

    let mut lobby = lobby.write().await;

    // TODO: propagate errors back over the socket?
    lobby.game.apply(event.clone());

    if let Some(db_pool) = &shared.db_pool {
        let mut db = db_pool.get().await?;
        use schema::game_event::dsl::game_event;

        insert_into(game_event)
            .values(&db::NewGameEvent {
                game_id: id,
                event: serde_json::to_value(&event)?,
            })
            .execute(&mut db)
            .await
            .wrap_err_with(|| format!("error querying game events ({id:?})"))?;

        log::info!("persisting event for game {id:?}");
    }

    lobby.state_updates.send(lobby.game.current.clone())?;

    Ok(())
}

async fn handle_undo(shared: &Shared, id: GameId, lobby: &RwLock<Lobby>) -> eyre::Result<()> {
    let mut lobby = lobby.write().await;

    if let Some(db_pool) = &shared.db_pool {
        log::info!("undoing last event for game {id:?}");

        let mut db = db_pool.get().await?;
        db.transaction(|db| {
            Box::pin(async move {
                use schema::game_event::dsl::{self, game_event, game_id, seq};

                // query the last event for this game
                let last_event_id: Option<i32> = game_event
                    .filter(game_id.eq(id))
                    .order_by(seq.desc())
                    .select(dsl::id)
                    .first(db)
                    .await
                    .optional()?;

                let Some(last_event_id) = last_event_id else {
                    return Ok(());
                };

                delete(game_event)
                    .filter(dsl::id.eq(last_event_id))
                    .execute(db)
                    .await
                    .wrap_err_with(|| format!("error querying game events ({id:?})"))
                    // sanity check that we deleted exactly one event
                    .map(|count| debug_assert_eq!(count, 1))
            })
        })
        .await?;
    }

    lobby.game.undo();
    lobby.state_updates.send(lobby.game.current.clone())?;

    Ok(())
}
