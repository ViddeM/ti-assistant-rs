use std::str::FromStr;

use eyre::{Context, ContextCompat};

use crate::data::components::system::{systems, System, SystemId, MECATOL_REX_ID};

/// The galactic map.
#[derive(Debug, Clone)]
pub struct HexMap {
    /// All tiles that are in the game (at start).
    pub tiles: Vec<Tile>,
    /// How many rings there are in the galactic map (does not include tiles that are 'outside the galaxy').
    pub ring_count: u32,
}

impl HexMap {
    const RING_STARTING_INDICES: [usize; 7] = [0, 1, 7, 19, 37, 61, 91];
    const WORMHOLE_NEXUS_TILE_ID: &'static str = "82";
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
                let milty_system: MiltySystemId = s
                    .parse()
                    .wrap_err_with(|| format!("Milty system ID does not exist ({s:?})?"))?;

                let Some(system_id) = milty_system.to_system_id() else {
                    return Ok(None);
                };

                Ok(Some(systems.get(&system_id).wrap_err_with(|| {
                    format!("Milty system ID does not exist ({s:?})?")
                })?))
            })
            .collect::<eyre::Result<Vec<Option<&System>>>>()?;

        let meca = &systems[MECATOL_REX_ID];
        map.insert(0, Some(meca));

        let mut tiles = Vec::with_capacity(milty_string.len() + 1);

        let mut ring = 0;
        let mut ring_pos = 0;
        for (index, system) in map.iter().enumerate() {
            let coord = Coordinate {
                ring,
                position: ring_pos,
            };

            if Self::RING_STARTING_INDICES.contains(&(index + 1)) {
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
                .id
                .clone(),
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
            tiles.push(Tile {
                system: creuss_home_system.id.clone(),
                position: HexPosition::OutsideGalaxy,
            });
        }

        Ok(HexMap {
            tiles,
            ring_count: ring,
        })
    }
}

/// A tile in play.
#[derive(Debug, Clone)]
pub struct Tile {
    /// The id of the system.
    pub system: SystemId,
    /// Where it exists on the galactic map.
    pub position: HexPosition,
}

/// A position of a tile in the game.
#[derive(Debug, Clone)]
pub enum HexPosition {
    /// Outside of the galaxy, e.g. Creuss home system and Mallice.
    OutsideGalaxy,
    /// Inside the galactic grid.
    Pos(Coordinate),
}

/// A coordinate of a tile in the system.
#[derive(Debug, Clone)]
pub struct Coordinate {
    /// Which ring the coordinate is in (starting with mecatol rex on 0).
    pub ring: u32,
    /// Which position the tile is on starting from the column "above" mecatol rex at 0 and going clockwise around the galaxy.
    pub position: u32,
}

enum MiltySystemId {
    Number(u32),
    Hyperlane(HyperLaneTile),
    Empty,
}

impl FromStr for MiltySystemId {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u32>() {
            if n == 0 {
                return Ok(MiltySystemId::Empty);
            }

            return Ok(MiltySystemId::Number(n));
        }

        let mut chars = s.chars().collect::<Vec<char>>();
        let number = chars
            .pop()
            .wrap_err("Failed to extract number from milty system ID")?;

        let variant = chars
            .pop()
            .wrap_err("Failed to extract variant from system ID")?;

        let system_id = chars
            .into_iter()
            .collect::<String>()
            .parse::<u32>()
            .wrap_err("Failed to parse system id from milty")?;

        Ok(MiltySystemId::Hyperlane(HyperLaneTile {
            system_id,
            variant: variant.to_string(),
            number: number.into(),
        }))
    }
}

impl MiltySystemId {
    fn to_system_id(&self) -> Option<SystemId> {
        match self {
            MiltySystemId::Number(n) => Some(n.to_string()),
            MiltySystemId::Hyperlane(h) => Some(format!("{}{}", h.system_id, h.variant)),
            MiltySystemId::Empty => None,
        }
    }
}

struct HyperLaneTile {
    system_id: u32,
    variant: String,
    number: u32,
}
