use std::sync::Arc;

use dioxus::prelude::*;
use strum::Display;
use ti_helper_game_data::{common::player_id::PlayerId, state::game_state::GameState};

#[derive(Clone, Copy)]
pub struct PlayerViewContext {
    current: ReadSignal<PlayerView>,
    game_state: ReadSignal<Arc<GameState>>,
    set_view: Callback<Option<String>>,
}

impl PlayerViewContext {
    pub fn new(
        game_state: ReadSignal<Arc<GameState>>,
        current: ReadSignal<PlayerView>,
        set_view: Callback<Option<String>>,
    ) -> Self {
        Self {
            current,
            game_state,
            set_view,
        }
    }

    pub fn set_global(&self) {
        (self.set_view)(None);
    }

    pub fn set_player(&self, player_id: PlayerId) {
        (self.set_view)(Some(player_id.to_string()));
    }

    pub fn get(&self) -> ReadSignal<PlayerView> {
        self.current
    }

    pub fn is_global_or_speaker(&self) -> bool {
        match (self.current)() {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => (self.game_state)().is_speaker(&player_id),
        }
    }

    pub fn display(&self) -> String {
        self.current.read().to_string()
    }

    pub fn display_for(&self, id: &PlayerId) -> bool {
        match self.get()() {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => &player_id == id,
        }
    }

    /// Returns true if its either viewed from the active players perspective or the global view.
    pub fn is_active(&self) -> bool {
        match &*self.current.read() {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => self
                .game_state
                .read()
                .current_player
                .as_ref()
                .eq(&Some(player_id)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PlayerView {
    Global,
    #[strum(to_string = "{player_id}")]
    Player {
        player_id: PlayerId,
    },
}
