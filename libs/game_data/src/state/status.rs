use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::player_id::PlayerId,
    components::{
        objectives::{Objective, secret::SecretObjective},
        tech::Technology,
    },
};

/// State for the status phase.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StatusPhaseState {
    /// The public objectives each player scored this status phase, empty if they scored none. Only
    /// Civilized Society lets a player score more than one.
    pub scored_public_objectives: HashMap<PlayerId, Vec<Objective>>,
    /// What each player scored, or didn't score, for their private objective slot.
    pub scored_secret_objectives: HashMap<PlayerId, Option<SecretObjective>>,
    /// What objective was revealed during this status phase.
    pub revealed_objective: Option<Objective>,
    /// The faction technology each player gained from an entropic scar at the start of this status
    /// phase.
    pub entropic_scar_techs: HashMap<PlayerId, Technology>,
}

impl StatusPhaseState {
    /// Create a new empty state for a new status phase.
    pub fn new() -> Self {
        Self {
            scored_public_objectives: HashMap::new(),
            scored_secret_objectives: HashMap::new(),
            revealed_objective: None,
            entropic_scar_techs: HashMap::new(),
        }
    }

    /// Has all players scored their objectives (or skipped)?
    pub fn can_reveal_objective(&self, num_players: usize) -> bool {
        self.scored_public_objectives.len() == num_players
            && self.scored_secret_objectives.len() == num_players
    }

    /// Is everything done to go to the next phase?
    pub fn is_complete(&self, num_players: usize) -> bool {
        self.can_reveal_objective(num_players) && self.revealed_objective.is_some()
    }
}
