use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::agenda::AgendaElect,
    state::agenda::{AgendaRound, AgendaState, VoteState},
};

use crate::{
    components::{
        button::Button,
        dropdown::{AgendaDropdown, VoteOptionDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn AgendaActionsView(state: ReadSignal<AgendaState>) -> Element {
    let event = use_context::<EventContext>();

    rsx! {
        div { class: "card",
            if state().round == AgendaRound::Completed {
                div { class: "agenda-phase-complete-container",
                    h3 { "Ready all planets!" }
                    Button { onclick: move |_| event.send_event(Event::CompleteAgendaPhase) }
                }
            } else if let Some(vote) = state().vote {
                ActiveAgendaView { state: vote }
            } else {
                RevealAgendaView {}
            }
        }
    }
}

#[component]
fn RevealAgendaView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let speaker = use_memo(move || {
        gc.game_state()
            .speaker
            .clone()
            .expect("Speaker to exist during agenda phase")
    });

    let mut current_agenda = use_signal(|| None);

    let all_agendas = use_memo(move || {
        gc.game_options()
            .agendas
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    });
    let used_agendas = use_memo(move || {
        gc.game_state()
            .agenda_vote_history
            .iter()
            .map(|a| a.vote.agenda.clone())
            .collect::<Vec<_>>()
    });
    let available_agendas = use_memo(move || {
        let mut agendas = all_agendas()
            .iter()
            .filter(|&a| !used_agendas().contains(a))
            .cloned()
            .collect::<Vec<_>>();
        agendas.sort();
        agendas
    });

    rsx! {
        div {
            if view.is_global_or_speaker() {
                fieldset {
                    legend {
                        h2 { class: "reveal-agenda-title", "{speaker()}" }
                    }
                    div { class: "reveal-agenda-box",
                        label { r#for: "select-agenda-dropdown", "Reveal an agenda" }
                        AgendaDropdown {
                            id: "select-agenda-dropdown",
                            value: current_agenda,
                            options: available_agendas(),
                            on_select: move |agenda| current_agenda.set(agenda),
                        }
                        Button {
                            disabled: current_agenda().is_none(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::RevealAgenda {
                                        agenda: current_agenda().expect("Current agenda to be set"),
                                    })
                            },
                            "Reveal"
                        }
                    }
                }
            } else {
                div { style: "text-align: center",
                    h2 { "Waiting for {speaker()} to reveal agenda" }
                }
            }
        }
    }
}

#[component]
fn ActiveAgendaView(state: ReadSignal<VoteState>) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let agenda = use_memo(move || state().agenda.info().name);

    let speaker = use_memo(move || {
        gc.game_state()
            .speaker
            .clone()
            .expect("Speaker to exist during agenda phase")
    });

    let players = use_memo(move || gc.game_state().turn_order.clone());

    rsx! {
        div {
            h6 { "{agenda()}" }
            ol {
                li { "{speaker()}: Read the agenda" }
                li {
                    "When agenda is revealed"
                    br {}
                    Button { onclick: move |_| event.send_event(Event::VetoAgenda), "Veto" }
                }
                li { "After agenda is revealed" }
                li {
                    "Vote: "
                    for player in players().iter() {
                        PlayerVoteView {
                            key: "{player}",
                            player_id: player.clone(),
                            state,
                        }
                    }
                }
                if view.is_global_or_speaker() {
                    li {
                        ResolveOutcome { state }
                    }
                } else {
                    li { "Waiting for speaker ({speaker()}) to resolve outcome" }
                }
            }
        }
    }
}

#[component]
fn PlayerVoteView(player_id: ReadSignal<PlayerId>, state: ReadSignal<VoteState>) -> Element {
    rsx! {
        fieldset { class: "agenda-actions-container",
            legend { "{player_id()}" }
            PlayerVoteActionsView { player_id, state }
        }
    }
}

#[component]
fn PlayerVoteActionsView(player_id: ReadSignal<PlayerId>, state: ReadSignal<VoteState>) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&player_id())
            .cloned()
            .expect("Player to exist")
    });

    if player().faction == Faction::NekroVirus {
        return rsx! {
            p { "Nekro Virus cannot vote" }
        };
    }

    let player_vote = use_memo(move || state().player_votes[&player_id()].clone());

    if let Some(vote) = player_vote() {
        todo!("NOT DONE");
    } else if !view.is_global_or_speaker() {
        todo!("NOT DONE");
        return rsx! {
            p {}
        };
    }

    let mut vote_option = use_signal(|| None);
    let mut votes = use_signal(|| 0);
    let candidates = use_memo(move || state().candidates.clone());

    rsx! {
        div { class: "cast-vote-container",
            VoteOptionDropdown {
                value: vote_option,
                options: candidates(),
                on_select: move |opt| vote_option.set(opt),
            }
            //  TODO: Restrict player votes.
            input {
                r#type: "number",
                min: 0,
                max: 1000,
                value: votes,
                onchange: move |e: FormEvent| {
                    let s = e.value();

                    if let Ok(v) = s.parse() {
                        votes.set(v);
                    } else {
                        votes.set(votes());
                    }
                },
            }
            div {
                Button {
                    onclick: move |_| {
                        event
                            .send_event(Event::CastAgendaVote {
                                player: player_id(),
                                outcome: None,
                                votes: 0,
                            })
                    },
                    "Abstain"
                }
                Button {
                    disabled: votes() == 0 || vote_option().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::CastAgendaVote {
                                player: player_id(),
                                outcome: vote_option(),
                                votes: votes(),
                            })
                    },
                    "Vote"
                }
            }
        }
    }
}

#[component]
fn ResolveOutcome(state: ReadSignal<VoteState>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut outcome = use_signal(|| None);

    let candidates = use_memo(move || state().candidates.clone());

    let resolve = use_memo(move || {
        outcome()
            .map(|elect: AgendaElect| elect.to_display_value())
            .unwrap_or("Discard".to_string())
    });

    let players_that_can_vote = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .filter(|(_, player)| player.faction != Faction::NekroVirus)
            .map(|(p, _)| p.clone())
            .collect::<Vec<_>>()
    });
    let everyone_has_voted =
        use_memo(move || state().player_votes.len() == players_that_can_vote().len());

    rsx! {
        "Resolve Outcome"
        fieldset {
            legend { "Resolve" }

            div { class: "resolve-outcome-container",
                label { "Override outcome" }
                VoteOptionDropdown {
                    value: outcome,
                    on_select: move |o| outcome.set(o),
                    options: candidates(),
                }
                Button {
                    disabled: !everyone_has_voted(),
                    onclick: move |_| {
                        event
                            .send_event(Event::ResolveAgenda {
                                outcome: outcome(),
                            })
                    },
                    "Resolve {resolve()}"
                }
            }
        }
    }
}
