use std::collections::{BTreeMap, BTreeSet, HashMap};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    actions::strategic::StrategicSecondaryProgress,
    common::{faction::Faction, player_id::PlayerId},
    components::{
        action_card::ActionCard,
        frontier_card::FrontierCard,
        leaders::Leader,
        objectives::Objective,
        planet::Planet,
        planet_attachment::PlanetAttachment,
        relic::Relic,
        strategy_card::{StrategyCard, StrategyCardVersion},
        system::SystemId,
        tech::Technology,
    },
    state::player::Player,
};

use super::{GameState, fracture::FractureProgressState};

/// The current progress of an action-phase action.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t")]
pub enum ActionPhaseProgress {
    /// The progress of a strategy card.
    Strategic(StrategicProgress),
    /// The progress of a tactical action.
    Tactical(TacticalProgress),
    /// The progress of an action card.
    ActionCard(ActionCardProgress),
    /// The progress of a leader action.
    Leader(LeaderProgress),
    /// The progress of a frontier card.
    FrontierCard(FrontierCardProgress),
    /// The ACTION of the current player's breakthrough is being performed.
    Breakthrough(BreakthroughProgress),
    /// The progress of a relic action.
    Relic(RelicProgress),
    /// The progress of the roll / spawn.
    Fracture(FractureProgressState),
}

impl ActionPhaseProgress {
    /// Weather the progress is for a strategy card.
    pub fn is_strategy_card(&self) -> bool {
        matches!(self, ActionPhaseProgress::Strategic(_))
    }

    /// Weather the progress is for a tactical action.
    pub fn is_tactical(&self) -> bool {
        matches!(self, ActionPhaseProgress::Tactical(_))
    }

    /// The tactical action being performed, if any.
    pub fn tactical(&self) -> Option<&TacticalProgress> {
        match self {
            ActionPhaseProgress::Tactical(tactical) => Some(tactical),
            ActionPhaseProgress::Strategic(StrategicProgress {
                primary: Some(StrategicPrimaryProgress::Warfare { tactical }),
                ..
            }) => Some(tactical),
            _ => None,
        }
    }

    /// The tactical action being performed, if any.
    pub fn tactical_mut(&mut self) -> Option<&mut TacticalProgress> {
        match self {
            ActionPhaseProgress::Tactical(tactical) => Some(tactical),
            ActionPhaseProgress::Strategic(StrategicProgress {
                primary: Some(StrategicPrimaryProgress::Warfare { tactical }),
                ..
            }) => Some(tactical),
            _ => None,
        }
    }
}

/// Progress of a strategy card.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StrategicProgress {
    /// The strategy card being played.
    pub card: StrategyCard,
    /// What, if any, progress has been made for the primary part of the strategy card.
    pub primary: Option<StrategicPrimaryProgress>,
    /// What secondary actions other players have taken.
    pub other_players: HashMap<PlayerId, StrategicSecondaryProgress>,
}

impl StrategicProgress {
    pub fn is_done(&self) -> bool {
        let tactical_finished = match &self.primary {
            Some(StrategicPrimaryProgress::Warfare { tactical }) => tactical.is_finished(),
            _ => true,
        };
        (!self.card.has_tracked_primary() || self.primary.is_some()) && tactical_finished
    }
}

/// The progress of the primary section of a strategy card.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StrategicPrimaryProgress {
    /// Primary progress for the technology strategy card.
    Technology {
        /// What main technology was taken.
        first_tech: Option<Technology>,
        /// What, if any, extra tech was taken (and paid for).
        second_tech: Option<Technology>,
    },
    /// Primary progress for the politics strategy card.
    #[serde(rename_all = "camelCase")]
    Politics {
        /// Who the new speaker should be.
        new_speaker: PlayerId,
    },
    /// Primary progress for the imperial strategy card.
    Imperial {
        /// What objective, if any, was scored.
        objective: Option<Objective>,
    },
    /// Primary progress for a warfare strategy card whose primary is a tactical action.
    Warfare {
        /// The tactical action performed with the primary.
        tactical: Box<TacticalProgress>,
    },
}

impl StrategicPrimaryProgress {
    /// Returns the default value for the provided strategy card version and faction.
    pub fn default_for(
        version: StrategyCardVersion,
        faction: Faction,
    ) -> Option<StrategicPrimaryProgress> {
        if version.primary_is_tactical_action() {
            return Some(StrategicPrimaryProgress::Warfare {
                tactical: Default::default(),
            });
        }

        if version.card() == StrategyCard::Technology && faction == Faction::NekroVirus {
            return Some(StrategicPrimaryProgress::Technology {
                first_tech: None,
                second_tech: None,
            });
        }

        None
    }
}

/// Progress during a tactical action.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TacticalProgress {
    /// What system was activated, if any.
    pub activated_system: Option<SystemId>, // TODO: Maybe in the future we should track systems for all tactical actions (Could use some cool interactive map :eyes:)
    /// Which planets have been taken this far and who owned them before.
    pub taken_planets: HashMap<Planet, TakenFrom>,
    /// What planet attachments have been selected for the taken planets.
    /// NOTE: Does not include attachments kept when taken from another player.
    pub planet_attachments: HashMap<Planet, PlanetAttachment>,
    /// Technologies researched for planets taken this turn (Dangerous Wilds).
    pub researched_techs: HashMap<Planet, Technology>,
    /// The relics gained for planets taken this turn that grant a relic, or `None` if there was
    /// no relic to gain.
    pub gained_relics: HashMap<Planet, Option<Relic>>,
    /// The technologies researched with a Book of Latvinia gained this turn.
    pub book_of_latvinia_techs: Option<Vec<Technology>>,
}

/// Who owned a planet before it was taken during a tactical action.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "t", content = "player")]
pub enum TakenFrom {
    /// Nobody had ever owned the planet.
    NeverOwned,
    /// Nobody owned the planet, but somebody had before.
    Unowned,
    /// The player who owned the planet.
    Player(PlayerId),
}

impl TakenFrom {
    /// The player who owned the planet, if any.
    pub fn player(&self) -> Option<&PlayerId> {
        match self {
            TakenFrom::Player(player) => Some(player),
            TakenFrom::NeverOwned | TakenFrom::Unowned => None,
        }
    }
}

impl TacticalProgress {
    /// Whether `planet` was taken this turn without anybody having owned it before, which is
    /// when it's explored and grants its relic.
    pub fn is_first_owned(&self, planet: &Planet) -> bool {
        self.taken_planets.get(planet) == Some(&TakenFrom::NeverOwned)
    }

    /// Whether the player gains a relic for `planet`, which they haven't done yet.
    pub fn owes_relic(&self, planet: &Planet) -> bool {
        self.is_first_owned(planet)
            && planet.info().gain_relic
            && !self.gained_relics.contains_key(planet)
    }

    /// Whether every relic owed for the planets taken this turn has been gained, and a Book of
    /// Latvinia gained with them has been researched with.
    pub fn is_finished(&self) -> bool {
        !self
            .taken_planets
            .keys()
            .any(|planet| self.owes_relic(planet))
            && !self.owes_book_of_latvinia_research()
    }

    /// Whether a Book of Latvinia gained this turn still has to be researched with.
    pub fn owes_book_of_latvinia_research(&self) -> bool {
        self.book_of_latvinia_techs.is_none()
            && self
                .gained_relics
                .values()
                .any(|relic| relic == &Some(Relic::BookOfLatvinia))
    }
}

/// The progress of an action card being played.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActionCardProgress {
    /// Which card is being played.
    pub card: ActionCard,
}

/// The progress of a leader action being played.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
#[allow(missing_docs)]
pub enum LeaderProgress {
    /// This leader needs no special handling.
    Nothing { leader: Leader },
}

/// The progress of a breakthrough ACTION being performed.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BreakthroughProgress {
    /// Each other player's Visionaria Select decision: the technology they research, or `None`
    /// if they don't pay.
    pub visionaria_select: BTreeMap<PlayerId, Option<Technology>>,
}

impl BreakthroughProgress {
    /// The technologies researched with Visionaria Select that `deepwrought` doesn't own yet,
    /// which they gain as well.
    pub fn visionaria_select_gains(&self, deepwrought: &Player) -> BTreeSet<Technology> {
        (self.visionaria_select.values())
            .flatten()
            .filter(|tech| !deepwrought.has_tech(tech))
            .cloned()
            .collect()
    }
}

/// The progress of a frontier card being played.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FrontierCardProgress {
    /// Which card is being played.
    pub card: FrontierCard,
}

/// The progress of a frontier card being played.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RelicProgress {
    /// The relic that was gained or whose action is being taken.
    pub relic: Relic,
    /// Whether the relic was just gained, or its action is being taken.
    pub step: RelicStep,
}

/// What is being done with the relic of a [RelicProgress].
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelicStep {
    /// The relic was just gained, and its on-gain effect (the Book of Latvinia's research) is
    /// being resolved.
    Gain,
    /// The relic's action is being taken.
    Action,
}

impl GameState {
    /// The tactical action being performed.
    pub fn tactical_progress_mut(&mut self) -> anyhow::Result<&mut TacticalProgress> {
        self.action_progress
            .as_mut()
            .and_then(ActionPhaseProgress::tactical_mut)
            .context("Not in a tactical action")
    }
}
