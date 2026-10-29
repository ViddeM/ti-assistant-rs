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
    const CRIMSON_REBELLION_GATE: &'static str = "94";
    const CRIMSON_REBELLION_HOME_SYSTEM: &'static str = "118";

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
                .ok_or(HexMapError::SystemDoesntExist("Wormhole nexus"))?
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
                .ok_or(HexMapError::SystemDoesntExist("Creuss homesystem"))?;

            tiles.push(Tile {
                system: creuss_home_system.id.clone(),
                position: HexPosition::OutsideGalaxy,
            });
        }

        let mut hex_map = HexMap {
            tiles,
            ring_count: ring,
        };
        hex_map.place_crimson_rebellion_home_system();
        Ok(hex_map)
    }

    /// Places the Crimson Rebellion home system outside the galaxy if its gate is on the map.
    pub fn place_crimson_rebellion_home_system(&mut self) {
        let has_system = |id: &str| self.tiles.iter().any(|tile| tile.system == id);
        if has_system(Self::CRIMSON_REBELLION_GATE)
            && !has_system(Self::CRIMSON_REBELLION_HOME_SYSTEM)
        {
            self.tiles.push(Tile {
                system: Self::CRIMSON_REBELLION_HOME_SYSTEM.to_string(),
                position: HexPosition::OutsideGalaxy,
            });
        }
    }

    /// The coordinate just outside the galaxy where a tile that is outside the galaxy belongs, if it
    /// belongs next to a tile in the galaxy (the Crimson Rebellion home system next to its gate).
    pub fn outside_galaxy_coordinate(&self, tile: &Tile) -> Option<Coordinate> {
        if tile.position != HexPosition::OutsideGalaxy
            || tile.system != Self::CRIMSON_REBELLION_HOME_SYSTEM
        {
            return None;
        }

        let galaxy = self.tiles.iter().filter_map(|tile| match &tile.position {
            HexPosition::Pos(coord) => Some((&tile.system, coord)),
            HexPosition::OutsideGalaxy => None,
        });
        let outer_ring = galaxy.clone().map(|(_, coord)| coord.ring).max()?;
        let (_, gate) = galaxy
            .clone()
            .find(|(system, _)| *system == Self::CRIMSON_REBELLION_GATE)?;

        let mut coord = gate.outward_neighbor();
        while coord.ring <= outer_ring {
            coord = coord.outward_neighbor();
        }
        Some(coord)
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

impl Coordinate {
    /// An unrotated neighbor in the next ring out.
    fn outward_neighbor(&self) -> Coordinate {
        let ring = self.ring + 1;
        let position = match self.ring {
            0 => 0,
            r => (self.position * ring + r / 2) / r,
        };
        Coordinate {
            ring,
            position,
            rotation: 0,
        }
    }
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
            Variant::A => "A",
            Variant::B => "B",
        })
    }
}

impl TryFrom<char> for Variant {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(Self::A),
            'b' | 'B' => Ok(Self::B),
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

#[cfg(test)]
mod test {
    use crate::components::system::systems;

    use super::{HexMap, HexPosition, MiltySystemId};

    #[test]
    fn milty_variants_match_system_ids() {
        let systems = systems();
        for milty_id in ["96a", "96A", "96b", "96B"] {
            let id = milty_id
                .parse::<MiltySystemId>()
                .expect("to parse")
                .to_system_id()
                .expect("to have a system id");
            assert!(
                systems.contains_key(&id),
                "{milty_id} became {id}, which is not a known system"
            );
        }
    }

    #[test]
    fn crimson_rebellion_home_system_is_outside_the_galaxy_with_its_gate() {
        let crimson_rebellion_home_is_outside = |milty_string: &str| {
            HexMap::from_milty_string(milty_string, true)
                .expect("to parse")
                .tiles
                .iter()
                .any(|tile| tile.system == "118" && tile.position == HexPosition::OutsideGalaxy)
        };

        assert!(crimson_rebellion_home_is_outside("94 26 27 28 29 30"));
        assert!(!crimson_rebellion_home_is_outside("93 26 27 28 29 30"));
    }

    fn crimson_rebellion_home_systems(hex_map: &HexMap) -> Vec<&HexPosition> {
        hex_map
            .tiles
            .iter()
            .filter(|tile| tile.system == "118")
            .map(|tile| &tile.position)
            .collect()
    }

    #[test]
    fn crimson_rebellion_home_system_is_added_to_maps_without_it() {
        let mut hex_map = HexMap::from_milty_string("94 26 27 28 29 30", true).expect("to parse");
        hex_map.tiles.retain(|tile| tile.system != "118");

        hex_map.place_crimson_rebellion_home_system();

        assert_eq!(
            crimson_rebellion_home_systems(&hex_map),
            vec![&HexPosition::OutsideGalaxy]
        );
    }

    #[test]
    fn crimson_rebellion_home_system_is_not_added_twice() {
        let mut hex_map = HexMap::from_milty_string("94 26 27 28 29 30", true).expect("to parse");

        hex_map.place_crimson_rebellion_home_system();

        assert_eq!(
            crimson_rebellion_home_systems(&hex_map),
            vec![&HexPosition::OutsideGalaxy]
        );
    }

    #[test]
    fn crimson_rebellion_home_system_is_next_to_its_gate() {
        let home_system_coordinate = |milty_string: &str| {
            let hex_map = HexMap::from_milty_string(milty_string, true).expect("to parse");
            let home_system = hex_map
                .tiles
                .iter()
                .find(|tile| tile.system == "118")
                .expect("to have the home system");
            hex_map
                .outside_galaxy_coordinate(home_system)
                .map(|coord| (coord.ring, coord.position))
        };

        assert_eq!(
            home_system_coordinate("26 27 28 29 30 31 32 33 34 35 94 36 37 38 39 40 41 42"),
            Some((3, 6))
        );
        assert_eq!(
            home_system_coordinate("26 27 28 29 30 31 32 33 34 94 35 36 37 38 39 40 41 42"),
            Some((3, 5))
        );
        assert_eq!(
            home_system_coordinate("26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 94"),
            Some((3, 17))
        );
        assert_eq!(
            home_system_coordinate("26 27 94 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43"),
            Some((3, 6))
        );
    }

    #[test]
    fn only_the_crimson_rebellion_home_system_has_an_outside_galaxy_coordinate() {
        let hex_map = HexMap::from_milty_string("94 26 27 28 29 30", true).expect("to parse");

        let systems_with_coordinates = hex_map
            .tiles
            .iter()
            .filter(|tile| hex_map.outside_galaxy_coordinate(tile).is_some())
            .map(|tile| tile.system.as_str())
            .collect::<Vec<_>>();

        assert_eq!(systems_with_coordinates, vec!["118"]);
    }

    #[test]
    fn crimson_rebellion_systems_exist() {
        let systems = systems();
        assert!(systems.contains_key(HexMap::CRIMSON_REBELLION_GATE));
        assert!(systems.contains_key(HexMap::CRIMSON_REBELLION_HOME_SYSTEM));
    }
}
