use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_regular_icons::FaTrashCan};
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::planet::Planet,
    state::coexistence::Coexistence,
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlanetDropdown, PlayerDropdown},
    },
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn CoexistenceCard() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut player: Signal<Option<PlayerId>> = use_signal(|| None);
    let mut planet: Signal<Option<Planet>> = use_signal(|| None);

    let player_options = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    let planet_options = use_memo(move || {
        let game_state = gc.game_state();
        let Some(coexisting) = player() else {
            return vec![];
        };
        let mut planets = (game_state.players.iter())
            .filter(|(id, _)| **id != coexisting)
            .flat_map(|(_, p)| p.planets.keys().cloned())
            .filter(|p| {
                !game_state.coexistences.contains(&Coexistence {
                    planet: p.clone(),
                    player: coexisting.clone(),
                })
            })
            .collect::<Vec<_>>();
        planets.sort();
        planets.dedup();
        planets
    });

    let coexistences = use_memo(move || {
        gc.game_state()
            .coexistences
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "card coexistence-card",
            h2 { "Coexistence" }
            if coexistences().is_empty() {
                p { "No units are coexisting." }
            } else {
                table { class: "coexistence-table",
                    tbody {
                        for coexistence in coexistences().iter() {
                            tr {
                                td { class: "align-left", "{coexistence.player}" }
                                td { class: "align-left", "{coexistence.planet.info().name}" }
                                td { class: "align-right",
                                    Button {
                                        class: "unclaim-planet-button",
                                        onclick: {
                                            let coexistence = coexistence.clone();
                                            move |_| {
                                                event
                                                    .send_event(Event::CoexistenceEnd {
                                                        player: coexistence.player.clone(),
                                                        planet: coexistence.planet.clone(),
                                                    })
                                            }
                                        },
                                        Icon { class: "inline-icon", icon: FaTrashCan }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "coexistence-add",
                PlayerDropdown {
                    value: player,
                    options: player_options(),
                    on_select: move |p| {
                        player.set(p);
                        planet.set(None);
                    },
                }
                PlanetDropdown {
                    value: planet,
                    options: planet_options(),
                    disabled: player().is_none(),
                    on_select: move |p| planet.set(p),
                }
                Button {
                    disabled: player().is_none() || planet().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::CoexistenceBegin {
                                player: player().expect("Player to be selected"),
                                planet: planet().expect("Planet to be selected"),
                            });
                        planet.set(None);
                    },
                    "Add coexistence"
                }
            }
        }
    }
}
