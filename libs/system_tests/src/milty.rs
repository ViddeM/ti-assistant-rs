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
pub const FIVE_PLAYER_MAP: &str = "65 74 41 85A0 113 100 63 39 102 28 36 87A2 78 87A0 59 34 32 68 96a 47 49 95 50 64 94 24 83A0 85A0 83A1 35 93 80 67 0 107 110";

/// Milty data for the players `(name, faction, table order)` on [FIVE_PLAYER_MAP].
pub fn milty_data(players: &[(&str, Faction, u32)]) -> MiltyData {
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
            thunders_edge: true,
        },
        game_name: "Scenario".to_string(),
        hex_map: HexMap::from_milty_string(FIVE_PLAYER_MAP, true).expect("invalid milty map"),
    }
}
