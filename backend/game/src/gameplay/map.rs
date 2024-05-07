use eyre::ContextCompat;

use crate::data::components::system::{systems, System, SystemId, MECATOL_REX_ID};

/// The galactic map.
pub struct HexMap {
    tiles: Vec<Tile>,
    ring_count: u32,
}

impl HexMap {
    const RING_STARTING_INDICES: [usize; 7] = [0, 1, 7, 19, 37, 61, 91];
    const WORMHOLE_NEXUS_TILE_ID: &'static str = "80";
    const CREUSS_WORMHILE_ID: &'static str = "17";
    const CREUSS_HOME_SYSTEM: &'static str = "51";

    /// Create the map from a milty export string (space separated system ids).
    /// Example `87A1 89B3 47 87A4 89B0 78 37 64 46 29 72 22 24 63 44 40 23 76 50 30 48 28 43 83B2 67 69 34 27 77 26 36 74 83B2 79 19 38 53 42 59 7 0 0 14 21 0 4 39 71 15 80 68 52 0 0 17 75 0 58 41 60`
    /// Ring indicies:
    ///  - 0 = 0     |
    ///  - 1 = 1-6   |
    ///  - 2 = 7-18  |
    ///  - 3 = 19-36 |
    ///  - 4 = 37-60 |
    ///  - 5 = 61-90 |
    pub fn from_milty_string(milty_string: &str) -> eyre::Result<Self> {
        let systems = systems();
        let mut map: Vec<Option<&System>> = milty_string
            .split(' ')
            .map(|s| {
                if s == "0" {
                    return Ok(None);
                }

                Ok(Some(systems.get(s).wrap_err_with(|| {
                    format!("Milty system ID does not exist ({s:?})?")
                })?))
            })
            .collect::<eyre::Result<Vec<Option<&System>>>>()?;

        let meca = &systems[MECATOL_REX_ID];
        map.insert(0, Some(meca));

        let mut tiles = Vec::with_capacity(milty_string.len() + 1);

        let mut ring = 0;
        let mut ring_pos = 0;
        for (index, system) in map.into_iter().enumerate() {
            let coord = Coordinate {
                ring,
                position: ring_pos,
            };

            if Self::RING_STARTING_INDICES.contains(&index) {
                ring += 1;
                ring_pos = 0;
            } else {
                ring_pos += 1;
            }

            let Some(system) = system else {
                continue;
            };

            let tile = Tile {
                system: system.id.clone(),
                position: HexPosition::Pos(coord),
            };

            tiles.push(tile);
        }

        tiles.push(Tile {
            system: systems
                .get(Self::WORMHOLE_NEXUS_TILE_ID)
                .wrap_err("Wormhole nexus not in list of systems?")?
                .id,
            position: HexPosition::OutsideGalaxy,
        });

        if map.contains(&Some(
            systems
                .get(Self::CREUSS_WORMHILE_ID)
                .wrap_err("Creuss wormhole not in list of systems?")?,
        )) {
            let creuss_home_system = systems
                .get(Self::CREUSS_HOME_SYSTEM)
                .wrap_err("Creuss homesystem not in list of systems?")?;
            tiles.push(creuss_home_system);
        }

        todo!("NOT IMPLEMENTED");
    }
}

/// A tile in play.
pub struct Tile {
    system: SystemId,
    position: HexPosition,
}

/// A position of a tile in the game.
pub enum HexPosition {
    /// Outside of the galaxy, e.g. Creuss home system and Mallice.
    OutsideGalaxy,
    /// Inside the galactic grid.
    Pos(Coordinate),
}

/// A coordinate of a tile in the system.
pub struct Coordinate {
    /// Which ring the coordinate is in (starting with mecatol rex on 0).
    ring: u32,
    /// Which position the tile is on starting from the column "above" mecatol rex at 0 and going clockwise around the galaxy.
    position: u32,
}
