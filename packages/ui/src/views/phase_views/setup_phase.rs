use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{expansions::Expansion, faction::Faction, player_id::PlayerId},
    components::objectives::{Objective, ObjectiveKind},
    state::player::Player,
};

use crate::{
    components::{button::Button, dropdown::ObjectiveDropdown, faction_icon::FactionIcon},
    data::{event_context::EventContext, game_context::GameContext},
    views::phase_views::faction_setup_views::{
        argent_flight_setup::ArgentFlightSetup, council_keleres_setup::CouncilKeleresSetup,
        crimson_rebellion_setup::CrimsonRebellionSetup, winnu_setup::WinnuSetup,
    },
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
            .filter(|o| o.info().kind == ObjectiveKind::StageI)
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
