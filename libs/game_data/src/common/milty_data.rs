use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{faction::Faction, game_settings::Expansions, map::HexMap};

/// Data imported from miltydraft.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiltyData {
    /// Map from name to [MiltyPlayer] for the players participating in this game.
    pub players: HashMap<String, MiltyPlayer>,
    /// The expansions configured for the milty draft.
    pub expansions: Expansions,
    /// The configured name of the game in milty.
    pub game_name: String,
    /// The galactic map from the milty draft.
    pub hex_map: HexMap,
}

/// Player imported from miltydraft.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MiltyPlayer {
    /// The name of the player.
    pub name: String,
    /// The faction the player is playing.
    pub faction: Faction,
    /// The table order of the player, 0 being the starting speaker.
    pub order: u32,
}
