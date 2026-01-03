use std::sync::Arc;

use dioxus::signals::{ReadSignal, ReadableExt};
use ti_helper_game_data::{game_options::GameOptions, state::game_state::GameState};

#[derive(Clone, Copy)]
pub struct GameContext {
    game_state: ReadSignal<Arc<GameState>>,
    game_options: ReadSignal<Arc<GameOptions>>,
}

impl GameContext {
    pub fn new(
        game_state: ReadSignal<Arc<GameState>>,
        game_options: ReadSignal<Arc<GameOptions>>,
    ) -> Self {
        Self {
            game_state,
            game_options,
        }
    }

    pub fn game_state(&self) -> Arc<GameState> {
        Arc::clone(&self.game_state.read())
    }

    pub fn game_options(&self) -> Arc<GameOptions> {
        Arc::clone(&self.game_options.read())
    }
}
