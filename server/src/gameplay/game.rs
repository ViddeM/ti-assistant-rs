use serde::{Deserialize, Serialize};

use crate::gameplay::game_event_handler::update_game_state;

use super::{event::Event, game_state::GameState, player::Player};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub players: Vec<Player>,
    pub current: GameState,
    pub history: Vec<Event>,
}

impl Game {
    /// Apply an event and update the game state.
    ///
    /// If the event is not valid for the current state it is rejected.
    pub fn apply(&mut self, event: Event) {
        log::debug!("{event:?}");
        if let Err(e) = update_game_state(&mut self.current, event.clone()) {
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

        self.current = Default::default();
        for event in &self.history {
            update_game_state(&mut self.current, event.clone())
                .expect("wait... this worked before??");
        }
    }
}
