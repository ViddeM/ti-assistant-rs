use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{Event, FrontierCardAction},
    common::faction::Faction,
    components::{
        frontier_card::FrontierCard,
        planet::Planet,
        system::{SystemId, SystemType},
    },
    state::game_state::ActionPhaseProgress,
};

use crate::{
    components::{button::Button, dropdown::SystemDropdown},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::{phase_views::tactical_action::SelectPlanetAttachment, select_tech::SelectTechView},
};

#[component]
pub fn FrontierCardView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player()
            .expect("Current player to exist in action phase")
    });

    let card = use_memo(move || match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::FrontierCard(frontier)) => frontier.card.clone(),
        _ => panic!("Unexpected action progress for phase Frontier Card"),
    });

    rsx! {
        div { class: "card column",
            h2 { "{card().info().name}" }
            if view.is_active() {
                FrontierCardProgressView { card }
            } else {
                p { "Not your turn, currently {current_player()} is playing" }
            }
        }
    }
}

#[component]
fn FrontierCardProgressView(card: ReadSignal<FrontierCard>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player()
            .expect("Current player to exist in action phase")
    });

    let current_player_faction = use_memo(move || {
        gc.game_state()
            .players
            .get(&current_player())
            .map(|p| p.faction.clone())
            .expect("Current player to exist")
    });

    match card() {
        FrontierCard::EnigmaticDevice => rsx! {
            div { class: "screen-container",
                if current_player_faction() == Faction::NekroVirus {
                    div { class: "column",
                        p { "Nekro virus cannot research techs" }
                        Button {
                            onclick: move |_| {
                                event
                                    .send_event(Event::FrontierCardActionCommit {
                                        player: current_player(),
                                        data: None,
                                    });
                            },
                            "Commit"
                        }
                    }
                } else {
                    p { class: "warning-text", "Remember: pay 6 resources" }
                    SelectTechView {
                        player_id: current_player(),
                        on_select: move |tech| {
                            event
                                .send_event(Event::FrontierCardActionCommit {
                                    player: current_player(),
                                    data: Some(FrontierCardAction::EnigmaticDevice {
                                        tech,
                                    }),
                                });
                        },
                    }
                }
            }
        },
        FrontierCard::Mirage => rsx! {
            MirageView {}
        },
        _ => rsx! {
            Button {
                onclick: move |_| {
                    event
                        .send_event(Event::FrontierCardActionCommit {
                            player: current_player(),
                            data: None,
                        });
                },
                "Commit"
            }
        },
    }
}

#[component]
fn MirageView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_system = use_signal(|| None);
    let mut selected_attachment = use_signal(|| None);

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player()
            .expect("Current player to exist in action phase")
    });

    let systems_without_planets = use_memo(move || {
        let mut systems = gc
            .game_options()
            .systems
            .iter()
            .filter(|(_, s)| s.planets.is_empty())
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();
        systems.sort();
        systems
    });

    let used_systems_without_planets = use_memo(move || {
        let s: Option<Vec<SystemId>> =
            gc.game_state()
                .map_data
                .milty_information
                .as_ref()
                .map(|i| {
                    let mut s = i
                        .hex_map
                        .tiles
                        .iter()
                        .filter(|t| systems_without_planets().contains(&t.system))
                        .filter(|t| {
                            matches!(
                                gc.game_options()
                                    .systems
                                    .get(&t.system)
                                    .as_ref()
                                    .map(|s| &s.system_type),
                                Some(&SystemType::Hyperlane)
                            )
                        })
                        .map(|t| t.system.clone())
                        .collect::<Vec<SystemId>>();
                    s.sort();
                    s
                });
        s
    });

    rsx! {
        div { class: "column",
            if let Some(systems) = used_systems_without_planets() {
                fieldset { class: "center-row full-width",
                    legend { "Pick system" }
                    SystemDropdown {
                        value: selected_system,
                        on_select: move |s| selected_system.set(s),
                        options: systems,
                    }
                }
            } else {
                p { "Can only select system when game is imported from milty draft" }
            }

            fieldset {
                legend { "Select planet attachment" }
                div { class: "column",
                    if used_systems_without_planets().is_none() || selected_system().is_some() {
                        SelectPlanetAttachment {
                            planet: Planet::Mirage,
                            attachment: selected_attachment,
                            previous_owner: None,
                            select_attachment: move |att| selected_attachment.set(Some(att)),
                        }
                    } else {
                        p { "Select system first..." }
                    }
                }
            }

            Button {
                disabled: used_systems_without_planets().is_some() && selected_system().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::FrontierCardActionCommit {
                            player: current_player(),
                            data: Some(FrontierCardAction::Mirage {
                                system: selected_system(),
                                attachment: selected_attachment(),
                            }),
                        })
                },
                "Commit"
            }
        }
    }
}
