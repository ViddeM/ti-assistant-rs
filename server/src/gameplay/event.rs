use serde::{Deserialize, Serialize};

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

    // TODO: Score objectives & Reveal objectives
    CompleteStatusPhase,
    // TODO: Agenda phase
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategicPrimaryAction {
    Technology {
        tech: Technology,
        extra: Option<Technology>,
    },
    #[serde(rename_all = "camelCase")]
    Politics { new_speaker: PlayerId },
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
