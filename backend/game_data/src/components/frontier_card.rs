use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::common::expansions::Expansion;

/// A frontier card.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, TS)]
#[ts(export)]
#[allow(missing_docs)]
pub enum FrontierCard {
    /* PoK */
    DerelictVessel,
    EnigmaticDevice,
    GammaRelay,
    IonStorm,
    LostCrew,
    MerchantStation,
    Mirage,
    UnknownRelicFragment,
    /* Codex III */
    DeadWorld,
    EntropicField,
    KeleresShip,
    MajorEntropicField,
    MinorEntropicField,
}

/// When this frontier card can be played.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, TS)]
#[ts(export)]
pub enum FrontierCardType {
    /// As an action.
    Action,
    /// We don't handle this atm.
    Unhandled,
}

/// All relevant information for a frontier card.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct FrontierCardInfo {
    /// Which frontier card this is in regards to.
    pub card: FrontierCard,
    /// The 'pretty' name of the action card.
    pub name: String,
    /// Which expansion this card comes from.
    pub expansion: Expansion,
    /// The number of cards that exists in a deck.
    pub num_in_deck: usize,
    /// Weather this frontier carries with it an action or not.
    pub frontier_type: FrontierCardType,
}

macro_rules! fi {
    ($card:ident, $name:literal, $exp:ident, $num:literal) => {
        fi!($card, $name, $exp, $num, Unhandled)
    };

    ($card:ident, $name:literal, $exp:ident, $num:literal, $type: ident) => {
        FrontierCardInfo {
            card: FrontierCard::$card,
            name: $name.to_string(),
            expansion: Expansion::$exp,
            num_in_deck: $num,
            frontier_type: FrontierCardType::$type,
        }
    };
}

impl FrontierCard {
    /// Returns the [FrontierCardInfo] for the frontier card.
    pub fn info(&self) -> FrontierCardInfo {
        match self {
            FrontierCard::DerelictVessel => {
                fi!(DerelictVessel, "Derelict Vessel", ProphecyOfKings, 2)
            }
            FrontierCard::EnigmaticDevice => {
                fi!(
                    EnigmaticDevice,
                    "Enigmatic Device",
                    ProphecyOfKings,
                    2,
                    Action
                )
            }
            FrontierCard::GammaRelay => fi!(GammaRelay, "Gamma Relay", ProphecyOfKings, 1),
            FrontierCard::IonStorm => fi!(IonStorm, "Ion Storm", ProphecyOfKings, 1),
            FrontierCard::LostCrew => fi!(LostCrew, "Lost Crew", ProphecyOfKings, 2),
            FrontierCard::MerchantStation => {
                fi!(MerchantStation, "Merchant Station", ProphecyOfKings, 2)
            }
            FrontierCard::Mirage => fi!(Mirage, "Mirage", ProphecyOfKings, 1),
            FrontierCard::UnknownRelicFragment => fi!(
                UnknownRelicFragment,
                "Unknown Relic Fragment",
                ProphecyOfKings,
                3
            ),
            FrontierCard::DeadWorld => fi!(DeadWorld, "Dead World", CodexIII, 1),
            FrontierCard::EntropicField => fi!(EntropicField, "Entropic Field", CodexIII, 1),
            FrontierCard::KeleresShip => fi!(KeleresShip, "Keleres Ship", CodexIII, 2),
            FrontierCard::MajorEntropicField => {
                fi!(MajorEntropicField, "Major Entropic Field", CodexIII, 1)
            }
            FrontierCard::MinorEntropicField => {
                fi!(MinorEntropicField, "Minor Entropic Field", CodexIII, 1)
            }
        }
    }
}
