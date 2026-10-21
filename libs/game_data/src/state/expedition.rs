use serde::{Deserialize, Serialize};

use crate::{common::player_id::PlayerId, components::system::SystemId};

/// The current state of the Thunder's Edge expedition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TEExpedition {
    pub completed: Option<TECompletion>,
    pub progress: TEExpeditionProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TECompletion {
    pub completed_by: PlayerId,
    pub placed_in: SystemId,
}

/// The progress of the Thunder's Edge expedition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TEExpeditionProgress {
    /// The player (if any) that spent 5 resources.
    pub resources: Option<PlayerId>,
    /// The player (if any) that discarded 2 action cards.
    pub action_cards: Option<PlayerId>,
    /// The player (if any) that spent 5 influence.
    pub influence: Option<PlayerId>,
    /// The player (if any) that discarded an unscored secret objective.
    pub secret_objective: Option<PlayerId>,
    /// The player (if any) that exhausted a tech speciality planet.
    pub tech_speciality_planet: Option<PlayerId>,
    /// The player (if any) that spent 3 trade goods.
    pub trade_goods: Option<PlayerId>,
}
