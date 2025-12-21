use crate::{
    messages::{WsMessage, WsMessageOut},
    requests::new_game::NewGame,
};
use dioxus::{
    fullstack::{JsonEncoding, WebSocketOptions, Websocket},
    prelude::*,
};
use std::sync::Arc;
use ti_helper_game_data::{actions::event::Event, game_id::GameId};

#[cfg(feature = "server")]
use {
    crate::server_side::{
        lobby::{Lobby, generate_game_name},
        state,
    },
    anyhow::Context,
    chrono::{DateTime, Utc},
    dioxus::{fullstack::TypedWebsocket, server::axum::Extension},
    ti_helper_db::queries,
    ti_helper_game_logic::gameplay::{error::GameError, game::Game},
    tokio::{select, sync::RwLock},
};

/// Echo the user input on the server.
#[post("/api/game", ext: Extension<Arc<state::State>>)]
pub async fn new_game(data: NewGame) -> Result<GameId, ServerFnError> {
    let id = GameId::random();
    let name = generate_game_name(id);

    if let Some(db_pool) = &ext.db_pool {
        log::info!("persisting new game {id:?} in database");
        queries::create_game(db_pool, id.into(), name)
            .await
            .context("failed to create game")?;
    }

    let mut game = Game::default();

    let set_settings_event = data
        .to_new_game_event()
        .await
        .context("Failed to create event from new game")?;

    let now = Utc::now();
    game.apply_or_err(set_settings_event.clone(), now)
        .context("failed to apply event to game")?;

    if let Some(db_pool) = &ext.db_pool {
        log::info!("persisting event for game {id:?}");

        queries::insert_game_event(db_pool, id, serde_json::to_value(&set_settings_event)?, now)
            .await
            .with_context(|| format!("error querying game events ({id:?})"))?;
    }

    let lobby = Lobby::new(game);
    let mut lobbies = ext.lobbies.list.write().await;

    if lobbies.contains_key(&id) {
        return Err(ServerFnError::new(format!("new game id collision: {id:?}")));
    }
    lobbies.insert(id, Arc::clone(&lobby));

    log::info!("created new game {id:?}");

    // TODO: GameID should be shared between client & server but isn't currently so we use a string instead.
    Ok(id)
}

pub type TIWebsocket = Websocket<WsMessage, WsMessageOut, JsonEncoding>;

#[get("/api/game/{game_id}", ext: Extension<Arc<state::State>>)]
pub async fn join_game(
    game_id: GameId,
    options: WebSocketOptions,
) -> Result<TIWebsocket, ServerFnError> {
    Ok(options.on_upgrade(move |mut socket| async move {
        let (game_id, lobby) = match join_game_inner(&mut socket, game_id, &ext).await {
            Ok(info) => info,
            Err(err) => {
                log::error!("Failed to join game: {err:?}");
                return;
            }
        };

        if let Err(err) = run_client(&mut socket, game_id, lobby, &ext).await {
            log::error!("Failed to run client coms: {err:?}");
        }
    }))
}

#[cfg(feature = "server")]
pub type TIWebsocketServer = TypedWebsocket<WsMessage, WsMessageOut, JsonEncoding>;

#[cfg(feature = "server")]
async fn join_game_inner(
    socket: &mut TIWebsocketServer,
    game_id: GameId,
    state: &state::State,
) -> anyhow::Result<(GameId, Arc<RwLock<Lobby>>)> {
    let state::State {
        lobbies,
        db_pool,
        opt: _,
    } = state;

    let mut list = lobbies.list.write().await;

    if let Some(lobby) = list.get(&game_id) {
        Ok((game_id, Arc::clone(lobby)))
    } else if let Some(db_pool) = &db_pool {
        log::info!("Loading game {game_id:?} from DB");

        if let Err(e) = queries::get_game_by_id(db_pool, &game_id)
            .await
            .with_context(|| format!("Failed to retrieve game {game_id:?} from DB"))
        {
            socket
                .send(WsMessageOut::NotFound(game_id))
                .await
                .context("failed to send game not found message to client")?;

            return Err(e);
        }

        let events = queries::get_events_for_game(db_pool, &game_id)
            .await
            .with_context(|| format!("error querying game events ({game_id:?})"))
            .context("failed to retrieve game events from DB")?;

        log::info!("replaying {} events for game {game_id:?}", events.len());

        let mut game = Game::default();

        for record in events {
            let event = serde_json::from_value(record.event.clone())
                .with_context(|| format!("failed to parse event from DB, record: {record:?}"))?;

            game.apply(event, record.timestamp);
        }

        log::info!("loaded game {game_id:?}");

        let lobby = Lobby::new(game);

        list.insert(game_id, Arc::clone(&lobby));

        Ok((game_id, lobby))
    } else {
        socket
            .send(WsMessageOut::not_found(game_id))
            .await
            .context("failed to send game not found message to client")?;
        anyhow::bail!("no lobby with id {game_id:?}");
    }
}

#[cfg(feature = "server")]
async fn run_client(
    socket: &mut TIWebsocketServer,
    game_id: GameId,
    lobby: Arc<RwLock<Lobby>>,
    state: &state::State,
) -> anyhow::Result<()> {
    socket
        .send(WsMessageOut::JoinedGame(game_id))
        .await
        .context("Failed to send joined game message")?;

    socket
        .send(WsMessageOut::game_options(
            &lobby.read().await.game.current.game_settings.expansions,
        ))
        .await
        .context("Failed to send game options")?;

    let mut state_updates = {
        let lobby = lobby.read().await;
        socket
            .send(WsMessageOut::GameState(lobby.game.current.clone()))
            .await
            .context("failed to send game state message to client")?;

        // make sure we subscribe while we are holding the game state lock to avoid silly races
        lobby.state_updates.subscribe()
    };

    loop {
        select! {
            update = state_updates.recv() => {
                log::debug!("Sendng state update to client"); // TODO: Figure out a way to retrieve an addr or smth that we can log here.
                socket.send(WsMessageOut::GameState(update?)).await.context("failed to send game state message to client")?;
            }
            message = socket.recv() => {
                let message = message.context("failed to receive message from client")?;

                match message {
                    WsMessage::Undo => handle_undo(state, game_id, &lobby).await.context("failed to handle undo event")?,
                    WsMessage::Event(event) => {
                        match handle_event(state, game_id, &lobby, event).await {
                            Ok(_) => {},
                            Err(EventError::HandleEventError(e)) => {
                                socket.send(WsMessageOut::event_err(e)).await.context("failed to send error message to client")?;
                            },
                            Err(EventError::InternalError(err)) => return Err(err),
                        }
                    }
                };
            }
        }
    }
}

#[cfg(feature = "server")]
enum EventError {
    HandleEventError(String),
    InternalError(GameError),
}

#[cfg(feature = "server")]
async fn handle_event(
    state: &state::State,
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

    store_and_propagate_event(state, id, event, now, &lobby)
        .await
        .map_err(EventError::InternalError)?;

    Ok(())
}

#[cfg(feature = "server")]
async fn store_and_propagate_event(
    state: &state::State,
    id: GameId,
    event: Event,
    timestamp: DateTime<Utc>,
    lobby: &Lobby,
) -> anyhow::Result<()> {
    if let Some(db_pool) = &state.db_pool {
        log::info!("persisting event for game {id:?}");

        queries::insert_game_event(db_pool, id, serde_json::to_value(&event)?, timestamp)
            .await
            .with_context(|| format!("error querying game events ({id:?})"))?;
    }

    lobby.state_updates.send(lobby.game.current.clone())?;

    Ok(())
}

#[cfg(feature = "server")]
async fn handle_undo(
    state: &state::State,
    id: GameId,
    lobby: &RwLock<Lobby>,
) -> anyhow::Result<()> {
    let mut lobby = lobby.write().await;

    if let Some(db_pool) = &state.db_pool {
        log::info!("undoing last event for game {id:?}");
        queries::delete_latest_event_for_game(db_pool, &id).await?;
    }

    lobby.game.undo();
    lobby.state_updates.send(lobby.game.current.clone())?;

    Ok(())
}
