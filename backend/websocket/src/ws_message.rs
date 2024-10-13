use std::sync::Arc;

use serde::{Deserialize, Serialize};

use ti_helper_db::game_id::GameId;
use ti_helper_game_data::common::{
    game_settings::{Expansions, GameSettings},
    milty_data::MiltyData,
};
use ti_helper_game_logic::{
    game_options::GameOptions,
    gameplay::{event::Event, game_state::GameState},
};
use ti_helper_milty::MiltyImport;

/// Websocket messages that can be received.
#[derive(Debug, Clone, Deserialize)]
#[allow(missing_docs)]
pub enum WsMessageIn {
    JoinGame(GameId),
    NewGame(NewGame),
    Event(Event),

    /// Undo the most recent [Event].
    Undo,
}

/// information required for a new game.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGame {
    points: u32,
    game_config: GameConfig,
}

impl NewGame {
    /// Creates the appropriate new game event for this [NewGame].
    pub async fn to_new_game_event(&self) -> eyre::Result<Event> {
        Ok(match &self.game_config {
            GameConfig::CustomGameConfig {
                pok,
                cod1,
                cod2,
                cod3,
            } => Event::SetSettings {
                settings: GameSettings {
                    max_points: self.points,
                    expansions: Expansions {
                        prophecy_of_kings: *pok,
                        codex_1: *cod1,
                        codex_2: *cod2,
                        codex_3: *cod3,
                    },
                },
            },
            GameConfig::ImportFromMilty {
                milty_game_id,
                milty_tts_string,
            } => {
                let milty_data =
                    MiltyData::import_from_milty(milty_game_id, milty_tts_string).await?;
                log::debug!("Milty data import {milty_data:?}");
                Event::ImportFromMilty {
                    max_points: self.points,
                    milty_data: Box::new(milty_data),
                }
            }
        })
    }
}

/// The game configuration.
#[derive(Debug, Clone, Deserialize)]
pub enum GameConfig {
    /// Config specified by the user.
    #[serde(rename_all = "camelCase")]
    CustomGameConfig {
        /// If Prophecy of kings is to be used.
        pok: bool,
        /// If Codex I should be used.
        cod1: bool,
        /// If Codex II should be used.
        cod2: bool,
        /// If Codex III should be used.
        cod3: bool,
    },
    /// Game config imported from milty draft.
    #[serde(rename_all = "camelCase")]
    ImportFromMilty {
        /// The game id from milty (from the URL).
        milty_game_id: String,
        /// The milty tts map string.
        milty_tts_string: String,
    },
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

    /// Response to [WsMessageIn::Event] when the event cannot be handled.
    HandleEventError(String),

    /// Response of [WsMessageIn::JoinGame] or a [WsMessageIn::NewGame] with the game id.
    ///
    /// Will be followed by a [WsMessageOut::GameState] message with the latest state of the game.
    JoinedGame(GameId),

    /// Response of [WsMessageIn::JoinGame] when the game doesn't exist.
    NotFound(GameId),
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
