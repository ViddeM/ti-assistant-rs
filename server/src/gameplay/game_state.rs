use std::collections::{HashMap, HashSet};

use eyre::{bail, eyre};
use serde::{Deserialize, Serialize};

use crate::data::components::{
    phase::Phase, planet::Planet, strategy_card::StrategyCard, system::SystemId,
};

use super::{
    error::GameError,
    player::{Player, PlayerId},
};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    /// The current phase of the game.
    pub phase: Phase,

    /// Which players are in the game.
    pub players: HashMap<PlayerId, Player>,

    /// The order that players are sitting around the table, starting with the speaker.
    pub table_order: Vec<PlayerId>,

    /// The current turn order, either based on table order or strategy cards (initiative).
    pub turn_order: Vec<PlayerId>,

    /// Which players hold which strategy cards.
    pub strategy_card_holders: HashMap<StrategyCard, PlayerId>,

    /// The current player, if any.
    pub current_player: Option<PlayerId>,

    /// Which strategy cards have been spent this action phase.
    pub spent_strategy_cards: HashSet<StrategyCard>,

    /// Which players have passed this phase.
    pub passed_players: HashSet<PlayerId>,

    /// Tracks progress of a strategy card action.
    pub action_progress: Option<ActionPhaseProgress>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionPhaseProgress {
    #[serde(rename_all = "camelCase")]
    Strategic {
        card: StrategyCard,
        other_players: HashMap<PlayerId, bool>,
    },
    Tactical(TacticalProgress),
}

impl ActionPhaseProgress {
    pub fn is_strategy_card(&self) -> bool {
        matches!(self, ActionPhaseProgress::Strategic { .. })
    }

    pub fn is_tactical(&self) -> bool {
        matches!(self, ActionPhaseProgress::Tactical { .. })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TacticalProgress {
    pub activated_system: Option<SystemId>, // TODO: Maybe in the future we should track systems for all tactical actions (Could use some cool interactive map :eyes:)
    pub taken_planets: Vec<Planet>,
}

impl TacticalProgress {
    /// Returns true if the provided system is the currently activated system or there isn't an activated system.
    fn system_is_activated_or_none(&self, system_id: &SystemId) -> bool {
        match self.activated_system.as_ref() {
            Some(id) => id == system_id,
            None => true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyCardProgress {
    pub card: StrategyCard,
    pub other_players: HashMap<PlayerId, bool>,
}

impl GameState {
    pub fn calculate_action_turn_order(&mut self) -> Result<(), GameError> {
        self.turn_order = self.table_order.clone();

        // sort players by the smallest number of the strategy cards they hold (initiative order)
        self.turn_order.sort_by_key(|player| {
            self.strategy_card_holders
                .iter()
                .filter(|&(_, holder)| holder == player)
                .map(|(card, _)| card.card_number())
                .min()
        });

        Ok(())
    }

    /// Advance to the next players turn, if all players have passed, advance to the next phase.
    pub fn advance_turn(&mut self) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        let next_player = self.next_player_after(current_player)?;

        // everyone has passed, i guess. what do?
        if next_player.is_none() {
            match self.phase {
                Phase::Action => {
                    self.calculate_action_turn_order()?;
                    self.passed_players.clear();
                    self.phase = Phase::Status;
                }
                _ => bail!("wtf"),
            }
        } else {
            self.current_player = next_player;
        }

        Ok(())
    }

    pub fn next_player_after(&self, after: &PlayerId) -> Result<Option<PlayerId>, GameError> {
        let next_player = self
            .turn_order
            .iter()
            .skip_while(|&player| player != after)
            .skip(1)
            .chain(self.turn_order.iter())
            .find(|&player| !self.passed_players.contains(player))
            .cloned();

        Ok(next_player)
    }

    pub fn current_player(&self) -> Result<&PlayerId, GameError> {
        self.current_player
            .as_ref()
            .ok_or_else(|| eyre!("no active player"))
    }

    pub fn assert_player_turn(&self, player: &PlayerId) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        if current_player != player {
            bail!("wrong players turn, expected {player:?}, got {current_player:?}");
        }

        Ok(())
    }

    pub fn assert_phase(&self, phase: Phase) -> Result<(), GameError> {
        if self.phase != phase {
            bail!(
                "invalid game state, expected {phase:?}, was {:?}",
                self.phase
            );
        }
        Ok(())
    }

    pub fn get_current_player(&mut self) -> Result<&mut Player, GameError> {
        let current_player_id = match self.current_player.as_ref() {
            Some(p) => p,
            None => bail!("invalid game state, expected there to be a player"),
        };

        let current_player = match self.players.get_mut(current_player_id) {
            Some(p) => p,
            None => bail!("invalid game state, expected current player (id: {current_player_id:?}) to be in the players map")
        };

        Ok(current_player)
    }
}