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

    pub fn set_spectator(&self) {
        (self.set_view)(Some(PlayerView::SPECTATOR.to_string()));
    }

    pub fn set_player(&self, player_id: PlayerId) {
        (self.set_view)(Some(player_id.to_string()));
    }

    pub fn get(&self) -> ReadSignal<PlayerView> {
        self.current
    }

    pub fn is_spectator(&self) -> bool {
        matches!(&*self.current.read(), PlayerView::Spectator)
    }

    pub fn is_global_or_speaker(&self) -> bool {
        self.current
            .read()
            .is_global_or_speaker_in(&self.game_state.read())
    }

    pub fn display(&self) -> String {
        self.current.read().to_string()
    }

    pub fn display_for(&self, id: &PlayerId) -> bool {
        self.current.read().shows(id)
    }

    /// Returns true if its either viewed from the active players perspective or the global view.
    pub fn is_active(&self) -> bool {
        self.current.read().is_active_in(&self.game_state.read())
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum PlayerView {
    Global,
    #[strum(to_string = "{player_id}")]
    Player {
        player_id: PlayerId,
    },
    Spectator,
}

impl PlayerView {
    pub const SPECTATOR: &'static str = "Spectator";

    pub fn from_playing_as(playing_as: Option<&str>, state: &GameState) -> Self {
        let Some(name) = playing_as else {
            return PlayerView::Global;
        };
        match state.players.get_key_value(name) {
            Some((player_id, _)) => PlayerView::Player {
                player_id: player_id.clone(),
            },
            None if name == Self::SPECTATOR => PlayerView::Spectator,
            None => PlayerView::Global,
        }
    }

    pub fn shows(&self, id: &PlayerId) -> bool {
        match self {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => player_id == id,
            PlayerView::Spectator => false,
        }
    }

    pub fn is_global_or_speaker_in(&self, state: &GameState) -> bool {
        match self {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => state.is_speaker(player_id),
            PlayerView::Spectator => false,
        }
    }

    pub fn is_active_in(&self, state: &GameState) -> bool {
        match self {
            PlayerView::Global => true,
            PlayerView::Player { player_id } => state.current_player.as_ref() == Some(player_id),
            PlayerView::Spectator => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use ti_helper_game_data::{
        common::{color::Color, faction::Faction, game_settings::Expansions},
        state::player::NewPlayer,
    };

    use super::*;

    fn state_with(names: &[&str]) -> GameState {
        let mut state = GameState::default();
        for name in names {
            let player = NewPlayer {
                name: name.to_string(),
                faction: Faction::Arborec,
                color: Color::Pink,
            }
            .setup(&Expansions::default());
            state.players.insert(PlayerId::from(*name), player);
        }
        state.speaker = Some("Alice".into());
        state.current_player = Some("Alice".into());
        state
    }

    #[test]
    fn spectator_matches_no_player() {
        let state = state_with(&["Alice"]);
        assert!(!PlayerView::Spectator.shows(&"Alice".into()));
        assert!(!PlayerView::Spectator.is_active_in(&state));
        assert!(!PlayerView::Spectator.is_global_or_speaker_in(&state));
    }

    #[test]
    fn global_matches_everyone() {
        let state = state_with(&["Alice"]);
        assert!(PlayerView::Global.shows(&"Alice".into()));
        assert!(PlayerView::Global.is_active_in(&state));
        assert!(PlayerView::Global.is_global_or_speaker_in(&state));
    }

    #[test]
    fn from_playing_as_spectator() {
        let state = state_with(&["Alice"]);
        assert_eq!(
            PlayerView::from_playing_as(Some("Spectator"), &state),
            PlayerView::Spectator
        );
    }

    #[test]
    fn from_playing_as_prefers_a_real_player() {
        let state = state_with(&["Alice", "Spectator"]);
        assert_eq!(
            PlayerView::from_playing_as(Some("Spectator"), &state),
            PlayerView::Player {
                player_id: "Spectator".into()
            }
        );
    }

    #[test]
    fn from_playing_as_unknown_is_global() {
        let state = state_with(&["Alice"]);
        assert_eq!(
            PlayerView::from_playing_as(None, &state),
            PlayerView::Global
        );
        assert_eq!(
            PlayerView::from_playing_as(Some("Bob"), &state),
            PlayerView::Global
        );
    }
}
