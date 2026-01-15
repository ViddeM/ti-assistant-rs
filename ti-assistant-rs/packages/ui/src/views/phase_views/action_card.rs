use std::str::FromStr;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{ActionCardAction, Event},
    common::player_id::PlayerId,
    components::{
        action_card::ActionCard,
        tech::{TechOrigin, TechType, Technology},
    },
    state::game_state::ActionPhaseProgress,
};

use crate::{
    components::{
        button::Button,
        dropdown::{Dropdown, TechDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::select_tech::SelectTechView,
};

const ACTION_CARD_SCSS: Asset = asset!("/assets/styling/views/phase_views/action_card.scss");

#[component]
pub fn ActionCardView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::ActionCard(card)) => {
            rsx! {
                document::Stylesheet { href: ACTION_CARD_SCSS }

                div { class: "card",
                    h2 { "{card.card.info().name}" }
                    {
                        if view.is_active() {
                            rsx! {
                                ActionCardProgressView { card: card.card.clone() }
                            }
                        } else {
                            rsx! {
                                p { "Not your turn, currently {current_player()} is playing" }
                            }
                        }
                    }
                }
            }
        }
        _ => rsx! {},
    }
}

#[component]
fn ActionCardProgressView(card: ActionCard) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist")
    });

    let send_commit_message = use_callback(move |action| {
        event.send_event(Event::ActionCardActionCommit {
            player: current_player_id(),
            data: action,
        })
    });

    match card {
        ActionCard::FocusedResearch => {
            rsx! {
                div {
                    SelectTechView {
                        player_id: current_player_id(),
                        on_select: move |tech| send_commit_message(
                            Some(ActionCardAction::FocusedResearch {
                                tech,
                            }),
                        ),
                    }
                }
            }
        }
        ActionCard::DivertFunding => {
            rsx! {
                DivertFundingView { send_commit_message }
            }
        }
        ActionCard::Plagiarize => {
            rsx! {
                PlagiarizeView { send_commit_message }
            }
        }
        _ => {
            rsx! {
                div { class: "commit-button-row",
                    Button { onclick: move |_| send_commit_message(None), "Commit" }
                }
            }
        }
    }
}

#[component]
fn DivertFundingView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut remove_tech = use_signal(|| None);
    let mut gain_tech: Signal<Option<Technology>> = use_signal(|| None);

    let player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("There to be a current player in action phase")
    });

    let deletable_techs = use_memo(move || {
        let player = gc
            .game_state()
            .players
            .get(&player_id())
            .cloned()
            .expect("Current player to exist");

        player
            .technologies
            .iter()
            .filter(|t| {
                let i = t.info();
                if i.origin != TechOrigin::Base {
                    return false;
                }

                if i.tech_type == TechType::UnitUpgrade {
                    return false;
                }

                true
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "divert-funding-container",
            fieldset { class: "tech-change-container",
                legend { "Remove tech" }
                TechDropdown {
                    value: remove_tech,
                    on_select: move |tech| remove_tech.set(tech),
                    options: deletable_techs(),
                }
            }

            fieldset { class: "tech-change-container",
                legend { "Gain tech" }
                {
                    if let Some(t) = gain_tech.read().as_ref() {
                        rsx! {
                            p { "{t.info().name}" }
                        }
                    } else {
                        rsx! {
                            SelectTechView {
                                player_id: player_id(),
                                on_select: move |tech| {
                                    gain_tech.set(Some(tech));
                                },
                            }
                        }
                    }
                }
            }

            Button {
                disabled: remove_tech.read().is_none() || gain_tech.read().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::ActionCardActionCommit {
                            player: player_id(),
                            data: Some(ActionCardAction::DivertFunding {
                                remove_tech: remove_tech().expect("No remove tech selected"),
                                take_tech: gain_tech().expect("No gain tech selected"),
                            }),
                        })
                },
            }
        }
    }
}

#[component]
fn PlagiarizeView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();

    let selected_player = use_signal(|| None);
    let selected_tech = use_signal(|| None);

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("No current player")
    });
    let players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .keys()
            .filter(|&p| !p.eq(&current_player_id()))
            .cloned()
            .collect::<Vec<_>>();

        players.sort();

        players
    });

    rsx! {
        table { class: "plagiarize-table",
            thead {
                tr {
                    th { colspan: 2,
                        h6 { "Select neighbours tech" }
                    }
                }
                tr {
                    td { class: "align-center", "Player" }
                    td { class: "align-center", "Tech" }
                }
            }
            tbody {
                {
                    players()
                        .iter()
                        .map(|p| {
                            rsx! {
                                PlagiarizePlayerRow {
                                    key: "{p}",
                                    player_id: p.clone(),
                                    selected_player: selected_player.clone(),
                                    selected_tech: selected_tech.clone(),
                                }
                            }
                        })
                }
            }
            tfoot {
                tr {
                    th { colspan: 2,
                        Button {
                            disabled: selected_player().is_none() || selected_tech().is_none(),
                            onclick: move |_| {
                                send_commit_message(
                                    Some(ActionCardAction::Plagiarize {
                                        tech: selected_tech().expect("Tech to be selected"),
                                    }),
                                )
                            },
                            "Plagiarize Tech"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct PlagiarizePlayerRowProps {
    player_id: PlayerId,
    selected_player: Signal<Option<PlayerId>>,
    selected_tech: Signal<Option<Technology>>,
}

#[component]
fn PlagiarizePlayerRow(
    PlagiarizePlayerRowProps {
        player_id,
        mut selected_player,
        mut selected_tech,
    }: PlagiarizePlayerRowProps,
) -> Element {
    let gc = use_context::<GameContext>();
    let current_player_techs = use_memo(move || {
        let current_player = gc
            .game_state()
            .current_player
            .clone()
            .expect("Current player to exist");
        gc.game_state()
            .players
            .get(&current_player)
            .cloned()
            .expect("Current player to exist")
            .technologies
            .into_iter()
            .collect::<Vec<_>>()
    });
    let p1 = player_id.clone();
    let available_player_techs = use_memo(move || {
        let mut techs = gc
            .game_state()
            .players
            .get(&p1)
            .expect("Player to exist")
            .technologies
            .iter()
            .filter(|t| t.info().origin == TechOrigin::Base)
            .filter(|t| !current_player_techs().contains(t))
            .map(|t| (t.clone(), t.info()))
            .collect::<Vec<_>>();

        techs.sort_by(|(a, _), (b, _)| a.cmp(b));

        techs
    });

    let p2 = player_id.clone();
    let value = use_memo(move || {
        let tech = if selected_player().eq(&Some(p2.clone())) {
            selected_tech()
        } else {
            None
        };

        tech.map(|t| t.to_string()).unwrap_or_default()
    });

    rsx! {
        tr {
            td { class: "align-right", "{player_id}: " }
            td { class: "align-left",
                Dropdown {
                    disabled: available_player_techs().is_empty(),
                    value: value(),
                    oninput: move |e: FormEvent| {
                        selected_player.set(Some(player_id.clone()));
                        let v = e.value();
                        if v.is_empty() {
                            selected_tech.set(None);
                        } else {
                            selected_tech.set(Some(Technology::from_str(&v).expect("Tech to be valid")))
                        }
                    },
                    option { value: "", "--Select a tech--" }
                    {
                        available_player_techs()
                            .iter()
                            .map(|(t, info)| {
                                rsx! {
                                    option { key: "{t}", value: "{t.to_string()}", "{info.name}" }
                                }
                            })
                    }
                }
            }
        }
    }
}
