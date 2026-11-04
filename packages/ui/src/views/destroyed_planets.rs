use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, components::planet::Planet, state::game_state::PlanetLocation,
};

use crate::{
    components::{button::Button, dropdown::PlanetDropdown},
    data::{event_context::EventContext, game_context::GameContext},
};

/// The planets in the game that are on the board, sorted.
pub fn planets_on_board(gc: &GameContext) -> Vec<Planet> {
    let game_state = gc.game_state();
    let mut planets = gc
        .game_options()
        .planet_infos
        .keys()
        .filter(|p| game_state.planet_location(p) != PlanetLocation::NotOnBoard)
        .cloned()
        .collect::<Vec<_>>();
    planets.sort();
    planets
}

/// The destroyed planets, which can be restored, and a planet on the board to destroy.
#[component]
pub fn DestroyedPlanets() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_planet = use_signal(|| None);

    let destroyed_planets = use_memo(move || gc.game_state().map_data.destroyed_planets.clone());
    let planets_on_board = use_memo(move || planets_on_board(&gc));

    rsx! {
        div { class: "card column margin-top",
            h3 { "Destroyed Planets" }
            if destroyed_planets().is_empty() {
                p { "No planets have been destroyed." }
            }
            for planet in destroyed_planets() {
                div {
                    key: "{planet}",
                    class: "row full-width space-between margin-top",
                    p { class: "margin-right", "{planet.info().name}" }
                    Button {
                        onclick: {
                            let planet = planet.clone();
                            move |_| {
                                event
                                    .send_event(Event::SetPlanetDestroyed {
                                        planet: planet.clone(),
                                        destroyed: false,
                                    })
                            }
                        },
                        "Restore"
                    }
                }
            }
            div { class: "margin-top full-width",
                PlanetDropdown {
                    class: "full-width",
                    value: selected_planet,
                    on_select: move |p| selected_planet.set(p),
                    options: planets_on_board(),
                }
            }
            Button {
                class: "margin-top",
                disabled: selected_planet().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::SetPlanetDestroyed {
                            planet: selected_planet().expect("Planet to be selected"),
                            destroyed: true,
                        });
                    selected_planet.set(None);
                },
                "Destroy"
            }
        }
    }
}
