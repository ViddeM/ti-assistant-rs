use std::collections::HashMap;

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
            ActionPhaseProgress::Tactical(tactical)
            | ActionPhaseProgress::Strategic(StrategicProgress {
                primary: Some(StrategicPrimaryProgress::Warfare { tactical }),
                ..
            }) => Some(tactical),
            _ => None,
        }
    }

    /// The tactical action being performed, if any.
    pub fn tactical_mut(&mut self) -> Option<&mut TacticalProgress> {
        match self {
            ActionPhaseProgress::Tactical(tactical)
            | ActionPhaseProgress::Strategic(StrategicProgress {
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
        !self.card.has_tracked_primary() || self.primary.is_some()
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
        tactical: TacticalProgress,
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
    /// Which planets have been taken this far and the player who owned them previously (if any).
    pub taken_planets: HashMap<Planet, Option<PlayerId>>,
    /// What planet attachments have been selected for the taken planets.
    /// NOTE: Does not include attachments kept when taken from another player.
    pub planet_attachments: HashMap<Planet, PlanetAttachment>,
    /// Technologies researched for planets taken this turn (Dangerous Wilds).
    pub researched_techs: HashMap<Planet, Technology>,
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
