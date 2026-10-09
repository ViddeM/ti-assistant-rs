use dioxus::{html::geometry::WheelDelta, prelude::*};
use ti_helper_game_data::components::phase::Phase;

use crate::{
    data::game_context::GameContext,
    views::map_view_mode::{
        layout::{Bounds, MapLayout, TILE_HEIGHT, TILE_WIDTH, TokenKind, build_layout},
        tile_images::get_tile_image,
    },
};

mod layout;
mod planet_offset;
mod tile_images;

const MAP_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/map_view_mode.scss");

const MIRAGE_TOKEN: Asset = asset!("/assets/images/map/tokens/mirage_token.webp");
const DESTROYED_PLANET_TOKEN: Asset =
    asset!("/assets/images/map/tokens/destroyed_planet_token.webp");

/// How far (in map units) the camera can be moved from the center of the galaxy.
const MAX_PAN_X: f64 = 4000.0;
const MAX_PAN_Y: f64 = 2000.0;

/// Map units per screen pixel, lower is more zoomed in.
const MIN_SCALE: f64 = 0.5;
const MAX_SCALE: f64 = 15.0;

/// Zoom factor (exponent) per pixel scrolled when fully zoomed in/out, and how many pixels a 'line' and a 'page' of scrolling count as.
const WHEEL_ZOOM_SPEED_CLOSE: f64 = 0.0006;
const WHEEL_ZOOM_SPEED_FAR: f64 = 0.002;
const WHEEL_LINE_PIXELS: f64 = 40.0;
const WHEEL_PAGE_PIXELS: f64 = 800.0;

/// A flat-topped hexagon, centered on (0, 0), the size of a tile.
const TILE_POINTS: &str = "-182,0 -91,-158.5 91,-158.5 182,0 91,158.5 -91,158.5";

/// A flat-topped hexagon, centered on (0, 0), slightly smaller than a tile.
const EMPTY_SLOT_POINTS: &str = "-176,0 -88,-153 88,-153 176,0 88,153 -88,153";

/// How much extra space to leave around the map when fitting it to the screen.
const FIT_MARGIN: f64 = 1.05;

#[derive(Clone, Copy, Debug, PartialEq)]
struct View {
    /// Center of the view in map units.
    center_x: f64,
    center_y: f64,
    /// Map units per screen pixel.
    scale: f64,
}

impl View {
    fn fit(bounds: &Bounds, (width, height): (f64, f64)) -> Self {
        let (center_x, center_y) = bounds.center();
        let scale = (bounds.width() as f64 * FIT_MARGIN / width.max(1.0))
            .max(bounds.height() as f64 * FIT_MARGIN / height.max(1.0));

        Self {
            center_x: center_x as f64,
            center_y: center_y as f64,
            scale: scale.clamp(MIN_SCALE, MAX_SCALE),
        }
    }

    /// How fast the zoom is: faster when zoomed out and slower when zoomed in close to the tiles.
    fn zoom_speed(&self) -> f64 {
        let zoomed_out = ((self.scale / MIN_SCALE).ln() / (MAX_SCALE / MIN_SCALE).ln()).clamp(0.0, 1.0);
        WHEEL_ZOOM_SPEED_CLOSE + (WHEEL_ZOOM_SPEED_FAR - WHEEL_ZOOM_SPEED_CLOSE) * zoomed_out
    }

    fn view_box(&self, (width, height): (f64, f64)) -> String {
        let w = width * self.scale;
        let h = height * self.scale;
        format!(
            "{} {} {} {}",
            self.center_x - w / 2.0,
            self.center_y - h / 2.0,
            w,
            h
        )
    }

    fn panned(self, delta_x_px: f64, delta_y_px: f64) -> Self {
        Self {
            center_x: (self.center_x - delta_x_px * self.scale).clamp(-MAX_PAN_X, MAX_PAN_X),
            center_y: (self.center_y - delta_y_px * self.scale).clamp(-MAX_PAN_Y, MAX_PAN_Y),
            scale: self.scale,
        }
    }

    /// Zooms by `factor` whilst keeping the map position under the cursor (in pixels, relative to the map) in place.
    fn zoomed(self, factor: f64, cursor: (f64, f64), (width, height): (f64, f64)) -> Self {
        let scale = (self.scale * factor).clamp(MIN_SCALE, MAX_SCALE);

        let from_center_x = cursor.0 - width / 2.0;
        let from_center_y = cursor.1 - height / 2.0;
        let map_x = self.center_x + from_center_x * self.scale;
        let map_y = self.center_y + from_center_y * self.scale;

        Self {
            center_x: (map_x - from_center_x * scale).clamp(-MAX_PAN_X, MAX_PAN_X),
            center_y: (map_y - from_center_y * scale).clamp(-MAX_PAN_Y, MAX_PAN_Y),
            scale,
        }
    }
}

#[component]
pub fn MapViewMode() -> Element {
    let gc = use_context::<GameContext>();

    let layout = use_memo(move || build_layout(&gc.game_state()));
    let setup_done =
        use_memo(move || !matches!(gc.game_state().phase, Phase::Creation | Phase::Setup));

    rsx! {
        document::Stylesheet { href: MAP_VIEW_MODE_SCSS }

        if layout().is_none() {
            p { class: "map-message", "Only games imported from milty can be rendered." }
        } else if !setup_done() {
            p { class: "map-message", "Setup must be finalized to view the map." }
        } else if let Some(layout) = layout() {
            MapSvg { layout }
        }
    }
}

#[component]
fn MapSvg(layout: ReadSignal<MapLayout>) -> Element {
    // `None` means 'fit the whole map on screen'.
    let mut view = use_signal(|| None::<View>);
    let mut size = use_signal(|| (1280.0, 720.0));
    let mut drag_from = use_signal(|| None::<(f64, f64)>);

    let current_view = move || view().unwrap_or_else(|| View::fit(&layout.read().bounds(), size()));

    rsx! {
        div { class: "map-view-container",
            button { class: "map-reset-button", onclick: move |_| view.set(None), "Reset view" }
            svg {
                class: "map-svg",
                view_box: current_view().view_box(size()),
                onresize: move |e| {
                    if let Ok(s) = e.data().get_content_box_size() {
                        size.set((s.width, s.height));
                    }
                },
                onwheel: move |e| {
                    e.prevent_default();
                    let delta_y = match e.delta() {
                        WheelDelta::Pixels(d) => d.y,
                        WheelDelta::Lines(d) => d.y * WHEEL_LINE_PIXELS,
                        WheelDelta::Pages(d) => d.y * WHEEL_PAGE_PIXELS,
                    };
                    let cursor = e.element_coordinates();
                    let current = current_view();
                    let factor = (delta_y * current.zoom_speed()).exp();
                    view.set(Some(current.zoomed(factor, (cursor.x, cursor.y), size())));
                },
                onpointerdown: move |e| {
                    let p = e.client_coordinates();
                    drag_from.set(Some((p.x, p.y)));
                },
                onpointermove: move |e| {
                    if let Some((last_x, last_y)) = drag_from() {
                        let p = e.client_coordinates();
                        view.set(Some(current_view().panned(p.x - last_x, p.y - last_y)));
                        drag_from.set(Some((p.x, p.y)));
                    }
                },
                onpointerup: move |_| drag_from.set(None),
                onpointerleave: move |_| drag_from.set(None),

                MapContents { layout }
            }
        }
    }
}

/// Split out from [MapSvg] so that panning and zooming doesn't re-render all the tiles.
#[component]
fn MapContents(layout: ReadSignal<MapLayout>) -> Element {
    let layout = layout.read();

    rsx! {
        for (i, (x, y)) in layout.empty_slots.iter().enumerate() {
            polygon {
                key: "empty-{i}",
                class: "map-empty-slot",
                points: EMPTY_SLOT_POINTS,
                transform: "translate({x} {y})",
            }
        }

        for (i, tile) in layout.tiles.iter().enumerate() {
            g {
                key: "tile-{i}-{tile.system_id}",
                transform: "translate({tile.x} {tile.y}) rotate({tile.rotation_degrees})",
                if let Some(image) = get_tile_image(&tile.system_id) {
                    image {
                        href: image,
                        x: -TILE_WIDTH / 2.0,
                        y: -TILE_HEIGHT / 2.0,
                        width: TILE_WIDTH,
                        height: TILE_HEIGHT,
                    }
                } else {
                    polygon { class: "map-missing-tile", points: TILE_POINTS }
                }
            }
        }

        for (i, token) in layout.tokens.iter().enumerate() {
            image {
                key: "token-{i}",
                href: match token.kind {
                    TokenKind::Mirage => MIRAGE_TOKEN,
                    TokenKind::DestroyedPlanet => DESTROYED_PLANET_TOKEN,
                },
                x: token.x - token.width / 2.0,
                y: token.y - token.height / 2.0,
                width: token.width,
                height: token.height,
            }
        }

        for (i, tile) in layout.tiles.iter().enumerate() {
            text {
                key: "tile-id-{i}-{tile.system_id}",
                class: "map-tile-id",
                x: tile.x,
                y: tile.y,
                "{tile.system_id}"
            }
        }

        for owner in layout.owners.iter() {
            g {
                key: "owner-{owner.planet}",
                class: "map-owner map-owner-{owner.color.name()}",
                rect {
                    class: "map-owner-bg",
                    x: owner.x - 50.0,
                    y: owner.y - 12.5,
                    width: 100,
                    height: 25,
                }
                text { class: "map-owner-text", x: owner.x, y: owner.y, "{owner.name}" }
            }
        }
    }
}
