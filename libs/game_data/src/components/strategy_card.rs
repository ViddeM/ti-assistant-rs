use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoEnumIterator};
use strum_macros::{Display, EnumIter};

use crate::common::{
    component::{Component, replacements},
    expansions::Expansion,
    game_settings::Expansions,
};

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

    /// Whether the primary ability of this card has effects that are tracked, and so has to be
    /// performed with a [crate::actions::strategic::StrategicPrimaryAction].
    pub fn has_tracked_primary(&self) -> bool {
        matches!(
            self,
            StrategyCard::Politics | StrategyCard::Technology | StrategyCard::Imperial
        )
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

    /// The version of this strategy card that is in play with the provided expansions, i.e. the
    /// newest version whose expansion is enabled.
    pub fn version(&self, expansions: &Expansions) -> StrategyCardVersion {
        StrategyCardVersion::iter()
            .find(|v| v.card() == *self && v.is_enabled_in(expansions))
            .expect("every strategy card has a base version")
    }
}

/// A printed version of a strategy card. The versions of each card are listed oldest first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[allow(missing_docs)]
pub enum StrategyCardVersion {
    Leadership,
    Diplomacy,
    DiplomacyOmega,
    Politics,
    Construction,
    ConstructionOmega,
    ConstructionOmegaOmega,
    Trade,
    Warfare,
    WarfareOmega,
    Technology,
    Imperial,
}

impl StrategyCardVersion {
    /// The strategy card this is a version of.
    pub fn card(&self) -> StrategyCard {
        match self {
            StrategyCardVersion::Leadership => StrategyCard::Leadership,
            StrategyCardVersion::Diplomacy => StrategyCard::Diplomacy,
            StrategyCardVersion::DiplomacyOmega => StrategyCard::Diplomacy,
            StrategyCardVersion::Politics => StrategyCard::Politics,
            StrategyCardVersion::Construction => StrategyCard::Construction,
            StrategyCardVersion::ConstructionOmega => StrategyCard::Construction,
            StrategyCardVersion::ConstructionOmegaOmega => StrategyCard::Construction,
            StrategyCardVersion::Trade => StrategyCard::Trade,
            StrategyCardVersion::Warfare => StrategyCard::Warfare,
            StrategyCardVersion::WarfareOmega => StrategyCard::Warfare,
            StrategyCardVersion::Technology => StrategyCard::Technology,
            StrategyCardVersion::Imperial => StrategyCard::Imperial,
        }
    }
}

impl Component for StrategyCardVersion {
    replacements! {
        Diplomacy -> DiplomacyOmega,
        Construction -> ConstructionOmega -> ConstructionOmegaOmega,
        Warfare -> WarfareOmega,
    }

    fn expansion(&self) -> Expansion {
        match self {
            StrategyCardVersion::Leadership => Expansion::Base,
            StrategyCardVersion::Diplomacy => Expansion::Base,
            StrategyCardVersion::DiplomacyOmega => Expansion::Codex,
            StrategyCardVersion::Politics => Expansion::Base,
            StrategyCardVersion::Construction => Expansion::Base,
            StrategyCardVersion::ConstructionOmega => Expansion::ProphecyOfKings,
            StrategyCardVersion::ConstructionOmegaOmega => Expansion::ThundersEdge,
            StrategyCardVersion::Trade => Expansion::Base,
            StrategyCardVersion::Warfare => Expansion::Base,
            StrategyCardVersion::WarfareOmega => Expansion::ThundersEdge,
            StrategyCardVersion::Technology => Expansion::Base,
            StrategyCardVersion::Imperial => Expansion::Base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_strategy_card_has_exactly_one_base_version() {
        for card in StrategyCard::iter() {
            let base_versions = StrategyCardVersion::iter()
                .filter(|v| v.card() == card && v.expansion() == Expansion::Base)
                .count();
            assert_eq!(base_versions, 1, "{card}");
        }
    }

    #[test]
    fn the_newest_enabled_version_is_in_play() {
        use StrategyCardVersion::*;

        let pok = Expansions {
            prophecy_of_kings: true,
            ..Default::default()
        };
        let pok_codex_1 = Expansions {
            prophecy_of_kings: true,
            codex_1: true,
            ..Default::default()
        };
        let pok_te = Expansions {
            prophecy_of_kings: true,
            thunders_edge: true,
            ..Default::default()
        };
        let te = Expansions {
            thunders_edge: true,
            ..Default::default()
        };

        let cases = [
            (Expansions::default(), [Diplomacy, Construction, Warfare]),
            (pok, [Diplomacy, ConstructionOmega, Warfare]),
            (pok_codex_1, [DiplomacyOmega, ConstructionOmega, Warfare]),
            (pok_te, [Diplomacy, ConstructionOmegaOmega, WarfareOmega]),
            (te, [Diplomacy, ConstructionOmegaOmega, WarfareOmega]),
        ];

        for (expansions, expected) in cases {
            let versions = [
                StrategyCard::Diplomacy,
                StrategyCard::Construction,
                StrategyCard::Warfare,
            ]
            .map(|card| card.version(&expansions));
            assert_eq!(versions, expected, "{expansions:?}");

            for card in StrategyCard::iter() {
                assert_eq!(card.version(&expansions).card(), card, "{expansions:?}");
            }
        }
    }
}
