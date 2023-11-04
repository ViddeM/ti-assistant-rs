#![recursion_limit = "256"]
#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::{net::SocketAddr, str::FromStr, sync::Arc};

use chrono::Local;
use clap::Parser;
use cron::Schedule;
use diesel::{insert_into, ExpressionMethods, QueryDsl};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use eyre::{bail, Context};
use lobby::GameId;
use tokio::{
    net::{TcpListener, TcpStream},
    select, spawn,
    time::sleep,
};
use ws_message::WsMessageOut;

use crate::{
    game::Game,
    lobby::{generate_game_name, Lobbies, Lobby},
    websocket_client::WsClient,
    ws_message::WsMessageIn,
};

pub mod data;
pub mod db;
pub mod example_game;
pub mod game;
pub mod lobby;
pub mod phases;
pub mod player;
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

    /// Cron string that defines when to unload inactive games from memory.
    ///
    /// Format: "sec min hour day_of_month month day_of_week year"
    #[clap(long, env = "MEM_GC_CRON")]
    mem_gc_cron: Option<String>,
}

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let opt = Arc::new(Opt::parse());

    color_eyre::install()?;
    pretty_env_logger::init();

    let server = TcpListener::bind((opt.host.as_str(), opt.port))
        .await
        .wrap_err_with(|| format!("Failed to listen on {}:{}", opt.host, opt.port))?;
    log::info!("Listening on {}:{}", opt.host, opt.port);

    let lobbies = Arc::new(Lobbies::default());

    if let Some(mem_gc_cron) = &opt.mem_gc_cron {
        if opt.database_url.is_none() {
            log::warn!("you have specified mem_gc_cron without specifying database_url");
            log::warn!("beware: this means games will be deleted when the cron job fires");
        }

        let schedule =
            Schedule::from_str(mem_gc_cron).wrap_err("failed to parse mem_gc_cron string")?;
        spawn(unload_inactive_games(schedule, Arc::clone(&lobbies)));
    }

    loop {
        // TODO: figure out if this error should really be fatal
        let (stream, from) = server.accept().await.wrap_err("Failed to accept client")?;
        spawn(handle_client(
            Arc::clone(&opt),
            stream,
            from,
            Arc::clone(&lobbies),
        ));
    }
}

pub async fn handle_client(
    opt: Arc<Opt>,
    stream: TcpStream,
    from: SocketAddr,
    lobbies: Arc<Lobbies>,
) {
    async fn inner(
        opt: &Opt,
        stream: TcpStream,
        from: SocketAddr,
        lobbies: Arc<Lobbies>,
    ) -> eyre::Result<()> {
        let mut ws_client = WsClient::accept(stream).await;

        let message = ws_client.receive_message::<WsMessageIn>().await?;

        let (id, lobby) = match message {
            WsMessageIn::NewGame => {
                let id = GameId::random();
                let name = generate_game_name(id);

                if let Some(database_uri) = &opt.database_url {
                    log::info!("persisting new game {id:?} in database");

                    // TODO: use a connection pool
                    let mut db = AsyncPgConnection::establish(database_uri).await?;

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
                let mut list = lobbies.list.write().await;

                if let Some(lobby) = list.get(&id) {
                    (id, Arc::clone(lobby))
                } else if let Some(database_uri) = &opt.database_url {
                    // TODO: use a connection pool
                    let mut db = AsyncPgConnection::establish(database_uri).await?;

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
                    let WsMessageIn::Event(event) = message else {
                        bail!("got unexpected event: {message:?}");
                    };

                    let mut lobby = lobby.write().await;

                    log::debug!("applying event {event:?}");
                    // TODO: propagate errors back over the socket?
                    lobby.game.apply(event.clone());

                    if let Some(database_uri) = &opt.database_url {
                        // TODO: use a connection pool
                        let mut db = AsyncPgConnection::establish(database_uri).await?;

                        log::info!("persisting event for game {id:?}");
                        use schema::game_event::dsl::{game_event};
                        insert_into(game_event)
                            .values(&db::NewGameEvent {
                                game_id: id,
                                event: serde_json::to_value(&event)?,
                            })
                            .execute(&mut db)
                            .await
                            .wrap_err_with(|| format!("error querying game events ({id:?})"))?;
                    }

                    lobby.state_updates.send(lobby.game.current.clone())?;
                }
            }
        }
    }

    if let Err(e) = inner(&opt, stream, from, lobbies).await {
        log::warn!("disconnecting {from} because of error: {e:#}");
    } else {
        log::info!("{from} disconnected");
    }
}

/// Background tasks that periodically goes through all games in memory and unloads the ones with
/// no clients.
pub async fn unload_inactive_games(cron: Schedule, lobbies: Arc<Lobbies>) {
    log::info!("scheduling inactive games gc task");

    loop {
        let Some(next_gc) = cron.upcoming(Local).next() else {
            break;
        };

        log::debug!("next gc at {next_gc}");

        let Ok(sleep_for) = (next_gc - Local::now()).to_std() else {
            break;
        };

        sleep(sleep_for).await;

        log::debug!("unloading inactive games");

        let mut list = lobbies.list.write().await;
        let mut delete_queue = Vec::new();
        for (game_id, lobby) in list.iter() {
            let Ok(lobby) = lobby.try_write() else {
                // if the lock was already held, a client must be connected, and the game must
                // still be active
                continue;
            };

            // each client task must hold a state update receiver.
            if lobby.state_updates.receiver_count() > 0 {
                // if there are no receivers, there can be no clients.
                continue;
            }

            delete_queue.push(*game_id);
        }

        if delete_queue.is_empty() {
            log::debug!("no inactive games to unload");
        } else {
            log::info!("unloading {} inactive games", delete_queue.len());
        }

        for game_id in delete_queue {
            log::debug!("game {game_id:?} looks inactive. unloading it.");
            list.remove(&game_id);
        }
    }

    log::info!("inactive games gc task exiting");
}
