use serde::{Deserialize, Serialize};

use crate::data::components::{planet::Planet, strategy_card::StrategyCard, tech::Technology};

use super::player::{NewPlayer, PlayerId};

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
        did_secondary: bool,
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
}
