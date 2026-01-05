use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{expansions::Expansion, faction::Faction, player_id::PlayerId},
    components::{
        objectives::{Objective, ObjectiveKind},
        tech::{TechOrigin, Technology},
    },
    state::player::Player,
};

use crate::{
    components::{
        button::Button,
        dropdown::{FactionDropdown, ObjectiveDropdown, TechDropdown},
        faction_icon::FactionIcon,
    },
    data::{event_context::EventContext, game_context::GameContext},
};

const SETUP_PHASE_SCSS: Asset = asset!("/assets/styling/views/phase_views/setup_phase.scss");

#[component]
pub fn SetupPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let speaker_chosen = use_memo(move || gc.game_state().speaker.is_some());
    let revealed_objectives = use_memo(move || gc.game_state().score.revealed_objectives.len());
    let unfinished_players = use_memo(move || {
        gc.game_state()
            .players
            .keys()
            .filter(|p| {
                !gc.game_state()
                    .player_initialization_finished(p)
                    .expect("Unexpected error")
            })
            .count()
    });

    /* TODO: Should also include faction specific setup not being complete */
    let setup_complete = use_memo(move || speaker_chosen() || revealed_objectives() > 0);

    rsx! {
        document::Stylesheet { href: SETUP_PHASE_SCSS }

        div { class: "card setup-container",
            h2 { "Setup" }

            PlayersSetup {}

            ObjectivesSetup {}

            Button {
                disabled: !setup_complete() || revealed_objectives == 0 || unfinished_players() > 0,
                onclick: move |_| event.send_event(Event::StartGame),
                "Start Game"
            }
        }
    }
}

#[component]
fn PlayersSetup() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let players = use_memo(move || {
        let game_state = Arc::clone(&gc.game_state());

        let mut players = game_state.players.clone().into_iter().collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| {
            let a_pos = gc
                .game_state()
                .table_order
                .iter()
                .position(|p| p.eq(a))
                .expect("Player to be at the table!");
            let b_pos = gc
                .game_state()
                .table_order
                .iter()
                .position(|p| p.eq(b))
                .expect("Player to be at the table!");
            a_pos.cmp(&b_pos)
        });
        players
    });

    rsx! {
        div { class: "player-specific-setup-container",
            h3 { "Player specific setup" }
            div { class: "players-setup-container",
                {
                    players()
                        .into_iter()
                        .map(|(name, player)| {
                            let player = Arc::new(player);
                            let n = Arc::clone(&name);
                            rsx! {
                                fieldset {
                                    key: "{n}",
                                    class: format!("player-color-border-{} setup-player-fieldset", player.color.name())
                                        .as_str(),
                                    legend { class: format!("player-color-border-{} setup-player-legend", player.color.name()).as_str(),
                                        "{&name}"
                                    }











                                    div { class: "setup-row",
                                        if gc.game_state().is_speaker(&name) {
                                            p { "Speaker" }
                                        } else {
                                            Button {
                                                disabled: gc.game_state().is_speaker(&name),
                                                onclick: move |_| {
                                                    event
                                                        .send_event(Event::SetupSpeaker {
                                                            player: n.clone(),
                                                        })
                                                },
                                                "Set Speaker"
                                            }
                                        }
                                        FactionIcon { faction: player.faction }
                                    }

                                    {player.faction.name()}
                                    FactionSpecificSetup { player_id: Arc::clone(&name), player: Arc::clone(&player) }
                                }
                            }
                        })
                }
            }
        }
    }
}

#[component]
fn FactionSpecificSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    match player.faction {
        Faction::Winnu => {
            rsx! {
                WinnuSetup { player_id, player }
            }
        }
        Faction::ArgentFlight => {
            rsx! {
                ArgentFlightSetup { player_id, player }
            }
        }
        Faction::CouncilKeleres => {
            rsx! {
                CouncilKeleresSetup { player_id, player }
            }
        }
        f if f.expansion() == Expansion::ThundersEdge => {
            rsx! {
                div { "Thunder's edge factions not yet implemented" }
            }
        }
        _ => rsx! {
            div { "No faction specific setup" }
        },
    }
}

#[component]
fn WinnuSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_tech = use_signal::<Option<Technology>>(|| None);

    let available_techs = use_memo(move || {
        let game_options = gc.game_options();
        let mut techs = game_options
            .technologies
            .iter()
            .filter(|(_, info)| info.origin == TechOrigin::Base && info.requirements.is_empty())
            .map(|(tech, _)| tech.clone())
            .collect::<Vec<_>>();
        techs.sort();
        techs
    });

    let tech = player.technologies.iter().next().map(|t| t.info().name);

    rsx! {
        div {
            {
                if let Some(tech) = tech {
                    rsx! {
                        p { "{tech}" }
                    }
                } else {
                    rsx! {
                        div { class: "setup-column",
                            TechDropdown {
                                value: selected_tech,
                                options: available_techs(),
                                on_select: move |t| selected_tech.set(t),
                            }
                            Button {
                                disabled: selected_tech().is_none(),
                                onclick: move |_| {
                                    event
                                        .send_event(Event::SetupPlayerTechs {
                                            player: Arc::clone(&player_id),
                                            technologies: vec![selected_tech().expect("Tech to be selected")],
                                        })
                                },
                                "Select"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ArgentFlightSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let event = use_context::<EventContext>();

    let mut first_tech = use_signal(|| None);
    let mut second_tech = use_signal(|| None);

    let possible_techs = use_memo(|| {
        let mut techs = vec![
            Technology::NeuralMotivator,
            Technology::SarweenTools,
            Technology::PlasmaScoring,
        ];

        techs.sort();

        techs
    });

    let mut taken_techs = player.technologies.iter().collect::<Vec<_>>();
    taken_techs.sort();

    rsx! {
        div { class: "setup-column",
            {
                if taken_techs.len() > 0 {
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
                            options: possible_techs(),
                            on_select: move |t| first_tech.set(t),
                        }
                        TechDropdown {
                            value: second_tech,
                            options: possible_techs(),
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

#[component]
fn CouncilKeleresSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let event = use_context::<EventContext>();
    let gc = use_context::<GameContext>();

    let mut first_tech: Signal<Option<Technology>> = use_signal(|| None);
    let mut second_tech: Signal<Option<Technology>> = use_signal(|| None);
    let mut selected_faction: Signal<Option<Faction>> = use_signal(|| None);

    let possible_techs = use_memo(move || {
        let mut techs = gc
            .game_state()
            .players
            .values()
            .map(|p| p.technologies.iter())
            .flatten()
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
                if taken_techs.len() > 0 {
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
                                .iter()
                                .map(|(p, _)| {
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

#[component]
fn ObjectivesSetup() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut first_objective: Signal<Option<Objective>> = use_signal(|| None);
    let mut second_objective: Signal<Option<Objective>> = use_signal(|| None);

    let available_objectives = use_memo(move || {
        let mut objectives = gc
            .game_options()
            .objectives
            .iter()
            .filter(|(_, i)| i.kind == ObjectiveKind::StageI)
            .map(|(o, _)| o)
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    let first_available_objectives = use_memo(move || {
        available_objectives()
            .iter()
            .filter(|o| !second_objective().as_ref().eq(&Some(o)))
            .cloned()
            .collect::<Vec<_>>()
    });

    let second_available_objectives = use_memo(move || {
        available_objectives()
            .iter()
            .filter(|o| !first_objective().as_ref().eq(&Some(o)))
            .cloned()
            .collect::<Vec<_>>()
    });

    let revealed_objectives = use_memo(move || {
        let mut objectives = gc
            .game_state()
            .score
            .revealed_objectives
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    rsx! {
        div { class: "reveal-objectives-container",
            h3 { "Select initial objectives" }
            {
                if revealed_objectives.is_empty() {
                    rsx! {
                        ObjectiveDropdown {
                            value: first_objective,
                            options: first_available_objectives(),
                            on_select: move |obj| first_objective.set(obj),
                        }
                        ObjectiveDropdown {
                            value: second_objective,
                            options: second_available_objectives(),
                            on_select: move |obj| second_objective.set(obj),
                        }
                        Button {
                            disabled: first_objective().is_none() || second_objective().is_none(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::RevealInitialObjectives {
                                        first_objective: first_objective()
                                            .expect("First objective to be selected"),
                                        second_objective: second_objective()
                                            .expect("Second objective to be selected"),
                                    })
                            },
                            "Select Objectives"
                        }
                    }
                } else {
                    rsx! {
                        {
                            revealed_objectives
                                .iter()
                                .map(|obj| {
                                    rsx! {
                                        p { key: "{obj}", "{obj.info().name}" }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
