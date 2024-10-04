use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::data::common::expansions::Expansion;

/// Settings for a game.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GameSettings {
    /// The points required to trigger win-condition for this game.
    pub max_points: u32,
    /// Which expansions are in play this game.
    pub expansions: Expansions,
    /// The milty draft map string for this game, if one was used, or none.
    pub milty_string: Option<String>,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            max_points: 10,
            expansions: Default::default(),
            milty_string: None,
        }
    }
}

/// Which expansions are in use.
#[derive(Clone, Default, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
#[allow(missing_docs)]
pub struct Expansions {
    pub prophecy_of_kings: bool,
    pub codex_1: bool,
    pub codex_2: bool,
    pub codex_3: bool,
}

impl Expansions {
    /// The maximum number of players allowed for this expansion configuration.
    pub fn max_number_of_players(&self) -> usize {
        if self.prophecy_of_kings {
            return 8;
        }
        6
    }

    /// Returns true if the provided expansion is enabled.
    pub fn is_enabled(&self, expansion: &Expansion) -> bool {
        match expansion {
            Expansion::Base => true,
            Expansion::ProphecyOfKings => self.prophecy_of_kings,
            Expansion::Codex => self.codex_1,
            Expansion::CodexII => self.codex_2,
            Expansion::CodexIII => self.codex_3,
        }
    }
}
