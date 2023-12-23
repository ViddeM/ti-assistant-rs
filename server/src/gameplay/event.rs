use serde::{Deserialize, Serialize};

use crate::data::components::agenda::{Agenda, AgendaElect};
use crate::data::components::objectives::public::PublicObjective;
use crate::data::components::objectives::{secret::SecretObjective, Objective};
use crate::data::components::{
    action_card::ActionCard, planet::Planet, strategy_card::StrategyCard, tech::Technology,
};

use super::{
    game_state::StrategicSecondaryProgress,
    player::{NewPlayer, PlayerId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /* -- SETUP PHASE EVENTS -- */
    AddPlayer {
        player: NewPlayer,
    },
    StartGame,

    /* -- STRATEGY PHASE EVENTS -- */
    TakeStrategyCard {
        player: PlayerId,
        card: StrategyCard,
    },
    CompleteStrategyPhase,

    /* -- ACTION PHASE EVENTS -- */
    TacticalActionBegin {
        player: PlayerId,
    },

    TacticalActionTakePlanet {
        player: PlayerId,
        planet: Planet,
    },

    TacticalActionCommit {
        player: PlayerId,
    },

    StrategicActionBegin {
        player: PlayerId,
        card: StrategyCard,
    },

    #[serde(rename_all = "camelCase")]
    StrategicActionPrimary {
        player: PlayerId,
        action: StrategicPrimaryAction,
    },

    #[serde(rename_all = "camelCase")]
    StrategicActionSecondary {
        player: PlayerId,
        action: StrategicSecondaryAction,
    },

    StrategicActionCommit,

    ActionCardActionBegin {
        player: PlayerId,
        card: ActionCard,
    },

    ActionCardActionCommit {
        player: PlayerId,
        data: Option<ActionCardInfo>,
    },

    PassAction {
        player: PlayerId,
    },

    /* -- STATUS PHASE EVENTS -- */
    ScorePublicObjective {
        player: PlayerId,
        objective: Objective,
    },

    ScoreSecretObjective {
        player: PlayerId,
        objective: SecretObjective,
    },

    RevealPublicObjective {
        objective: PublicObjective,
    },

    // TODO: Score objectives
    CompleteStatusPhase,

    /* -- ACTION PHASE EVENTS -- */
    RevealAgenda {
        agenda: Agenda,
    },

    ResolveAgenda {
        outcome: AgendaElect,
    },

    CompleteAgendaPhase,

    /* -- ANY PHASE EVENTS -- */
    /// Give `giver` players Support for the Throne to `receiver`.
    GiveSupportForTheThrone {
        giver: PlayerId,
        receiver: PlayerId,
    },

    /// Set the `extra_points` value for a player to the given value.
    SetExtraPoints {
        player: PlayerId,
        value: i8,
    },

    /// Set the current custodians, usually set to the first player to take mecatol rex.
    SetCustodians {
        player: Option<PlayerId>,
    },

    /// Increment the `extra_points` value for a player with the given value.
    AddExtraPoints {
        player: PlayerId,
        value: i8,
    },

    /// Increment the number of points received from the imperial strategy card for a player with the given value.
    AddImperial {
        player: PlayerId,
        value: i8,
    },

    /// Unscore a revealed objective.
    UnscoreObjective {
        player: PlayerId,
        objective: Objective,
    },

    /// Unscore a secret objective.
    UnscoreSecretObjective {
        player: PlayerId,
        objective: SecretObjective,
    },

    /// Add the tech to the players list of techs.
    AddTechToPlayer {
        player: PlayerId,
        tech: Technology,
    },

    /// Remove the tech from the players list of techs.
    RemoveTechFromPlayer {
        player: PlayerId,
        tech: Technology,
    },

    /// Pause/unpause time-tracking.
    TrackTime {
        paused: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategicPrimaryAction {
    Technology {
        tech: Technology,
        extra: Option<Technology>,
    },

    #[serde(rename_all = "camelCase")]
    Politics { new_speaker: PlayerId },

    #[serde(rename_all = "camelCase")]
    Imperial { score_objective: Objective },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategicSecondaryAction {
    Skip,
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology { tech: Technology },
    Imperial,
}

impl StrategicSecondaryAction {
    pub fn is_for_card(&self, card: StrategyCard) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, card) {
            (StrategicSecondaryAction::Skip, _) => true,
            (StrategicSecondaryAction::Leadership, StrategyCard::Leadership) => true,
            (StrategicSecondaryAction::Diplomacy, StrategyCard::Diplomacy) => true,
            (StrategicSecondaryAction::Politics, StrategyCard::Politics) => true,
            (StrategicSecondaryAction::Construction, StrategyCard::Construction) => true,
            (StrategicSecondaryAction::Trade, StrategyCard::Trade) => true,
            (StrategicSecondaryAction::Warfare, StrategyCard::Warfare) => true,
            (StrategicSecondaryAction::Technology { .. }, StrategyCard::Technology) => true,
            (StrategicSecondaryAction::Imperial, StrategyCard::Imperial) => true,
            _ => false,
        }
    }

    pub fn skipped(&self) -> bool {
        matches!(self, StrategicSecondaryAction::Skip)
    }
}

impl From<StrategicSecondaryAction> for StrategicSecondaryProgress {
    fn from(value: StrategicSecondaryAction) -> Self {
        match value {
            StrategicSecondaryAction::Skip => Self::Skipped,
            StrategicSecondaryAction::Leadership => Self::Leadership,
            StrategicSecondaryAction::Diplomacy => Self::Diplomacy,
            StrategicSecondaryAction::Politics => Self::Politics,
            StrategicSecondaryAction::Construction => Self::Construction,
            StrategicSecondaryAction::Trade => Self::Trade,
            StrategicSecondaryAction::Warfare => Self::Warfare,
            StrategicSecondaryAction::Technology { tech } => Self::Technology { tech },
            StrategicSecondaryAction::Imperial => Self::Imperial,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionCardInfo {
    FocusedResearch {
        tech: Technology,
    },
    #[serde(rename_all = "camelCase")]
    DivertFunding {
        remove_tech: Technology,
        take_tech: Technology,
    },
    Plagiarize {
        tech: Technology,
    },
}

pub fn action_matches_action_card(action: &Option<ActionCardInfo>, card: &ActionCard) -> bool {
    match card {
        ActionCard::FocusedResearch => {
            matches!(action, Some(ActionCardInfo::FocusedResearch { .. }))
        }
        ActionCard::DivertFunding => {
            matches!(action, Some(ActionCardInfo::DivertFunding { .. }))
        }
        ActionCard::Plagiarize => {
            matches!(action, Some(ActionCardInfo::Plagiarize { .. }))
        }
        _ => action.is_none(),
    }
}
