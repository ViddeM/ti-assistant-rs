use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::{planet::Planet, tech::Technology},
};

use crate::{
    components::{button::Button, dropdown::PlanetDropdown},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::{destroyed_planets::planets_on_board, select_tech::SelectTechView},
};

#[component]
pub fn PostSetupPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let pending = use_memo(move || {
        let game_state = gc.game_state();
        (game_state.table_order.iter())
            .filter(|p| game_state.rapid_mobilization.contains(*p))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "column screen-container",
            div { class: "card column full-width",
                h2 { "Rapid Mobilization" }
                ol {
                    li {
                        "Place 1 infantry on each planet adjacent to your home system (or the Creuss Gate / The Sorrow). Gain control of and ready those planets, but don't explore them."
                    }
                    li {
                        "Place 1 space dock on a planet you control, and your flagship and 3 fighters in its system."
                    }
                    li { "Gain your breakthrough (already done)." }
                    li { "Research 1 technology, or 2 for the Nomad." }
                }
            }
            for player_id in pending() {
                RapidMobilizationPlayer { key: "{player_id}", player_id }
            }
            Button {
                class: "margin-top",
                disabled: !pending().is_empty(),
                onclick: move |_| event.send_event(Event::CompletePostSetupPhase),
                "Start Strategy Phase"
            }
        }
    }
}

#[component]
fn RapidMobilizationPlayer(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let mut planets: Signal<Vec<Planet>> = use_signal(Vec::new);
    let mut technologies: Signal<Vec<Technology>> = use_signal(Vec::new);

    let faction = use_memo({
        let player_id = player_id.clone();
        move || (gc.game_state().players.get(&player_id)).map(|p| p.faction)
    });
    let research_count = use_memo(move || {
        faction()
            .map(|f| f.rapid_mobilization_research_count())
            .unwrap_or(0)
    });
    let available_planets = use_memo(move || {
        let game_state = gc.game_state();
        planets_on_board(&gc)
            .into_iter()
            .filter(|p| game_state.is_rapid_mobilization_planet(p))
            .filter(|p| !planets.read().contains(p))
            .collect::<Vec<_>>()
    });

    if !view.display_for(&player_id) {
        return rsx! {
            div { class: "card column margin-top full-width",
                h2 { "{player_id}" }
                p { "Waiting for {player_id}." }
            }
        };
    }

    rsx! {
        div { class: "card column margin-top full-width",
            h2 { "{player_id}" }
            h3 { "Planets" }
            for planet in planets() {
                p { key: "{planet}", class: "margin-top", "{planet.info().name}" }
            }
            div { class: "margin-top full-width",
                PlanetDropdown {
                    key: "{planets().len()}",
                    class: "full-width",
                    value: None,
                    options: available_planets(),
                    on_select: move |p| {
                        if let Some(p) = p {
                            planets.write().push(p);
                        }
                    },
                }
            }
            Button {
                class: "margin-top",
                disabled: planets().is_empty(),
                onclick: move |_| planets.set(Vec::new()),
                "Clear"
            }
            h3 { class: "margin-top", "Technologies" }
            if faction() == Some(Faction::NekroVirus) {
                p { "The Nekro Virus can't research technology: gain 3 command tokens instead." }
            }
            for tech in technologies() {
                p { key: "{tech}", class: "margin-top", "{tech.info().name}" }
            }
            if technologies().len() < research_count() {
                SelectTechView {
                    key: "{technologies().len()}",
                    player_id: player_id.clone(),
                    filtered_techs: technologies(),
                    on_select: move |tech| technologies.write().push(tech),
                }
            }
            if research_count() > 0 {
                Button {
                    class: "margin-top",
                    disabled: technologies().is_empty(),
                    onclick: move |_| technologies.set(Vec::new()),
                    "Clear"
                }
            }
            Button {
                class: "margin-top",
                disabled: technologies().len() != research_count(),
                onclick: move |_| {
                    event
                        .send_event(Event::ResolveRapidMobilization {
                            player: player_id.clone(),
                            planets: planets().into_iter().collect(),
                            technologies: technologies().into_iter().collect(),
                        })
                },
                "Done"
            }
        }
    }
}
