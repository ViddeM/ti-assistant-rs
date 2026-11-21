use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::components::{ocean::Ocean, planet::Planet};

/// State that is specific to a single faction, and only exists for players playing that faction.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum FactionState {
    /// The faction has no faction specific state.
    #[default]
    None,
    /// State of [crate::common::faction::Faction::FirmamentObsidian].
    Firmament(FirmamentState),
    /// State of [crate::common::faction::Faction::DeepwroughtScholarate].
    Deepwrought(DeepwroughtState),
}

/// State of the Deepwrought Scholarate.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeepwroughtState {
    /// The planets the Deepwrought player's units coexist on.
    pub coexisting: BTreeSet<Planet>,
    /// The ocean card in play for a coexisting planet, if one was gained for it.
    pub oceans: BTreeMap<Planet, Ocean>,
}

impl FactionState {
    /// Returns true if there is no faction specific state.
    pub fn is_none(&self) -> bool {
        matches!(self, FactionState::None)
    }
}

/// State of the Firmament / The Obsidian.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirmamentState {
    /// Which side of the faction is currently in play.
    pub side: FirmamentSide,
}

impl Default for FirmamentState {
    fn default() -> Self {
        Self {
            side: FirmamentSide::Firmament,
        }
    }
}

/// The two sides of the Firmament / The Obsidian.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FirmamentSide {
    /// The starting side.
    Firmament,
    /// The side the faction flips to using 'Puppets of the Blade'.
    Obsidian,
}
