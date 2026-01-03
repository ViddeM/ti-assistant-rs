use serde::{Deserialize, Serialize};
use strum::EnumString;
use strum_macros::{Display, EnumIter};

/// A strategy card.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Display,
)]
#[allow(missing_docs)]
pub enum StrategyCard {
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology,
    Imperial,
}

impl StrategyCard {
    /// What initiative this strategy card has.
    pub fn card_number(&self) -> u8 {
        match self {
            StrategyCard::Leadership => 1,
            StrategyCard::Diplomacy => 2,
            StrategyCard::Politics => 3,
            StrategyCard::Construction => 4,
            StrategyCard::Trade => 5,
            StrategyCard::Warfare => 6,
            StrategyCard::Technology => 7,
            StrategyCard::Imperial => 8,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            StrategyCard::Leadership => "leadership",
            StrategyCard::Diplomacy => "diplomacy",
            StrategyCard::Politics => "politics",
            StrategyCard::Construction => "construction",
            StrategyCard::Trade => "trade",
            StrategyCard::Warfare => "warfare",
            StrategyCard::Technology => "technology",
            StrategyCard::Imperial => "imperial",
        }
    }
}
