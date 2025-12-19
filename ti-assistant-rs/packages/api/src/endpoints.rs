use std::sync::Arc;

use dioxus::prelude::*;

use crate::requests::new_game::NewGame;

#[cfg(feature = "server")]
use {
    crate::server_side::{
        lobby::{Lobby, generate_game_name},
        state,
    },
    anyhow::Context,
    chrono::Utc,
    dioxus::server::axum::Extension,
    ti_helper_db::game_id::GameId,
    ti_helper_db::queries,
    ti_helper_game_logic::gameplay::game::Game,
};

/// Echo the user input on the server.
#[post("/api/game", ext: Extension<state::State>)]
pub async fn new_game(data: NewGame) -> Result<String, ServerFnError> {
    let id = GameId::random();
    let name = generate_game_name(id);

    if let Some(db_pool) = &ext.db_pool {
        log::info!("persisting new game {id:?} in database");
        queries::create_game(db_pool, id, name)
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
    Ok(id.to_string())
}
