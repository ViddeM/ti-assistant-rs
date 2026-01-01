use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaTrash, Icon};
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::{
        button::Button,
        dropdown::{AgendaDropdown, PlayerDropdown},
        info_button::InfoButton,
    },
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
};

const LAWS_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/laws_view_mode.scss");

#[component]
pub fn LawsViewMode() -> Element {
    rsx! {
        document::Stylesheet { href: LAWS_VIEW_MODE_SCSS }

        div { class: "laws-view-container",
            ActiveLawsTable {}
            AddLawForm {}
        }
    }
}

#[component]
fn ActiveLawsTable() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let laws = use_memo(move || {
        let mut laws = gc.game_state().laws.keys().cloned().collect::<Vec<_>>();
        laws.sort();
        laws
    });

    rsx! {
        div { class: "card",
            table { class: "laws-table",
                thead {
                    tr {
                        th { colspan: 3, class: "align-center",
                            h2 { "Active laws" }
                        }
                    }
                }
                tbody {
                    {

                        if laws().is_empty() {
                            rsx! {
                                tr {
                                    td { colspan: 3, class: "align-center", "No laws in play" }
                                }
                            }
                        } else {
                            rsx! {
                                for law in laws().iter() {
                                    tr { key: "{law}",
                                        td {
                                            Button {
                                                class: "delete-law-button",
                                                onclick: {
                                                    let l = law.clone();
                                                    move |_| { event.send_event(Event::RepealLaw { law: l.clone() }) }
                                                },
                                                Icon { class: "inline-icon", icon: FaTrash }
                                            }
                                        }
                                        td { "{law.info().name" }
                                        td {
                                            InfoButton { info: Info::Agenda(law.clone()) }
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

#[component]
fn AddLawForm() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut agenda = use_signal(|| None);

    let mut player: Signal<PlayerId> = use_signal(|| "".into());
    let mut votes = use_signal(|| 0);
    let mut vote_option = use_signal(|| String::new());

    let vote_state = use_memo(move || {
        gc.game_state()
            .agenda_override_state
            .clone()
            .map(|s| s.vote_state)
    });
    let outcome = use_signal(|| vote_state().map(|s| s.expected_outcome).flatten());

    let all_agendas = use_memo(move || {
        let mut laws = gc
            .game_options()
            .agendas
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        laws.sort();
        laws
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

    let players_that_have_voted = use_memo(move || {
        vote_state()
            .map(|v| v.player_votes.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    });
    let players_left_to_vote = use_memo(move || {
        gc.game_state()
            .players
            .keys()
            .filter(|&p| !players_that_have_voted().contains(p))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "card column",
            h2 { "Add Agenda" }
            {
                if let Some(state) = vote_state() {
                    rsx! {
                        div { class: "form-container",
                            h3 {
                                "{state.agenda.info().name}"
                                InfoButton { info: Info::Agenda(state.agenda.clone()) }
                            }



                            Button { onclick: move |_| event.send_event(Event::AddAgendaCancel), "Cancel Adding Agenda" }

                            for (player , votes) in state.player_votes.iter() {
                                p { key: "{player}",
                                    "{player}: "
                                    {votes.clone().expect("Player to be in votes map").outcome.to_display_value()}
                                    " "
                                    {votes.clone().expect("Player votes to exist").votes.to_string()}
                                }
                            }

                            fieldset {
                                legend { "Cast Votes" }

                                form {
                                    class: "form-container",
                                    onsubmit: move |_| {
                                        event
                                            .send_event(Event::AddAgendaPlayerVote {
                                                player: player(),
                                                outcome: outcome().expect("Outcome to be selected"),
                                                votes: votes(),
                                            })
                                    },
                                    PlayerDropdown {
                                        value: player(),
                                        on_select: move |p| player.set(p),
                                        options: players_left_to_vote(),
                                    }
                                    input {
                                        r#type: "number",
                                        class: "margin-left",
                                        placeholder: "Votes (0)",
                                        min: 0,
                                        max: 1000,
                                        value: votes(),
                                        onchange: move |e| {
                                            if let Ok(val) = e.value().parse() {
                                                votes.set(val);
                                            } else {
                                                votes.set(votes());
                                            }
                                        },
                                    }
                                    "TODO"
                                }
                            }
                        }
                    }
                } else {
                    rsx! {
                        form { class: "form-container",
                            AgendaDropdown {
                                value: agenda(),
                                options: available_agendas(),
                                on_select: move |a| agenda.set(a),
                            }
                            Button {
                                class: "margin-top",
                                disabled: agenda().is_none(),
                                onclick: {
                                    let a = agenda().clone();
                                    move |_| {
                                        event
                                            .send_event(Event::AddAgendaBegin {
                                                agenda: a.clone().expect("Agenda to be set"),
                                            })
                                    }
                                },
                                "Add Agenda"
                            }
                        }
                    }
                }
            }
        }
    }
}
