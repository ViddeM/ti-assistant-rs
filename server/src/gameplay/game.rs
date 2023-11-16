use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::gameplay::game_event_handler::update_game_state;

use super::{event::Event, game_state::GameState, player::Player};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub players: Vec<Player>,
    pub current: Arc<GameState>,
    pub history: Vec<Event>,
}

impl Game {
    /// Apply an event and update the game state.
    ///
    /// If the event is not valid for the current state it is rejected.
    pub fn apply(&mut self, event: Event) {
        log::debug!("{event:?}");
        let state = Arc::make_mut(&mut self.current);
        if let Err(e) = update_game_state(state, event.clone()) {
            log::warn!("event not valid for current state");
            log::warn!("{e}");
            return;
        }

        log::info!("{:#?}", self.current);
        self.history.push(event);
    }

    /// Undo the last event
    pub fn undo(&mut self) {
        self.history.pop();

        let mut state = GameState::default();
        for event in &self.history {
            update_game_state(&mut state, event.clone()).expect("wait... this worked before??");
        }
        self.current = Arc::new(state);
    }
}
