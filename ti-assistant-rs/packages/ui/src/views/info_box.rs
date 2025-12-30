use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaPause, FaPlay},
    Icon,
};
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
    let gc = use_context::<GameContext>();
    let player = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or("None".to_string())
    });

    let round = use_memo(move || gc.game_state().round);

    let play_pause_icon = use_memo(move || {
        if gc.game_state().time_tracking_paused {
            rsx! {
                Icon {
                    class: "inline-icon",
                    width: None,
                    height: None,
                    icon: FaPlay,
                }
            }
        } else {
            rsx! {
                Icon {
                    class: "inline-icon",
                    width: None,
                    height: None,
                    icon: FaPause,
                }
            }
        }
    });

    rsx! {
        document::Stylesheet { href: INFO_BOX_SCSS }

        div { class: "card game-info-card",
            h4 { "Game: {game_id}" }
            div { class: "view-mode-button-group",
                ViewModeButton { view_mode: ViewMode::Game, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Score, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Techs, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Planets, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Laws, current: view_mode }
                ViewModeButton { view_mode: ViewMode::Map, current: view_mode }
            }
            p { class: "margin-top", "Round: {round()}" }
            p { "Current player: {current_player}" }
            p { "Currently viewing: {player.display()}" }
            p { "Current phase: {gc.game_state().phase}" }
            div {
                Button {
                    class: "margin-right",
                    onclick: move |_| {
                        if gc.game_state().time_tracking_paused { event.play() } else { event.pause() }
                    },
                    {play_pause_icon}
                }
                Button { onclick: move |_| event.undo(), "Undo" }
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
