use std::{f32::consts::PI, ops::Div};

use bevy::{core::Zeroable, prelude::*};
use ti_helper_game::{
    data::components::system::SystemId,
    gameplay::map::{Coordinate, HexMap, HexPosition},
};

const TILE_WIDTH: f32 = 364.0;
const TILE_HEIGHT: f32 = 317.0;

const TILE_QUARTER_WIDTH: f32 = TILE_WIDTH * 0.25;
const TILE_THREE_QUARTER_WIDTH: f32 = TILE_QUARTER_WIDTH * 3.0;
const TILE_HALF_HEIGHT: f32 = TILE_HEIGHT * 0.5;

const ROTATION_STEP: f32 = -PI / 3.0;

#[derive(Component, Clone, Debug)]
pub struct SystemVisuals {
    pub system_id: SystemId,
    pub tile_pos: Vec2,
}

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("slider_regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 32.0,
        color: Color::WHITE,
    };

    // TODO: Handle rotation of tiles
    let milty_string = "87A1 89B3 47 87A4 89B0 78 37 64 46 29 72 22 24 63 44 40 23 76 50 30 48 28 43 83B2 67 69 34 27 77 26 36 74 83B2 79 19 38 53 42 59 7 0 0 14 21 0 4 39 71 15 80 68 52 0 0 17 75 0 58 41 60";
    let hex_map = HexMap::from_milty_string(milty_string).expect("failed to parse milty string");

    let mut outside_galaxy_count = 0;

    for tile in hex_map.tiles {
        let (tile_pos, rotation) = match tile.position {
            HexPosition::OutsideGalaxy => {
                let pos = Vec2::new(-5.5, -2.0 + 4.0 * outside_galaxy_count as f32);

                outside_galaxy_count += 1;

                (pos, 0.0)
            }
            HexPosition::Pos(coord) => {
                let rot = coord.rotation as f32;
                let pos = get_tile_position(coord);

                (pos, rot)
            }
        };

        let position = Transform::from_translation(tile_pos_to_visual_pos(tile_pos))
            .with_rotation(Quat::from_rotation_z(rotation * ROTATION_STEP));

        let system_visuals = SystemVisuals {
            system_id: tile.system.clone(),
            tile_pos,
        };

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(format!("tiles/{}.png", tile.system)),
                transform: position,
                ..default()
            },
            system_visuals,
        ));

        commands.spawn((Text2dBundle {
            text: Text::from_section(tile.system, text_style.clone())
                .with_justify(JustifyText::Left),
            transform: position.with_rotation(Quat::zeroed()),
            ..default()
        },));
    }
}

pub fn tile_pos_to_visual_pos(tile_pos: Vec2) -> Vec3 {
    Vec3::new(
        tile_pos.x * TILE_THREE_QUARTER_WIDTH,
        tile_pos.y * TILE_HEIGHT + (TILE_HEIGHT / 2.0)
            - if (tile_pos.x as i32) % 2 == 0 {
                TILE_HALF_HEIGHT
            } else {
                0.0
            },
        1.0,
    )
}

pub fn tile_offset_to_visual_pos(tile_pos: Vec2) -> Vec3 {
    Vec3::new(tile_pos.x * TILE_WIDTH, tile_pos.y * TILE_HEIGHT, 0.0)
}

fn get_tile_position(coord: Coordinate) -> Vec2 {
    if coord.ring == 0 {
        return Vec2::ZERO;
    }

    let radius = coord.ring as i32;
    let ring_pos = coord.position as i32;

    let full = (radius * 6) as i32;
    let half = (radius * 3) as i32;
    let quarter = ((radius as f32) * 1.5).ceil() as i32;

    let x_right_half = ring_pos % half;
    let x_offset = if x_right_half >= quarter {
        half - x_right_half
    } else {
        x_right_half
    };

    let x_absolute = x_offset.min(radius); // The x value increases for each position in the top-right quadrant until we reach the 'radius' width, then it goes down from there.
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

    let y_steps = (half_steps as f32).div(2.0).ceil() as i32 + full_steps;

    let y = radius - y_steps;

    Vec2::new(x as f32, y as f32)
}

#[cfg(test)]
mod test {
    use bevy::math::Vec2;
    use ti_helper_game::gameplay::map::Coordinate;

    use super::get_tile_position;

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

    fn test_tile(ring: u32, position: u32, expected_x: i32, expected_y: i32) {
        let t = get_tile_position(Coordinate {
            ring,
            position,
            rotation: 0,
        });
        assert_eq!(
            Vec2::new(expected_x as f32, expected_y as f32),
            t,
            "testing ring {ring} position {position} expected ({expected_x}, {expected_y})"
        );
    }
}
