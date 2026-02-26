use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::components::system::{SystemId, systems};

pub type HexMapResult<T> = Result<T, HexMapError>;

#[derive(thiserror::Error, Debug)]
pub enum HexMapError {
    #[error("Invalid system id: {0}")]
    InvalidSystemId(String),
    #[error("The system {0} did not exist in the list of systems (this is a bug on our end!)")]
    SystemDoesntExist(&'static str),
    #[error("")]
    UnsupportedSystemVariant(char),
}

/// The galactic map.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HexMap {
    /// All tiles that are in the game (at start).
    pub tiles: Vec<Tile>,
    /// How many rings there are in the galactic map (does not include tiles that are 'outside the galaxy').
    pub ring_count: u32,
}

impl HexMap {
    const RING_STARTING_INDICES: [usize; 7] = [0, 1, 7, 19, 37, 61, 91];
    const WORMHOLE_NEXUS_TILE_ID: &'static str = "82";
    const CREUSS_WORMHILE_ID: u32 = 17;
    const CREUSS_HOME_SYSTEM: &'static str = "51";

    /// The system ID for the mecatol rex system.
    const MECATOL_REX_ID: u32 = 18;
    /// The system ID for the mecatol rex system in the Thunder's Edge expansion.
    const MECATOL_REX_OMEGA_ID: u32 = 112;

    /// Create the map from a milty export string (space separated system ids).
    /// Example `87A1 89B3 47 87A4 89B0 78 37 64 46 29 72 22 24 63 44 40 23 76 50 30 48 28 43 83B2 67 69 34 27 77 26 36 74 83B2 79 19 38 53 42 59 7 0 0 14 21 0 4 39 71 15 80 68 52 0 0 17 75 0 58 41 60`
    /// Ring indicies:
    ///  - 0 = 0     |
    ///  - 1 = 1-6   |
    ///  - 2 = 7-18  |
    ///  - 3 = 19-36 |
    ///  - 4 = 37-60 |
    ///  - 5 = 61-90 |
    pub fn from_milty_string(milty_string: &str, use_te_tiles: bool) -> HexMapResult<Self> {
        let systems = systems();

        let mut map: Vec<MiltySystemId> = milty_string
            .split(' ')
            .map(|s| s.parse::<MiltySystemId>())
            .collect::<HexMapResult<Vec<MiltySystemId>>>()?;

        let mecatol_rex_id = if use_te_tiles {
            Self::MECATOL_REX_OMEGA_ID
        } else {
            Self::MECATOL_REX_ID
        };
        map.insert(
            0,
            MiltySystemId::Standard {
                number: mecatol_rex_id,
                variant: None,
            },
        );

        let mut tiles = Vec::with_capacity(milty_string.len() + 1);

        let mut ring = 0;
        let mut ring_pos = 0;
        for (index, system) in map.iter().enumerate() {
            let coord = Coordinate {
                ring,
                position: ring_pos,
                rotation: system.get_rotation(),
            };

            if Self::RING_STARTING_INDICES.contains(&(index + 1)) {
                ring += 1;
                ring_pos = 0;
            } else {
                ring_pos += 1;
            }

            let Some(system) = system.to_system_id() else {
                continue;
            };

            let tile = Tile {
                system,
                position: HexPosition::Pos(coord),
            };

            tiles.push(tile);
        }

        // TODO: This should only be done if playing with PoK?
        tiles.push(Tile {
            system: systems
                .get(Self::WORMHOLE_NEXUS_TILE_ID)
                .ok_or_else(|| HexMapError::SystemDoesntExist("Wormhole nexus"))?
                .id
                .clone(),
            position: HexPosition::OutsideGalaxy,
        });

        if map.contains(&MiltySystemId::Standard {
            number: Self::CREUSS_WORMHILE_ID,
            variant: None,
        }) {
            let creuss_home_system = systems
                .get(Self::CREUSS_HOME_SYSTEM)
                .ok_or_else(|| HexMapError::SystemDoesntExist("Creuss homesystem"))?;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tile {
    /// The id of the system.
    pub system: SystemId,
    /// Where it exists on the galactic map.
    pub position: HexPosition,
}

/// A position of a tile in the game.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HexPosition {
    /// Outside of the galaxy, e.g. Creuss home system and Mallice.
    OutsideGalaxy,
    /// Inside the galactic grid.
    Pos(Coordinate),
}

/// A coordinate of a tile in the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinate {
    /// Which ring the coordinate is in (starting with mecatol rex on 0).
    pub ring: u32,
    /// Which position the tile is on starting from the column "above" mecatol rex at 0 and going clockwise around the galaxy.
    pub position: u32,
    /// How rotated the tile is.
    pub rotation: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum MiltySystemId {
    Standard {
        number: u32,
        variant: Option<Variant>,
    },
    Hyperlane(HyperLaneTile),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
enum Variant {
    A,
    B,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Variant::A => "a",
            Variant::B => "b",
        })
    }
}

impl TryFrom<char> for Variant {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            v => Err(format!("Unsupported system variant {v}")),
        }
    }
}

impl FromStr for MiltySystemId {
    type Err = HexMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u32>() {
            if n == 0 {
                return Ok(MiltySystemId::Empty);
            }

            return Ok(MiltySystemId::Standard {
                number: n,
                variant: None,
            });
        }

        let mut chars = s.chars().collect::<Vec<char>>();

        let last_char = chars
            .pop()
            .ok_or_else(|| HexMapError::InvalidSystemId(s.to_string()))?;

        let Some(rotation) = last_char.to_digit(10) else {
            // Its the variant of a normal tile
            let number = chars
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
                .map_err(|_| HexMapError::InvalidSystemId(s.to_string()))?;

            let variant = last_char
                .try_into()
                .map_err(|_| HexMapError::InvalidSystemId(s.to_string()))?;

            return Ok(MiltySystemId::Standard {
                number,
                variant: Some(variant),
            });
        };

        let variant = chars
            .pop()
            .ok_or_else(|| HexMapError::InvalidSystemId(s.to_string()))?;

        let system_id = chars
            .into_iter()
            .collect::<String>()
            .parse::<u32>()
            .map_err(|_| HexMapError::InvalidSystemId(s.to_string()))?;

        Ok(MiltySystemId::Hyperlane(HyperLaneTile {
            system_id,
            variant: variant.to_string(),
            rotation,
        }))
    }
}

impl MiltySystemId {
    fn to_system_id(&self) -> Option<SystemId> {
        match self {
            MiltySystemId::Standard { number, variant } => {
                let var_name = variant.as_ref().map(|v| v.to_string()).unwrap_or_default();
                Some(format!("{number}{var_name}",))
            }
            MiltySystemId::Hyperlane(h) => Some(format!("{}{}", h.system_id, h.variant)),
            MiltySystemId::Empty => None,
        }
    }

    fn get_rotation(&self) -> u32 {
        match self {
            MiltySystemId::Hyperlane(h) => h.rotation,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct HyperLaneTile {
    system_id: u32,
    variant: String,
    rotation: u32,
}
