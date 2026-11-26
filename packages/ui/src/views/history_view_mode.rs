use chrono::{DateTime, Local, NaiveDate, Utc};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaChevronDown, FaChevronUp},
};
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::{button::Button, spinner::Spinner},
    data::{game_context::GameContext, server_data::ServerDataContext},
};

const HISTORY_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/history_view_mode.scss");

#[component]
pub fn HistoryViewMode() -> Element {
    let gc = use_context::<GameContext>();
    let server_data = use_context::<ServerDataContext>();

    use_effect(move || {
        let _ = gc.game_state();
        server_data.request_history();
    });

    rsx! {
        document::Stylesheet { href: HISTORY_VIEW_MODE_SCSS }

        div { class: "history-view-container",
            match server_data.history() {
                None => rsx! {
                    Spinner {}
                },
                Some(history) => {
                    let mut previous_date = None;
                    let entries = history
                        .iter()
                        .enumerate()
                        .rev()
                        .map(|(index, (event, timestamp))| {
                            let date = timestamp.with_timezone(&Local).date_naive();
                            let new_date = (previous_date != Some(date)).then_some(date);
                            previous_date = Some(date);
                            (index, event.clone(), *timestamp, new_date)
                        })
                        .collect::<Vec<_>>();
                    rsx! {
                        div { class: "card history-list",
                            for (index, event, timestamp, new_date) in entries {
                                HistoryEntry {
                                    key: "{index}",
                                    index,
                                    event,
                                    timestamp,
                                    new_date,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HistoryEntry(
    index: usize,
    event: Event,
    timestamp: DateTime<Utc>,
    new_date: Option<NaiveDate>,
) -> Element {
    let mut show_event = use_signal(|| false);

    let summary = event.summary();
    let time = timestamp
        .with_timezone(&Local)
        .format("%H:%M:%S")
        .to_string();

    rsx! {
        if let Some(date) = new_date {
            div { class: "history-date", "{date.format(\"%Y-%m-%d\")}" }
        }
        div { class: "history-entry",
            div { class: "history-entry-line",
                span { class: "history-index", "#{index}" }
                span { class: "history-event-name", {event_name(&event)} }
                span { class: "history-time", "{time}" }
            }
            div { class: "history-entry-line",
                span { class: "history-summary",
                    if let Some(player) = event.player() {
                        HistoryPlayer { player }
                        if !summary.is_empty() {
                            " · "
                        }
                    }
                    "{summary}"
                }
                Button {
                    class: "history-event-toggle",
                    title: "Show the full event",
                    aria_expanded: show_event(),
                    onclick: move |_| show_event.toggle(),
                    if show_event() {
                        Icon { class: "inline-icon", icon: FaChevronUp }
                    } else {
                        Icon { class: "inline-icon", icon: FaChevronDown }
                    }
                }
            }
            if show_event() {
                pre { class: "history-event",
                    {serde_json::to_string_pretty(&event).unwrap_or_else(|e| e.to_string())}
                }
            }
        }
    }
}

#[component]
pub fn HistoryPlayer(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let class = gc
        .game_state()
        .players
        .get(&player)
        .map(|p| format!("player-color-{}", p.color.name()));

    rsx! {
        span { class, "{player}" }
    }
}

pub fn event_name(event: &Event) -> String {
    let name: &'static str = event.into();
    let mut words = String::new();
    for c in name.chars() {
        if c.is_uppercase() && !words.is_empty() {
            words.push(' ');
        }
        words.push(c);
    }
    words
}
