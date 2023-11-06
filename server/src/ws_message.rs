use serde::{Deserialize, Serialize};

use crate::{
    game_options::GameOptions,
    gameplay::{event::Event, game_state::GameState},
    lobby::GameId,
};

#[derive(Debug, Clone, Deserialize)]
pub enum WsMessageIn {
    JoinGame(GameId),
    NewGame,
    Event(Event),
}

#[derive(Debug, Clone, Serialize)]
pub enum WsMessageOut {
    GameOptions(GameOptions),
    GameState(GameState),
    JoinedGame(GameId),
}

impl WsMessageOut {
    pub fn game_options() -> Self {
        Self::GameOptions(GameOptions::default())
    }

    pub fn game_state(state: GameState) -> Self {
        Self::GameState(state)
    }

    pub fn join_game(game_id: GameId) -> Self {
        Self::JoinedGame(game_id)
    }
}
