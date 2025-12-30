use std::collections::HashMap;

use dioxus::prelude::*;
use strum::{Display, EnumString};
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{agenda::Agenda, objectives::ObjectiveKind, relic::Relic},
    state::{player::Player, score::ScorableAgenda},
};

use crate::{
    components::{
        button::Button, faction_button::FactionButton, faction_icon::FactionIcon,
        info_button::InfoButton,
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
    let agenda_scores = use_memo(move || {
        let mut agendas = gc.game_state().score.agenda_scores.clone();
        agendas.sort_by(|a, b| a.get_agenda().cmp(&b.get_agenda()));
        agendas
    });
    let sftt_scores = use_memo(move || {
        gc.game_state().score.support_for_the_throne.iter().fold(
            HashMap::<PlayerId, usize>::new(),
            |mut map, (a, b)| {
                map.entry(b.clone())
                    .and_modify(|curr| *curr += 1)
                    .or_insert(1);
                map
            },
        )
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

                // STAGE II Public Objectives
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

                // Secret Objectives
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Secrets",
                    styling_prefix: StylingPrefix::Secret,
                }
                tr {
                    {players().into_iter().map(|(id, _)| rsx! {
                        td { class: "align-center",
                            PlayerSecretObjectivesScore { player_id: id }
                        }
                    })}
                }

                // Agendas
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Agendas",
                    styling_prefix: StylingPrefix::Agenda,
                }
                {agenda_scores().into_iter().enumerate().map(|(i, a)| rsx! {
                    AgendaScoreRow {
                        scorable_agenda: a,
                        top_border: i > 0,
                        players: players().into_iter().map(|(p, _)| p).collect::<Vec<_>>(),
                    }
                })}

                // Relics
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Relics",
                    styling_prefix: StylingPrefix::Relic,
                }
                RelicPointRows {}

                // Support for the Throne
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Support for the Throne",
                    styling_prefix: StylingPrefix::Spftt,
                }
                tr {
                    {
                        players()
                            .into_iter()
                            .map(|(id, _)| {
                                let score = sftt_scores().get(&id).cloned().unwrap_or_default();
                                rsx! {
                                    td { class: "align-center", "{score}" }
                                }
                            })
                    }
                }

                // Imperial
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Imperial",
                    styling_prefix: StylingPrefix::Imperial,
                }
                tr {
                    {
                        players()
                            .into_iter()
                            .map(|(id, _)| {
                                let score = gc
                                    .game_state()
                                    .score
                                    .imperial
                                    .get(&id)
                                    .cloned()
                                    .unwrap_or_default();
                                rsx! {
                                    td { class: "align-center",
                                        IncDecView {
                                            points: score,
                                            change_points: move |new_points| {
                                                event
                                                    .send_event(Event::AddImperial {
                                                        player: id.clone(),
                                                        value: new_points - score,
                                                    })
                                            },
                                        }
                                    }
                                }
                            })
                    }
                }

                // Extra points (manual modifications)
                TableSectionHeader {
                    player_count: player_count(),
                    title: "Extra Points",
                    styling_prefix: StylingPrefix::Extra,
                }
                tr {
                    {

                        players()
                            .into_iter()
                            .map(|(id, _)| {
                                let score = gc
                                    .game_state()
                                    .score
                                    .extra_points
                                    .get(&id)
                                    .cloned()
                                    .unwrap_or_default();
                                rsx! {
                                    td { class: "align-center",
                                        IncDecView {
                                            points: score,
                                            change_points: move |new_points| {
                                                event
                                                    .send_event(Event::AddExtraPoints {
                                                        player: id.clone(),
                                                        value: new_points - score,
                                                    })
                                            },
                                        }
                                    }
                                }
                            })
                    }
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

#[component]
fn PlayerSecretObjectivesScore(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    let player_secrets = use_memo(move || {
        gc.game_state()
            .score
            .secret_objectives
            .get(&player_id)
            .map(|s| s.len())
            .unwrap_or_default()
    });

    rsx! {
        p { "{player_secrets()}" }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct AgendaScoreRowProps {
    scorable_agenda: ScorableAgenda,
    top_border: bool,
    players: Vec<PlayerId>,
}

#[component]
fn AgendaScoreRow(
    AgendaScoreRowProps {
        scorable_agenda,
        top_border,
        players,
    }: AgendaScoreRowProps,
) -> Element {
    let gc = use_context::<GameContext>();

    let heading = |agenda: Agenda| {
        rsx! {
            SubSectionHeading {
                player_count: players.len(),
                top_border,
                name: agenda.info().name,
                info: Info::Agenda(agenda),
            }
        }
    };

    match scorable_agenda {
        ScorableAgenda::Mutiny {
            players_that_voted_for,
            for_won,
        } => rsx! {
            {heading(Agenda::Mutiny)}
            tr {
                {
                    players
                        .iter()
                        .map(|p| {
                            let score = if players_that_voted_for.contains(&p) {
                                if for_won { 1 } else { -1 }
                            } else {
                                0
                            };
                            rsx! {
                                td { class: "align-center", "{score}" }
                            }
                        })
                }
            }
        },
        ScorableAgenda::HolyPlanetOfIxth { planet } => {
            let planet_owner = gc
                .game_state()
                .players
                .iter()
                .find(|(_, p)| p.planets.contains_key(&planet))
                .map(|(id, _)| id)
                .cloned()
                .expect("Planet owner to exist");

            rsx! {
                {heading(Agenda::HolyPlanetOfIxth)}
                tr {
                    {
                        players
                            .iter()
                            .map(|p| {
                                let score = if planet_owner.eq(p) { 1 } else { 0 };
                                rsx! {
                                    td { class: "align-center", "{score}" }
                                }
                            })
                    }
                }
            }
        }
        ScorableAgenda::SeedOfAnEmpire { players_elected } => rsx! {
            {heading(Agenda::SeedOfAnEmpire)}
            tr {
                {
                    players
                        .iter()
                        .map(|p| {
                            let score = if players_elected.contains(p) { 1 } else { 0 };
                            rsx! {
                                td { class: "align-center", "{score}" }
                            }
                        })
                }
            }
        },
        ScorableAgenda::PoliticalCensure { player } => rsx! {
            {heading(Agenda::ShardOfTheThrone)}
            tr {
                {

                    players
                        .iter()
                        .map(|p| {
                            let score = if player.eq(p) { 1 } else { 0 };
                            rsx! {
                                td { class: "align-center", "{score}" }
                            }
                        })
                }
            }
        },
        ScorableAgenda::ShardOfTheThrone { player } => rsx! {
            {heading(Agenda::ShardOfTheThrone)}
            tr {
                {
                    players
                        .iter()
                        .map(|p| {
                            let score = if player.eq(p) { 1 } else { 0 };
                            rsx! {
                                td { class: "align-center", "{score}" }
                            }
                        })
                }
            }
        },
        ScorableAgenda::TheCrownOfEmphidia { player } => rsx! {
            {heading(Agenda::TheCrownOfEmphidia)}
            tr {
                {
                    players
                        .iter()
                        .map(|p| {
                            let score = if player.eq(p) { 1 } else { 0 };
                            rsx! {
                                td { class: "align-center", "{score}" }
                            }
                        })
                }
            }
        },
    }
}

#[component]
fn RelicPointRows() -> Element {
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

    let crown_of_emphidia_enabled = use_memo(move || {
        gc.game_options()
            .relics
            .contains_key(&Relic::TheCrownOfEmphidia)
    });
    let crown_of_emphidia_holder =
        use_memo(move || gc.game_state().score.crown_of_emphidia.clone());
    let crown_of_emphidia_render = use_memo(move || {
        if crown_of_emphidia_enabled() {
            Some(rsx! {
                SubSectionHeading {
                    player_count: players().len(),
                    top_border: false,
                    name: "The Crown of Emphidia",
                    info: Info::Relic(Relic::TheCrownOfEmphidia),
                }
                tr {
                    {
                        players()
                            .into_iter()
                            .map(|(id, p)| {
                                let is_owner = crown_of_emphidia_holder().eq(&Some(id.clone()));

                                rsx! {
                                    td { class: "align-center",
                                        FactionButton {
                                            faction: p.faction,
                                            selected: is_owner,
                                            onclick: move || {
                                                event
                                                    .send_event(Event::SetCrownOfEmphidiaOwner {
                                                        player: if is_owner { None } else { Some(id.clone()) },
                                                    })
                                            },
                                        }
                                    }
                                }
                            })
                    }
                }
            })
        } else {
            None
        }
    });

    let shard_of_the_throne_enabled = use_memo(move || {
        gc.game_options()
            .relics
            .contains_key(&Relic::ShardOfTheThrone)
    });
    let shard_of_the_throne_holder =
        use_memo(move || gc.game_state().score.shard_of_the_throne.clone());
    let shard_of_the_throne_render = use_memo(move || {
        if shard_of_the_throne_enabled() {
            Some(rsx! {
                SubSectionHeading {
                    player_count: players().len(),
                    top_border: true,
                    name: "Shard of the Throne",
                    info: Info::Relic(Relic::ShardOfTheThrone),
                }
                tr {
                    {
                        players()
                            .into_iter()
                            .map(|(id, p)| {
                                let is_holder = shard_of_the_throne_holder().eq(&Some(id.clone()));
                                rsx! {
                                    td { class: "align-center",
                                        FactionButton {
                                            faction: p.faction,
                                            selected: is_holder,
                                            onclick: move || {
                                                event
                                                    .send_event(Event::SetShardForTheThroneOwner {
                                                        player: if is_holder { None } else { Some(id.clone()) },
                                                    })
                                            },
                                        }
                                    }
                                }
                            })
                    }
                }
            })
        } else {
            None
        }
    });

    rsx! {
        {crown_of_emphidia_render()}
        {shard_of_the_throne_render()}
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct IncDecViewProps {
    points: i8,
    change_points: EventHandler<i8>,
}

#[component]
fn IncDecView(
    IncDecViewProps {
        points,
        change_points,
    }: IncDecViewProps,
) -> Element {
    rsx! {
        Button {
            onclick: move |_| change_points(points + 1),
            class: "inc-dec-button",
            "^"
        }
        p { "{points}" }
        Button {
            onclick: move |_| change_points(points - 1),
            class: "inc-dec-button",
            disabled: points == 0,
            "v"
        }
    }
}
