use std::sync::Arc;

use serde::{Deserialize, Serialize};

use ti_helper_db::game_id::GameId;
use ti_helper_game::{
    game_options::GameOptions,
    gameplay::{event::Event, game_state::GameState},
};

/// Websocket messages that can be received.
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub enum WsMessageIn {
    JoinGame(GameId),
    NewGame,
    Event(Event),

    /// Undo the most recent [Event].
    Undo,
}

/// Messages that can be sent to a client.
#[derive(Debug, Clone, Serialize)]
#[allow(missing_docs)]
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
    /// Returns a new [WsMessageOut::GameOptions] event.
    pub fn game_options() -> Self {
        Self::GameOptions(Default::default())
    }

    /// Returns a new [WsMessageOut::GameState] event from the provided state.
    pub fn game_state(state: Arc<GameState>) -> Self {
        Self::GameState(state)
    }

    /// Returns a new [WsMessageOut::JoinGame] event from the provided game_id.
    pub fn join_game(game_id: GameId) -> Self {
        Self::JoinedGame(game_id)
    }
}
