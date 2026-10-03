//! This crate contains all shared UI for the workspace.

#[cfg(feature = "server")]
pub use api::server_side;

pub use api::endpoints;
pub use api::requests;
pub use ti_helper_game_data::*;

pub use api::messages::WsMessageOut;

pub mod components;
pub mod data;
pub mod views;

use dioxus::prelude::*;

const GLOBAL_SCSS: Asset = asset!("/assets/styling/globals.scss");
const FONT_SCSS: Asset = asset!("/assets/styling/fonts.scss");

#[component]
pub fn Setup() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: GLOBAL_SCSS }
        document::Link { rel: "stylesheet", href: FONT_SCSS }
    }
}
