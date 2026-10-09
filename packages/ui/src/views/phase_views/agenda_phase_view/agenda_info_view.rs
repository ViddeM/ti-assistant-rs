use dioxus::prelude::*;
use ti_helper_game_data::{
    components::agenda::AgendaElect,
    state::agenda::{AgendaRecord, AgendaState, Vote, VoteState},
};

use crate::{
    components::info_button::InfoButton,
    data::{game_context::GameContext, info_context::Info},
};

#[component]
pub fn AgendaInfoView(state: ReadSignal<AgendaState>) -> Element {
    let gc = use_context::<GameContext>();

    let previous_votes_this_round = use_memo(move || {
        gc.game_state()
            .agenda_vote_history
            .iter()
            .filter(|vote| vote.round == gc.game_state().round)
            .cloned()
            .collect::<Vec<_>>()
    });

    let vote = use_memo(move || state().vote.clone());

    rsx! {
        div { class: "card",
            h2 { "Agenda Phase" }

            fieldset {
                legend {
                    h3 { "Previous agendas" }
                }

                for vote in previous_votes_this_round() {
                    PreviousVoteDisplay { record: vote }
                }
            }

            fieldset {
                legend {
                    h3 { "Current agenda" }
                }

                if let Some(vote) = vote() {
                    CurrentVoteDisplay { state: vote }
                }
            }
        }
    }
}

#[component]
fn PreviousVoteDisplay(record: ReadSignal<AgendaRecord>) -> Element {
    let agenda = use_memo(move || record().vote.agenda);
    let elect_display = use_memo(move || record().vote.elect.display());
    let outcome = use_memo(move || {
        {
            record().outcome.map(|out| match out {
                AgendaElect::ForOrAgainst(for_or_against) => for_or_against.to_string(),
                AgendaElect::Player(player) => player.to_string(),
                AgendaElect::StrategyCard(strategy_card) => strategy_card.to_string(),
                AgendaElect::Law(agenda) => agenda.to_string(),
                AgendaElect::SecretObjective(secret_objective) => secret_objective.to_string(),
                AgendaElect::Planet(planet) => planet.to_string(),
                AgendaElect::PlanetWithTrait(planet) => planet.to_string(),
                AgendaElect::CulturalPlanet(planet) => planet.to_string(),
                AgendaElect::HazardousPlanet(planet) => planet.to_string(),
                AgendaElect::IndustrialPlanet(planet) => planet.to_string(),
            })
        }
        .unwrap_or("Discarded".to_string())
    });

    rsx! {
        div { key: "{agenda()}",
            div { class: "agenda-info-row",
                h6 { "{agenda().info().name}" }
                InfoButton { info: Info::Agenda(agenda()) }
            }
            p { "{elect_display()}: {outcome()}" }
        }
    }
}

#[component]
fn CurrentVoteDisplay(state: ReadSignal<VoteState>) -> Element {
    let agenda = use_memo(move || state().agenda);

    rsx! {
        div {
            div { class: "agenda-info-row",
                h6 { "{agenda().info().name}" }
                InfoButton { info: Info::Agenda(agenda()) }
            }
            p { "Votes" }
            ol {
                for vote in state().outcomes_by_votes.iter() {
                    li { key: "{vote.outcome.to_display_value()}",
                        "{vote.outcome.to_display_value()} - {vote.votes}"
                    }
                }
            }
        }
    }
}
