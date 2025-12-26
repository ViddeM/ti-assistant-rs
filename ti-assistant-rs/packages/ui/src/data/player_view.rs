use dioxus::prelude::*;
use strum::{Display, EnumString};
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

    pub fn get(&self) -> ReadSignal<PlayerView> {
        self.current.into()
    }

    pub fn display(&self) -> String {
        self.current.read().to_string()
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
