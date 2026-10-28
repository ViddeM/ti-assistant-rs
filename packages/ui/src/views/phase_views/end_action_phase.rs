use std::collections::BTreeSet;

use dioxus::prelude::*;
use strum::IntoEnumIterator;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{
        action_card::ActionCard,
        planet::Planet,
        system::{System, SystemId},
    },
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
    views::{
        book_of_latvinia_research::BookOfLatviniaResearch,
        thunders_edge_view_mode::{
            THUNDERS_EDGE_VIEW_MODE_SCSS, expedition_token::ExpeditionToken,
        },
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
    let owns_thunders_edge = use_memo(move || {
        gc.game_state().players[&current_player_id()]
            .planets
            .contains_key(&Planet::ThundersEdge)
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
        document::Stylesheet { href: THUNDERS_EDGE_VIEW_MODE_SCSS }

        div { class: "card end-action-phase-view-container",
            h2 { "End player turn?" }
            if has_passed() {
                p { "{current_player_id()} has passed" }
            }

            if let Some(expedition) = expedition() {
                if !expedition.is_full() || completion_required() {
                    Expedition {
                        expedition,
                        progress: thunders_edge_progress(),
                        player_id: current_player_id(),
                        interactive: view.is_active(),
                    }
                }
            }

            BookOfLatviniaResearch {}

            if view.is_active() {
                fieldset { class: "end-of-turn-section",
                    legend { "Turn" }
                    div { class: "end-of-turn-buttons",
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
                        if owns_thunders_edge() {
                            Button {
                                disabled: has_passed() || completion_required(),
                                onclick: move |_| {
                                    event
                                        .send_event(Event::TakeAnotherTurn {
                                            player: current_player_id(),
                                        })
                                },
                                "Jupiter Brain: take 1 more action"
                            }
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
                }
                EndOfTurnActionCards { disabled: completion_required() }
            } else {
                p { "Not your turn, currently {current_player_id()} is playing" }
            }
        }
    }
}

/// Crisis and Puppets on a String, which any player can play at the end of the turn.
#[component]
fn EndOfTurnActionCards(disabled: bool) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_card = use_signal(|| None);

    let crisis_enabled =
        use_memo(move || gc.game_options().action_cards.contains(&ActionCard::Crisis));
    let puppets_enabled = use_memo(move || {
        gc.game_options()
            .action_cards
            .contains(&ActionCard::PuppetsOnAString)
    });

    let players = use_memo(move || gc.game_state().players.keys().cloned().collect::<Vec<_>>());
    let passed_players = use_memo(move || {
        gc.game_state()
            .passed_players
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });
    let can_skip_turn = use_memo(move || {
        let state = gc.game_state();
        state.players.len() - state.passed_players.len() >= 2
    });
    let can_perform_action = use_memo(move || {
        !passed_players().is_empty() && gc.game_state().interrupted_turn.is_none()
    });

    if !crisis_enabled() && !puppets_enabled() {
        return rsx! {};
    }

    rsx! {
        fieldset { class: "end-of-turn-section",
            legend { "Action cards" }
            div { class: "end-of-turn-buttons",
                if crisis_enabled() {
                    Button {
                        disabled: disabled || !can_skip_turn() || selected_card() == Some(ActionCard::Crisis),
                        onclick: move |_| selected_card.set(Some(ActionCard::Crisis)),
                        "Crisis"
                    }
                }
                if puppets_enabled() {
                    Button {
                        disabled: disabled || !can_perform_action()
                            || selected_card() == Some(ActionCard::PuppetsOnAString),
                        onclick: move |_| selected_card.set(Some(ActionCard::PuppetsOnAString)),
                        "Puppets on a String"
                    }
                }
            }
            match selected_card() {
                Some(ActionCard::Crisis) => rsx! {
                    PlayEndOfTurnActionCard {
                        card: ActionCard::Crisis,
                        players: players(),
                        on_play: move |player| event.send_event(Event::PlayCrisis { player }),
                    }
                },
                Some(ActionCard::PuppetsOnAString) => rsx! {
                    PlayEndOfTurnActionCard {
                        card: ActionCard::PuppetsOnAString,
                        players: passed_players(),
                        on_play: move |player| event.send_event(Event::PlayPuppetsOnAString { player }),
                    }
                },
                _ => rsx! {},
            }
        }
    }
}

/// Choose the player that plays `card`, `player_question` explains who that is.
#[component]
fn PlayEndOfTurnActionCard(
    card: ActionCard,
    players: Vec<PlayerId>,
    on_play: EventHandler<PlayerId>,
) -> Element {
    let mut player = use_signal(|| None);

    rsx! {
        div { class: "column margin-top",
            p { "{card.info().name}: {card.info().effect}" }
            p { class: "margin-top", "Played by:" }
            PlayerDropdown {
                value: player,
                options: players,
                on_select: move |p| player.set(p),
            }
            Button {
                class: "margin-top",
                disabled: player().is_none(),
                onclick: move |_| on_play(player().expect("Player to be selected")),
                "Play"
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
    player_id: PlayerId,
    interactive: bool,
) -> Element {
    if matches!(
        progress,
        Some(ThundersEdgeProgress::AwaitingPlacement { .. })
    ) {
        return rsx! {
            CompleteExpedition { expedition, player_id, interactive }
        };
    }

    rsx! {
        CommitToExpedition {
            expedition,
            committed: progress.is_some(),
            player_id,
            interactive,
        }
    }
}

#[component]
fn CommitToExpedition(
    expedition: TEExpedition,
    committed: bool,
    player_id: PlayerId,
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
        preview_progress.insert(slot.clone(), player_id.clone());
    }

    rsx! {
        fieldset { class: "expedition-commit column",
            legend { "Thunder's Edge expedition" }

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
                        let player_id = player_id.clone();
                        move |_| {
                            event
                                .send_event(Event::CommitToThundersEdgeExpedition {
                                    player: player_id.clone(),
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
fn CompleteExpedition(
    expedition: ReadSignal<TEExpedition>,
    player_id: PlayerId,
    interactive: bool,
) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let contributors = use_memo(move || {
        expedition()
            .get_top_expedition_contributors()
            .unwrap_or_default()
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    });

    let mut selected_system: Signal<Option<SystemId>> = use_signal(|| None);
    let mut selected_winner: Signal<Option<PlayerId>> = use_signal({
        let contributors = contributors.clone();
        move || (contributors().len() == 1).then(|| contributors()[0].clone())
    });
    let valid_systems = use_memo(move || {
        let mut systems = System::valid_thunders_edge_systems()
            .into_iter()
            .collect::<Vec<_>>();
        systems.sort();
        systems
    });

    let winner_slots = use_memo(move || {
        selected_winner().map(|winner| {
            expedition()
                .progress
                .values()
                .filter(|p| **p == winner)
                .count()
        })
    });
    let contested = use_memo(move || contributors().len() > 1);
    let p1 = player_id.clone();
    let player_color = use_memo(move || gc.game_state().players[&p1].color.clone());

    rsx! {
        fieldset { class: "expedition-complete",
            legend { "Thunder's Edge" }

            ExpeditionToken { progress: expedition().progress.clone() }

            p { "Expedition complete, place Thunder's Edge." }

            fieldset { class: "player-color-border-{player_color()} action-player-container column",
                legend { "{player_id}" }

                if interactive {
                    SystemDropdown {
                        class: "margin-top",
                        value: selected_system,
                        options: valid_systems,
                        on_select: move |s| selected_system.set(s),
                    }

                    PlayerDropdown {
                        class: "margin-top",
                        value: selected_winner,
                        options: contributors(),
                        on_select: move |p| selected_winner.set(p),
                        disabled: !contested(),
                    }

                    if let Some(slots) = winner_slots() {
                        p { class: "margin-top",
                            "{selected_winner().unwrap()} places {slots} infantry on Thunder's Edge."
                        }
                    }

                    Button {
                        class: "margin-top",
                        disabled: selected_system().is_none() || selected_winner().is_none(),
                        onclick: {
                            let player = player_id.clone();
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
                    p { "Waiting for {player_id} to place Thunder's Edge." }
                }
            }
        }
    }
}
