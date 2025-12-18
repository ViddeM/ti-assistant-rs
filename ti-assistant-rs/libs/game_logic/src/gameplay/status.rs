use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::objectives::{secret::SecretObjective, Objective},
};

/// State for the status phase.
#[derive(Clone, Default, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StatusPhaseState {
    /// What each player scored, or didn't score, for their public objective slot.
    pub scored_public_objectives: HashMap<PlayerId, Option<Objective>>,
    /// What each player scored, or didn't score, for their private objective slot.
    pub scored_secret_objectives: HashMap<PlayerId, Option<SecretObjective>>,
    /// What objective was revealed during this status phase.
    pub revealed_objective: Option<Objective>,
    /// The number of objectives expected to have been revealed before we start revealing stage II cards.
    pub expected_objectives_before_stage_two: usize,
}

impl StatusPhaseState {
    /// Create a new empty state for a new status phase.
    pub fn new(expected_objectives_before_stage_two: usize) -> Self {
        Self {
            scored_public_objectives: HashMap::new(),
            scored_secret_objectives: HashMap::new(),
            revealed_objective: None,
            expected_objectives_before_stage_two,
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
