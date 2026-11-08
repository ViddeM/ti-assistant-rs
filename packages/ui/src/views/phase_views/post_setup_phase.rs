use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{
        planet::Planet,
        tech::{TechOrigin, Technology},
    },
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlanetDropdown, TechDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::destroyed_planets::planets_on_board,
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
            }
            div { class: "card column",
                h2 { "Post-Setup" }
                p { "The Fracture is in play and every player has their breakthrough. Each player:" }
                ol { class: "post-setup-steps",
                    li {
                        "Place 1 infantry on each planet adjacent to your home system (or the Creuss Gate / The Sorrow). Gain control of and ready those planets, but don't explore them."
                    }
                    li {
                        "Place 1 space dock on a planet you control, and your flagship and 3 fighters in its system."
                    }
                    li { "Research 1 technology, or 2 for the Nomad." }
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
    let researchable_techs = use_memo(move || {
        let game_state = gc.game_state();
        let Some(p) = game_state.players.get(&player()) else {
            return Vec::new();
        };
        let mut techs = (gc.game_options().technologies.iter())
            .filter(|t| !p.has_tech(t))
            .filter(|t| match t.info().origin {
                TechOrigin::Base => true,
                TechOrigin::Faction(faction) => faction == p.faction,
            })
            .cloned()
            .collect::<Vec<_>>();
        techs.sort();
        techs
    });
    let tech_options = move |slot: usize| {
        let picked = technologies();
        researchable_techs()
            .into_iter()
            .filter(|t| {
                (picked.iter().enumerate()).all(|(i, pick)| i == slot || pick.as_ref() != Some(t))
            })
            .collect::<Vec<_>>()
    };
    let all_picked = use_memo(move || technologies().iter().all(Option::is_some));

    rsx! {
        fieldset { class: "player-color-border-{color} rapid-mobilization-fieldset",
            legend { class: "player-color-border-{color} rapid-mobilization-legend", "{player}" }
            if resolved() {
                p { "Resolved" }
            } else {
                p { class: "rapid-mobilization-label", "Planets" }
                for planet in planets() {
                    div { key: "{planet}", class: "row space-between",
                        p { class: "margin-right", "{planet.info().name}" }
                        Button {
                            onclick: {
                                let planet = planet.clone();
                                move |_| planets.write().retain(|p| p != &planet)
                            },
                            "Remove"
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
                        TechDropdown {
                            key: "{slot}",
                            value: technologies().get(slot).cloned().flatten(),
                            options: tech_options(slot),
                            on_select: move |t| technologies.write()[slot] = t,
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
