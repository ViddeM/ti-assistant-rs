use std::sync::Arc;

use dioxus::prelude::*;
use dioxus::server::axum::Extension;
use ti_helper_db::game_id::GameId;
use ti_helper_db::queries;
use ti_helper_game_logic::gameplay::game::Game;

use crate::server_side::lobby::generate_game_name;
use crate::server_side::requests::new_game::NewGame;
use crate::server_side::state::State;

/// Echo the user input on the server.
#[post("/api/game", state: Extension<Arc<State>>)]
pub async fn new_game(
    data: NewGame,
    // Extension(state): Extension<State>,
) -> Result<GameId, ServerFnError> {
    use chrono::Utc;

    use crate::server_side::lobby::Lobby;

    let id = GameId::random();
    let name = generate_game_name(id);

    if let Some(db_pool) = &state.db_pool {
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

    if let Some(db_pool) = &state.db_pool {
        log::info!("persisting event for game {id:?}");

        queries::insert_game_event(db_pool, id, serde_json::to_value(&set_settings_event)?, now)
            .await
            .with_context(|| format!("error querying game events ({id:?})"))?;
    }

    let lobby = Lobby::new(game);
    let mut lobbies = state.lobbies.list.write().await;

    if lobbies.contains_key(&id) {
        return Err(ServerFnError::new(format!("new game id collision: {id:?}")));
    }
    lobbies.insert(id, Arc::clone(&lobby));

    log::info!("created new game {id:?}");
    Ok(id)
}
