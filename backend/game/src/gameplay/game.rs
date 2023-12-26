use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::gameplay::game_event_handler::update_game_state;

use super::{event::Event, game_state::GameState, player::Player};

/// A game.
#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    /// The players in the game.
    pub players: Vec<Player>,
    /// The current game state.
    pub current: Arc<GameState>,
    /// What events have occurred and when this far in the game.
    pub history: Vec<(Event, DateTime<Utc>)>,
}

impl Game {
    /// Apply an event and update the game state.
    ///
    /// If the event is not valid for the current state it is rejected.
    pub fn apply(&mut self, event: Event, timestamp: DateTime<Utc>) {
        log::debug!("{event:?}");
        let state = Arc::make_mut(&mut self.current);
        if let Err(e) = update_game_state(state, event.clone(), timestamp) {
            log::warn!("event not valid for current state");
            log::warn!("{e}");
            return;
        }

        log::info!("{:#?}", self.current);
        self.history.push((event, timestamp));
    }

    /// Undo the last event
    pub fn undo(&mut self) {
        self.history.pop();

        let mut state = GameState::default();
        for (event, timestamp) in &self.history {
            update_game_state(&mut state, event.clone(), *timestamp)
                .expect("wait... this worked before??");
        }
        self.current = Arc::new(state);
    }
}
