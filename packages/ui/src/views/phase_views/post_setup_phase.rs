use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaTrash};
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{planet::Planet, tech::Technology},
};

use crate::{
    components::{
        button::Button,
        dropdown::PlanetDropdown,
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::{destroyed_planets::planets_on_board, select_tech::SelectTechView},
};

const POST_SETUP_PHASE_SCSS: Asset =
    asset!("/assets/styling/views/phase_views/post_setup_phase.scss");

#[component]
pub fn PostSetupPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let players = use_memo(move || {
        (gc.game_state().table_order.iter())
            .filter(|p| view.display_for(p))
            .cloned()
            .collect::<Vec<_>>()
    });
    let all_resolved = use_memo(move || gc.game_state().rapid_mobilization.is_empty());

    rsx! {
        document::Stylesheet { href: POST_SETUP_PHASE_SCSS }

        div { class: "post-setup-phase-container",
            div { class: "card column",
                h2 { "Rapid Mobilization" }
                for player in players() {
                    RapidMobilizationPlayer { key: "{player}", player }
                }
                Button {
                    class: "margin-top",
                    disabled: !all_resolved(),
                    onclick: move |_| event.send_event(Event::CompletePostSetupPhase),
                    "Start Strategy Phase"
                }
            }
        }
    }
}

#[component]
fn RapidMobilizationPlayer(player: ReadSignal<PlayerId>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let color = use_memo(move || {
        gc.game_state()
            .players
            .get(&player())
            .map(|p| p.color.name())
            .unwrap_or_default()
    });
    let resolved = use_memo(move || !gc.game_state().rapid_mobilization.contains(&player()));
    let resolution = use_memo(move || {
        gc.game_state()
            .rapid_mobilization_resolved
            .get(&player())
            .cloned()
    });
    let research_count = use_memo(move || {
        gc.game_state()
            .players
            .get(&player())
            .map(|p| p.faction.rapid_mobilization_research_count())
            .unwrap_or(0)
    });

    let mut planets: Signal<Vec<Planet>> = use_signal(Vec::new);
    let mut technologies: Signal<Vec<Option<Technology>>> =
        use_signal(move || vec![None; research_count()]);

    let available_planets = use_memo(move || {
        let game_state = gc.game_state();
        planets_on_board(&gc)
            .into_iter()
            .filter(|p| game_state.is_rapid_mobilization_planet(p))
            .filter(|p| !planets.read().contains(p))
            .collect::<Vec<_>>()
    });
    let mut open_slot: Signal<Option<usize>> = use_signal(|| None);
    let all_picked = use_memo(move || technologies().iter().all(Option::is_some));

    rsx! {
        fieldset { class: "player-color-border-{color} rapid-mobilization-fieldset",
            legend { class: "player-color-border-{color} rapid-mobilization-legend", "{player}" }
            if resolved() {
                if let Some(resolution) = resolution() {
                    p { class: "rapid-mobilization-label", "Planets" }
                    if resolution.planets.is_empty() {
                        p { "None" }
                    }
                    for planet in resolution.planets.iter() {
                        p { key: "{planet}", "{planet.info().name}" }
                    }
                    p { class: "rapid-mobilization-label", "Technologies" }
                    if resolution.technologies.is_empty() {
                        p { "None" }
                    }
                    for tech in resolution.technologies.iter() {
                        p { key: "{tech:?}", "{tech.info().name}" }
                    }
                } else {
                    p { "Resolved" }
                }
            } else {
                p { class: "rapid-mobilization-label", "Planets" }
                for planet in planets() {
                    div { key: "{planet}", class: "row space-between",
                        p { class: "margin-right", "{planet.info().name}" }
                        Button {
                            class: "rapid-mobilization-trash-button",
                            onclick: {
                                let planet = planet.clone();
                                move |_| planets.write().retain(|p| p != &planet)
                            },
                            Icon { class: "inline-icon", icon: FaTrash }
                        }
                    }
                }
                div {
                    PlanetDropdown {
                        key: "{planets().len()}",
                        value: None,
                        options: available_planets(),
                        on_select: move |p| {
                            if let Some(p) = p {
                                planets.write().push(p);
                            }
                        },
                    }
                }
                if research_count() == 0 {
                    p { class: "rapid-mobilization-label",
                        "Can't research technology: gain 3 command tokens."
                    }
                } else {
                    p { class: "rapid-mobilization-label",
                        if research_count() == 1 {
                            "Research 1 technology"
                        } else {
                            "Research {research_count()} technologies"
                        }
                    }
                    for slot in 0..research_count() {
                        if open_slot() == Some(slot) {
                            SelectTechView {
                                key: "{slot}",
                                player_id: player(),
                                filtered_techs: Some(
                                    technologies()
                                        .into_iter()
                                        .enumerate()
                                        .filter(|(i, _)| *i != slot)
                                        .filter_map(|(_, t)| t)
                                        .collect::<Vec<_>>(),
                                ),
                                on_select: move |tech| {
                                    technologies.write()[slot] = Some(tech);
                                    open_slot.set(None);
                                },
                            }
                            Button { onclick: move |_| open_slot.set(None), "Cancel" }
                        } else {
                            div { key: "{slot}", class: "row space-between",
                                p { class: "margin-right",
                                    if let Some(t) = technologies().get(slot).cloned().flatten() {
                                        "{t.info().name}"
                                    } else {
                                        "No technology selected"
                                    }
                                }
                                Button { onclick: move |_| open_slot.set(Some(slot)),
                                    if technologies().get(slot).is_some_and(Option::is_some) {
                                        "Change Tech"
                                    } else {
                                        "Select Tech"
                                    }
                                }
                            }
                        }
                    }
                }
                Button {
                    class: "rapid-mobilization-done",
                    disabled: !all_picked(),
                    onclick: move |_| {
                        event
                            .send_event(Event::ResolveRapidMobilization {
                                player: player(),
                                planets: planets().into_iter().collect(),
                                technologies: technologies().into_iter().flatten().collect(),
                            })
                    },
                    "Done"
                }
            }
        }
    }
}
