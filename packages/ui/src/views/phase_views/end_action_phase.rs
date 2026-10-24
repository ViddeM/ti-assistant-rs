use std::collections::BTreeSet;

use dioxus::prelude::*;
use strum::IntoEnumIterator;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::system::{System, SystemId},
    state::{
        expedition::{TEExpedition, TEExpeditionSlot},
        game_state::{ActionPhaseProgress, ThundersEdgeProgress},
    },
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlayerDropdown, SystemDropdown},
        info_button::InfoButton,
    },
    data::{
        event_context::EventContext,
        game_context::GameContext,
        info_context::{Info, InfoDescription},
        player_view::PlayerViewContext,
    },
    views::thunders_edge_view_mode::{
        THUNDERS_EDGE_VIEW_MODE_SCSS, expedition_token::ExpeditionToken,
    },
};

const END_ACTION_PHASE_SCSS: Asset =
    asset!("/assets/styling/views/phase_views/end_action_phase.scss");

#[component]
pub fn EndActionPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist")
    });
    let has_passed = use_memo(move || {
        gc.game_state()
            .passed_players
            .contains(&current_player_id())
    });
    let expedition = use_memo(move || gc.game_state().te_expedition.clone());
    let thunders_edge_progress = use_memo(move || match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::ThundersEdge(progress)) => Some(progress.clone()),
        _ => None,
    });
    let completion_required = use_memo(move || {
        matches!(
            thunders_edge_progress(),
            Some(ThundersEdgeProgress::AwaitingPlacement { .. })
        )
    });

    rsx! {
        document::Stylesheet { href: END_ACTION_PHASE_SCSS }

        div { class: "card end-action-phase-view-container",
            h2 { "End player turn?" }
            if has_passed() {
                p { "{current_player_id()} has passed" }
            }

            if let Some(expedition) = expedition() {
                if !expedition.is_full() || completion_required() {
                    document::Stylesheet { href: THUNDERS_EDGE_VIEW_MODE_SCSS }
                    Expedition {
                        expedition,
                        progress: thunders_edge_progress(),
                        player: current_player_id(),
                        interactive: view.is_active(),
                    }
                }
            }

            {
                if view.is_active() {
                    rsx! {
                        Button {
                            disabled: has_passed() || completion_required(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::TakeAnotherTurn {
                                        player: current_player_id(),
                                    })
                            },
                            "Take another turn"
                        }
                        Button {
                            disabled: completion_required(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::EndTurn {
                                        player: current_player_id(),
                                    })
                            },
                            "End Turn"
                        }
                    }
                } else {
                    rsx! {
                        p { "Not your turn, currently {current_player_id()} is playing" }
                    }
                }
            }
        }
    }
}

/// The Thunder's Edge expedition, the current player can commit to it or, if they claimed the
/// final slot, place Thunder's Edge.
#[component]
fn Expedition(
    expedition: TEExpedition,
    progress: Option<ThundersEdgeProgress>,
    player: PlayerId,
    interactive: bool,
) -> Element {
    if matches!(
        progress,
        Some(ThundersEdgeProgress::AwaitingPlacement { .. })
    ) {
        return rsx! {
            CompleteExpedition { expedition, player, interactive }
        };
    }

    rsx! {
        CommitToExpedition {
            expedition,
            committed: progress.is_some(),
            player,
            interactive,
        }
    }
}

#[component]
fn CommitToExpedition(
    expedition: TEExpedition,
    committed: bool,
    player: PlayerId,
    interactive: bool,
) -> Element {
    let event = use_context::<EventContext>();

    let mut selected_slot: Signal<Option<TEExpeditionSlot>> = use_signal(|| None);

    let can_commit = interactive && !committed;

    let selectable = if can_commit {
        TEExpeditionSlot::iter()
            .filter(|slot| !expedition.progress.contains_key(slot))
            .collect::<BTreeSet<_>>()
    } else {
        BTreeSet::new()
    };

    // The slot might have been claimed (by an admin) after it was selected.
    let selected = selected_slot().filter(|slot| selectable.contains(slot));

    // Preview the player's pending claim on the wheel before it's committed.
    let mut preview_progress = expedition.progress.clone();
    if let Some(slot) = &selected {
        preview_progress.insert(slot.clone(), player.clone());
    }

    rsx! {
        div { class: "expedition-commit",
            h3 { "Thunder's Edge expedition" }

            if interactive {
                if committed {
                    p { "You have committed to the expedition this turn." }
                } else {
                    p { "You may commit to the expedition by claiming a slot." }
                }
            }

            ExpeditionToken {
                progress: preview_progress,
                selectable,
                selected: selected.clone(),
                on_select: move |slot| selected_slot.set(Some(slot)),
            }

            if let Some(slot) = selected {
                div { class: "warning-text",
                    InfoButton { info: slot_info(&slot) }
                    span { "{slot.description()}" }
                }
                Button {
                    onclick: {
                        let slot = slot.clone();
                        let player = player.clone();
                        move |_| {
                            event
                                .send_event(Event::CommitToThundersEdgeExpedition {
                                    player: player.clone(),
                                    slot: slot.clone(),
                                });
                            selected_slot.set(None);
                        }
                    },
                    "Commit to expedition"
                }
            }
        }
    }
}

fn slot_info(slot: &TEExpeditionSlot) -> Info {
    Info::Custom {
        title: slot.description().to_string(),
        subtitle: "Thunder's Edge expedition".to_string(),
        description: InfoDescription::Description(format!(
            "At the end of your turn you can commit to the expedition by claiming this slot and paying its cost: {}. \
            Each slot can only be claimed once, and the first slot you claim unlocks your faction's breakthrough.",
            slot.description()
        )),
    }
}

#[component]
fn CompleteExpedition(expedition: TEExpedition, player: PlayerId, interactive: bool) -> Element {
    let event = use_context::<EventContext>();

    let contributors = expedition
        .get_top_expedition_contributors()
        .ok()
        .flatten()
        .unwrap_or_default()
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let mut selected_system: Signal<Option<SystemId>> = use_signal(|| None);
    let mut selected_winner: Signal<Option<PlayerId>> = use_signal({
        let contributors = contributors.clone();
        move || (contributors.len() == 1).then(|| contributors[0].clone())
    });
    let valid_systems = use_memo(move || {
        let mut systems = System::valid_thunders_edge_systems()
            .into_iter()
            .collect::<Vec<_>>();
        systems.sort();
        systems
    });

    let winner_slots = selected_winner().map(|winner| {
        expedition
            .progress
            .values()
            .filter(|p| **p == winner)
            .count()
    });
    let contested = contributors.len() > 1;

    rsx! {
        div { class: "expedition-complete",
            h3 { "Thunder's Edge" }

            ExpeditionToken { progress: expedition.progress.clone() }

            p { "All expedition slots are claimed, {player} claimed the final slot and places Thunder's Edge." }

            if interactive {
                p { "Choose a system without planets, supernova or wormholes (not The Fracture)." }
                SystemDropdown {
                    class: "margin-top",
                    value: selected_system,
                    options: valid_systems,
                    on_select: move |s| selected_system.set(s),
                }

                if contested {
                    p { "Several players claimed the most slots, choose who takes control." }
                } else {
                    p { "The player with the most slots takes control." }
                }
                PlayerDropdown {
                    class: "margin-top",
                    value: selected_winner,
                    options: contributors.clone(),
                    on_select: move |p| selected_winner.set(p),
                    disabled: !contested,
                }
                if let Some(slots) = winner_slots {
                    p { "{selected_winner().unwrap()} places {slots} infantry on Thunder's Edge." }
                }

                Button {
                    class: "margin-top",
                    disabled: selected_system().is_none() || selected_winner().is_none(),
                    onclick: {
                        let player = player.clone();
                        move |_| {
                            event
                                .send_event(Event::CompleteThundersEdgeExpedition {
                                    player: player.clone(),
                                    placed_in: selected_system().expect("System to be set"),
                                    won_by: selected_winner().expect("Winner to be set"),
                                });
                        }
                    },
                    "Place Thunder's Edge"
                }
            } else {
                p { "Waiting for {player} to place Thunder's Edge." }
            }
        }
    }
}
