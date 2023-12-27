use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
};

use eyre::bail;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::data::components::{
    agenda::{Agenda, AgendaElect, AgendaElectKind, AgendaKind, ForOrAgainst},
    planet::{Planet, PlanetTrait},
    strategy_card::StrategyCard,
};

use super::{game_state::GameState, player::PlayerId};

/// State for the agenda phase.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgendaState {
    /// Round number (e.g.. 1 or 2)
    pub round: AgendaRound,

    /// State of the current agenda vote. This is `None` until an agenda is revealed.
    pub vote: Option<VoteState>,
}

/// Agenda phase rounds.
#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AgendaRound {
    /// First round of the Agenda phase.
    #[default]
    Round1,

    /// Second round of the Agenda phase.
    Round2,

    /// The agenda phase is completed.
    Completed,
}

impl AgendaRound {
    /// Get the next [AgendaStage].
    pub fn next(&self) -> AgendaRound {
        match self {
            AgendaRound::Round1 => AgendaRound::Round2,
            _ => AgendaRound::Completed,
        }
    }
}

/// State of an agenda vote.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteState {
    /// The revealed agenda.
    pub agenda: Agenda,

    /// The kind of revealed agenda.
    pub kind: AgendaKind,

    /// The elect kind of the revealed agenda.
    pub elect: AgendaElectKind,

    /// All the possible things you can vote for.
    pub candidates: Vec<AgendaElect>,

    /// Player-cast votes.
    pub player_votes: HashMap<PlayerId, (u16, AgendaElect)>,

    /// Votes tallied on a per-outcome basis.
    ///
    /// Calculated by calling [VoteState::tally_votes].
    pub outcomes_by_votes: Vec<(u16, AgendaElect)>,

    /// The outcome of the vote, if it were to end.
    ///
    /// If the expected outcome can't be determined (i.e. in case of a tie), this is `None`.
    /// Calculated by calling [VoteState::tally_votes].
    pub expected_outcome: Option<AgendaElect>,
}

impl VoteState {
    /// Create the default [VoteState] for the given [Agenda].
    ///
    /// Returns an error if the agenda has no eligible targets for the vote.
    pub fn new(agenda: Agenda, game: &GameState) -> eyre::Result<Self> {
        let info = agenda.info();

        // list eligible planets for the vote, given the provided trait filter
        let planets = |trait_filter: fn(Option<PlanetTrait>) -> bool| {
            let eligible_planets: Vec<_> = Planet::iter()
                .filter(|planet| trait_filter(planet.planet_info().planet_trait))
                // planet must be owned by a player
                .filter(|planet| {
                    game.players
                        .values()
                        .any(|player| player.planets.contains(planet))
                })
                .map(AgendaElect::Planet)
                .collect();

            if eligible_planets.is_empty() {
                bail!("no eligible planets");
            } else {
                Ok(eligible_planets)
            }
        };

        let candidates = match info.elect {
            AgendaElectKind::ForOrAgainst => vec![
                AgendaElect::ForOrAgainst(ForOrAgainst::For),
                AgendaElect::ForOrAgainst(ForOrAgainst::Against),
            ],
            AgendaElectKind::Player => game
                .players
                .keys()
                .cloned()
                .map(AgendaElect::Player)
                .collect(),
            AgendaElectKind::StrategyCard => StrategyCard::iter()
                .map(AgendaElect::StrategyCard)
                .collect(),
            AgendaElectKind::SecretObjective => {
                let eligible_objectives: Vec<_> = game
                    .score
                    .secret_objectives
                    .values()
                    .flatten()
                    .copied()
                    .map(AgendaElect::SecretObjective)
                    .collect();

                if eligible_objectives.is_empty() {
                    bail!("no scored secret objectives");
                }

                eligible_objectives
            }
            AgendaElectKind::Planet => planets(|_| true)?,
            AgendaElectKind::PlanetWithTrait => planets(|t| t.is_some())?,
            AgendaElectKind::CulturalPlanet => planets(|t| t == Some(PlanetTrait::Cultural))?,
            AgendaElectKind::HazardousPlanet => planets(|t| t == Some(PlanetTrait::Hazardous))?,
            AgendaElectKind::IndustrialPlanet => planets(|t| t == Some(PlanetTrait::Industrial))?,

            // TODO: implement this after we implement law tracking
            AgendaElectKind::Law => bail!("no laws in play"),
        };

        Ok(VoteState {
            agenda,
            kind: info.kind,
            elect: info.elect,
            candidates,
            player_votes: Default::default(),
            outcomes_by_votes: Default::default(),
            expected_outcome: None,
        })
    }

    /// Compute [VoteState::outcome_by_votes] and [VoteState::expected_outcome].
    pub fn tally_votes(&mut self) {
        let votes_by_outcome: BTreeMap<AgendaElect, u16> = self
            .player_votes
            .values()
            .cloned()
            .fold(BTreeMap::new(), |mut acc, (votes, outcome)| {
                let entry = acc.entry(outcome).or_insert(0);
                *entry = entry.saturating_add(votes);
                acc
            });

        let mut outcome_by_votes: Vec<(u16, AgendaElect)> = votes_by_outcome
            .into_iter()
            .filter(|(_, votes)| *votes > 0)
            .map(|(outcome, votes)| (votes, outcome))
            .collect();
        outcome_by_votes.sort_by_key(|(votes, _)| Reverse(*votes));

        // TODO: take speaker tie-breaker vote into account
        self.expected_outcome = outcome_by_votes.first().and_then(|(votes, outcome)| {
            // if the first outcome has more votes than any other outcome, it is expected to pass
            let Some((second_number_of_votes, _)) = outcome_by_votes.get(1) else {
                return Some(outcome.clone());
            };

            (votes > second_number_of_votes).then(|| outcome.clone())
        });
        self.outcomes_by_votes = outcome_by_votes;
    }
}
