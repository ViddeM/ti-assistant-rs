#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![warn(clippy::large_futures)]
#![allow(dead_code, clippy::single_match)]

use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use clap::Parser;
use eyre::{bail, Context};
use ti_helper_db::{
    db::{self, DbPool},
    game_id::GameId,
    queries,
};
use ti_helper_game::gameplay::{error::GameError, event::Event, game::Game};
use ti_helper_websocket::{
    websocket_client::WsClient,
    ws_message::{WsMessageIn, WsMessageOut},
};
use tokio::{
    net::{TcpListener, TcpStream},
    select, spawn,
    sync::RwLock,
};

use crate::lobby::{generate_game_name, Lobbies, Lobby};

pub mod gc;
mod insert_demo_games;
pub mod lobby;

#[derive(Parser)]
pub struct Opt {
    #[clap(long, env = "BIND_HOST", default_value = "0.0.0.0")]
    host: String,

    #[clap(long, env = "BIND_PORT", default_value = "5555")]
    port: u16,

    /// Postgres URI
    #[clap(long = "db", env = "DATABASE_URL")]
    database_url: Option<String>,

    /// Weather or not the demo games should overwrite any existing games with the same ID in the DB.
    #[clap(long, env = "OVERWRITE_DB_DEMO_GAMES")]
    overwrite_db_games: bool,

    /// Weather or not to insert demo games into the DB at startup.
    #[clap(long, env = "DEMO_GAMES_SKIP_DB")]
    demo_games_skip_db: bool,

    /// The directory of the demo games.
    #[clap(long, env = "DEMO_GAMES_DIR")]
    demo_games_dir: PathBuf,

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

    if !opt.demo_games_skip_db {
        let Some(db_pool) = &db_pool else {
            eyre::bail!("DEMO_GAMES_SKIP_DB is not set but no DB has been configured");
        };

        insert_demo_games::insert_demo_games(&opt, db_pool).await?;
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
        log::debug!("new connection from {from}");
        spawn(handle_client(Arc::clone(&shared), stream, from));
    }
}

pub async fn handle_client(shared: Arc<Shared>, stream: TcpStream, from: SocketAddr) {
    async fn inner(shared: &Shared, stream: TcpStream, from: SocketAddr) -> eyre::Result<()> {
        let Shared {
            lobbies, db_pool, ..
        } = shared;

        let mut ws_client = WsClient::accept(stream).await?;

        let message = ws_client.receive_message::<WsMessageIn>().await?;

        let (id, lobby) = match message {
            WsMessageIn::NewGame(new_game) => {
                let id = GameId::random();
                let name = generate_game_name(id);

                if let Some(db_pool) = &db_pool {
                    log::info!("persisting new game {id:?} in database");
                    queries::create_game(db_pool, id, name)
                        .await
                        .wrap_err_with(|| format!("insert new game {id:?} in db"))?;
                }

                let mut game = Game::default();

                let set_settings_event = Event::SetSettings {
                    settings: new_game.into(),
                };
                let now = Utc::now();
                game.apply_or_err(set_settings_event.clone(), now)?;
                if let Some(db_pool) = &shared.db_pool {
                    log::info!("persisting event for game {id:?}");

                    queries::insert_game_event(
                        db_pool,
                        id,
                        serde_json::to_value(&set_settings_event)?,
                        now,
                    )
                    .await
                    .wrap_err_with(|| format!("error querying game events ({id:?})"))?;
                }

                let lobby = Lobby::new(game);
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
                    log::info!("loading game {id:?} from db");

                    if let Err(e) = queries::get_game_by_id(db_pool, &id)
                        .await
                        .wrap_err_with(|| format!("Failed to retrieve game {id:?} from DB"))
                    {
                        ws_client.send_message(&WsMessageOut::not_found(id)).await?;
                        return Err(e);
                    }

                    let events = queries::get_events_for_game(db_pool, &id)
                        .await
                        .wrap_err_with(|| format!("error querying game events ({id:?})"))?;

                    log::info!("replaying {} events for game {id:?}", events.len());
                    let mut game = Game::default();
                    for record in events {
                        let event = serde_json::from_value(record.event)?;
                        game.apply(event, record.timestamp);
                    }

                    log::info!("loaded game {id:?}");
                    let lobby = Lobby::new(game);
                    list.insert(id, Arc::clone(&lobby));

                    (id, lobby)
                } else {
                    ws_client.send_message(&WsMessageOut::not_found(id)).await?;
                    bail!("no lobby with id {id:?}");
                }
            }
            _ => {
                bail!("got unexpected initial message: {message:?}")
            }
        };

        ws_client
            .send_message(&WsMessageOut::game_options(
                &lobby.read().await.game.current.game_settings.expansions,
            ))
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
                        WsMessageIn::Event(event) => {
                            match handle_event(shared, id, &lobby, event).await {
                                Ok(_) => {},
                                Err(EventError::HandleEventError(e)) => {
                                    ws_client.send_message(&WsMessageOut::event_err(e)).await?;
                                },
                                Err(EventError::InternalError(err)) => return Err(err),
                            }
                        }

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

enum EventError {
    HandleEventError(String),
    InternalError(GameError),
}

async fn handle_event(
    shared: &Shared,
    id: GameId,
    lobby: &RwLock<Lobby>,
    event: Event,
) -> Result<(), EventError> {
    log::debug!("applying event {event:?}");

    let mut lobby = lobby.write().await;

    let now = Utc::now();

    if let Err(e) = lobby.game.apply_or_err(event.clone(), now) {
        log::warn!("Event not valid for the current state, err: {e:?}");
        return Err(EventError::HandleEventError(e.to_string()));
    }

    store_and_propagate_event(shared, id, event, now, &lobby)
        .await
        .map_err(EventError::InternalError)?;

    Ok(())
}

async fn store_and_propagate_event(
    shared: &Shared,
    id: GameId,
    event: Event,
    timestamp: DateTime<Utc>,
    lobby: &Lobby,
) -> eyre::Result<()> {
    if let Some(db_pool) = &shared.db_pool {
        log::info!("persisting event for game {id:?}");

        queries::insert_game_event(db_pool, id, serde_json::to_value(&event)?, timestamp)
            .await
            .wrap_err_with(|| format!("error querying game events ({id:?})"))?;
    }

    lobby.state_updates.send(lobby.game.current.clone())?;

    Ok(())
}

async fn handle_undo(shared: &Shared, id: GameId, lobby: &RwLock<Lobby>) -> eyre::Result<()> {
    let mut lobby = lobby.write().await;

    if let Some(db_pool) = &shared.db_pool {
        log::info!("undoing last event for game {id:?}");
        queries::delete_latest_event_for_game(db_pool, &id).await?;
    }

    lobby.game.undo();
    lobby.state_updates.send(lobby.game.current.clone())?;

    Ok(())
}
