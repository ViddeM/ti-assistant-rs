use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::data::common::expansions::Expansion;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, TS)]
#[ts(export)]
#[allow(missing_docs)]
pub enum Relic {
    /* PoK */
    DominusOrb,
    MawOfWorlds,
    ScepterOfEmelpar,
    ShardOfTheThrone,
    StellarConverter,
    TheCodex,
    TheCrownOfEmphidia,
    TheCrownOfThalnos,
    TheObsidian,
    TheProphetsTears,
    /* Codex II */
    DynamisCore,
    JrXs4550,
    NanoForge,
}

/// When this relic can be used.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, TS)]
#[ts(export)]
pub enum RelicPlay {
    /// As an action.
    Action,
    /// During the agenda phase.
    Agenda,
    /// The relic has an effect as long as a player is in possession of it.
    Possession,
    /// Can be played after a tactical action has been performed.
    AfterTactical,
    /// We don't handle this atm.
    Unhandled,
}

/// All relevant information for a relic card.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[ts(export)]
pub struct RelicInfo {
    /// Which card this refers to.
    pub card: Relic,
    /// The 'pretty' name of this relic.
    pub name: String,
    /// Which expansion this relic came with.
    pub expansion: Expansion,
    /// When the relic can be played / is relevant.
    pub play: RelicPlay,
}

macro_rules! ri {
    ($card:ident, $name:literal, $exp:ident) => {
        ri!($card, $name, $exp, Unhandled)
    };

    ($card:ident, $name:literal, $exp:ident, $play:ident) => {
        RelicInfo {
            card: Relic::$card,
            name: $name.to_string(),
            expansion: Expansion::$exp,
            play: RelicPlay::$play,
        }
    };
}

impl Relic {
    /// Returns the [RelicInfo] for the frontier card.
    pub fn info(&self) -> RelicInfo {
        match self {
            Relic::DominusOrb => ri!(DominusOrb, "Dominus Orb", ProphecyOfKings),
            Relic::MawOfWorlds => ri!(MawOfWorlds, "Maw of Worlds", ProphecyOfKings, Agenda),
            Relic::ScepterOfEmelpar => ri!(ScepterOfEmelpar, "Scepter of Emelpar", ProphecyOfKings),
            Relic::ShardOfTheThrone => ri!(
                ShardOfTheThrone,
                "Shard of the Throne",
                ProphecyOfKings,
                Possession
            ),
            Relic::StellarConverter => ri!(
                StellarConverter,
                "Stellar Converter",
                ProphecyOfKings,
                Action
            ),
            Relic::TheCodex => ri!(TheCodex, "The Codex", ProphecyOfKings, Action),
            Relic::TheCrownOfEmphidia => ri!(
                TheCrownOfEmphidia,
                "The Crown of Emphidia",
                ProphecyOfKings,
                AfterTactical
            ),
            Relic::TheCrownOfThalnos => ri!(
                TheCrownOfThalnos,
                "The Crown of Thalnos",
                ProphecyOfKings,
                Unhandled
            ),
            Relic::TheObsidian => ri!(TheObsidian, "The Obsidian", ProphecyOfKings),
            Relic::TheProphetsTears => {
                ri!(TheProphetsTears, "The Prophet's Tears", ProphecyOfKings)
            }
            Relic::DynamisCore => ri!(DynamisCore, "Dynamis Core", CodexII, Action),
            Relic::JrXs4550 => ri!(JrXs4550, "JR-XS455-0", CodexII, Action),
            Relic::NanoForge => ri!(NanoForge, "Nano-Forge", CodexII, Action),
        }
    }
}
