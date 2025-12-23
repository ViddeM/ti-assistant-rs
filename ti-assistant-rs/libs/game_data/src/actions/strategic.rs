use serde::{Deserialize, Serialize};

use crate::{
    common::player_id::PlayerId,
    components::{objectives::Objective, strategy_card::StrategyCard, tech::Technology},
};

/// Primary action taken during a strategy card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategicPrimaryAction {
    /// The primary action for the technology card.
    Technology {
        /// What tech shall be taken.
        tech: Technology,
        /// What extra tech shall be taken (if any).
        extra: Option<Technology>,
    },

    /// The primary action for the politics card.
    #[serde(rename_all = "camelCase")]
    Politics {
        /// Who the new speaker should be.
        new_speaker: PlayerId,
    },

    /// The primary action for the imperial card.
    #[serde(rename_all = "camelCase")]
    Imperial {
        /// The objective that should be scored, if any.
        score_objective: Option<Objective>,
    },
}

/// The actions taken for the secondary part of a strategy card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum StrategicSecondaryAction {
    Skip,
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology {
        /// The tech that was taken.
        tech: Technology,
    },
    /// The special secondary that the Universities of Jol-Nar has for technology.
    #[serde(rename_all = "camelCase")]
    TechnologyJolNar {
        first_tech: Technology,
        second_tech: Option<Technology>,
    },
    Imperial,
}

impl StrategicSecondaryAction {
    /// Weather the action is the provided strategy card or not.
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
            (StrategicSecondaryAction::TechnologyJolNar { .. }, StrategyCard::Technology) => true,
            (StrategicSecondaryAction::Imperial, StrategyCard::Imperial) => true,
            _ => false,
        }
    }

    /// Weather the player skipped taking their secondary action.
    pub fn skipped(&self) -> bool {
        matches!(self, StrategicSecondaryAction::Skip)
    }
}

/// The progress of the secondary portion of a strategy card.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[allow(missing_docs)]
pub enum StrategicSecondaryProgress {
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology {
        /// What tech was taken.
        tech: Technology,
    },
    /// Special technology secondary for the Universities of Jol-Nar
    #[serde(rename_all = "camelCase")]
    TechnologyJolNar {
        first_tech: Technology,
        second_tech: Option<Technology>,
    },
    Imperial,
    Skipped,
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
            StrategicSecondaryAction::TechnologyJolNar {
                first_tech,
                second_tech,
            } => Self::TechnologyJolNar {
                first_tech,
                second_tech,
            },
            StrategicSecondaryAction::Imperial => Self::Imperial,
        }
    }
}
