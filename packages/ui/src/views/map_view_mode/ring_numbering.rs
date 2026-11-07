use dioxus::prelude::*;
use ti_helper_game_data::common::map::Coordinate;

use super::{
    MAP_VIEW_MODE_SCSS, TILE_POINTS,
    layout::{TILE_HEIGHT, TILE_WIDTH, get_tile_position, tile_pos_to_visual_pos, to_svg},
};

/// Mecatol Rex and the two rings around it, each tile labeled with its position in its ring.
#[component]
pub fn RingNumbering() -> Element {
    let width = TILE_WIDTH * 4.0;
    let height = TILE_HEIGHT * 5.0;
    let view_box = format!("{} {} {} {}", -width / 2.0, -height / 2.0, width, height);

    let tiles = (0..=2u32).flat_map(|ring| {
        (0..(6 * ring).max(1)).map(move |position| {
            let coord = Coordinate {
                ring,
                position,
                rotation: 0,
            };
            let (x, y) = to_svg(tile_pos_to_visual_pos(get_tile_position(&coord)));
            let label = if ring == 0 {
                "MR".to_string()
            } else {
                position.to_string()
            };
            (ring, position, x, y, label)
        })
    });

    rsx! {
        document::Stylesheet { href: MAP_VIEW_MODE_SCSS }

        svg { class: "ring-numbering", view_box,
            for (ring , position , x , y , label) in tiles {
                g {
                    key: "{ring}-{position}",
                    class: "ring-numbering-ring-{ring}",
                    transform: "translate({x} {y})",
                    polygon { points: TILE_POINTS }
                    text { "{label}" }
                }
            }
        }
    }
}
