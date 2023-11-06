use serde::{Deserialize, Serialize};

use crate::data::components::{planet::Planet, strategy_card::StrategyCard, tech::Technology};

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

    ComponentAction {
        player: PlayerId,
        component: (), // TODO
    },

    PassAction {
        player: PlayerId,
    },

    /* -- STATUS PHASE EVENTS -- */
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
        matches!(
            (self, card),
            (StrategicSecondaryAction::Skip, _)
                | (
                    StrategicSecondaryAction::Leadership,
                    StrategyCard::Leadership
                )
                | (StrategicSecondaryAction::Diplomacy, StrategyCard::Diplomacy)
                | (StrategicSecondaryAction::Politics, StrategyCard::Politics)
                | (
                    StrategicSecondaryAction::Construction,
                    StrategyCard::Construction
                )
                | (StrategicSecondaryAction::Trade, StrategyCard::Trade)
                | (StrategicSecondaryAction::Warfare, StrategyCard::Warfare)
                | (
                    StrategicSecondaryAction::Technology { .. },
                    StrategyCard::Technology
                )
                | (StrategicSecondaryAction::Imperial, StrategyCard::Imperial)
        )
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
