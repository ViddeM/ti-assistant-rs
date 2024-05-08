use std::{f32::consts::PI, ops::Div};

use bevy::prelude::*;
use ti_helper_game::gameplay::map::{Coordinate, HexMap, HexPosition};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("slider_regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 18.0,
        color: Color::WHITE,
    };

    commands.spawn(Camera2dBundle::default());

    let milty_string = "87A1 89B3 47 87A4 89B0 78 37 64 46 29 72 22 24 63 44 40 23 76 50 30 48 28 43 83B2 67 69 34 27 77 26 36 74 83B2 79 19 38 53 42 59 7 0 0 14 21 0 4 39 71 15 80 68 52 0 0 17 75 0 58 41 60";
    let hex_map = HexMap::from_milty_string(milty_string).expect("failed to parse milty string");

    let mut outside_galaxy_count = 0;

    for tile in hex_map.tiles {
        let position = match tile.position {
            HexPosition::OutsideGalaxy => {
                let transform = Transform::from_translation(Vec3::new(
                    450.0,
                    150.0 - (outside_galaxy_count as f32 * 300.0),
                    1.0,
                ));

                outside_galaxy_count += 1;
                transform
            }
            HexPosition::Pos(coord) => {
                let dist = 330.0;
                let tile_pos = get_tile_position(coord) * Vec2::new(dist, dist);

                Transform::from_translation(Vec3::new(tile_pos.x, tile_pos.y, 1.0))
            }
        };

        commands.spawn(SpriteBundle {
            texture: asset_server.load(format!("tiles/{}.png", tile.system)),
            transform: position,
            ..default()
        });

        // commands.spawn(Text2dBundle {
        //     text: Text::from_section(tile.system, text_style.clone())
        //         .with_justify(JustifyText::Left),
        //     transform: position,
        //     ..default()
        // });
    }
}

const RING_TOTAL_ANGLE: f32 = 360.0;
fn get_tile_position(coord: Coordinate) -> Vec2 {
    let tiles_in_ring = num_tiles_in_ring(coord.ring);
    let half = (tiles_in_ring as f32).div(2.0) as i32;
    let quarter = (half as f32).div(2.0).floor() as i32;

    let coord_x = coord.position as i32;
    let x_val = coord_x % quarter;
    let x_val_2 = if coord_x <= half {
        x_val
    } else {
        quarter - x_val
    };
    let x = if coord_x >= half { -x_val_2 } else { x_val_2 };

    let coord_y = coord.ring * 2;

    let angle = (RING_TOTAL_ANGLE.div(tiles_in_ring as f32) * coord.position as f32).to_radians()
        - (PI * 3.0).div(2.0);

    Vec2::from_angle(angle) * Vec2::new(coord.ring as f32, coord.ring as f32)
}

fn num_tiles_in_ring(ring: u32) -> u32 {
    if ring == 0 {
        return 1;
    }

    ring * 6
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_OUTCOMES: [(u32, u32); 7] =
        [(0, 1), (1, 6), (2, 12), (3, 18), (4, 24), (5, 30), (6, 36)];

    #[test]
    fn rings_return_expected_number() {
        for (ring, expected) in EXPECTED_OUTCOMES.iter() {
            assert_eq!(num_tiles_in_ring(*ring), *expected);
        }
    }
}
