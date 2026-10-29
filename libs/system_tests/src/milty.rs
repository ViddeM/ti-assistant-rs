//! Offline stand-ins for data imported from milty draft.

use ti_helper_game_data::common::{
    faction::Faction,
    game_settings::Expansions,
    map::HexMap,
    milty_data::{MiltyData, MiltyPlayer},
};

/// The map of a real 5 player milty draft (with Thunder's Edge tiles).
///
/// It contains the home systems of the Ral Nel Consortium, the Deepwrought Scholarate and the
/// Firmament / The Obsidian, so those factions must be in the game. The other two players'
/// home systems are empty (`0`), so any factions can be used for them.
pub const FIVE_PLAYER_MAP: &str = "65 74 41 85A0 113 100 63 39 102 28 36 87A2 78 87A0 59 34 32 68 96a 47 49 95 50 64 0 24 83A0 85A0 83A1 35 93 80 67 0 107 110";

/// A map with every Thunder's Edge system tile, except the hyperlanes and the 96B side of the
/// Firmament / The Obsidian home system. The Fracture (125, 126 and 127) is placed outside the
/// galaxy, and so is the Crimson Rebellion home system (118).
///
/// It contains the home systems of the Firmament / The Obsidian, the Deepwrought Scholarate, the
/// Crimson Rebellion (its gate, 94), the Ral Nel Consortium and the Last Bastion, so those factions
/// must be in the game. The sixth home system and the slots without a tile are empty (`0`).
pub const EVERY_THUNDERS_EDGE_TILE_MAP: &str = "97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 115 116 117 96a 0 0 95 0 113 94 114 0 93 0 0 92 0 0 0 0 0";

/// Milty data for the players `(name, faction, table order)` on [FIVE_PLAYER_MAP].
pub fn milty_data(players: &[(&str, Faction, u32)]) -> MiltyData {
    milty_data_on_map(FIVE_PLAYER_MAP, players)
}

/// Milty data for the players `(name, faction, table order)` on the milty `map`.
pub fn milty_data_on_map(map: &str, players: &[(&str, Faction, u32)]) -> MiltyData {
    MiltyData {
        players: players
            .iter()
            .map(|&(name, faction, order)| {
                let player = MiltyPlayer {
                    name: name.to_string(),
                    faction,
                    order,
                };
                (name.to_string(), player)
            })
            .collect(),
        expansions: Expansions {
            prophecy_of_kings: true,
            codex_1: true,
            codex_2: true,
            codex_3: true,
            codex_4: true,
            thunders_edge: true,
        },
        game_name: "Scenario".to_string(),
        hex_map: HexMap::from_milty_string(map, true).expect("invalid milty map"),
    }
}
