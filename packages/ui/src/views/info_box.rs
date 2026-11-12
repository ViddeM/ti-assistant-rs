use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaPause, FaPlay},
};
use ti_helper_game_data::{actions::event::Event, components::phase::Phase, game_id::GameId};

use crate::{
    components::{button::Button, info_button::InfoButton},
    data::{
        event_context::EventContext,
        game_context::GameContext,
        info_context::Info,
        player_view::PlayerViewContext,
        view_mode::{ViewMode, ViewModeContext},
    },
    views::{
        galactic_events_view_mode::has_admin_section,
        history_view_mode::{HistoryPlayer, event_name},
    },
};

const INFO_BOX_SCSS: Asset = asset!("/assets/styling/views/info_box.scss");
const INFO_MODAL_SCSS: Asset = asset!("/assets/styling/components/info_modal.scss");

#[component]
pub fn InfoBox() -> Element {
    let game_id = use_context::<GameId>();
    let gc = use_context::<GameContext>();
    let player = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();
    let mut undo_entry = use_signal(|| None);

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or("None".to_string())
    });

    let round = use_memo(move || gc.game_state().round);

    let galactic_events = use_memo(move || {
        let mut events = gc
            .game_state()
            .galactic_events
            .in_play
            .iter()
            .copied()
            .collect::<Vec<_>>();
        events.sort_by_key(|e| e.info().name);
        events
    });

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

    let relics_enabled = use_memo(move || !gc.game_options().relics.is_empty());
    let te_enabled = use_memo(move || gc.game_state().game_settings.expansions.thunders_edge);
    let has_galactic_event_sections = use_memo(move || {
        gc.game_state()
            .galactic_events
            .in_play
            .iter()
            .any(has_admin_section)
    });

    rsx! {
        document::Stylesheet { href: INFO_BOX_SCSS }

        div { class: "card game-info-card",
            h4 { "Game: {game_id}" }
            div { class: "view-mode-button-group",
                ViewModeButton { view_mode: ViewMode::Game }
                ViewModeButton { view_mode: ViewMode::Score }
                ViewModeButton { view_mode: ViewMode::Techs }
                ViewModeButton { view_mode: ViewMode::Planets }
                if relics_enabled() {
                    ViewModeButton { view_mode: ViewMode::Relics }
                }
                ViewModeButton { view_mode: ViewMode::Laws }
                ViewModeButton { view_mode: ViewMode::Map }
            }
            if te_enabled() {
                ViewModeButton { class: "margin-top", view_mode: ViewMode::ThundersEdge }
            }
            if has_galactic_event_sections() {
                ViewModeButton { class: "margin-top", view_mode: ViewMode::GalacticEvents }
            }
            ViewModeButton { class: "margin-top", view_mode: ViewMode::History }
            p { class: "margin-top", "Round: {round()}" }
            p { "Current player: {current_player}" }
            p { "Currently viewing: {player.display()}" }
            p { "Current phase: {gc.game_state().phase}" }
            if gc.game_state().phase != Phase::Creation && !galactic_events().is_empty() {
                p { class: "margin-top",
                    if galactic_events().len() == 1 {
                        "Galactic event:"
                    } else {
                        "Galactic events:"
                    }
                }
                for galactic_event in galactic_events() {
                    p {
                        key: "{galactic_event}",
                        class: "game-info-galactic-event",
                        InfoButton {
                            info: Info::GalacticEvent(galactic_event),
                            visibility: "hidden",
                        }
                        "{galactic_event.info().name}"
                        InfoButton { info: Info::GalacticEvent(galactic_event) }
                    }
                }
            }
            div {
                Button {
                    class: "margin-right",
                    onclick: move |_| {
                        if gc.game_state().time_tracking_paused { event.play() } else { event.pause() }
                    },
                    {play_pause_icon}
                }
                Button {
                    disabled: gc.game_state().last_event.is_none(),
                    onclick: move |_| undo_entry.set(gc.game_state().last_event.clone()),
                    "Undo"
                }
            }
        }

        if let Some(entry) = undo_entry() {
            UndoConfirm { entry, on_close: move |_| undo_entry.set(None) }
        }
    }
}

#[component]
fn UndoConfirm(entry: (Event, DateTime<Utc>), on_close: EventHandler) -> Element {
    let event = use_context::<EventContext>();
    let name = event_name(&entry.0);
    let player = entry.0.player();

    rsx! {
        document::Stylesheet { href: INFO_MODAL_SCSS }

        div {
            class: "info-modal-bg",
            tabindex: "-1",
            onmounted: move |e| async move {
                let _ = e.set_focus(true).await;
            },
            onclick: move |_| on_close.call(()),
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    on_close.call(());
                }
            },
            div { class: "info-modal", onclick: |e| e.stop_propagation(),
                h2 { "Undo this event?" }
                hr {}
                p { "{name}" }
                if let Some(player) = player {
                    p {
                        "Played by "
                        HistoryPlayer { player }
                    }
                }
                div { class: "undo-confirm-buttons",
                    Button { onclick: move |_| on_close.call(()), "Cancel" }
                    Button {
                        onclick: move |_| {
                            event.undo(entry.clone());
                            on_close.call(());
                        },
                        "Undo"
                    }
                }
            }
        }
    }
}

#[component]
fn ViewModeButton(class: Option<String>, view_mode: ViewMode) -> Element {
    let ctx = use_context::<ViewModeContext>();
    let disabled = ctx.get() == view_mode;

    rsx! {
        Button {
            disabled,
            class: class.unwrap_or_default(),
            onclick: move |_| ctx.set(view_mode),
            "{view_mode.label()}"
        }
    }
}
