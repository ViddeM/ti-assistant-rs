use api::endpoints::game_history;
use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId, game_id::GameId};

use crate::{components::spinner::Spinner, data::game_context::GameContext};

const HISTORY_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/history_view_mode.scss");

#[component]
pub fn HistoryViewMode() -> Element {
    let game_id = use_context::<GameId>();
    let gc = use_context::<GameContext>();

    let history = use_resource(move || async move {
        let _ = gc.game_state();
        game_history(game_id).await
    });

    rsx! {
        document::Stylesheet { href: HISTORY_VIEW_MODE_SCSS }

        div { class: "history-view-container",
            match &*history.read() {
                None => rsx! {
                    Spinner {}
                },
                Some(Err(err)) => rsx! {
                    p { "Failed to load the history: {err}" }
                },
                Some(Ok(history)) => rsx! {
                    div { class: "card",
                        table { class: "history-table",
                            thead {
                                tr {
                                    th { "#" }
                                    th { "Event" }
                                    th { "Player" }
                                    th { "Time" }
                                }
                            }
                            tbody {
                                for (index, (event, timestamp)) in history.iter().enumerate().rev() {
                                    tr { key: "{index}",
                                        td { "{index}" }
                                        td { {event_name(event)} }
                                        td {
                                            if let Some(player) = event.player() {
                                                HistoryPlayer { player }
                                            }
                                        }
                                        td {
                                            {timestamp.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string()}
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
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
