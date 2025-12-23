use dioxus::prelude::*;
use ti_helper_game_data::game_id::GameId;

use crate::{
    components::button::Button,
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
        view_mode::ViewMode,
    },
};

const INFO_BOX_SCSS: Asset = asset!("/assets/styling/views/info_box.scss");

#[component]
pub fn InfoBox(view_mode: Signal<ViewMode>) -> Element {
    let game_id = use_context::<GameId>();
    let game_context = use_context::<GameContext>();
    let player = use_context::<PlayerViewContext>();
    let ec = use_context::<EventContext>();

    let game_state = game_context.game_state();

    rsx! {
        document::Stylesheet { href: INFO_BOX_SCSS }

        div { class: "card",
            b { "Game: {game_id}" }
            div { class: "view-mode-button-group",
                ViewModeButton { view_mode: ViewMode::Game, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Score, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Techs, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Planets, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Laws, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Map, current: view_mode }
            }
            div {
                b { "Round: {game_state.round}" }
            }
            div {
                b { "Currently viewing: {player.display()}" }
            }
            div {
                if game_state.time_tracking_paused {
                    Button { onclick: move |_| ec.pause(), "Pause" }
                } else {
                    Button { onclick: move |_| ec.play(), "Play" }
                }
                Button { onclick: move |_| ec.undo(), "Undo" }
            }
        }
    }
}

#[component]
fn ViewModeButton(current: Signal<ViewMode>, view_mode: ViewMode) -> Element {
    let disabled = current.read().eq(&view_mode);

    rsx! {
        Button { disabled, onclick: move |_| current.set(view_mode), "{view_mode.to_string()}" }
    }
}
