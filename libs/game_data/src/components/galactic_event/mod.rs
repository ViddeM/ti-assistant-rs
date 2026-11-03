use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use strum_macros::EnumIter;

use crate::common::{
    component::{Component, default_is_enabled_in},
    expansions::Expansion,
    game_settings::Expansions,
};

/// Galactic events, which change the rules of the game.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    EnumIter,
    EnumString,
    Display,
)]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
pub enum GalacticEvent {
    /* Codex IV */
    MinorFactions,
    TotalWar,
    AgeOfCommerce,
    AgeOfExploration,
    /* Thunder's Edge */
    StellarAtomics,
    AgeOfFighters,
    CivilizedSociety,
    DangerousWilds,
    CallOfTheVoid,
    HiddenAgenda,
    WildWildGalaxy,
    CulturalExchangeProgram,
    CosmicPhenomenae,
    AdventOfTheWarSun,
    MercenariesForHire,
    RapidMobilization,
    WeirdWormholes,
    MonumentsToTheAges,
    ZealousOrthodoxy,
    ConventionsOfWarAbandoned,
}

/// All relevant information for a galactic event.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GalacticEventInfo {
    /// Which galactic event this refers to.
    pub event: GalacticEvent,
    /// The 'pretty' name of this galactic event.
    pub name: String,
    /// How complex this galactic event is to play with, from 1 to 3.
    pub complexity: u8,
    /// Which expansion this galactic event came with.
    pub expansion: Expansion,
    /// An expansion this galactic event needs on top of its own.
    pub also_requires: Expansion,
    /// The text of the galactic event card.
    pub description: String,
}

macro_rules! info {
    (event: $event:ident, name: $name:literal, complexity: $complexity:literal, expansion: $expansion:ident, also_requires: $also_requires:ident) => {
        GalacticEventInfo {
            event: GalacticEvent::$event,
            name: $name.to_string(),
            complexity: $complexity,
            expansion: Expansion::$expansion,
            also_requires: Expansion::$also_requires,
            description: include_str!(concat!("description/", stringify!($event))).to_string(),
        }
    };
    (event: $event:ident, name: $name:literal, complexity: $complexity:literal, expansion: $expansion:ident) => {
        info! { event: $event, name: $name, complexity: $complexity, expansion: $expansion, also_requires: Base }
    };
}

impl GalacticEvent {
    /// Get the [GalacticEventInfo] of this [GalacticEvent].
    pub fn info(&self) -> GalacticEventInfo {
        match self {
            GalacticEvent::MinorFactions => info! {
                event: MinorFactions,
                name: "Minor Factions",
                complexity: 2,
                expansion: CodexIV
            },
            GalacticEvent::TotalWar => info! {
                event: TotalWar,
                name: "Total War",
                complexity: 3,
                expansion: CodexIV
            },
            GalacticEvent::AgeOfCommerce => info! {
                event: AgeOfCommerce,
                name: "Age of Commerce",
                complexity: 1,
                expansion: CodexIV
            },
            GalacticEvent::AgeOfExploration => info! {
                event: AgeOfExploration,
                name: "Age of Exploration",
                complexity: 2,
                expansion: CodexIV,
                also_requires: ProphecyOfKings
            },
            GalacticEvent::StellarAtomics => info! {
                event: StellarAtomics,
                name: "Stellar Atomics",
                complexity: 2,
                expansion: ThundersEdge
            },
            GalacticEvent::AgeOfFighters => info! {
                event: AgeOfFighters,
                name: "Age of Fighters",
                complexity: 3,
                expansion: ThundersEdge
            },
            GalacticEvent::CivilizedSociety => info! {
                event: CivilizedSociety,
                name: "Civilized Society",
                complexity: 2,
                expansion: ThundersEdge
            },
            GalacticEvent::DangerousWilds => info! {
                event: DangerousWilds,
                name: "Dangerous Wilds",
                complexity: 1,
                expansion: ThundersEdge
            },
            GalacticEvent::CallOfTheVoid => info! {
                event: CallOfTheVoid,
                name: "Call of the Void",
                complexity: 1,
                expansion: ThundersEdge
            },
            GalacticEvent::HiddenAgenda => info! {
                event: HiddenAgenda,
                name: "Hidden Agenda",
                complexity: 2,
                expansion: ThundersEdge
            },
            GalacticEvent::WildWildGalaxy => info! {
                event: WildWildGalaxy,
                name: "Wild, Wild Galaxy",
                complexity: 3,
                expansion: ThundersEdge
            },
            GalacticEvent::CulturalExchangeProgram => info! {
                event: CulturalExchangeProgram,
                name: "Cultural Exchange Program",
                complexity: 2,
                expansion: ThundersEdge
            },
            GalacticEvent::CosmicPhenomenae => info! {
                event: CosmicPhenomenae,
                name: "Cosmic Phenomenae",
                complexity: 2,
                expansion: ThundersEdge
            },
            GalacticEvent::AdventOfTheWarSun => info! {
                event: AdventOfTheWarSun,
                name: "Advent of the War Sun",
                complexity: 1,
                expansion: ThundersEdge
            },
            GalacticEvent::MercenariesForHire => info! {
                event: MercenariesForHire,
                name: "Mercenaries for Hire",
                complexity: 1,
                expansion: ThundersEdge
            },
            GalacticEvent::RapidMobilization => info! {
                event: RapidMobilization,
                name: "Rapid Mobilization",
                complexity: 1,
                expansion: ThundersEdge
            },
            GalacticEvent::WeirdWormholes => info! {
                event: WeirdWormholes,
                name: "Weird Wormholes",
                complexity: 3,
                expansion: ThundersEdge
            },
            GalacticEvent::MonumentsToTheAges => info! {
                event: MonumentsToTheAges,
                name: "Monuments to the Ages",
                complexity: 2,
                expansion: ThundersEdge
            },
            GalacticEvent::ZealousOrthodoxy => info! {
                event: ZealousOrthodoxy,
                name: "Zealous Orthodoxy",
                complexity: 1,
                expansion: ThundersEdge
            },
            GalacticEvent::ConventionsOfWarAbandoned => info! {
                event: ConventionsOfWarAbandoned,
                name: "Conventions of War Abandoned",
                complexity: 3,
                expansion: ThundersEdge
            },
        }
    }

    /// Whether the app tracks an ACTION of this galactic event, see
    /// [GalacticEventAction](crate::actions::event::GalacticEventAction).
    pub fn has_action(&self) -> bool {
        matches!(self, GalacticEvent::TotalWar)
    }
}

impl Component for GalacticEvent {
    fn expansion(&self) -> Expansion {
        self.info().expansion
    }

    fn is_enabled_in(&self, expansions: &Expansions) -> bool {
        expansions.is_enabled(&self.info().also_requires) && default_is_enabled_in(self, expansions)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use strum::IntoEnumIterator;

    use super::*;

    fn enabled_in(expansions: Expansions) -> Vec<GalacticEvent> {
        GalacticEvent::iter()
            .filter(|event| event.is_enabled_in(&expansions))
            .collect()
    }

    #[test]
    fn galactic_event_info_matches_event() {
        for event in GalacticEvent::iter() {
            let tag = event.info().event;
            assert_eq!(tag, event, "{event:?} has info for {tag:?}");
        }
    }

    #[test]
    fn galactic_events_have_their_own_description() {
        let mut seen: HashMap<String, GalacticEvent> = HashMap::new();
        for event in GalacticEvent::iter() {
            let description = event.info().description;
            assert!(
                !description.trim().is_empty(),
                "{event:?} has no description"
            );
            if let Some(other) = seen.insert(description, event) {
                panic!("{event:?} and {other:?} have the same description");
            }
        }
    }

    #[test]
    fn galactic_events_need_their_expansions() {
        use GalacticEvent::*;

        assert_eq!(
            enabled_in(Expansions {
                codex_4: true,
                ..Default::default()
            }),
            vec![MinorFactions, TotalWar, AgeOfCommerce]
        );
        assert_eq!(
            enabled_in(Expansions {
                prophecy_of_kings: true,
                codex_4: true,
                ..Default::default()
            }),
            vec![MinorFactions, TotalWar, AgeOfCommerce, AgeOfExploration]
        );
        assert_eq!(
            enabled_in(Expansions {
                thunders_edge: true,
                ..Default::default()
            }),
            vec![
                StellarAtomics,
                AgeOfFighters,
                CivilizedSociety,
                DangerousWilds,
                CallOfTheVoid,
                HiddenAgenda,
                WildWildGalaxy,
                CulturalExchangeProgram,
                CosmicPhenomenae,
                AdventOfTheWarSun,
                MercenariesForHire,
                RapidMobilization,
                WeirdWormholes,
                MonumentsToTheAges,
                ZealousOrthodoxy,
                ConventionsOfWarAbandoned,
            ]
        );
    }
}
