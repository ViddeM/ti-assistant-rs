use std::collections::BTreeMap;

use dioxus::prelude::*;
use ti_helper_game_data::{common::player_id::PlayerId, state::expedition::TEExpeditionSlot};

use crate::{
    components::{faction_icon::get_faction_icon, ti_icon::TiIconType},
    data::game_context::GameContext,
};

/// Radius of the token, the view box is centered on the middle of the token.
const OUTER_R: f64 = 96.0;

/// Circumradius of the hexagon in the middle of the token.
const HEX_R: f64 = 18.0;

/// Distance from the center to the cost icon of each wedge, and the size of the resource/influence icons.
const ICON_R: f64 = 36.0;
const ICON_SIZE: f64 = 23.0;

/// Distance from the center to the middle of the visible part of a control token slot, and the size of the faction icon in a claimed slot.
const SLOT_R: f64 = 75.0;
const FACTION_ICON_SIZE: f64 = 22.0;

/// The corners of the control token slot of the wedge pointing straight up, it extends past the rim and is clipped to the token.
const SLOT_CORNERS: [(f64, f64); 6] = [
    (-30.0, -110.0),
    (30.0, -110.0),
    (30.0, -64.0),
    (17.0, -56.0),
    (-17.0, -56.0),
    (-30.0, -64.0),
];

/// Radius of the circular arrows shown in an unclaimed slot.
const ARROW_R: f64 = 7.0;

/// The order of the wedges on the token, clockwise starting from the top.
const SLOTS_CLOCKWISE: [TEExpeditionSlot; 6] = [
    TEExpeditionSlot::Resources,
    TEExpeditionSlot::ActionCards,
    TEExpeditionSlot::Influence,
    TEExpeditionSlot::SecretObjective,
    TEExpeditionSlot::TechSpecialityPlanet,
    TEExpeditionSlot::TradeGoods,
];

fn css_name(slot: &TEExpeditionSlot) -> &'static str {
    match slot {
        TEExpeditionSlot::Resources => "resources",
        TEExpeditionSlot::ActionCards => "action-cards",
        TEExpeditionSlot::Influence => "influence",
        TEExpeditionSlot::SecretObjective => "secret-objective",
        TEExpeditionSlot::TechSpecialityPlanet => "tech-speciality-planet",
        TEExpeditionSlot::TradeGoods => "trade-goods",
    }
}

/// The point `r` away from the center in the direction `deg`, where 0° is up and angles go clockwise.
fn polar(deg: f64, r: f64) -> (f64, f64) {
    let rad = deg.to_radians();
    (r * rad.sin(), -r * rad.cos())
}

/// A hexagon with its corners on the wedge boundaries, i.e. with its sides facing the wedges.
fn hex_points(cx: f64, cy: f64, r: f64) -> String {
    (0..6)
        .map(|k| {
            let (x, y) = polar(30.0 + 60.0 * k as f64, r);
            format!("{:.2},{:.2}", cx + x, cy + y)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// The wedge centered on `center_deg`, from its side of the center hexagon out to the rim.
fn wedge_path(center_deg: f64) -> String {
    let (x1, y1) = polar(center_deg - 30.0, HEX_R);
    let (x2, y2) = polar(center_deg - 30.0, OUTER_R);
    let (x3, y3) = polar(center_deg + 30.0, OUTER_R);
    let (x4, y4) = polar(center_deg + 30.0, HEX_R);

    format!(
        "M {x1:.2} {y1:.2} L {x2:.2} {y2:.2} A {OUTER_R} {OUTER_R} 0 0 1 {x3:.2} {y3:.2} L {x4:.2} {y4:.2} Z"
    )
}

/// The control token slot of the wedge centered on `center_deg`.
///
/// Rotated here rather than with a `transform`, Firefox drew part of the bottom wedge wrong with a rotated (and filtered) slot.
fn slot_points(center_deg: f64) -> String {
    let (sin, cos) = center_deg.to_radians().sin_cos();
    SLOT_CORNERS
        .iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x * cos - y * sin, x * sin + y * cos))
        .collect::<Vec<_>>()
        .join(" ")
}

/// One of the two arrows of the "place a control token here" mark: an arc from `start` to `end` degrees, and the points of its head.
fn slot_arrow(start: f64, end: f64) -> (String, String) {
    let (x1, y1) = polar(start, ARROW_R);
    let (x2, y2) = polar(end, ARROW_R);
    let arc = format!("M {x1:.2} {y1:.2} A {ARROW_R} {ARROW_R} 0 0 1 {x2:.2} {y2:.2}");

    let (ox, oy) = polar(end, ARROW_R + 3.0);
    let (tx, ty) = polar(end + 25.0, ARROW_R);
    let (ix, iy) = polar(end, ARROW_R - 3.0);
    let head = format!("{ox:.2},{oy:.2} {tx:.2},{ty:.2} {ix:.2},{iy:.2}");

    (arc, head)
}

/// A fixed sprinkle of stars for the space background, as (x, y, radius).
fn stars() -> Vec<(f64, f64, f64)> {
    let mut seed: u32 = 0x7e5eed;
    let mut next = move || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        seed as f64 / u32::MAX as f64
    };

    (0..70)
        .map(|_| {
            let (x, y) = polar(next() * 360.0, OUTER_R * next().sqrt());
            (x, y, 0.2 + next() * 0.5)
        })
        .collect()
}

/// The expedition, drawn like the expedition side of the Thunder's Edge token.
#[component]
pub fn ExpeditionToken(
    progress: BTreeMap<TEExpeditionSlot, PlayerId>,
    selected_player: ReadSignal<Option<PlayerId>>,
) -> Element {
    let claimed = SLOTS_CLOCKWISE
        .iter()
        .filter(|slot| progress.contains_key(slot))
        .count();

    rsx! {
        svg {
            class: "te-token",
            view_box: "-100 -100 200 200",
            role: "group",
            "aria-label": "Thunder's Edge expedition",

            defs {
                radialGradient { id: "te-halo-gradient",
                    stop {
                        offset: "95%",
                        stop_color: "#4696ff",
                        stop_opacity: 0.45,
                    }
                    stop {
                        offset: "100%",
                        stop_color: "#4696ff",
                        stop_opacity: 0,
                    }
                }
                radialGradient { id: "te-space-gradient",
                    stop { offset: "0%", stop_color: "#18203f" }
                    stop { offset: "70%", stop_color: "#080b18" }
                    stop { offset: "100%", stop_color: "#020308" }
                }
                clipPath { id: "te-disc-clip",
                    circle { r: OUTER_R }
                }
                for k in 0..6 {
                    clipPath { key: "{k}", id: "te-wedge-clip-{k}",
                        path { d: wedge_path(k as f64 * 60.0) }
                    }
                }
            }

            circle { fill: "url(#te-halo-gradient)", r: 100 }
            circle { fill: "url(#te-space-gradient)", r: OUTER_R }

            g { clip_path: "url(#te-disc-clip)",
                for (i , (x , y , r)) in stars().into_iter().enumerate() {
                    circle {
                        key: "star-{i}",
                        class: "te-star",
                        cx: x,
                        cy: y,
                        r,
                    }
                }

                for (i , slot) in SLOTS_CLOCKWISE.iter().cloned().enumerate() {
                    ExpeditionWedge {
                        key: "{i}",
                        index: i,
                        slot: slot.clone(),
                        picked_by: progress.get(&slot).cloned(),
                        selected_player,
                    }
                }
            }

            g { class: "te-separators",
                for k in 0..6 {
                    {
                        let (x1, y1) = polar(30.0 + 60.0 * k as f64, HEX_R);
                        let (x2, y2) = polar(30.0 + 60.0 * k as f64, OUTER_R);
                        rsx! {
                            line {
                                key: "{k}",
                                x1,
                                y1,
                                x2,
                                y2,
                            }
                        }
                    }
                }
                circle { r: OUTER_R - 0.5 }
            }

            polygon { class: "te-center", points: hex_points(0.0, 0.0, HEX_R) }
            text { class: "te-center-count", x: 0, y: 0, "{claimed}/6" }
        }
    }
}

#[component]
fn ExpeditionWedge(
    index: usize,
    slot: TEExpeditionSlot,
    picked_by: Option<PlayerId>,
    selected_player: ReadSignal<Option<PlayerId>>,
) -> Element {
    let gc = use_context::<GameContext>();

    let center = index as f64 * 60.0;
    let claimable = selected_player.read().is_some() && *selected_player.read() != picked_by;
    let owner = picked_by.as_ref().and_then(|p| {
        gc.game_state()
            .players
            .get(p)
            .map(|player| (player.faction, player.color))
    });

    let mut class = format!("te-wedge te-wedge-{}", css_name(&slot));
    if claimable {
        class.push_str(" te-wedge-claimable");
    }
    if picked_by.is_some() {
        class.push_str(" te-wedge-claimed");
    }

    let (icon_x, icon_y) = polar(center, ICON_R);
    let (slot_x, slot_y) = polar(center, SLOT_R);
    let physical_slot = slot_points(center);
    let arrows = [slot_arrow(-70.0, 70.0), slot_arrow(110.0, 250.0)];
    let tooltip = format!("<title>{}</title>", slot.description());

    rsx! {
        g { class,
            path {
                class: "te-wedge-shape",
                d: wedge_path(center),
                clip_path: "url(#te-wedge-clip-{index})",
                "aria-label": slot.description(),
                // Dioxus doesn't have the svg `title` element, but setting it as inner html still gives a native tooltip.
                dangerous_inner_html: tooltip,
                onclick: move |_| {
                    if claimable {
                        println!("TODO: NOT IMPLEMENTED");
                    }
                },
            }

            polygon { class: "te-slot-glow", points: physical_slot.clone() }
            polygon { class: "te-slot", points: physical_slot }

            SlotIcon { slot: slot.clone(), x: icon_x, y: icon_y }

            if let Some((faction, color)) = owner {
                circle {
                    class: "te-slot-ring-outline",
                    cx: slot_x,
                    cy: slot_y,
                    r: FACTION_ICON_SIZE / 2.0 + 4.2,
                }
                circle {
                    class: "te-slot-ring te-slot-ring-{color.name()}",
                    cx: slot_x,
                    cy: slot_y,
                    r: FACTION_ICON_SIZE / 2.0 + 2.5,
                }
                image {
                    href: get_faction_icon(&faction),
                    x: slot_x - FACTION_ICON_SIZE / 2.0,
                    y: slot_y - FACTION_ICON_SIZE / 2.0,
                    width: FACTION_ICON_SIZE,
                    height: FACTION_ICON_SIZE,
                }
            } else {
                g {
                    class: "te-slot-mark",
                    transform: "rotate({center}) translate(0 -{SLOT_R})",
                    for (i , (arc , head)) in arrows.into_iter().enumerate() {
                        g { key: "{i}",
                            path { d: arc }
                            polygon { points: head }
                        }
                    }
                }
            }
        }
    }
}

/// The slot icon for a wedge.
#[component]
fn SlotIcon(slot: TEExpeditionSlot, x: f64, y: f64) -> Element {
    match slot {
        TEExpeditionSlot::Resources | TEExpeditionSlot::Influence => {
            let icon = if slot == TEExpeditionSlot::Resources {
                TiIconType::ResourceFilled
            } else {
                TiIconType::InfluenceFilled
            };

            rsx! {
                image {
                    href: icon.get_asset(),
                    x: x - ICON_SIZE / 2.0,
                    y: y - ICON_SIZE / 2.0,
                    width: ICON_SIZE,
                    height: ICON_SIZE,
                }
                text { class: "te-number", x, y, "5" }
            }
        }
        TEExpeditionSlot::ActionCards => rsx! {
            text { class: "te-number te-number-large", x: x - 7.0, y, "2" }
            rect {
                class: "te-glyph-card",
                x: x + 3.0 - 4.0,
                y: y - 5.5,
                width: 8,
                height: 11,
                rx: 1.2,
                transform: "rotate(-14 {x + 3.0} {y})",
            }
            rect {
                class: "te-glyph-card",
                x: x + 7.0 - 4.0,
                y: y - 5.0,
                width: 8,
                height: 11,
                rx: 1.2,
                transform: "rotate(10 {x + 7.0} {y + 0.5})",
            }
        },
        TEExpeditionSlot::SecretObjective => rsx! {
            text { class: "te-number te-number-large", x: x - 7.0, y, "1" }
            image {
                href: TiIconType::SecretObjective.get_asset(),
                x: x + 6.0 - 6.5,
                y: y - 6.5,
                width: 13,
                height: 13,
            }
        },
        TEExpeditionSlot::TechSpecialityPlanet => {
            let icons = [
                (TiIconType::BioticFilled, -1.0, -1.0),
                (TiIconType::PropulsionFilled, 1.0, -1.0),
                (TiIconType::WarfareFilled, -1.0, 1.0),
                (TiIconType::CyberneticFilled, 1.0, 1.0),
            ];

            rsx! {
                for (i , (icon , dx , dy)) in icons.into_iter().enumerate() {
                    image {
                        key: "{i}",
                        href: icon.get_asset(),
                        x: x + dx * 6.0 - 5.5,
                        y: y + dy * 6.0 - 5.5,
                        width: 11,
                        height: 11,
                    }
                }
            }
        }
        TEExpeditionSlot::TradeGoods => rsx! {
            text { class: "te-number te-number-large", x: x - 7.0, y, "3" }
            image {
                href: TiIconType::TradeGoods.get_asset(),
                x: x + 6.5 - 7.5,
                y: y - 7.5,
                width: 15,
                height: 15,
            }
        },
    }
}
