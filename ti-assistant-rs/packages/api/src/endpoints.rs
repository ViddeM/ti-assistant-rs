use std::sync::Arc;

#[cfg(feature = "server")]
use dioxus::fullstack::TypedWebsocket;
use dioxus::{
    fullstack::{JsonEncoding, WebSocketOptions, Websocket},
    prelude::*,
};
#[cfg(feature = "server")]
use tokio::sync::RwLock;

use crate::{
    messages::{WsMessage, WsMessageOut},
    requests::new_game::NewGame,
};

use ti_helper_game_data::game_id::GameId;

#[cfg(feature = "server")]
use {
    crate::server_side::{
        lobby::{Lobby, generate_game_name},
        state,
    },
    anyhow::Context,
    chrono::Utc,
    dioxus::server::axum::Extension,
    ti_helper_db::queries,
    ti_helper_game_logic::gameplay::game::Game,
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

#[post("/api/game/{game_id}", ext: Extension<Arc<state::State>>)]
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

        if let Err(err) = run_client(&mut socket, game_id, lobby).await {
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
        opt,
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
                .with_context(|| format!("faield to parse event from DB, record: {record:?}"))?;

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
) -> anyhow::Result<()> {
    socket
        .send(WsMessageOut::JoinedGame(game_id))
        .await
        .context("Failed to send joined game message")?;

    Ok(())
}
