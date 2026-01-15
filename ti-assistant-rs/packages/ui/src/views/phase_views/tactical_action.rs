use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{planet::Planet, planet_attachment::PlanetAttachment},
    state::game_state::{ActionPhaseProgress, TacticalProgress},
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlanetAttachmentDropdown, PlanetDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

const TACTICAL_ACTION_VIEW: Asset =
    asset!("/assets/styling/views/phase_views/tactical_action.scss");

#[component]
pub fn TacticalActionView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let tactical = use_memo(move || gc.game_state().action_progress.clone());

    rsx! {
        document::Stylesheet { href: TACTICAL_ACTION_VIEW }

        if let Some(progress) = tactical() {
            div { class: "card tactical-container",
                if let ActionPhaseProgress::Tactical(t) = progress {
                    h2 { "Tactical" }
                    if view.is_active() {
                        TacticalActionProgressView { progress: t }
                    } else {
                        p { "Not your turn, waiting for {current_player()}" }
                    }
                }
            }
        }
    }
}

#[component]
fn TacticalActionProgressView(progress: ReadSignal<TacticalProgress>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let taken_planets = use_memo(move || {
        progress()
            .taken_planets
            .iter()
            .map(|(planet, previous_owner)| (planet.clone(), previous_owner.clone()))
            .collect::<Vec<_>>()
    });

    let current_player_planets = use_memo(move || {
        let player = gc
            .game_state()
            .players
            .iter()
            .find(|&(p, _)| p.eq(&current_player()))
            .map(|(_, p)| p.clone())
            .expect("Current player to be in list of players");
        let mut planets = player.planets.keys().cloned().collect::<Vec<_>>();
        planets.sort();
        planets
    });

    // Only allow players to take mirage if someone else owns it, otherwise they'll need to use a frontier action.
    let any_player_owns_mirage = use_memo(move || {
        gc.game_state()
            .players
            .values()
            .find(|p| p.planets.contains_key(&Planet::Mirage))
            .is_some()
    });

    let activated_system = use_memo(move || progress().activated_system);
    let available_planets_in_system = use_memo(move || {
        let mut planets = gc
            .game_options()
            .systems
            .values()
            .filter(|&s| activated_system().as_ref().eq(&Some(&s.id)))
            .flat_map(|s| s.planets.iter())
            .filter(|&p| {
                !current_player_planets().contains(p)
                    && !taken_planets().iter().find(|(ps, _)| ps.eq(p)).is_some()
            })
            .cloned()
            .collect::<Vec<_>>();
        planets.sort();
        planets
    });

    let all_planets_not_owned = use_memo(move || {
        let mut planets = gc
            .game_options()
            .planet_infos
            .iter()
            .filter(|&(p, _)| !p.eq(&Planet::Mirage) || any_player_owns_mirage())
            .filter(|(p, _)| !current_player_planets().contains(p))
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>();
        planets.sort();
        planets
    });

    let mut selected_planet = use_signal(|| None);

    let attachments = use_memo(move || progress().planet_attachments);

    rsx! {
        if taken_planets().len() > 0 {
            for (planet , previous_owner) in taken_planets().iter() {
                fieldset { key: "{planet}",
                    legend { "{planet.info().name}" }

                    div { class: "column",
                        SelectPlanetAttachment {
                            planet: planet.clone(),
                            attachment: attachments().get(planet).cloned(),
                            previous_owner: previous_owner.clone(),
                            select_attachment: {
                                let planet = planet.clone();
                                move |attachment| {
                                    event
                                        .send_event(Event::TacticalActionAttachPlanetAttachment {
                                            player: current_player(),
                                            planet: planet.clone(),
                                            attachment,
                                        })
                                }
                            },
                        }
                    }
                }
            }
            if !available_planets_in_system().is_empty() {
                fieldset {
                    legend { "Take another planet" }
                    div { class: "select-another-planet-container",
                        for planet in available_planets_in_system() {
                            Button {
                                key: "{planet}",
                                onclick: {
                                    let planet = planet.clone();
                                    move |_| {
                                        event
                                            .send_event(Event::TacticalActionTakePlanet {
                                                player: current_player(),
                                                planet: planet.clone(),
                                            })
                                    }
                                },
                                "{planet.info().name}"
                            }
                        }
                    }
                }
            }
        } else {
            div { class: "take-planet-container",
                label { "Take planet:" }
                PlanetDropdown {
                    options: all_planets_not_owned(),
                    value: selected_planet(),
                    on_select: move |planet| selected_planet.set(planet),
                }
                Button {
                    disabled: selected_planet().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::TacticalActionTakePlanet {
                                player: current_player(),
                                planet: selected_planet().expect("Selected planet to be set"),
                            })
                    },
                    "Take"
                }
            }
        }
        Button {
            onclick: move |_| {
                event
                    .send_event(Event::TacticalActionCommit {
                        player: current_player(),
                    })
            },
            "End Tactical"
        }
    }
}

#[component]
fn SelectPlanetAttachment(
    planet: ReadSignal<Planet>,
    attachment: ReadSignal<Option<PlanetAttachment>>,
    previous_owner: ReadSignal<Option<PlayerId>>,
    select_attachment: EventHandler<PlanetAttachment>,
) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_attachment = use_signal(|| None);

    let planet_info = use_memo(move || planet().info());

    if planet_info().planet_traits.is_empty() {
        return rsx! { "Cannot explore" };
    }

    if let Some(prev) = previous_owner() {
        return rsx! {
            p { "Taken from {prev}" }
        };
    }

    if let Some(a) = attachment() {
        return rsx! {
            p { "{a.info().name}" }
        };
    }

    let available_attachments = use_memo(move || {
        let mut a = gc
            .game_options()
            .planet_attachments
            .iter()
            .filter(|(_, info)| {
                if let Some(t) = info.planet_trait.as_ref() {
                    return planet_info().planet_traits.contains(t);
                }

                planet_info().planet_traits.is_empty()
            })
            .filter(|(a, _)| a.is_real())
            .map(|(a, _)| a.clone())
            .collect::<Vec<_>>();

        a.sort();

        a
    });

    rsx! {
        PlanetAttachmentDropdown {
            value: selected_attachment,
            options: available_attachments(),
            on_select: move |att| selected_attachment.set(att),
        }
        Button {
            class: "margin-top",
            disabled: available_attachments().is_empty() || selected_attachment().is_none(),
            onclick: move |_| select_attachment(
                selected_attachment().expect("Attachment to be selected"),
            ),
            "Attach"
        }
    }
}
