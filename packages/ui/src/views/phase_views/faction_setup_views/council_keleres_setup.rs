use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::tech::{TechOrigin, Technology},
    state::player::Player,
};

use crate::{
    components::{
        button::Button,
        dropdown::{FactionDropdown, TechDropdown},
    },
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn CouncilKeleresSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let event = use_context::<EventContext>();
    let gc = use_context::<GameContext>();

    let mut first_tech: Signal<Option<Technology>> = use_signal(|| None);
    let mut second_tech: Signal<Option<Technology>> = use_signal(|| None);
    let mut selected_faction: Signal<Option<Faction>> = use_signal(|| None);

    // TODO: This doesn't seem to work maybe?
    let possible_techs = use_memo(move || {
        let mut techs = gc
            .game_state()
            .players
            .values()
            .flat_map(|p| p.technologies.iter())
            .filter(|t| t.info().origin == TechOrigin::Base)
            .cloned()
            .collect::<Vec<_>>();
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

    let taken_factions = use_memo(move || {
        gc.game_state()
            .players
            .values()
            .map(|p| p.faction)
            .collect::<Vec<_>>()
    });
    let possible_factions = use_memo(move || {
        let mut fs = vec![
            Faction::MentakCoalition,
            Faction::XxchaKingdom,
            Faction::ArgentFlight,
        ]
        .into_iter()
        .filter(|f| !taken_factions().contains(f))
        .collect::<Vec<_>>();
        fs.sort();
        fs
    });

    let mut taken_techs = player.technologies.iter().collect::<Vec<_>>();
    taken_techs.sort();

    let t_p = Arc::clone(&player_id);
    let f_p = Arc::clone(&player_id);

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
                                        player: Arc::clone(&t_p),
                                        technologies: vec![
                                            first_tech().expect("Tech to be selected"),
                                            second_tech().expect("Tech to be selected"),
                                        ],
                                    })
                            },
                            "Select Technology"
                        }
                    }
                }
            }
            {
                if player.planets.is_empty() {
                    rsx! {
                        FactionDropdown {
                            value: selected_faction,
                            options: possible_factions(),
                            on_select: move |f| selected_faction.set(f),
                        }
                        Button {
                            disabled: selected_faction().is_none(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::SetupTheTribunii {
                                        player: Arc::clone(&f_p),
                                        faction: selected_faction().expect("Faction to be selected"),
                                    });
                            },
                            "Select Faction"
                        }
                    }
                } else {
                    rsx! {
                        {
                            player
                                .planets
                                .keys()
                                .map(|p| {
                                    rsx! {
                                        p { key: "{p}", "{p.info().name}" }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
