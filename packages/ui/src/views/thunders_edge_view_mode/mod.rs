use std::collections::BTreeMap;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::system::System,
    state::expedition::{TECompletion, TEExpedition, TEExpeditionSlot},
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlayerDropdown, SystemDropdown},
    },
    data::{event_context::EventContext, game_context::GameContext},
    views::thunders_edge_view_mode::expedition_token::ExpeditionToken,
};

mod breakthrough;
mod expedition_token;

const THUNDERS_EDGE_VIEW_MODE_SCSS: Asset =
    asset!("/assets/styling/views/thunders_edge_view_mode.scss");

#[component]
pub fn ThundersEdgeViewMode() -> Element {
    let gc = use_context::<GameContext>();

    let thunders_edge = use_memo(move || gc.game_state().te_expedition.clone());

    rsx! {
        document::Stylesheet { href: THUNDERS_EDGE_VIEW_MODE_SCSS }

        if let Some(thunders_edge) = thunders_edge() {
            Expedition { progress: thunders_edge.progress.clone() }
            Completed { completed: thunders_edge.completed.clone() }
            Breakthroughs {}
        } else {
            div { class: "card te-error-card",
                p { "Thunder's edge not enabled but you are in Thunder's Edge view mode?" }
                p { "This is a bug, please report it!" }
            }
        }
    }
}

#[component]
fn Expedition(progress: BTreeMap<TEExpeditionSlot, PlayerId>) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_player: Signal<Option<PlayerId>> = use_signal(|| None);

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "card column",
            h3 { "Thunder's Edge Expedition" }

            p { "Select a player to claim" }
            PlayerDropdown {
                value: selected_player,
                options: players(),
                on_select: move |p| selected_player.set(p),
            }

            ExpeditionToken { progress, selected_player }
        }
    }
}

#[component]
fn Completed(completed: ReadSignal<Option<TECompletion>>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_player = use_signal(move || completed().map(|c| c.completed_by));
    let mut selected_system = use_signal(|| completed().map(|c| c.placed_in));
    let players = use_memo(move || gc.game_state().players.keys().cloned().collect::<Vec<_>>());
    let valid_systems = use_memo(move || {
        System::valid_thunders_edge_systems()
            .into_iter()
            .collect::<Vec<_>>()
    });

    let nothing_changed = use_memo(move || {
        let Some(p) = selected_player() else {
            return false;
        };
        let Some(s) = selected_system() else {
            return false;
        };
        completed()
            == Some(TECompletion {
                completed_by: p,
                placed_in: s,
            })
    });

    use_effect(move || {
        if let Some(c) = completed() {
            selected_player.set(Some(c.completed_by));
            selected_system.set(Some(c.placed_in));
        } else {
            selected_player.set(None);
            selected_system.set(None);
        }
    });

    rsx! {
        div { class: "card column margin-top",
            h3 { "Completed" }
            PlayerDropdown {
                class: "margin-top",
                value: selected_player,
                options: players(),
                on_select: move |p| { selected_player.set(p) },
            }
            SystemDropdown {
                class: "margin-top",
                value: selected_system,
                options: valid_systems,
                on_select: move |s| selected_system.set(s),
            }
            Button {
                class: "margin-top",
                disabled: nothing_changed() || selected_player().is_none() || selected_system().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::SetThundersEdgeCompletion {
                            new_completion: Some(TECompletion {
                                completed_by: selected_player().expect("Player to be set"),
                                placed_in: selected_system().expect("System to be set"),
                            }),
                        });
                },
                "Change"
            }
            Button {
                class: "margin-top",
                disabled: completed().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::SetThundersEdgeCompletion {
                            new_completion: None,
                        });
                },
                "Remove"
            }
        }
    }
}

#[component]
fn Breakthroughs() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || gc.game_state().players.clone());

    rsx! {
        div { class: "card margin-top",
            h3 { "Breakthroughs" }
            for (id , player) in players().into_iter() {
                breakthrough::PlayerBreakthrough { key: "{id}", player_id: id, player }
            }
        }
    }
}
