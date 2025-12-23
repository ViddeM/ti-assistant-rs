use std::fmt::Display;

use dioxus::prelude::*;
use ti_helper_game_data::common::player_id::PlayerId;

#[derive(Clone, Copy)]
pub struct PlayerViewContext {
    current: Signal<PlayerView>,
}

impl PlayerViewContext {
    pub fn new() -> Self {
        Self {
            current: Signal::new(PlayerView::Global),
        }
    }

    pub fn set_global(&mut self) {
        self.current.set(PlayerView::Global);
    }

    pub fn set_player(&mut self, player_id: PlayerId) {
        self.current.set(PlayerView::Player {
            player_id: player_id,
        })
    }

    pub fn display(&self) -> String {
        self.current.read().to_string()
    }
}

#[derive(Debug, Clone)]
pub enum PlayerView {
    Global,
    Player { player_id: PlayerId },
}

impl Display for PlayerView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlayerView::Global => "Global".to_string(),
                PlayerView::Player { player_id } => format!("Player: {player_id}"),
            },
        )
    }
}
