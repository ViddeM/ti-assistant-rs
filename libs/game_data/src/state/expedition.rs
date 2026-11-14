use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{common::player_id::PlayerId, components::system::SystemId};

/// The current state of the Thunder's Edge expedition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TEExpedition {
    /// How the expedition was completed, or None if it hasn't been.
    pub completed: Option<TECompletion>,
    /// Who has claimed which slot, and when.
    pub progress: BTreeMap<TEExpeditionSlot, ExpeditionClaim>,
    /// The player whose commit claimed the final slot, and who must place Thunder's Edge before
    /// the turn can end. Slots claimed from the admin page don't set it.
    pub pending_placement: Option<PlayerId>,
}

/// Who claimed an expedition slot, and in which round and turn.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExpeditionClaim {
    pub player: PlayerId,
    pub round: u32,
    pub turn: u32,
    /// Whether the player made the claim with their commit, rather than from the admin page.
    pub committed: bool,
}

impl TEExpedition {
    /// True if all slots are claimed.
    pub fn is_full(&self) -> bool {
        self.progress.len() >= TEExpeditionSlot::iter().count()
    }

    /// True if the final slot was claimed by a commit, but the expedition hasn't been placed yet.
    pub fn is_awaiting_placement(&self) -> bool {
        self.is_full() && self.completed.is_none() && self.pending_placement.is_some()
    }

    /// The player that has claimed each slot.
    pub fn claimed_by(&self) -> BTreeMap<TEExpeditionSlot, PlayerId> {
        (self.progress.iter())
            .map(|(slot, claim)| (slot.clone(), claim.player.clone()))
            .collect()
    }

    /// True if `player` has committed to a slot in the given round and turn.
    pub fn has_committed(&self, player: &PlayerId, round: u32, turn: u32) -> bool {
        (self.progress.values()).any(|claim| {
            claim.committed
                && &claim.player == player
                && claim.round == round
                && claim.turn == turn
        })
    }

    pub fn get_top_expedition_contributors(&self) -> Option<Vec<PlayerId>> {
        if !self.is_full() {
            return None;
        }

        let claimed_by = self.claimed_by();
        let mut slots = HashMap::new();
        for player in claimed_by.values() {
            *slots.entry(player).or_insert(0) += 1;
        }

        let max_slots = slots.values().cloned().max().unwrap_or(0);
        let top_contributors = slots
            .into_iter()
            .filter(|(_, count)| *count == max_slots)
            .map(|(player, _)| player.clone())
            .collect::<Vec<_>>();

        Some(top_contributors)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TECompletion {
    /// The player that won the expedition and received Thunder's Edge.
    pub won_by: PlayerId,
    pub placed_in: SystemId,
}

/// The progress of the Thunder's Edge expedition.
///
/// The variants are declared in the order the slots are laid out (clockwise) on the token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum TEExpeditionSlot {
    /// Spend 5 resources.
    Resources,
    /// Discard 2 action cards.
    ActionCards,
    /// Spend 5 influence.
    Influence,
    /// Discard an unscored secret objective.
    SecretObjective,
    /// Exhaust a tech speciality planet.
    TechSpecialityPlanet,
    /// Spend 3 trade goods.
    TradeGoods,
}

impl TEExpeditionSlot {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Resources => "Spend 5 resources",
            Self::ActionCards => "Discard 2 Action Cards",
            Self::Influence => "Spend 5 influence",
            Self::SecretObjective => "Discard 1 unscored Secret Objective",
            Self::TechSpecialityPlanet => "Exhaust 1 Technology Speciality Planet",
            Self::TradeGoods => "Spend 3 Trade Goods",
        }
    }
}
