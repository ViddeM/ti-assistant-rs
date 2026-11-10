use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::tech::Technology,
    state::player::Player,
};

use crate::{
    components::{button::Button, dropdown::TechDropdown},
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn ArgentFlightSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let event = use_context::<EventContext>();
    let gc = use_context::<GameContext>();

    let mut first_tech = use_signal(|| None::<Technology>);
    let mut second_tech = use_signal(|| None::<Technology>);

    let possible_techs = use_memo(move || {
        let mut techs = Faction::ArgentFlight
            .get_starting_tech_options(&gc.game_state())
            .expect("Argent Flight to have starting tech options");
        techs.sort();
        techs
    });
    let first_tech_possible = use_memo(move || {
        possible_techs()
            .iter()
            .filter(|t| !second_tech().as_ref().eq(&Some(t)))
            .cloned()
            .collect::<Vec<_>>()
    });
    let second_tech_possible = use_memo(move || {
        possible_techs()
            .iter()
            .filter(|t| !first_tech().as_ref().eq(&Some(t)))
            .cloned()
            .collect::<Vec<_>>()
    });

    let mut taken_techs = player.technologies.iter().collect::<Vec<_>>();
    taken_techs.sort();

    rsx! {
        div { class: "setup-column",
            {
                if !taken_techs.is_empty() {
                    rsx! {
                        {
                            taken_techs
                                .iter()
                                .map(|t| {
                                    rsx! {
                                        p { key: "{t}", "{t.info().name}" }
                                    }
                                })
                        }
                    }
                } else {
                    rsx! {
                        TechDropdown {
                            value: first_tech,
                            options: first_tech_possible(),
                            on_select: move |t| first_tech.set(t),
                        }
                        TechDropdown {
                            value: second_tech,
                            options: second_tech_possible(),
                            on_select: move |t| second_tech.set(t),
                        }
                        Button {
                            disabled: first_tech().is_none() || second_tech().is_none(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::SetupPlayerTechs {
                                        player: Arc::clone(&player_id),
                                        technologies: vec![first_tech().unwrap(), second_tech().unwrap()],
                                    });
                            },
                            "Choose"
                        }
                    }
                }
            }
        }
    }
}
