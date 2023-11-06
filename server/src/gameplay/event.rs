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
        match self {
            StrategicSecondaryAction::Skip => true,
            _ => false,
        }
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
