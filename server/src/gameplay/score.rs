use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::data::components::objectives::{secret::SecretObjective, Objective};

use super::player::PlayerId;

/// Everything game state that to player points.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Score {
    /// The amount of points required to win the game.
    pub max_points: i8,

    /// The amount of point that each player has.
    pub player_points: HashMap<PlayerId, i8>,

    /// Map from revealed objectives to the players that have scored them.
    pub revealed_objectives: HashMap<Objective, HashSet<PlayerId>>,

    /// Completed secret objectives, by player.
    pub secret_objectives: HashMap<PlayerId, HashSet<SecretObjective>>,

    /// The player who took the custodian token from Mecatol Rex.
    pub custodians: Option<PlayerId>,

    /// Map from receiver to giver of Support for the Throne.
    pub support_for_the_throne: HashMap<PlayerId, PlayerId>,

    /// Manually assigned points
    pub extra_points: HashMap<PlayerId, i8>,
}

impl Score {
    /// Update [Score::player_points] to the correct values.
    pub fn update_player_points(&mut self, all_players: &[PlayerId]) {
        self.max_points = 10; // TODO set up score

        for player in all_players {
            let mut player_points = 0;

            // Check players revealed objectives
            player_points += self
                .revealed_objectives
                .iter()
                .filter(|(_objectiv, has_scored)| has_scored.contains(player))
                .map(|(objective, _)| objective.get_objective_info().points)
                .next()
                .unwrap_or(0);

            player_points += self
                .secret_objectives
                .get(player)
                .map(|scored| scored.len())
                .unwrap_or(0) as i8;

            // Check if player has the custodians
            player_points += self
                .custodians
                .as_ref()
                .filter(|&owner| owner == player)
                .map(|_| 1)
                .unwrap_or(0);

            // count the number of Support for the Thrones the player has
            player_points += self
                .support_for_the_throne
                .keys()
                .filter(|&owner| owner == player)
                .count() as i8;

            player_points += self.extra_points.get(player).unwrap_or(&0);

            self.player_points.insert(Arc::clone(player), player_points);
        }
    }
}
