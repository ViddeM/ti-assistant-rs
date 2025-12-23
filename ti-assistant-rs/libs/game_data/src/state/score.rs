use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::{
    common::player_id::PlayerId,
    components::{
        agenda::{Agenda, AgendaElect, ForOrAgainst},
        objectives::{Objective, secret::SecretObjective},
        planet::Planet,
    },
    enum_map::EnumMap,
};

use super::{agenda::AgendaRecord, player::Player};

/// Everything game state that to player points.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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

    /// Agendas that provide points.
    pub agenda_scores: Vec<ScorableAgenda>,

    /// The player who took the custodians token from Mecatol Rex.
    pub custodians: Option<PlayerId>,
}

impl Score {
    /// Update [Score::player_points] to the correct values.
    pub fn update_player_points(&mut self, all_players: &HashMap<PlayerId, Player>) {
        self.max_points = 10; // TODO set up score

        for (player_id, player) in all_players {
            let mut player_points = 0;

            // Check players completed public objectives
            player_points += self
                .revealed_objectives
                .iter()
                .filter(|(_objective, has_scored)| has_scored.contains(player_id))
                .map(|(objective, _)| objective.info().points)
                .sum::<i8>();

            // Check players completed secret objectives
            player_points += self
                .secret_objectives
                .get(player_id)
                .map(|scored| scored.len())
                .unwrap_or(0) as i8;

            // Count the number of Support for the Thrones the player has
            player_points += self
                .support_for_the_throne
                .values()
                .filter(|&owner| owner == player_id)
                .count() as i8;

            // Manually assigned points modifier
            player_points += self.extra_points.get(player_id).unwrap_or(&0);

            // Points gained from playing Imperial
            player_points += self.imperial.get(player_id).unwrap_or(&0);

            // Check if player has the custodians
            player_points += i8::from(self.custodians.as_ref() == Some(player_id));

            // Check if the player has the Shard of the Throne relic
            if Some(player_id) == self.shard_of_the_throne.as_ref() {
                player_points += 1;
            }

            // Check if the player has the crown of emphidia relic
            if Some(player_id) == self.crown_of_emphidia.as_ref() {
                player_points += 1;
            }

            player_points += self
                .agenda_scores
                .iter()
                .map(|score| score.get_score_for_player(player_id, &player))
                .sum::<i8>();

            // TODO: player_points could in theory be negative here, is that allowed or should we reset it to 0 in that case?

            self.player_points
                .insert(Arc::clone(player_id), player_points);
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

    /// If applicable, add the provided agenda record to the scoring table.
    pub fn add_agenda_record(&mut self, agenda_record: &AgendaRecord) {
        let Some(outcome) = agenda_record.outcome.as_ref() else {
            // Agenda was discarded.
            return;
        };

        // Assumes that the outcome is valid for the agenda (that should be handled elsewhere).
        let scorable_agenda = match (agenda_record.vote.agenda, outcome) {
            (Agenda::HolyPlanetOfIxth, AgendaElect::CulturalPlanet(planet)) => {
                ScorableAgenda::HolyPlanetOfIxth {
                    planet: planet.clone(),
                }
            }
            (Agenda::ShardOfTheThrone, AgendaElect::Player(player)) => {
                ScorableAgenda::ShardOfTheThrone {
                    player: player.clone(),
                }
            }
            (Agenda::TheCrownOfEmphidia, AgendaElect::Player(player)) => {
                ScorableAgenda::TheCrownOfEmphidia {
                    player: player.clone(),
                }
            }
            (Agenda::Mutiny, AgendaElect::ForOrAgainst(for_or_against)) => ScorableAgenda::Mutiny {
                players_that_voted_for: agenda_record
                    .vote
                    .player_votes
                    .iter()
                    .filter(|(_, vote)| {
                        if let Some(AgendaElect::ForOrAgainst(ForOrAgainst::For)) =
                            vote.as_ref().map(|v| v.get_outcome())
                        {
                            true
                        } else {
                            false
                        }
                    })
                    .map(|(p, _)| p.clone())
                    .collect(),
                for_won: for_or_against == &ForOrAgainst::For,
            },
            (Agenda::SeedOfAnEmpire, AgendaElect::ForOrAgainst(for_or_against)) => {
                let (players, _) = match for_or_against {
                    ForOrAgainst::For => {
                        self.player_points.iter().map(|(p, s)| (p.clone(), s)).fold(
                            (vec![], i8::MIN),
                            |(mut players_with_most_point, most_points), (player, score)| {
                                if score > &most_points {
                                    (vec![player], score.clone())
                                } else if score == &most_points {
                                    players_with_most_point.push(player);
                                    (players_with_most_point, most_points)
                                } else {
                                    (players_with_most_point, most_points)
                                }
                            },
                        )
                    }
                    ForOrAgainst::Against => {
                        self.player_points.iter().map(|(p, s)| (p.clone(), s)).fold(
                            (vec![], i8::MAX),
                            |(mut players_with_least_points, least_points), (player, score)| {
                                if score < &least_points {
                                    (vec![player], score.clone())
                                } else if score == &least_points {
                                    players_with_least_points.push(player);
                                    (players_with_least_points, least_points)
                                } else {
                                    (players_with_least_points, least_points)
                                }
                            },
                        )
                    }
                };
                ScorableAgenda::SeedOfAnEmpire {
                    players_elected: players.clone(),
                }
            }
            (Agenda::PoliticalCensure, AgendaElect::Player(player)) => {
                ScorableAgenda::PoliticalCensure {
                    player: player.clone(),
                }
            }

            _ => return,
        };

        self.agenda_scores.push(scorable_agenda);
    }

    /// Handle that the particular agenda is repealed, only has an effect if it is a scorable agenda.
    pub fn handle_law_repealed(&mut self, agenda: &Agenda) {
        let Some((index, _)) = self
            .agenda_scores
            .iter()
            .enumerate()
            .find(|(_, a)| &a.get_agenda() == agenda)
        else {
            return;
        };

        self.agenda_scores.remove(index);
    }
}

/// Agendas that provide points and to whom.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "electableAgendaKind", content = "value")]
pub enum ScorableAgenda {
    /// The owner of this planet has a point.
    HolyPlanetOfIxth {
        /// The planet in question.
        planet: Planet,
    },
    /// The player has a point but it can be lost if someone wins a combat against them.
    ShardOfTheThrone {
        /// The player that currently has the card.
        player: PlayerId,
    },
    /// The player has a point but it can be lost if someone takes a planet in their home system.
    TheCrownOfEmphidia {
        /// The player that currently has the card.
        player: PlayerId,
    },
    /// The players that voted for either gains or loses a point depending on whether for won or not.
    #[serde(rename_all = "camelCase")]
    Mutiny {
        /// The players that voted for this agenda.
        players_that_voted_for: Vec<PlayerId>,
        /// Whether the alternative 'for' won or not.
        for_won: bool,
    },
    /// Either the players with the most points gets a point or the players with the fewest.
    #[serde(rename_all = "camelCase")]
    SeedOfAnEmpire {
        /// The players that got a point.
        players_elected: Vec<PlayerId>,
    },
    /// The player gets a point but if they lose this card (i.e. this law is discarded or smth) they lose that point.
    PoliticalCensure {
        /// The player in question.
        player: PlayerId,
    },
}

impl ScorableAgenda {
    fn get_score_for_player(&self, player_id: &PlayerId, player: &Player) -> i8 {
        match self {
            ScorableAgenda::HolyPlanetOfIxth { planet } => {
                if player.planets.contains_key(planet) {
                    return 1;
                }
            }
            ScorableAgenda::ShardOfTheThrone { player } => {
                if player == player_id {
                    return 1;
                }
            }
            ScorableAgenda::TheCrownOfEmphidia { player } => {
                if player == player_id {
                    return 1;
                }
            }
            ScorableAgenda::Mutiny {
                players_that_voted_for,
                for_won: true,
            } => {
                if players_that_voted_for.contains(player_id) {
                    return 1;
                }
            }
            ScorableAgenda::Mutiny {
                players_that_voted_for,
                for_won: false,
            } => {
                if players_that_voted_for.contains(player_id) {
                    return -1;
                }
            }
            ScorableAgenda::SeedOfAnEmpire { players_elected } => {
                if players_elected.contains(player_id) {
                    return 1;
                }
            }
            ScorableAgenda::PoliticalCensure { player } => {
                if player == player_id {
                    return 1;
                }
            }
        }

        0
    }

    fn get_agenda(&self) -> Agenda {
        match self {
            ScorableAgenda::HolyPlanetOfIxth { .. } => Agenda::HolyPlanetOfIxth,
            ScorableAgenda::ShardOfTheThrone { .. } => Agenda::ShardOfTheThrone,
            ScorableAgenda::TheCrownOfEmphidia { .. } => Agenda::TheCrownOfEmphidia,
            ScorableAgenda::Mutiny { .. } => Agenda::Mutiny,
            ScorableAgenda::SeedOfAnEmpire { .. } => Agenda::SeedOfAnEmpire,
            ScorableAgenda::PoliticalCensure { .. } => Agenda::PoliticalCensure,
        }
    }
}
