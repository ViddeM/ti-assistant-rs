use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ti_helper_game_data::{
    actions::event::Event,
    common::game_settings::Expansions,
    game_id::GameId,
    game_options::GameOptions,
    state::{game_state::GameState, time_stats::TimeStats},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum WsMessageIn {
    Event(Event),
    Undo((Event, DateTime<Utc>)),
    RequestHistory,
    RequestTimeStats,
}

/// Messages that can be sent to a client.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum WsMessageOut {
    /// Initial message sent by the server that contains all general info about game components
    /// that the frontend will need.
    // TODO: do we still need this when we have a rust frontend? Or should we just build it right into the binary? (Its still useful as game content filtered based on the currently enabled settings, expansions, etc)
    GameOptions(Arc<GameOptions>),

    /// An update of the current game state.
    GameState(Arc<GameState>),

    /// Response to [WsMessageIn::Event] when the event cannot be handled.
    HandleEventError(String),

    /// Response of [WsMessageIn::JoinGame] or a [WsMessageIn::NewGame] with the game id.
    ///
    /// Will be followed by a [WsMessageOut::GameState] message with the latest state of the game.
    JoinedGame(GameId),

    /// Response of [WsMessageIn::JoinGame] when the game doesn't exist.
    NotFound(GameId),

    /// Response to [WsMessageIn::Undo] when the event is no longer the last one.
    UndoRejected,

    /// Response to [WsMessageIn::RequestHistory] with every event of the game and when it occurred.
    History(Arc<Vec<(Event, DateTime<Utc>)>>),

    /// Response to [WsMessageIn::RequestTimeStats] with how time has been spent over the game.
    TimeStats(Arc<TimeStats>),
}

impl WsMessageOut {
    /// Returns a new [WsMessageOut::GameOptions] event.
    pub fn game_options(expansions: &Expansions) -> Self {
        Self::GameOptions(Arc::new(GameOptions::new(expansions)))
    }

    /// Returns a new [WsMessageOut::GameState] event from the provided state.
    pub fn game_state(state: Arc<GameState>) -> Self {
        Self::GameState(state)
    }

    /// Returns a new [WsMessageOut::JoinedGame] event from the provided game_id.
    pub fn join_game(game_id: GameId) -> Self {
        Self::JoinedGame(game_id)
    }

    /// Returns a new [WsMessageOut::NotFound] event from the provided game_id.
    pub fn not_found(game_id: GameId) -> Self {
        Self::NotFound(game_id)
    }

    /// Returns a new [WsMessageOut::HandleEventError] from the provided error.
    pub fn event_err(error: String) -> Self {
        Self::HandleEventError(error)
    }
}
