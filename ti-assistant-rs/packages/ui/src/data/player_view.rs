use std::sync::Arc;

use dioxus::prelude::*;
use strum::Display;
use ti_helper_game_data::{common::player_id::PlayerId, state::game_state::GameState};

#[derive(Clone, Copy)]
pub struct PlayerViewContext {
    current: Signal<PlayerView>,
    game_state: ReadSignal<Arc<GameState>>,
}

impl PlayerViewContext {
    pub fn new(game_state: ReadSignal<Arc<GameState>>) -> Self {
        Self {
            current: Signal::new(PlayerView::Global),
            game_state,
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

    pub fn get(&self) -> ReadSignal<PlayerView> {
        self.current.into()
    }

    pub fn display(&self) -> String {
        self.current.read().to_string()
    }

    pub fn display_for(&self, id: PlayerId) -> bool {
        match self.get()() {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => player_id == id,
        }
    }

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

#[derive(Debug, Clone, Display)]
pub enum PlayerView {
    Global,
    #[strum(to_string = "Player: {player_id}")]
    Player {
        player_id: PlayerId,
    },
}
