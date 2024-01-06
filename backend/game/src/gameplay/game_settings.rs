use serde::{Deserialize, Serialize};

/// Settings for a game.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct GameSettings {
    pub max_points: u32,
    pub expansion: Expansions,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            max_points: 10,
            expansion: Default::default(),
        }
    }
}

/// Which expansions are in use.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Expansions {
    pub prophecy_of_kings: bool,
    pub codex_1: bool,
    pub codex_2: bool,
    pub codex_3: bool,
}
