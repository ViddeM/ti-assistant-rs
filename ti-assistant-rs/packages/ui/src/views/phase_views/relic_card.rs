use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{Event, RelicAction},
    components::{
        planet_attachment::PlanetAttachment,
        relic::Relic,
        system::{System, SystemType},
    },
    state::game_state::ActionPhaseProgress,
};

use crate::{
    components::{button::Button, dropdown::PlanetDropdown},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn RelicCardView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::Relic(r)) => {
            rsx! {
                div { class: "card column",
                    h2 { "{r.relic.info().name}" }
                    {
                        if view.is_active() {
                            rsx! {
                                RelicProgressView { card: r.relic.clone() }
                            }
                        } else {
                            rsx! {
                                p { "Not your turn, currently {current_player_id()}" }
                            }
                        }
                    }
                }
            }
        }
        _ => rsx! {},
    }
}

#[component]
fn RelicProgressView(card: Relic) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    match card {
        Relic::StellarConverter => rsx! {
            StellarConvertersView {}
        },
        Relic::NanoForge => rsx! {
            NanoForgeView {}
        },
        _ => rsx! {
            Button {
                onclick: move |_| {
                    event
                        .send_event(Event::RelicActionCommit {
                            player: current_player_id(),
                            data: None,
                        })
                },
            }
        },
    }
}

#[component]
fn StellarConvertersView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let mut selected_planet = use_signal(|| None);

    let planets_with_legendary_attachments = use_memo(move || {
        gc.game_state()
            .players
            .values()
            .map(|p| p.planets.iter())
            .flatten()
            .filter(|(_, attachments)| {
                attachments
                    .iter()
                    .filter(|a| a.info().set_legendary)
                    .count()
                    > 0
            })
            .map(|(p, _)| p)
            .cloned()
            .collect::<Vec<_>>()
    });
    let available_planets = use_memo(move || {
        let mut planets = gc
            .game_options()
            .planet_infos
            .keys()
            .filter(|p| !p.is_mecatol_rex())
            .filter(|p| !p.info().is_legendary)
            .filter(|&p| !planets_with_legendary_attachments().contains(p))
            .filter(|p| {
                let go = gc.game_options().clone();
                let (_, info) = go
                    .systems
                    .iter()
                    .find(|(_, info)| info.planets.contains(p))
                    .expect("Planet to be in a system");

                !matches!(info.system_type, SystemType::HomeSystem(..))
            })
            .cloned()
            .collect::<Vec<_>>();

        planets.sort();

        planets
    });

    rsx! {
        fieldset {
            legend { "Stellar Converters" }
            div { class: "column",
                PlanetDropdown {
                    value: selected_planet(),
                    on_select: move |p| {
                        selected_planet.set(p);
                    },
                    options: available_planets(),
                }
                Button {
                    class: "margin-top",
                    disabled: selected_planet().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::RelicActionCommit {
                                player: current_player_id(),
                                data: Some(RelicAction::StellarConverter {
                                    planet: selected_planet().expect("Planet to be selected"),
                                }),
                            })
                    },
                    "Commit"
                }
            }
        }
    }
}

#[component]
fn NanoForgeView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_planet = use_signal(|| None);

    let available_planets = use_memo(move || {
        let state = gc.game_state();
        let mut planets = state
            .players
            .values()
            .flat_map(|p| p.planets.iter().map(|(p, a)| (p.clone(), p.info(), a)))
            .filter(|(_, i, _)| !i.is_legendary)
            .filter(|(_, _, attachments)| !attachments.contains(&PlanetAttachment::NanoForge))
            .filter(|(p, _, _)| {
                let system_type = System::for_planet(p)
                    .expect("Planet to have system")
                    .system_type;
                !matches!(system_type, SystemType::HomeSystem(..))
            })
            .map(|(p, _, _)| p)
            .collect::<Vec<_>>();
        planets.sort();
        planets
    });
    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to be set for action phase")
    });

    rsx! {
        fieldset {
            legend { "Nano-Forge" }
            div { class: "column",
                PlanetDropdown {
                    value: selected_planet,
                    options: available_planets(),
                    on_select: move |p| selected_planet.set(p),
                }
                Button {
                    class: "margin-top",
                    disabled: selected_planet().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::RelicActionCommit {
                                player: current_player_id(),
                                data: Some(RelicAction::NanoForge {
                                    planet: selected_planet().expect("Planet to be set"),
                                }),
                            })
                    },
                    "Commit"
                }
            }
        }
    }
}
