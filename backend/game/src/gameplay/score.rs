use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    data::components::objectives::{secret::SecretObjective, Objective},
    enum_map::EnumMap,
};

use super::player::PlayerId;

/// Everything game state that to player points.
#[derive(Clone, Default, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Score {
    /// The amount of points required to win the game.
    pub max_points: i8,

    /// The amount of point that each player has.
    pub player_points: HashMap<PlayerId, i8>,

    /// Map from revealed objectives to the players that have scored them.
    pub revealed_objectives: EnumMap<Objective, HashSet<PlayerId>>,

    /// Completed secret objectives, by player.
    pub secret_objectives: HashMap<PlayerId, HashSet<SecretObjective>>,

    /// Map from giver to receiver of Support for the Throne.
    pub support_for_the_throne: HashMap<PlayerId, PlayerId>,

    /// Which (if any) player has the Shard of the Throne relic.
    pub shard_of_the_throne: Option<PlayerId>,

    /// Which (if any) player has played the Crown of Emphidia relic.
    pub crown_of_emphidia: Option<PlayerId>,

    /// Manually assigned points
    pub extra_points: HashMap<PlayerId, i8>,

    /// Points gained by playing the Imperial strategy card action while holding Mecatol Rex.
    pub imperial: HashMap<PlayerId, i8>,

    /// The player who took the custodians token from Mecatol Rex.
    pub custodians: Option<PlayerId>,
}

impl Score {
    /// Update [Score::player_points] to the correct values.
    pub fn update_player_points(&mut self, all_players: &[PlayerId]) {
        self.max_points = 10; // TODO set up score

        for player in all_players {
            let mut player_points = 0;

            // Check players completed public objectives
            player_points += self
                .revealed_objectives
                .iter()
                .filter(|(_objective, has_scored)| has_scored.contains(player))
                .map(|(objective, _)| objective.info().points)
                .sum::<i8>();

            // Check players completed secret objectives
            player_points += self
                .secret_objectives
                .get(player)
                .map(|scored| scored.len())
                .unwrap_or(0) as i8;

            // Count the number of Support for the Thrones the player has
            player_points += self
                .support_for_the_throne
                .values()
                .filter(|&owner| owner == player)
                .count() as i8;

            // Manually assigned points modifier
            player_points += self.extra_points.get(player).unwrap_or(&0);

            // Points gained from playing Imperial
            player_points += self.imperial.get(player).unwrap_or(&0);

            // Check if player has the custodians
            player_points += i8::from(self.custodians.as_ref() == Some(player));

            // Check if the player has the Shard of the Throne relic
            if Some(player) == self.shard_of_the_throne.as_ref() {
                player_points += 1;
            }

            // Check if the player has the crown of emphidia relic
            if Some(player) == self.crown_of_emphidia.as_ref() {
                player_points += 1;
            }

            self.player_points.insert(Arc::clone(player), player_points);
        }
    }

    /// Get the number of scored objectives for this player (not the points).
    pub fn scored_objectives_count(&self, player: &PlayerId) -> usize {
        self.scored_public_objectives_count(player) + self.scored_secret_objectives_count(player)
    }

    /// Get the number of scored public objectives for this player (not the points).
    pub fn scored_public_objectives_count(&self, player: &PlayerId) -> usize {
        self.revealed_objectives
            .iter()
            .filter(|(_objective, has_scored)| has_scored.contains(player))
            .count()
    }

    /// Get the number of scored secret objectives for this player (not the points).
    pub fn scored_secret_objectives_count(&self, player: &PlayerId) -> usize {
        self.secret_objectives
            .get(player)
            .map(|secrets| secrets.len())
            .unwrap_or(0)
    }
}
