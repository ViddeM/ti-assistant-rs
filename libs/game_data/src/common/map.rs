use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{
    common::faction::Faction,
    components::system::{AnomalyType, System, SystemId, SystemType, systems},
};

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
    const THE_FRACTURE: [&'static str; 3] = ["125", "126", "127"];
    const MUAAT_SUPERNOVA: &'static str = "81";

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

        let mut hex_map = HexMap {
            tiles,
            ring_count: ring,
        };
        hex_map.place_creuss_home_system();
        hex_map.place_crimson_rebellion_home_system();
        if use_te_tiles {
            hex_map.place_the_fracture();
        }
        Ok(hex_map)
    }

    /// Places the Creuss home system outside the galaxy if the Creuss gate is on the map.
    pub fn place_creuss_home_system(&mut self) {
        let gate = Self::CREUSS_WORMHILE_ID.to_string();
        let has_system = |id: &str| self.tiles.iter().any(|tile| tile.system == id);
        if has_system(&gate) && !has_system(Self::CREUSS_HOME_SYSTEM) {
            self.tiles.push(Tile {
                system: Self::CREUSS_HOME_SYSTEM.to_string(),
                position: HexPosition::OutsideGalaxy,
            });
        }
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

    /// Places the tiles of the Fracture that aren't on the map outside the galaxy.
    pub fn place_the_fracture(&mut self) {
        for system in Self::THE_FRACTURE {
            if !self.tiles.iter().any(|tile| tile.system == system) {
                self.tiles.push(Tile {
                    system: system.to_string(),
                    position: HexPosition::OutsideGalaxy,
                });
            }
        }
    }

    /// The coordinates in the rings around Mecatol Rex, up to the outermost ring of the galaxy, that
    /// have no tile, sorted by ring and position.
    pub fn empty_galaxy_slots(&self) -> Vec<Coordinate> {
        let occupied = self
            .tiles
            .iter()
            .filter_map(|tile| match &tile.position {
                HexPosition::Pos(coord) => Some((coord.ring, coord.position)),
                HexPosition::OutsideGalaxy => None,
            })
            .collect::<HashSet<_>>();

        (1..=self.galaxy_outer_ring())
            .flat_map(|ring| (0..ring * 6).map(move |position| (ring, position)))
            .filter(|slot| !occupied.contains(slot))
            .map(|(ring, position)| Coordinate {
                ring,
                position,
                rotation: 0,
            })
            .collect()
    }

    /// Places the home system chosen by The Council Keleres in the only empty slot of the galaxy,
    /// which milty leaves for its home system, if there is exactly one and the system isn't on
    /// the map yet.
    pub fn place_keleres_home_system(&mut self, system: SystemId) {
        if self.tiles.iter().any(|tile| tile.system == system) {
            return;
        }

        let Ok([slot]) = <[Coordinate; 1]>::try_from(self.empty_galaxy_slots()) else {
            return;
        };

        self.tiles.push(Tile {
            system,
            position: HexPosition::Pos(slot),
        });
    }

    /// The system a minor faction puts in the galaxy: the Creuss gate for the Ghosts of Creuss, the
    /// Crimson Rebellion gate for the Crimson Rebellion, and the home system for the others (the
    /// Firmament side for the Firmament / The Obsidian). None for a faction without a home system
    /// of its own.
    pub fn minor_faction_system(faction: Faction) -> Option<SystemId> {
        match faction {
            Faction::GhostsOfCreuss => Some(Self::CREUSS_WORMHILE_ID.to_string()),
            Faction::CrimsonRebellion => Some(Self::CRIMSON_REBELLION_GATE.to_string()),
            _ => systems()
                .into_values()
                .find(|system| {
                    system.system_type == SystemType::HomeSystem(faction) && system.id != "96B"
                })
                .map(|system| system.id),
        }
    }

    /// The systems in the galaxy that a minor faction system can replace: every system around
    /// Mecatol Rex except home systems, hyperlanes and the Creuss and Crimson Rebellion gates,
    /// sorted by number.
    pub fn minor_faction_slots(&self) -> Vec<SystemId> {
        let systems = systems();
        let gates = [
            Self::CREUSS_WORMHILE_ID.to_string(),
            Self::CRIMSON_REBELLION_GATE.to_string(),
        ];
        let mecatol_rex = [
            Self::MECATOL_REX_ID.to_string(),
            Self::MECATOL_REX_OMEGA_ID.to_string(),
        ];
        let mut slots = self
            .tiles
            .iter()
            .filter(|tile| matches!(&tile.position, HexPosition::Pos(coord) if coord.ring >= 1))
            .filter(|tile| !gates.contains(&tile.system) && !mecatol_rex.contains(&tile.system))
            .filter(|tile| {
                systems
                    .get(&tile.system)
                    .is_some_and(|system| system.system_type == SystemType::Normal)
            })
            .map(|tile| tile.system.clone())
            .collect::<Vec<_>>();
        slots.sort_by_key(|id| id.parse::<u32>().unwrap_or(u32::MAX));
        slots
    }

    /// Replaces the system `replaced` in the galaxy with the system of the minor faction `faction`
    /// (see [Self::minor_faction_system]), and places the Creuss and Crimson Rebellion home
    /// systems outside the galaxy if their gates are on the map.
    pub fn place_minor_faction_system(&mut self, faction: Faction, replaced: &str) {
        let Some(system) = Self::minor_faction_system(faction) else {
            return;
        };

        for tile in &mut self.tiles {
            if let HexPosition::Pos(coord) = &mut tile.position
                && tile.system == replaced
            {
                coord.rotation = 0;
                tile.system = system.clone();
            }
        }

        self.place_creuss_home_system();
        self.place_crimson_rebellion_home_system();
    }

    /// The coordinate just outside the galaxy where a tile that is outside the galaxy belongs, if it
    /// belongs next to a tile in the galaxy (the Crimson Rebellion home system next to its gate).
    pub fn outside_galaxy_coordinate(&self, tile: &Tile) -> Option<Coordinate> {
        if tile.position != HexPosition::OutsideGalaxy
            || tile.system != Self::CRIMSON_REBELLION_HOME_SYSTEM
        {
            return None;
        }

        let outer_ring = self.galaxy_outer_ring();
        let gate = self.tiles.iter().find_map(|tile| match &tile.position {
            HexPosition::Pos(coord) if tile.system == Self::CRIMSON_REBELLION_GATE => Some(coord),
            _ => None,
        })?;

        let mut coord = gate.outward_neighbor();
        while coord.ring <= outer_ring {
            coord = coord.outward_neighbor();
        }
        Some(coord)
    }

    /// The outermost ring of the galaxy: the outermost ring with a tile, but not past the rings
    /// imported from milty, so tiles placed outside them (Age of Exploration) don't make the
    /// galaxy bigger.
    pub fn galaxy_outer_ring(&self) -> u32 {
        self.tiles
            .iter()
            .filter_map(|tile| match &tile.position {
                HexPosition::Pos(coord) => Some(coord.ring),
                HexPosition::OutsideGalaxy => None,
            })
            .max()
            .unwrap_or(0)
            .min(self.ring_count.saturating_sub(1))
    }

    /// Whether the system is a red or blue tile that isn't on the map and isn't one of the
    /// special tiles (Mecatol Rex, the Creuss gate, the Muaat supernova, the Wormhole Nexus, the
    /// Crimson Rebellion gate and the Fracture), i.e. a tile that can be drawn for Age of
    /// Exploration.
    pub fn is_unused_red_or_blue_tile(&self, system: &System) -> bool {
        let id = system.id.as_str();
        let is_special = [
            Self::MECATOL_REX_ID,
            Self::MECATOL_REX_OMEGA_ID,
            Self::CREUSS_WORMHILE_ID,
        ]
        .iter()
        .any(|number| id == number.to_string())
            || [
                Self::MUAAT_SUPERNOVA,
                Self::WORMHOLE_NEXUS_TILE_ID,
                Self::CRIMSON_REBELLION_GATE,
            ]
            .contains(&id)
            || Self::THE_FRACTURE.contains(&id);

        system.system_type == SystemType::Normal
            && !is_special
            && !self.tiles.iter().any(|tile| tile.system == system.id)
    }

    /// Whether a tile on the map is an anomaly of the given type.
    pub fn has_anomaly(&self, anomaly: &AnomalyType) -> bool {
        let systems = systems();
        self.tiles.iter().any(|tile| {
            systems
                .get(&tile.system)
                .is_some_and(|system| system.anomalies.contains(anomaly))
        })
    }

    /// The empty coordinates that touch at least 2 tiles that aren't home systems or hyperlanes,
    /// where a tile drawn for Age of Exploration can be placed, sorted by ring and position. The
    /// home systems of `minor_factions` are minor faction systems, so they count.
    pub fn exploration_positions(&self, minor_factions: &BTreeSet<Faction>) -> Vec<Coordinate> {
        let systems = systems();
        let galaxy = self
            .tiles
            .iter()
            .filter_map(|tile| match &tile.position {
                HexPosition::Pos(coord) => Some((tile, (coord.ring, coord.position))),
                HexPosition::OutsideGalaxy => None,
            })
            .collect::<Vec<_>>();
        let occupied = galaxy.iter().map(|(_, slot)| *slot).collect::<HashSet<_>>();
        let counting = galaxy
            .iter()
            .filter(|(tile, _)| {
                systems
                    .get(&tile.system)
                    .is_some_and(|system| match &system.system_type {
                        SystemType::HomeSystem(faction) => minor_factions.contains(faction),
                        SystemType::Hyperlane => false,
                        SystemType::Normal => true,
                    })
            })
            .map(|(_, slot)| *slot)
            .collect::<HashSet<_>>();
        let touches_counting = |coord: &Coordinate| {
            coord
                .neighbors()
                .iter()
                .filter(|neighbor| counting.contains(&(neighbor.ring, neighbor.position)))
                .count()
        };

        counting
            .iter()
            .flat_map(|&(ring, position)| {
                Coordinate {
                    ring,
                    position,
                    rotation: 0,
                }
                .neighbors()
            })
            .filter(|coord| !occupied.contains(&(coord.ring, coord.position)))
            .filter(|coord| touches_counting(coord) >= 2)
            .map(|coord| (coord.ring, coord.position))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|(ring, position)| Coordinate {
                ring,
                position,
                rotation: 0,
            })
            .collect()
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Coordinate {
    /// Which ring the coordinate is in (starting with mecatol rex on 0).
    pub ring: u32,
    /// Which position the tile is on starting from the column "above" mecatol rex at 0 and going clockwise around the galaxy.
    pub position: u32,
    /// How rotated the tile is.
    pub rotation: u32,
}

/// The axial directions to the neighbors of a hex, clockwise from the one directly above it.
const DIRECTIONS: [(i32, i32); 6] = [(0, 1), (1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1)];

impl Coordinate {
    /// The 6 unrotated coordinates around this one.
    pub fn neighbors(&self) -> Vec<Coordinate> {
        let (q, r) = self.to_axial();
        DIRECTIONS
            .iter()
            .map(|(dq, dr)| Coordinate::from_axial((q + dq, r + dr)))
            .collect()
    }

    /// The axial coordinates of the hex: Mecatol Rex is `(0, 0)`, `(0, 1)` is directly above it
    /// and `(1, 0)` is above it to the right.
    fn to_axial(&self) -> (i32, i32) {
        if self.ring == 0 {
            return (0, 0);
        }
        let ring = self.ring as i32;
        let position = self.position as i32;
        let side = (position / ring) as usize;
        let (corner_q, corner_r) = DIRECTIONS[side % 6];
        let (step_q, step_r) = DIRECTIONS[(side + 2) % 6];
        let steps = position % ring;
        (
            corner_q * ring + step_q * steps,
            corner_r * ring + step_r * steps,
        )
    }

    fn from_axial((q, r): (i32, i32)) -> Coordinate {
        let ring = ((q.abs() + r.abs() + (q + r).abs()) / 2) as u32;
        let position = (0..ring * 6)
            .find(|&position| {
                Coordinate {
                    ring,
                    position,
                    rotation: 0,
                }
                .to_axial()
                    == (q, r)
            })
            .unwrap_or(0);
        Coordinate {
            ring,
            position,
            rotation: 0,
        }
    }

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
    use std::collections::BTreeSet;

    use crate::{
        common::faction::Faction,
        components::system::{AnomalyType, systems},
    };

    use super::{Coordinate, HexMap, HexPosition, MiltySystemId, Tile};

    fn coordinate(ring: u32, position: u32) -> Coordinate {
        Coordinate {
            ring,
            position,
            rotation: 0,
        }
    }

    fn slots(coordinates: &[Coordinate]) -> Vec<(u32, u32)> {
        coordinates
            .iter()
            .map(|coord| (coord.ring, coord.position))
            .collect()
    }

    fn coordinates_up_to_ring(rings: u32) -> impl Iterator<Item = Coordinate> {
        (0..=rings)
            .flat_map(|ring| (0..(ring * 6).max(1)).map(move |position| coordinate(ring, position)))
    }

    #[test]
    fn neighbors_of_the_tile_above_mecatol_rex() {
        assert_eq!(
            slots(&coordinate(1, 0).neighbors())
                .into_iter()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([(0, 0), (1, 1), (1, 5), (2, 0), (2, 1), (2, 11)])
        );
    }

    #[test]
    fn neighbors_are_six_adjacent_coordinates_both_ways() {
        for coord in coordinates_up_to_ring(5) {
            let neighbors = coord.neighbors();
            let distinct = slots(&neighbors).into_iter().collect::<BTreeSet<_>>();
            assert_eq!(distinct.len(), 6, "{coord:?} has {neighbors:?}");

            for neighbor in neighbors {
                assert!(
                    coord.ring.abs_diff(neighbor.ring) <= 1,
                    "{neighbor:?} is next to {coord:?}"
                );
                assert!(
                    neighbor.neighbors().contains(&coord),
                    "{neighbor:?} is next to {coord:?}, but not the other way around"
                );
            }
        }
    }

    /// Mecatol Rex and a ring 1 of tile 19, with `ring_one` (or no tile) at ring 1 position 0.
    fn mecatol_rex_and_ring_one(ring_one: Option<&str>) -> HexMap {
        let tiles = coordinates_up_to_ring(1)
            .filter_map(|coord| {
                let system = match (coord.ring, coord.position) {
                    (0, _) => Some("18"),
                    (1, 0) => ring_one,
                    _ => Some("19"),
                };
                system.map(|system| Tile {
                    system: system.to_string(),
                    position: HexPosition::Pos(coord),
                })
            })
            .collect();
        HexMap {
            tiles,
            ring_count: 2,
        }
    }

    #[test]
    fn exploration_positions_touch_two_systems() {
        assert_eq!(
            slots(&mecatol_rex_and_ring_one(Some("20")).exploration_positions(&BTreeSet::new())),
            [(2, 1), (2, 3), (2, 5), (2, 7), (2, 9), (2, 11)]
        );
    }

    #[test]
    fn minor_faction_systems_count_for_exploration_positions() {
        let hex_map = mecatol_rex_and_ring_one(Some("5"));
        assert_eq!(
            slots(&hex_map.exploration_positions(&BTreeSet::from([Faction::Arborec]))),
            [(2, 1), (2, 3), (2, 5), (2, 7), (2, 9), (2, 11)]
        );
        assert_eq!(
            slots(&hex_map.exploration_positions(&BTreeSet::from([Faction::FederationOfSol]))),
            [(2, 3), (2, 5), (2, 7), (2, 9)]
        );
    }

    #[test]
    fn home_systems_and_hyperlanes_dont_count_for_exploration_positions() {
        for system in ["1", "83A"] {
            assert_eq!(
                slots(
                    &mecatol_rex_and_ring_one(Some(system)).exploration_positions(&BTreeSet::new())
                ),
                [(2, 3), (2, 5), (2, 7), (2, 9)],
                "with {system} in ring 1"
            );
        }
    }

    #[test]
    fn a_gap_in_the_galaxy_is_an_exploration_position() {
        assert_eq!(
            slots(&mecatol_rex_and_ring_one(None).exploration_positions(&BTreeSet::new())),
            [(1, 0), (2, 3), (2, 5), (2, 7), (2, 9)]
        );
    }

    #[test]
    fn empty_galaxy_slots_are_the_slots_without_a_tile() {
        assert_eq!(
            slots(&mecatol_rex_and_ring_one(None).empty_galaxy_slots()),
            [(1, 0)]
        );
        assert!(
            mecatol_rex_and_ring_one(Some("20"))
                .empty_galaxy_slots()
                .is_empty()
        );
    }

    #[test]
    fn keleres_home_system_is_placed_in_the_only_empty_slot() {
        let mut hex_map = mecatol_rex_and_ring_one(None);

        hex_map.place_keleres_home_system("14".to_string());

        assert!(hex_map.tiles.contains(&Tile {
            system: "14".to_string(),
            position: HexPosition::Pos(coordinate(1, 0)),
        }));
        assert!(hex_map.empty_galaxy_slots().is_empty());
    }

    #[test]
    fn keleres_home_system_is_not_placed_with_two_empty_slots() {
        let mut hex_map = mecatol_rex_and_ring_one(None);
        hex_map
            .tiles
            .retain(|tile| tile.position != HexPosition::Pos(coordinate(1, 3)));
        let before = hex_map.clone();

        hex_map.place_keleres_home_system("14".to_string());

        assert_eq!(hex_map, before);
    }

    #[test]
    fn keleres_home_system_is_not_placed_twice() {
        let mut hex_map = mecatol_rex_and_ring_one(None);
        for tile in &mut hex_map.tiles {
            if tile.position == HexPosition::Pos(coordinate(1, 3)) {
                tile.system = "14".to_string();
            }
        }
        let before = hex_map.clone();

        hex_map.place_keleres_home_system("14".to_string());

        assert_eq!(hex_map, before);
    }

    fn positions(hex_map: &HexMap, system: &str) -> Vec<HexPosition> {
        hex_map
            .tiles
            .iter()
            .filter(|tile| tile.system == system)
            .map(|tile| tile.position.clone())
            .collect()
    }

    #[test]
    fn minor_faction_slots_are_the_replaceable_systems_in_the_galaxy() {
        let hex_map = HexMap::from_milty_string("41 17 94 1 83A0 26 18", true).expect("to parse");
        assert!(positions(&hex_map, "82").contains(&HexPosition::OutsideGalaxy));
        assert!(positions(&hex_map, "125").contains(&HexPosition::OutsideGalaxy));

        assert_eq!(hex_map.minor_faction_slots(), ["26", "41"]);
    }

    #[test]
    fn a_minor_faction_system_replaces_a_system() {
        let mut hex_map = HexMap::from_milty_string("26 27 28 29 30 31", true).expect("to parse");

        hex_map.place_minor_faction_system(Faction::Arborec, "27");

        assert_eq!(
            positions(&hex_map, "5"),
            [HexPosition::Pos(coordinate(1, 1))]
        );
        assert_eq!(positions(&hex_map, "27"), []);
    }

    #[test]
    fn minor_faction_systems() {
        for (faction, system, outside_galaxy) in [
            (Faction::GhostsOfCreuss, "17", Some("51")),
            (Faction::CrimsonRebellion, "94", Some("118")),
            (Faction::FirmamentObsidian, "96A", None),
        ] {
            let mut hex_map =
                HexMap::from_milty_string("26 27 28 29 30 31", true).expect("to parse");

            hex_map.place_minor_faction_system(faction, "27");

            assert_eq!(
                positions(&hex_map, system),
                [HexPosition::Pos(coordinate(1, 1))],
                "{faction:?}"
            );
            if let Some(outside_galaxy) = outside_galaxy {
                assert_eq!(
                    positions(&hex_map, outside_galaxy),
                    [HexPosition::OutsideGalaxy],
                    "{faction:?}"
                );
            }
        }
        assert_eq!(HexMap::minor_faction_system(Faction::CouncilKeleres), None);
    }

    #[test]
    fn unused_red_or_blue_tiles() {
        let systems = systems();
        let hex_map = mecatol_rex_and_ring_one(Some("20"));
        let is_unused = |id: &str| hex_map.is_unused_red_or_blue_tile(&systems[id]);

        for (id, what) in [
            ("19", "a tile on the map"),
            ("1", "a home system"),
            ("83A", "a hyperlane"),
            ("18", "Mecatol Rex"),
            ("112", "Mecatol Rex"),
            ("17", "the Creuss gate"),
            ("81", "the Muaat supernova"),
            ("82", "the Wormhole Nexus"),
            ("94", "the Crimson Rebellion gate"),
            ("125", "the Fracture"),
            ("126", "the Fracture"),
            ("127", "the Fracture"),
        ] {
            assert!(!is_unused(id), "{id} is {what}");
        }
        assert!(is_unused("26"), "26 is an unused blue tile");
        assert!(is_unused("41"), "41 is an unused red tile");
    }

    #[test]
    fn tiles_outside_the_imported_rings_are_outside_the_galaxy() {
        let three_rings = (19..55)
            .map(|system| system.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let mut hex_map = HexMap::from_milty_string(&three_rings, true).expect("to parse");
        assert_eq!(hex_map.galaxy_outer_ring(), 3);

        hex_map.tiles.push(Tile {
            system: "59".to_string(),
            position: HexPosition::Pos(coordinate(4, 0)),
        });

        assert_eq!(hex_map.galaxy_outer_ring(), 3);
    }

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

    fn fracture_tiles(hex_map: &HexMap) -> Vec<(&str, &HexPosition)> {
        hex_map
            .tiles
            .iter()
            .filter(|tile| HexMap::THE_FRACTURE.contains(&tile.system.as_str()))
            .map(|tile| (tile.system.as_str(), &tile.position))
            .collect()
    }

    #[test]
    fn the_fracture_is_outside_the_galaxy_with_thunders_edge() {
        let hex_map = HexMap::from_milty_string("26 27 28 29 30 31", true).expect("to parse");

        assert_eq!(
            fracture_tiles(&hex_map),
            vec![
                ("125", &HexPosition::OutsideGalaxy),
                ("126", &HexPosition::OutsideGalaxy),
                ("127", &HexPosition::OutsideGalaxy),
            ]
        );
    }

    #[test]
    fn the_fracture_is_not_on_the_map_without_thunders_edge() {
        let hex_map = HexMap::from_milty_string("26 27 28 29 30 31", false).expect("to parse");

        assert_eq!(fracture_tiles(&hex_map), vec![]);
    }

    #[test]
    fn missing_fracture_tiles_are_added() {
        let mut hex_map = HexMap::from_milty_string("26 27 28 29 30 31", true).expect("to parse");
        hex_map.tiles.retain(|tile| tile.system != "126");

        hex_map.place_the_fracture();

        assert_eq!(
            fracture_tiles(&hex_map),
            vec![
                ("125", &HexPosition::OutsideGalaxy),
                ("127", &HexPosition::OutsideGalaxy),
                ("126", &HexPosition::OutsideGalaxy),
            ]
        );
    }

    #[test]
    fn the_fracture_is_not_added_twice() {
        let mut hex_map = HexMap::from_milty_string("26 27 28 29 30 31", true).expect("to parse");
        let before = hex_map.clone();

        hex_map.place_the_fracture();

        assert_eq!(hex_map, before);
    }

    #[test]
    fn the_fracture_systems_exist() {
        let systems = systems();
        for system in HexMap::THE_FRACTURE {
            assert_eq!(
                systems.get(system).map(|system| &system.anomalies),
                Some(&vec![AnomalyType::TheFracture]),
                "{system}"
            );
        }
    }
}
