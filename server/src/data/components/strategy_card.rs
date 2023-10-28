use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    pub const ALL: [StrategyCard; 8] = [
        StrategyCard::Leadership,
        StrategyCard::Diplomacy,
        StrategyCard::Politics,
        StrategyCard::Construction,
        StrategyCard::Trade,
        StrategyCard::Warfare,
        StrategyCard::Technology,
        StrategyCard::Imperial,
    ];
}
