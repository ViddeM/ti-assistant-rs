use ti_helper_game_data::common::map::{HexMap, HexPosition};

use super::layout::{TILE_HEIGHT, get_tile_position, tile_pos_to_visual_pos, to_svg};

/// The space between a tile outside the galaxy and the tiles around it.
const OUTSIDE_GALAXY_GAP: f32 = TILE_HEIGHT * 0.25;

/// How far a tile outside the galaxy is moved at a time, until it has space around it.
const NUDGE_STEP: f32 = TILE_HEIGHT * 0.05;
const MAX_NUDGE_STEPS: u32 = 1000;

/// How much each step of rotation rotates a tile (clockwise).
const ROTATION_STEP_DEGREES: f32 = 60.0;

/// Where the column of tiles outside the galaxy on the left of the board starts, and the space between them (SVG y).
const LEFT_COLUMN_START_Y: f32 = TILE_HEIGHT * 1.5;
const LEFT_COLUMN_SPACING: f32 = TILE_HEIGHT * 4.0;

const ONE_CELL: &[(f32, f32)] = &[(0.0, 0.0)];

/// Places every tile of the map, in map order, as its center (SVG space) and its rotation in degrees.
///
/// The tiles outside the galaxy are moved away from the tiles placed before them until there's
/// [OUTSIDE_GALAXY_GAP] between them: the galaxy is placed first, then the tiles that belong next
/// to a tile in the galaxy and then the rest, in a column on the left of the board.
pub fn place_tiles(hex_map: &HexMap) -> Vec<((f32, f32), f32)> {
    let mut positions = vec![((0.0, 0.0), 0.0); hex_map.tiles.len()];
    let mut placed = Vec::new();

    for (index, tile) in hex_map.tiles.iter().enumerate() {
        if let HexPosition::Pos(coord) = &tile.position {
            let center = to_svg(tile_pos_to_visual_pos(get_tile_position(coord)));
            positions[index] = (center, coord.rotation as f32 * ROTATION_STEP_DEGREES);
            placed.push(center);
        }
    }
    let left = placed.iter().map(|(x, _)| *x).fold(0.0, f32::min);

    let mut others = Vec::new();
    for (index, tile) in hex_map.tiles.iter().enumerate() {
        if tile.position != HexPosition::OutsideGalaxy {
            continue;
        }
        match hex_map.outside_galaxy_coordinate(tile) {
            Some(coord) => {
                let start = to_svg(tile_pos_to_visual_pos(get_tile_position(&coord)));
                let center = nudge(start, away_from_center(start), ONE_CELL, &placed);
                positions[index] = (center, 0.0);
                placed.push(center);
            }
            None => others.push(index),
        }
    }

    for (slot, index) in others.into_iter().enumerate() {
        let start = (
            left,
            LEFT_COLUMN_START_Y - LEFT_COLUMN_SPACING * slot as f32,
        );
        let center = nudge(start, (-1.0, 0.0), ONE_CELL, &placed);
        positions[index] = (center, 0.0);
        placed.push(center);
    }

    positions
}

/// Moves `start` along `direction` until each of the `cells` (relative to it) is at least
/// [OUTSIDE_GALAXY_GAP] away from each of the `placed` cells.
fn nudge(
    start: (f32, f32),
    direction: (f32, f32),
    cells: &[(f32, f32)],
    placed: &[(f32, f32)],
) -> (f32, f32) {
    let mut position = start;
    for _ in 0..MAX_NUDGE_STEPS {
        let has_space = cells.iter().all(|(cell_x, cell_y)| {
            placed.iter().all(|(x, y)| {
                hex_distance(position.0 + cell_x - x, position.1 + cell_y - y)
                    >= TILE_HEIGHT + OUTSIDE_GALAXY_GAP
            })
        });
        if has_space {
            break;
        }
        position = (
            position.0 + direction.0 * NUDGE_STEP,
            position.1 + direction.1 * NUDGE_STEP,
        );
    }
    position
}

/// The distance between the centers of two flat-topped hexes, across the edges that face each
/// other. Hexes that touch are [TILE_HEIGHT] apart.
fn hex_distance(dx: f32, dy: f32) -> f32 {
    dy.abs().max(3f32.sqrt() / 2.0 * dx.abs() + dy.abs() / 2.0)
}

fn away_from_center((x, y): (f32, f32)) -> (f32, f32) {
    let length = x.hypot(y);
    if length == 0.0 {
        (0.0, -1.0)
    } else {
        (x / length, y / length)
    }
}

#[cfg(test)]
mod test {
    use ti_helper_game_data::common::map::{Coordinate, HexMap, HexPosition, Tile};

    use super::{NUDGE_STEP, OUTSIDE_GALAXY_GAP, hex_distance, place_tiles};
    use crate::views::map_view_mode::layout::TILE_HEIGHT;

    const CRIMSON_REBELLION_GATE: &str = "94";
    const OUTSIDE_GALAXY: [&str; 3] = ["82", "51", "118"];

    fn three_ring_map_with_gate_at(gate_position: u32) -> HexMap {
        let galaxy = (0..=3u32).flat_map(|ring| {
            (0..(ring * 6).max(1)).map(move |position| Tile {
                system: if ring == 3 && position == gate_position {
                    CRIMSON_REBELLION_GATE.to_string()
                } else {
                    "19".to_string()
                },
                position: HexPosition::Pos(Coordinate {
                    ring,
                    position,
                    rotation: 0,
                }),
            })
        });
        let outside = OUTSIDE_GALAXY.iter().map(|system| Tile {
            system: system.to_string(),
            position: HexPosition::OutsideGalaxy,
        });
        HexMap {
            tiles: galaxy.chain(outside).collect(),
            ring_count: 3,
        }
    }

    #[test]
    fn tiles_outside_the_galaxy_have_space_around_them() {
        for gate_position in 0..18 {
            let hex_map = three_ring_map_with_gate_at(gate_position);
            let positions = place_tiles(&hex_map);

            for (index, tile) in hex_map.tiles.iter().enumerate() {
                if tile.position != HexPosition::OutsideGalaxy {
                    continue;
                }
                let ((x, y), _) = positions[index];
                for (other_index, ((other_x, other_y), _)) in positions.iter().enumerate() {
                    let distance = hex_distance(x - other_x, y - other_y);
                    assert!(
                        other_index == index
                            || distance >= TILE_HEIGHT + OUTSIDE_GALAXY_GAP - NUDGE_STEP,
                        "with the gate at {gate_position}, {} is {distance} from {}",
                        tile.system,
                        hex_map.tiles[other_index].system
                    );
                }
            }
        }
    }
}
