use ti_helper_game_data::common::map::{HexMap, HexPosition};

use super::layout::{
    TILE_HALF_HEIGHT, TILE_HEIGHT, TILE_THREE_QUARTER_WIDTH, get_tile_position,
    tile_pos_to_visual_pos, to_svg,
};

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
const COCYTUS_AND_EGRESS: &[(f32, f32)] =
    &[(0.0, 0.0), (TILE_THREE_QUARTER_WIDTH, -TILE_HALF_HEIGHT)];
const LETHE_AND_EGRESS: &[(f32, f32)] =
    &[(0.0, 0.0), (-TILE_THREE_QUARTER_WIDTH, -TILE_HALF_HEIGHT)];
const STYX_BETWEEN_EMPTY_SYSTEMS: &[(f32, f32)] = &[
    (-TILE_THREE_QUARTER_WIDTH, TILE_HALF_HEIGHT),
    (0.0, 0.0),
    (TILE_THREE_QUARTER_WIDTH, TILE_HALF_HEIGHT),
];

/// The tiles of the Fracture from left to right, and their centers relative to Styx (SVG space).
type FractureLayout = [(&'static str, (f32, f32)); 3];

/// The Fracture with 3 players: one row above the board.
const FRACTURE_IN_A_ROW: FractureLayout = [
    ("125", (-3.0 * TILE_THREE_QUARTER_WIDTH, TILE_HALF_HEIGHT)),
    ("127", (0.0, 0.0)),
    ("126", (3.0 * TILE_THREE_QUARTER_WIDTH, TILE_HALF_HEIGHT)),
];

/// The Fracture with 4 or more players: along the board's top-left and top-right edges.
const FRACTURE_OVER_THE_TOP_CORNER: FractureLayout = [
    (
        "125",
        (-3.0 * TILE_THREE_QUARTER_WIDTH, 3.0 * TILE_HALF_HEIGHT),
    ),
    ("127", (0.0, 0.0)),
    (
        "126",
        (3.0 * TILE_THREE_QUARTER_WIDTH, 3.0 * TILE_HALF_HEIGHT),
    ),
];

/// The centers of the hexes the tile covers, relative to the center of the tile (SVG space).
pub fn tile_cells(system: &str) -> &'static [(f32, f32)] {
    match system {
        "125" => COCYTUS_AND_EGRESS,
        "126" => LETHE_AND_EGRESS,
        "127" => STYX_BETWEEN_EMPTY_SYSTEMS,
        _ => ONE_CELL,
    }
}

/// Places every tile of the map, in map order, as its center (SVG space) and its rotation in degrees.
///
/// The tiles outside the galaxy are moved away from the tiles placed before them until there's
/// [OUTSIDE_GALAXY_GAP] between them: the galaxy is placed first, then the tiles that belong next
/// to a tile in the galaxy, then the Fracture above the board and then the rest, in a column on
/// the left of the board.
pub fn place_tiles(hex_map: &HexMap, player_count: usize) -> Vec<((f32, f32), f32)> {
    let the_fracture = if player_count <= 3 {
        &FRACTURE_IN_A_ROW
    } else {
        &FRACTURE_OVER_THE_TOP_CORNER
    };
    let mut positions = vec![((0.0, 0.0), 0.0); hex_map.tiles.len()];
    let mut placed = Vec::new();

    for (index, tile) in hex_map.tiles.iter().enumerate() {
        if let HexPosition::Pos(coord) = &tile.position {
            let center = to_svg(tile_pos_to_visual_pos(get_tile_position(coord)));
            positions[index] = (center, coord.rotation as f32 * ROTATION_STEP_DEGREES);
            placed.extend(cells_at(center, tile_cells(&tile.system)));
        }
    }
    let left = placed.iter().map(|(x, _)| *x).fold(0.0, f32::min);
    let top = placed.iter().map(|(_, y)| *y).fold(0.0, f32::min);

    let mut fracture = Vec::new();
    let mut others = Vec::new();
    for (index, tile) in hex_map.tiles.iter().enumerate() {
        if tile.position != HexPosition::OutsideGalaxy {
            continue;
        }
        if let Some(coord) = hex_map.outside_galaxy_coordinate(tile) {
            let cells = tile_cells(&tile.system);
            let start = to_svg(tile_pos_to_visual_pos(get_tile_position(&coord)));
            let center = nudge(start, away_from_center(start), cells, &placed);
            positions[index] = (center, 0.0);
            placed.extend(cells_at(center, cells));
        } else if let Some((_, offset)) = the_fracture
            .iter()
            .find(|(system, _)| *system == tile.system)
        {
            fracture.push((index, *offset));
        } else {
            others.push(index);
        }
    }

    if !fracture.is_empty() {
        let cells = fracture
            .iter()
            .flat_map(|(index, offset)| {
                cells_at(*offset, tile_cells(&hex_map.tiles[*index].system))
            })
            .collect::<Vec<_>>();
        let styx = nudge((0.0, top), (0.0, -1.0), &cells, &placed);
        for (index, offset) in fracture {
            positions[index] = ((styx.0 + offset.0, styx.1 + offset.1), 0.0);
        }
        placed.extend(cells_at(styx, &cells));
    }

    for (slot, index) in others.into_iter().enumerate() {
        let cells = tile_cells(&hex_map.tiles[index].system);
        let start = (
            left,
            LEFT_COLUMN_START_Y - LEFT_COLUMN_SPACING * slot as f32,
        );
        let center = nudge(start, (-1.0, 0.0), cells, &placed);
        positions[index] = (center, 0.0);
        placed.extend(cells_at(center, cells));
    }

    positions
}

fn cells_at((x, y): (f32, f32), cells: &[(f32, f32)]) -> impl Iterator<Item = (f32, f32)> + '_ {
    cells
        .iter()
        .map(move |(cell_x, cell_y)| (x + cell_x, y + cell_y))
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
        let has_space = cells_at(position, cells).all(|(cell_x, cell_y)| {
            placed.iter().all(|(x, y)| {
                hex_distance(cell_x - x, cell_y - y) >= TILE_HEIGHT + OUTSIDE_GALAXY_GAP
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
    use std::collections::HashMap;

    use ti_helper_game_data::common::map::{Coordinate, HexMap, HexPosition, Tile};

    use super::{
        FRACTURE_IN_A_ROW, NUDGE_STEP, OUTSIDE_GALAXY_GAP, cells_at, hex_distance, place_tiles,
        tile_cells,
    };
    use crate::views::map_view_mode::layout::TILE_HEIGHT;

    const CRIMSON_REBELLION_GATE: &str = "94";
    const GATE_AT_THE_BOTTOM: u32 = 9;
    const OUTSIDE_GALAXY: [&str; 6] = ["82", "51", "118", "125", "126", "127"];
    const PLAYER_COUNTS: [usize; 2] = [3, 6];

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

    fn is_the_fracture(tile: &Tile) -> bool {
        FRACTURE_IN_A_ROW
            .iter()
            .any(|(system, _)| *system == tile.system)
    }

    /// The centers of the hexes of each tile (SVG space), in map order.
    fn placed_cells(hex_map: &HexMap, player_count: usize) -> Vec<Vec<(f32, f32)>> {
        hex_map
            .tiles
            .iter()
            .zip(place_tiles(hex_map, player_count))
            .map(|(tile, (center, _))| cells_at(center, tile_cells(&tile.system)).collect())
            .collect()
    }

    /// How far each hex of each tile of the Fracture is from the nearest hex of the galaxy, in the
    /// order of [tile_cells].
    fn fracture_distances_from_the_galaxy(
        hex_map: &HexMap,
        player_count: usize,
    ) -> HashMap<&str, Vec<f32>> {
        let cells = placed_cells(hex_map, player_count);
        let galaxy = hex_map
            .tiles
            .iter()
            .zip(&cells)
            .filter(|(tile, _)| tile.position != HexPosition::OutsideGalaxy)
            .flat_map(|(_, cells)| cells.iter().copied())
            .collect::<Vec<_>>();

        hex_map
            .tiles
            .iter()
            .zip(&cells)
            .filter(|(tile, _)| is_the_fracture(tile))
            .map(|(tile, cells)| {
                let distances = cells
                    .iter()
                    .map(|(x, y)| {
                        galaxy
                            .iter()
                            .map(|(galaxy_x, galaxy_y)| hex_distance(x - galaxy_x, y - galaxy_y))
                            .fold(f32::MAX, f32::min)
                    })
                    .collect();
                (tile.system.as_str(), distances)
            })
            .collect()
    }

    #[test]
    fn tiles_outside_the_galaxy_have_space_around_them() {
        for player_count in PLAYER_COUNTS {
            for gate_position in 0..18 {
                let hex_map = three_ring_map_with_gate_at(gate_position);
                let cells = placed_cells(&hex_map, player_count);
                let same_piece =
                    |a: &Tile, b: &Tile| a == b || is_the_fracture(a) && is_the_fracture(b);

                for (tile, tile_cells) in hex_map.tiles.iter().zip(&cells) {
                    if tile.position != HexPosition::OutsideGalaxy {
                        continue;
                    }
                    for (other, other_cells) in hex_map.tiles.iter().zip(&cells) {
                        if same_piece(tile, other) {
                            continue;
                        }
                        for (x, y) in tile_cells {
                            for (other_x, other_y) in other_cells {
                                let distance = hex_distance(x - other_x, y - other_y);
                                assert!(
                                    distance >= TILE_HEIGHT + OUTSIDE_GALAXY_GAP - NUDGE_STEP,
                                    "with {player_count} players and the gate at {gate_position}, {} is {distance} from {}",
                                    tile.system,
                                    other.system
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_fracture_is_above_the_galaxy_in_order() {
        for player_count in PLAYER_COUNTS {
            for gate_position in 0..18 {
                let hex_map = three_ring_map_with_gate_at(gate_position);
                let cells = placed_cells(&hex_map, player_count);
                let cells_of = |include: fn(&Tile) -> bool| {
                    hex_map
                        .tiles
                        .iter()
                        .zip(&cells)
                        .filter(move |(tile, _)| include(tile))
                        .flat_map(|(_, cells)| cells.iter().copied())
                };

                let lowest_fracture_hex = cells_of(is_the_fracture)
                    .map(|(_, y)| y)
                    .fold(f32::MIN, f32::max);
                let highest_galaxy_hex =
                    cells_of(|tile| tile.position != HexPosition::OutsideGalaxy)
                        .map(|(_, y)| y)
                        .fold(f32::MAX, f32::min);
                assert!(
                    lowest_fracture_hex < highest_galaxy_hex,
                    "with {player_count} players and the gate at {gate_position}"
                );

                let fracture_from_left_to_right = {
                    let mut fracture = hex_map
                        .tiles
                        .iter()
                        .zip(place_tiles(&hex_map, player_count))
                        .filter(|(tile, _)| is_the_fracture(tile))
                        .map(|(tile, ((x, _), _))| (x, tile.system.as_str()))
                        .collect::<Vec<_>>();
                    fracture.sort_by(|a, b| a.0.total_cmp(&b.0));
                    fracture
                        .into_iter()
                        .map(|(_, system)| system)
                        .collect::<Vec<_>>()
                };
                assert_eq!(fracture_from_left_to_right, vec!["125", "127", "126"]);
            }
        }
    }

    #[test]
    fn the_fracture_runs_along_the_top_edges_with_four_or_more_players() {
        let hex_map = three_ring_map_with_gate_at(GATE_AT_THE_BOTTOM);
        let next_to_the_galaxy = TILE_HEIGHT + 2.0 * OUTSIDE_GALAXY_GAP + NUDGE_STEP;

        for (system, distances) in fracture_distances_from_the_galaxy(&hex_map, 6) {
            for distance in distances {
                assert!(
                    distance <= next_to_the_galaxy,
                    "with 6 players, a hex of {system} is {distance} from the galaxy"
                );
            }
        }

        let with_three_players = fracture_distances_from_the_galaxy(&hex_map, 3);
        for (system, planet) in [("125", "Cocytus"), ("126", "Lethe")] {
            let distance = with_three_players[system][0];
            assert!(
                distance > next_to_the_galaxy,
                "with 3 players, {planet} is {distance} from the galaxy"
            );
        }
    }
}
