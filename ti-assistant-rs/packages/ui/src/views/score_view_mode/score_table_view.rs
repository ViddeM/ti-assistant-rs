use dioxus::prelude::*;
use strum::{Display, EnumString};
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::objectives::ObjectiveKind,
    state::player::Player,
};

use crate::{
    components::{
        faction_button::FactionButton, faction_icon::FactionIcon, info_button::InfoButton,
    },
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
};

#[component]
pub fn ScoreTableView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let players = use_memo(move || {
        let gs = gc.game_state().clone();
        let mut players = gs
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| a.to_lowercase().cmp(&b.to_lowercase()));
        players
    });
    let player_count = use_memo(move || players.len());
    let custodian = use_memo(move || gc.game_state().score.custodians.clone());

    let revealed_stage_one_objectives = use_memo(move || {
        let mut objectives = gc
            .game_state()
            .score
            .revealed_objectives
            .keys()
            .filter(|o| o.info().kind == ObjectiveKind::StageI)
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });
    let revealed_stage_two_objectives = use_memo(move || {
        let mut objectives = gc
            .game_state()
            .score
            .revealed_objectives
            .keys()
            .filter(|o| o.info().kind == ObjectiveKind::StageII)
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    rsx! {
        table { class: "card score-view-table",
            thead {
                tr {
                    {players().into_iter().map(|(_, p)| rsx! {
                        th { class: "score-view-table-header",
                            FactionIcon { faction: p.faction }
                        }
                    })}
                }
                tr {
                    {players().into_iter().map(|(p, _)| rsx! {
                        td { class: "align-center", "{gc.game_state().score.player_points.get(&p).unwrap()}p" }
                    })}
                }
            }
            tbody {
                // Custodians
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Custodians",
                    styling_prefix: StylingPrefix::Custodians,
                }
                tr {
                    {players().into_iter().map(move |(id, p)| rsx! {
                        td { class: "align-center",
                            FactionButton {
                                faction: p.faction,
                                selected: custodian().eq(&Some(id.clone())),
                                onclick: move |_| {
                                    event
                                        .send_event(Event::SetCustodians {
                                            player: if custodian().eq(&Some(id.clone())) {
                                                None
                                            } else {
                                                Some(id.clone())
                                            },
                                        })
                                },
                            }
                        }
                    })}
                }

                // STAGE I Public Objectives
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Stage I",
                    styling_prefix: StylingPrefix::StageOne,
                }
                {
                    revealed_stage_one_objectives()
                        .into_iter()
                        .enumerate()
                        .map(|(i, o)| {
                            rsx! {
                                FactionIconsRow {
                                    players: players(),
                                    index: i,
                                    name: o.info().name,
                                    info: Info::Objective(o),
                                    selected: move |p| {
                                        gc.game_state()
                                            .score
                                            .revealed_objectives
                                            .get(&o)
                                            .expect("Objective to exist")
                                            .contains(&p)
                                    },
                                    enable: move |p| {
                                        event
                                            .send_event(Event::ScorePublicObjective {
                                                player: p,
                                                objective: Some(o.clone()),
                                            })
                                    },
                                    disable: move |p| {
                                        event
                                            .send_event(Event::UnscoreObjective {
                                                player: p,
                                                objective: o.clone(),
                                            })
                                    },
                                }
                            }
                        })
                }

                TableSectionHeader {
                    player_count: player_count(),
                    title: "Stage II",
                    styling_prefix: StylingPrefix::StageTwo,
                }
                {
                    revealed_stage_two_objectives()
                        .into_iter()
                        .enumerate()
                        .map(|(i, o)| {
                            rsx! {
                                FactionIconsRow {
                                    players: players(),
                                    index: i,
                                    name: o.info().name,
                                    info: Info::Objective(o),
                                    selected: move |p| {
                                        gc.game_state()
                                            .score
                                            .revealed_objectives
                                            .get(&o)
                                            .expect("Objective to exist")
                                            .contains(&p)
                                    },
                                    enable: move |p| {
                                        event
                                            .send_event(Event::ScorePublicObjective {
                                                player: p,
                                                objective: Some(o.clone()),
                                            });
                                    },
                                    disable: move |p| {
                                        event
                                            .send_event(Event::UnscoreObjective {
                                                player: p,
                                                objective: o.clone(),
                                            });
                                    },
                                }
                            }
                        })
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
enum StylingPrefix {
    StageOne,
    StageTwo,
    Secret,
    Agenda,
    Relic,
    Custodians,
    Imperial,
    Spftt,
    Extra,
}

impl StylingPrefix {
    fn get_color_style(&self) -> String {
        format!("{}-color", self.to_string())
    }

    fn get_background_style(&self) -> String {
        format!("{}-background-color", self.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct TableSectionHeaderProps {
    player_count: usize,
    title: &'static str,
    styling_prefix: StylingPrefix,
}

#[component]
fn TableSectionHeader(
    TableSectionHeaderProps {
        player_count,
        title,
        styling_prefix,
    }: TableSectionHeaderProps,
) -> Element {
    let color = styling_prefix.get_color_style();
    let background = styling_prefix.get_background_style();

    rsx! {
        tr {
            th { colspan: player_count,
                div { class: "stage-container",
                    div { class: format!("{background} horizontal-line") }
                    h2 { class: format!("{color} stage-text"), "{title}" }
                    div { class: format!("{background} horizontal-line") }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct SubSectionHeadingProps {
    player_count: usize,
    top_border: bool,
    name: String,
    info: Info,
}

#[component]
fn SubSectionHeading(
    SubSectionHeadingProps {
        player_count,
        top_border,
        name,
        info,
    }: SubSectionHeadingProps,
) -> Element {
    rsx! {
        tr {
            th { colspan: player_count, class: if top_border { "border-top" } else { "" },
                InfoButton { info: info.clone(), visibility: "hidden" }
                "{name}"
                InfoButton { info }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct FactionIconsRowProps {
    players: Vec<(PlayerId, Player)>,
    index: usize,
    name: String,
    info: Info,
    selected: Callback<PlayerId, bool>,
    disable: EventHandler<PlayerId>,
    enable: EventHandler<PlayerId>,
}

#[component]
fn FactionIconsRow(
    FactionIconsRowProps {
        players,
        index,
        name,
        info,
        selected,
        disable,
        enable,
    }: FactionIconsRowProps,
) -> Element {
    rsx! {
        SubSectionHeading {
            player_count: players.len(),
            top_border: index > 0,
            name,
            info,
        }
        tr {
            {
                players
                    .into_iter()
                    .map(|(id, p)| {
                        let s = selected(id.clone());
                        rsx! {
                            td { class: "align-center",
                                FactionButton {
                                    faction: p.faction,
                                    selected: s,
                                    onclick: move |_| {
                                        if s {
                                            disable(id.clone());
                                        } else {
                                            enable(id.clone());
                                        }
                                    },
                                }
                            }
                        }
                    })
            }
        }
    }
}
