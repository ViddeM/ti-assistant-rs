use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::{common::player_id::PlayerId, components::system::SystemId};

/// The current state of the Thunder's Edge expedition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TEExpedition {
    pub completed: Option<TECompletion>,
    pub progress: BTreeMap<TEExpeditionSlot, PlayerId>,
}

impl TEExpedition {
    pub fn get_top_expedition_contributors(&self) -> anyhow::Result<Option<Vec<PlayerId>>> {
        if self.progress.len() < 6 {
            return Ok(None);
        }

        let mut slots = HashMap::new();
        for (_, player) in &self.progress {
            *slots.entry(player).or_insert(0) += 1;
        }

        let max_slots = slots.values().cloned().max().unwrap_or(0);
        let top_contributors = slots
            .into_iter()
            .filter(|(_, count)| *count == max_slots)
            .map(|(player, _)| player.clone())
            .collect::<Vec<_>>();

        Ok(Some(top_contributors))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TECompletion {
    pub completed_by: PlayerId,
    pub placed_in: SystemId,
}

/// The progress of the Thunder's Edge expedition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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
