use std::sync::Arc;

use serde::{Deserialize, Serialize};

use ti_helper_db::game_id::GameId;
use ti_helper_game::{
    game_options::GameOptions,
    gameplay::{event::Event, game_state::GameState},
};

#[derive(Debug, Clone, Deserialize)]
pub enum WsMessageIn {
    JoinGame(GameId),
    NewGame,
    Event(Event),

    /// Undo the most recent [Event].
    Undo,
}

#[derive(Debug, Clone, Serialize)]
pub enum WsMessageOut {
    /// Initial message sent by the server that contains all general info about game components
    /// that the frontend will need.
    GameOptions(Arc<GameOptions>),

    /// An update of the current game state.
    GameState(Arc<GameState>),

    /// Response of [WsMessageIn::JoinGame] or a [WsMessageIn::NewGame] with the game id.
    ///
    /// Will be followed by a [WsMessageOut::GameState] message with the latest state of the game.
    JoinedGame(GameId),
}

impl WsMessageOut {
    pub fn game_options() -> Self {
        Self::GameOptions(Default::default())
    }

    pub fn game_state(state: Arc<GameState>) -> Self {
        Self::GameState(state)
    }

    pub fn join_game(game_id: GameId) -> Self {
        Self::JoinedGame(game_id)
    }
}