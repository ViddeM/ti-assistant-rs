use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::faction::Faction,
    components::{
        galactic_event::GalacticEvent,
        planet::{OrbitalBodyType, Planet, PlanetTrait},
        planet_attachment::PlanetAttachment,
        relic::Relic,
        tech::Technology,
    },
    state::game_state::{ActionPhaseProgress, PlanetLocation, TakenFrom},
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlanetAttachmentDropdown, PlanetDropdown, RelicDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::{
        book_of_latvinia_research::BookOfLatviniaTechPicker,
        phase_views::action_phase::unowned_relics, select_tech::SelectTechView,
    },
};

const TACTICAL_ACTION_VIEW: Asset =
    asset!("/assets/styling/views/phase_views/tactical_action.scss");

#[component]
pub fn TacticalActionView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let finished = use_memo(move || {
        gc.game_state()
            .action_progress
            .as_ref()
            .and_then(ActionPhaseProgress::tactical)
            .is_none_or(|tactical| tactical.is_finished())
    });

    rsx! {
        div { class: "card tactical-container",
            h2 { "Tactical" }
            TacticalAction {}
            if view.is_active() {
                Button {
                    disabled: !finished(),
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
    }
}

#[component]
pub fn TacticalAction() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let progress = use_memo(move || {
        gc.game_state()
            .action_progress
            .as_ref()
            .and_then(ActionPhaseProgress::tactical)
            .cloned()
            .unwrap_or_default()
    });

    let taken_planets = use_memo(move || {
        progress()
            .taken_planets
            .iter()
            .map(|(planet, taken_from)| (planet.clone(), taken_from.clone()))
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

    let activated_system = use_memo(move || progress().activated_system);
    let available_planets_in_system = use_memo(move || {
        let Some(system) = activated_system() else {
            return Vec::new();
        };
        gc.game_state()
            .planets_in_system(&system)
            .into_iter()
            .filter(|p| {
                !current_player_planets().contains(p)
                    && taken_planets().iter().find(|(ps, _)| ps.eq(p)).is_none()
            })
            .collect::<Vec<_>>()
    });

    let all_planets_not_owned = use_memo(move || {
        let mut planets = gc
            .game_options()
            .planet_infos
            .iter()
            .filter(|(p, _)| {
                // Check that the planet has been spawned (if not part of a pre-defined system).
                gc.game_state().planet_location(p) != PlanetLocation::NotOnBoard
            })
            .filter(|(p, _)| !current_player_planets().contains(p))
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>();
        planets.sort();
        planets
    });

    let mut selected_planet = use_signal(|| None);

    let attachments = use_memo(move || progress().planet_attachments);

    rsx! {
        document::Stylesheet { href: TACTICAL_ACTION_VIEW }

        if !view.is_active() {
            p { "Waiting for {current_player()} to finish their tactical action" }
        } else if !taken_planets().is_empty() {
            for (planet, taken_from) in taken_planets().iter() {
                fieldset { key: "{planet}",
                    legend { "{planet.info().name}" }

                    div { class: "column",
                        SelectPlanetAttachment {
                            planet: planet.clone(),
                            attachment: attachments().get(planet).cloned(),
                            taken_from: taken_from.clone(),
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
                        DangerousWildsResearch {
                            planet: planet.clone(),
                            taken_from: taken_from.clone(),
                            tech: progress().researched_techs.get(planet).cloned(),
                        }
                        PlanetRelic {
                            planet: planet.clone(),
                            owes_relic: progress().owes_relic(planet),
                            gained_relic: progress().gained_relics.get(planet).cloned(),
                            owes_book_of_latvinia_research: progress().owes_book_of_latvinia_research(),
                        }
                        MonumentPrompt { planet: planet.clone() }
                        MinorFactionAlliance { planet: planet.clone() }
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
            fieldset {
                legend { "Take planet" }
                div { class: "column",
                    PlanetDropdown {
                        options: all_planets_not_owned(),
                        value: selected_planet(),
                        on_select: move |planet| selected_planet.set(planet),
                    }
                    Button {
                        class: "margin-top",
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
        }
    }
}

#[component]
fn MonumentPrompt(planet: ReadSignal<Planet>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let commodities = use_memo(move || gc.game_state().monument_commodities(&planet()));

    rsx! {
        if let Some(commodities) = commodities() {
            p {
                if commodities == 1 {
                    "Monument with 1 commodity"
                } else {
                    "Monument with {commodities} commodities"
                }
            }
            Button {
                onclick: move |_| {
                    event
                        .send_event(Event::DestroyMonument {
                            planet: planet(),
                        })
                },
                "Destroy monument"
            }
        }
    }
}

#[component]
fn MinorFactionAlliance(planet: ReadSignal<Planet>) -> Element {
    let gc = use_context::<GameContext>();

    let alliance = use_memo(move || {
        let game_state = gc.game_state();
        let current_player = game_state.current_player.as_ref()?;
        if !game_state.game_settings.expansions.prophecy_of_kings {
            return None;
        }
        game_state.minor_faction_alliance(current_player, &planet())
    });

    rsx! {
        if let Some(faction) = alliance() {
            p { "Take the alliance card of {faction.name()}: you control every planet in its system." }
        }
    }
}

#[component]
fn DangerousWildsResearch(
    planet: ReadSignal<Planet>,
    taken_from: ReadSignal<TakenFrom>,
    tech: ReadSignal<Option<Technology>>,
) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let current_player_faction = use_memo(move || {
        gc.game_state()
            .players
            .get(&current_player())
            .map(|p| p.faction)
            .expect("Current player to exist")
    });

    let applies = use_memo(move || {
        gc.game_state()
            .galactic_events
            .in_play
            .contains(&GalacticEvent::DangerousWilds)
            && taken_from() == TakenFrom::NeverOwned
            && gc
                .game_state()
                .planet_traits(&planet())
                .contains(&PlanetTrait::Hazardous)
    });

    let resources = use_memo(move || planet().info().resources);

    if !applies() {
        return rsx! {};
    }

    rsx! {
        div { class: "margin-top",
            if let Some(tech) = tech() {
                p { "Dangerous Wilds: researched {tech.info().name}" }
            } else if current_player_faction() == Faction::NekroVirus {
                p { "Nekro virus cannot research techs" }
            } else {
                p { class: "margin-top info-text",
                    "Dangerous Wilds: you may research 1 technology, ignoring up to {resources()} prerequisites"
                }
                SelectTechView {
                    player_id: current_player(),
                    on_select: move |tech| {
                        event
                            .send_event(Event::TacticalActionResearchTechnology {
                                player: current_player(),
                                planet: planet(),
                                tech,
                            })
                    },
                }
            }
        }
    }
}

#[component]
fn PlanetRelic(
    planet: ReadSignal<Planet>,
    owes_relic: ReadSignal<bool>,
    gained_relic: ReadSignal<Option<Option<Relic>>>,
    owes_book_of_latvinia_research: ReadSignal<bool>,
) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_relic: Signal<Option<Relic>> = use_signal(|| None);

    let available_relics = use_memo(move || unowned_relics(&gc));

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let gain = use_callback(move |relic: Option<Relic>| {
        event.send_event(Event::TacticalActionGainRelic {
            player: current_player(),
            planet: planet(),
            relic,
        })
    });

    rsx! {
        match gained_relic() {
            Some(Some(relic)) => rsx! {
                p { class: "margin-top", "Gained {relic.info().name}" }
                if relic == Relic::BookOfLatvinia && owes_book_of_latvinia_research() {
                    BookOfLatviniaTechPicker { player_id: current_player() }
                }
            },
            Some(None) => rsx! {
                p { class: "margin-top", "No relic to gain" }
            },
            None if owes_relic() => rsx! {
                div { class: "margin-top column",
                    p { class: "info-text", "The first player to gain {planet().info().name} gains 1 relic" }
                    RelicDropdown {
                        value: selected_relic,
                        options: available_relics(),
                        on_select: move |r| selected_relic.set(r),
                    }
                    div { class: "center-row margin-top",
                        Button {
                            class: "margin-right",
                            disabled: selected_relic().is_none(),
                            onclick: move |_| gain(selected_relic()),
                            "Gain relic"
                        }
                        Button { onclick: move |_| gain(None), "No relic" }
                    }
                }
            },
            None => rsx! {},
        }
    }
}

#[component]
pub fn SelectPlanetAttachment(
    planet: ReadSignal<Planet>,
    attachment: ReadSignal<Option<PlanetAttachment>>,
    taken_from: ReadSignal<TakenFrom>,
    select_attachment: EventHandler<PlanetAttachment>,
) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_attachment = use_signal(|| None);

    let planet_traits = use_memo(move || gc.game_state().planet_traits(&planet()));

    if planet_traits().is_empty() || planet().info().body_type == OrbitalBodyType::SpaceStation {
        return rsx! { "Cannot explore" };
    }

    match taken_from() {
        TakenFrom::Player(prev) => {
            return rsx! {
                p { "Taken from {prev}" }
            };
        }
        TakenFrom::NeverOwned | TakenFrom::Unowned => {}
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
                    return planet_traits().contains(t);
                }

                planet_traits().is_empty()
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
