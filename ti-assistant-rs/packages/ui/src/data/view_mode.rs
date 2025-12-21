use std::fmt::Display;

use dioxus::prelude::*;
use ti_helper_game_data::common::player_id::PlayerId;

#[derive(Clone, Copy)]
pub struct ViewModeContext {
    current: Signal<ViewMode>,
}

impl ViewModeContext {
    pub fn new() -> Self {
        Self {
            current: Signal::new(ViewMode::Global),
        }
    }

    pub fn set_global(&mut self) {
        self.current.set(ViewMode::Global);
    }

    pub fn set_player(&mut self, player_id: PlayerId) {
        self.current.set(ViewMode::Player {
            player_id: player_id,
        })
    }

    pub fn display(&self) -> String {
        self.current.read().to_string()
    }
}

#[derive(Debug, Clone)]
pub enum ViewMode {
    Global,
    Player { player_id: PlayerId },
}

impl Display for ViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ViewMode::Global => "Global".to_string(),
                ViewMode::Player { player_id } => format!("Player: {player_id}"),
            },
        )
    }
}
