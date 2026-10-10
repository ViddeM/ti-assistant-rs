use std::collections::{HashMap, HashSet};

use ti_helper_game_data::{
    common::{
        color::Color,
        map::{Coordinate, HexPosition},
    },
    components::{
        planet::Planet,
        planet_attachment::PlanetAttachment,
        system::{System, SystemId, systems},
    },
    state::game_state::GameState,
};

use crate::components::ti_token::TiToken;

use super::planet_offset::planet_offset;

pub const TILE_WIDTH: f32 = 364.0;
pub const TILE_HEIGHT: f32 = 317.0;

const TILE_THREE_QUARTER_WIDTH: f32 = TILE_WIDTH * 0.75;
const TILE_HALF_HEIGHT: f32 = TILE_HEIGHT * 0.5;

/// How much each step of rotation rotates a tile (clockwise).
const ROTATION_STEP_DEGREES: f32 = 60.0;

/// Offset of the owner label relative to the planet position (fraction of tile size).
const OWNER_LABEL_OFFSET: (f32, f32) = (0.0, 0.08);

// We can have multiple attachments for a single planet, positions for each.
const ATTACHMENT_OFFSETS: [(f32, f32); 5] = [
    (-0.12, 0.0),
    (-0.07, -0.06),
    (0.0, -0.1),
    (0.07, -0.06),
    (0.12, 0.0),
];
const ATTACHMENT_TOKEN_SIZE: (f32, f32) = (512.0 * 0.06, 512.0 * 0.06);

const MIRAGE_TOKEN_SIZE: (f32, f32) = (498.0 * 0.3, 448.0 * 0.3);
const DESTROYED_PLANET_TOKEN_SIZE: (f32, f32) = (512.0 * 0.2, 512.0 * 0.2);

/// A fully laid out map. All coordinates are in SVG space (x right, y down) with the origin at the center of the galaxy.
#[derive(Debug, Clone, PartialEq)]
pub struct MapLayout {
    pub tiles: Vec<TileLayout>,
    pub tokens: Vec<TokenLayout>,
    pub owners: Vec<OwnerLabel>,
    /// Centers of the positions inside the galaxy that have no tile, so they can be shown as placeholders.
    pub empty_slots: Vec<(f32, f32)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileLayout {
    pub system_id: SystemId,
    pub x: f32,
    pub y: f32,
    pub rotation_degrees: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenLayout {
    pub kind: TiToken,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OwnerLabel {
    pub planet: Planet,
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub color: Color,
}

/// The rectangle (in SVG space) that contains the whole map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Bounds {
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    pub fn center(&self) -> (f32, f32) {
        (
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }
}

impl MapLayout {
    pub fn bounds(&self) -> Bounds {
        let mut bounds = Bounds {
            min_x: f32::MAX,
            min_y: f32::MAX,
            max_x: f32::MIN,
            max_y: f32::MIN,
        };

        for tile in self.tiles.iter() {
            bounds.min_x = bounds.min_x.min(tile.x - TILE_WIDTH / 2.0);
            bounds.max_x = bounds.max_x.max(tile.x + TILE_WIDTH / 2.0);
            bounds.min_y = bounds.min_y.min(tile.y - TILE_HEIGHT / 2.0);
            bounds.max_y = bounds.max_y.max(tile.y + TILE_HEIGHT / 2.0);
        }

        if self.tiles.is_empty() {
            return Bounds {
                min_x: -TILE_WIDTH / 2.0,
                min_y: -TILE_HEIGHT / 2.0,
                max_x: TILE_WIDTH / 2.0,
                max_y: TILE_HEIGHT / 2.0,
            };
        }

        bounds
    }
}

/// Lays out the map for the provided game state. Returns `None` if the game has no map (i.e. it wasn't imported from milty).
pub fn build_layout(game_state: &GameState) -> Option<MapLayout> {
    let milty_information = game_state.map_data.milty_information.as_ref()?;

    let planet_owner = game_state
        .players
        .values()
        .flat_map(|player| {
            player
                .planets
                .keys()
                .cloned()
                .map(move |planet| (planet, player))
        })
        .collect::<HashMap<Planet, _>>();

    let planet_attachments = game_state
        .players
        .values()
        .flat_map(|p| &p.planets)
        .filter(|(_, attachments)| !attachments.is_empty())
        .collect::<HashMap<&Planet, &HashSet<PlanetAttachment>>>();

    let mut system_planets: HashMap<SystemId, Vec<Planet>> = systems()
        .into_iter()
        .map(|(system_id, system)| (system_id, system.planets))
        .collect();

    if let Some(system) = milty_information.mirage_system.as_ref() {
        system_planets
            .entry(system.clone())
            .or_default()
            .push(Planet::Mirage);
    }

    let mut destroyed_planets_in_system: HashMap<SystemId, Vec<Planet>> = HashMap::new();
    for planet in game_state
        .map_data
        .stellar_converter_destroyed_planets
        .iter()
    {
        // A planet that doesn't belong to a (single) system can't be shown on the map.
        if let Ok(system) = System::for_planet(planet) {
            destroyed_planets_in_system
                .entry(system.id)
                .or_default()
                .push(planet.clone());
        }
    }

    let mut tiles = Vec::new();
    let mut tokens = Vec::new();
    let mut owners = Vec::new();

    let mut occupied_slots = HashSet::new();
    let mut outer_ring = 0;

    let mut outside_galaxy_count = 0;
    for tile in milty_information.hex_map.tiles.iter() {
        let (tile_pos, rotation) =
            get_tile_pos_and_rotation(&tile.position, &mut outside_galaxy_count);

        if let HexPosition::Pos(coord) = &tile.position {
            occupied_slots.insert((coord.ring, coord.position));
            outer_ring = outer_ring.max(coord.ring);
        }

        let (x, y) = to_svg(tile_pos_to_visual_pos(tile_pos));
        tiles.push(TileLayout {
            system_id: tile.system.clone(),
            x,
            y,
            rotation_degrees: rotation * ROTATION_STEP_DEGREES,
        });

        for planet in system_planets.get(&tile.system).into_iter().flatten() {
            let Some(owner) = planet_owner.get(planet) else {
                continue;
            };
            let offset = planet_offset(planet);
            let (x, y) = to_svg(tile_with_offset_to_visual_pos(
                tile_pos,
                (
                    offset.0 + OWNER_LABEL_OFFSET.0,
                    offset.1 + OWNER_LABEL_OFFSET.1,
                ),
            ));
            owners.push(OwnerLabel {
                planet: planet.clone(),
                x,
                y,
                name: owner.name.clone(),
                color: owner.color,
            });

            if let Some(attachments) = planet_attachments.get(planet) {
                for (attachment_index, attachment) in attachments.iter().enumerate() {
                    let offset = planet_offset(planet);
                    let (x, y) = to_svg(tile_with_offset_to_visual_pos(
                        tile_pos,
                        (
                            offset.0 + ATTACHMENT_OFFSETS[attachment_index].0,
                            offset.1 + ATTACHMENT_OFFSETS[attachment_index].1,
                        ),
                    ));
                    tokens.push(TokenLayout {
                        kind: TiToken::from(attachment),
                        x,
                        y,
                        width: ATTACHMENT_TOKEN_SIZE.0,
                        height: ATTACHMENT_TOKEN_SIZE.1,
                    });
                }
            }
        }

        if milty_information.mirage_system.as_ref() == Some(&tile.system) {
            let (x, y) = to_svg(tile_with_offset_to_visual_pos(
                tile_pos,
                planet_offset(&Planet::Mirage),
            ));
            tokens.push(TokenLayout {
                kind: TiToken::Mirage,
                x,
                y,
                width: MIRAGE_TOKEN_SIZE.0,
                height: MIRAGE_TOKEN_SIZE.1,
            });
        }

        for planet in destroyed_planets_in_system
            .get(&tile.system)
            .into_iter()
            .flatten()
        {
            let (x, y) = to_svg(tile_with_offset_to_visual_pos(
                tile_pos,
                planet_offset(planet),
            ));
            tokens.push(TokenLayout {
                kind: TiToken::DestroyedPlanet,
                x,
                y,
                width: DESTROYED_PLANET_TOKEN_SIZE.0,
                height: DESTROYED_PLANET_TOKEN_SIZE.1,
            });
        }
    }

    let empty_slots = (1..=outer_ring)
        .flat_map(|ring| (0..ring * 6).map(move |position| (ring, position)))
        .filter(|slot| !occupied_slots.contains(slot))
        .map(|(ring, position)| {
            to_svg(tile_pos_to_visual_pos(get_tile_position(&Coordinate {
                ring,
                position,
                rotation: 0,
            })))
        })
        .collect();

    Some(MapLayout {
        tiles,
        tokens,
        owners,
        empty_slots,
    })
}

/// The layout math is done with y pointing up, SVG has y pointing down.
fn to_svg((x, y): (f32, f32)) -> (f32, f32) {
    (x, -y)
}

fn get_tile_pos_and_rotation(
    position: &HexPosition,
    outside_galaxy_count: &mut i32,
) -> ((f32, f32), f32) {
    match position {
        HexPosition::OutsideGalaxy => {
            let pos = (-5.5, -2.0 + 4.0 * (*outside_galaxy_count as f32));

            *outside_galaxy_count += 1;

            (pos, 0.0)
        }
        HexPosition::Pos(coord) => (get_tile_position(coord), coord.rotation as f32),
    }
}

fn tile_pos_to_visual_pos(tile_pos: (f32, f32)) -> (f32, f32) {
    (
        tile_pos.0 * TILE_THREE_QUARTER_WIDTH,
        tile_pos.1 * TILE_HEIGHT + (TILE_HEIGHT / 2.0)
            - if (tile_pos.0 as i32) % 2 == 0 {
                TILE_HALF_HEIGHT
            } else {
                0.0
            },
    )
}

fn tile_with_offset_to_visual_pos(tile_pos: (f32, f32), offset: (f32, f32)) -> (f32, f32) {
    let (x, y) = tile_pos_to_visual_pos(tile_pos);
    (x + offset.0 * TILE_WIDTH, y + offset.1 * TILE_HEIGHT)
}

/// Returns the position of the tile in the (hex) grid, with the center tile at (0, 0) and y pointing up.
fn get_tile_position(coord: &Coordinate) -> (f32, f32) {
    if coord.ring == 0 {
        return (0.0, 0.0);
    }

    let radius = coord.ring as i32;
    let ring_pos = coord.position as i32;

    let full = radius * 6;
    let half = radius * 3;
    let quarter = ((radius as f32) * 1.5).ceil() as i32;

    let x_right_half = ring_pos % half;
    let x_offset = if x_right_half >= quarter {
        half - x_right_half
    } else {
        x_right_half
    };

    // The x value increases for each position in the top-right quadrant until we reach the 'radius' width, then it goes down from there.
    let x_absolute = x_offset.min(radius);
    let x = if ring_pos <= half {
        x_absolute
    } else {
        -x_absolute
    };

    // Lets start by transforming everything into the top-right quadrant.
    let y_right_half = if ring_pos > half {
        full - ring_pos
    } else {
        ring_pos
    };

    let half_steps_top = y_right_half.min(radius);
    let half_steps_bottom = (radius - (half - y_right_half)).max(0).min(radius);
    let half_steps = half_steps_bottom + half_steps_top;

    let full_steps = (y_right_half - half_steps).max(0);

    let y_steps = (half_steps as f32 / 2.0).ceil() as i32 + full_steps;

    let y = radius - y_steps;

    (x as f32, y as f32)
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;
    use ti_helper_game_data::{common::map::Coordinate, components::planet::Planet};

    use super::{get_tile_position, planet_offset};

    #[test]
    fn center_maps_correctly() {
        test_tile(0, 0, 0, 0)
    }

    #[test]
    fn spokes_maps_correctly() {
        // Above center
        test_tile(1, 0, 0, 1);
        test_tile(2, 0, 0, 2);
        test_tile(3, 0, 0, 3);
        test_tile(4, 0, 0, 4);
        test_tile(5, 0, 0, 5);

        // Below center
        test_tile(1, 3, 0, -1);
        test_tile(2, 6, 0, -2);
        test_tile(3, 9, 0, -3);
        test_tile(4, 12, 0, -4);
        test_tile(5, 15, 0, -5);

        // Up-right
        test_tile(1, 1, 1, 0);
        test_tile(2, 2, 2, 1);
        test_tile(3, 3, 3, 1);
        test_tile(4, 4, 4, 2);
        test_tile(5, 5, 5, 2);

        // Down-right
        test_tile(1, 2, 1, -1);
        test_tile(2, 4, 2, -1);
        test_tile(3, 6, 3, -2);
        test_tile(4, 8, 4, -2);
        test_tile(5, 10, 5, -3);

        // Down-left
        test_tile(1, 4, -1, -1);
        test_tile(2, 8, -2, -1);
        test_tile(3, 12, -3, -2);
        test_tile(4, 16, -4, -2);
        test_tile(5, 20, -5, -3);

        // Up-left
        test_tile(1, 5, -1, 0);
        test_tile(2, 10, -2, 1);
        test_tile(3, 15, -3, 1);
        test_tile(4, 20, -4, 2);
        test_tile(5, 25, -5, 2);
    }

    #[test]
    fn off_spoke_maps_correctly() {
        test_tile(2, 1, 1, 1);
        test_tile(2, 5, 1, -2);
        test_tile(3, 7, 2, -2);
        test_tile(4, 18, -4, 0);
    }

    #[test]
    fn top_right_quadrant_maps_correctly() {
        test_tile(1, 0, 0, 1);
        test_tile(1, 1, 1, 0);
        test_tile(2, 0, 0, 2);
        test_tile(2, 1, 1, 1);
        test_tile(2, 2, 2, 1);
        test_tile(2, 3, 2, 0);
        test_tile(3, 0, 0, 3);
        test_tile(3, 1, 1, 2);
        test_tile(3, 3, 3, 1);
        test_tile(3, 4, 3, 0);
    }

    #[test]
    fn every_planet_has_an_offset() {
        for planet in Planet::iter() {
            let (x, y) = planet_offset(&planet);
            assert!(
                x.abs() <= 0.5 && y.abs() <= 0.5,
                "{planet:?} is outside the tile"
            );
        }
    }

    fn test_tile(ring: u32, position: u32, expected_x: i32, expected_y: i32) {
        let t = get_tile_position(&Coordinate {
            ring,
            position,
            rotation: 0,
        });
        assert_eq!(
            (expected_x as f32, expected_y as f32),
            t,
            "testing ring {ring} position {position} expected ({expected_x}, {expected_y})"
        );
    }
}
