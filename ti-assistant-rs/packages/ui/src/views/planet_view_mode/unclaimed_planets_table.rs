use dioxus::prelude::*;
use ti_helper_game_data::actions::event::Event;

use crate::{
    components::faction_button::FactionButton,
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn UnclaimedPlanetsTable() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| a.cmp(b));
        players
    });
    let claimed_planets = use_memo(move || {
        gc.game_state()
            .players
            .values()
            .flat_map(|p| p.planets.keys())
            .cloned()
            .collect::<Vec<_>>()
    });

    let mut planets_filter = use_signal(|| String::new());
    let planets_filter_lc = use_memo(move || planets_filter().to_lowercase());

    let unclaimed_planets = use_memo(move || {
        let mut planets = gc
            .game_options()
            .planet_infos
            .iter()
            .filter(|(p, _)| !claimed_planets().contains(p))
            .filter(|(_, i)| i.name.to_lowercase().contains(&planets_filter_lc()))
            .map(|(p, i)| (p.clone(), i.clone()))
            .collect::<Vec<_>>();
        planets.sort_by(|(_, a), (_, b)| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        planets
    });

    rsx! {
        div { class: "unclaimed-planets-container",
            div { class: "card screen-container",
                table {
                    thead {
                        tr {
                            th { colspan: players().len(),
                                h2 { "Unclaimed Planets" }
                            }
                        }
                        tr {
                            th { colspan: players().len(),
                                input {
                                    class: "planet-filter",
                                    placeholder: "Filter planets",
                                    value: planets_filter,
                                    oninput: move |e: FormEvent| {
                                        planets_filter.set(e.value());
                                    },
                                }
                            }
                        }
                    }
                    tbody {
                        for (planet , i) in unclaimed_planets().iter() {
                            Fragment { key: "{planet}",
                                tr {
                                    th { colspan: players().len(),
                                        h4 { "{i.name}" }
                                    }
                                }
                                tr {
                                    for (id , player) in players().iter() {
                                        td {
                                            key: "{id}",
                                            class: "align-center",
                                            FactionButton {
                                                faction: player.faction,
                                                selected: false,
                                                onclick: {
                                                    let player_id = id.clone();
                                                    let planet = planet.clone();
                                                    move |_| {
                                                        event
                                                            .send_event(Event::SetPlanetOwner {
                                                                player: Some(player_id.clone()),
                                                                planet: planet.clone(),
                                                            })
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
