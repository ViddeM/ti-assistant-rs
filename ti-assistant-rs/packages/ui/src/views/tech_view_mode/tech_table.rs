use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::tech::{TechCategory, TechOrigin, TechType, Technology},
};

use crate::{
    components::{
        faction_button::FactionButton,
        faction_icon::FactionIcon,
        info_button::InfoButton,
        ti_icon::{TiIcon, TiIconType},
    },
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
    views::tech_view_mode::TechSection,
};

#[component]
pub fn TechTable() -> Element {
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

    let player_techs = use_memo(move || {
        Arc::new(
            players()
                .iter()
                .map(|(id, p)| {
                    (
                        id.clone(),
                        p.technologies.iter().cloned().collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        )
    });
    let faction_techs = use_memo(move || {
        Arc::new(
            gc.game_options()
                .technologies
                .iter()
                .filter_map(|(t, i)| {
                    if let TechOrigin::Faction(f) = i.origin {
                        Some((f, t))
                    } else {
                        None
                    }
                })
                .fold(
                    HashMap::new(),
                    |mut map: HashMap<Faction, Vec<Technology>>, (faction, tech)| {
                        map.entry(faction)
                            .and_modify(|ts| ts.push(tech.clone()))
                            .or_insert(vec![tech.clone()]);
                        map
                    },
                ),
        )
    });
    let player_faction_specific_techs = use_memo(move || {
        Arc::new(
            players()
                .iter()
                .map(|(id, p)| {
                    let mut techs = faction_techs()
                        .get(&p.faction)
                        .expect("Players faction to exist")
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>();

                    techs.sort();

                    (id.clone(), Arc::new(techs))
                })
                .collect::<Vec<_>>(),
        )
    });
    let player_factions = use_memo(move || {
        Arc::new(
            players()
                .iter()
                .map(|(id, p)| (id.clone(), p.faction.clone()))
                .collect::<HashMap<_, _>>(),
        )
    });

    let toggle_tech_for_player = move |player_id: PlayerId, technology: Technology| {
        if player_techs()
            .iter()
            .find(|(p, _)| p.eq(&player_id))
            .expect("player to be in tech maps")
            .1
            .contains(&technology)
        {
            event.send_event(Event::RemoveTechFromPlayer {
                player: player_id.clone(),
                tech: technology.clone(),
            });
        } else {
            event.send_event(Event::AddTechToPlayer {
                player: player_id.clone(),
                tech: technology.clone(),
            });
        }
    };

    let techs = use_memo(move || {
        let mut techs = gc
            .game_options()
            .technologies
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        techs.sort();
        techs
    });

    let unit_upgrades = use_memo(move || {
        Arc::new(
            techs()
                .iter()
                .filter(|t| {
                    let i = t.info();
                    i.tech_type == TechType::UnitUpgrade && i.origin == TechOrigin::Base
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
    });

    let warfare = use_memo(move || {
        Arc::new(
            techs()
                .iter()
                .filter(|t| {
                    let i = t.info();
                    i.tech_type == TechType::Category(TechCategory::Warfare)
                        && i.origin == TechOrigin::Base
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
    });

    let biotic = use_memo(move || {
        Arc::new(
            techs()
                .iter()
                .filter(|t| {
                    let i = t.info();
                    i.tech_type == TechType::Category(TechCategory::Biotic)
                        && i.origin == TechOrigin::Base
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
    });

    let propulsion = use_memo(move || {
        Arc::new(
            techs()
                .iter()
                .filter(|t| {
                    let i = t.info();
                    i.tech_type == TechType::Category(TechCategory::Propulsion)
                        && i.origin == TechOrigin::Base
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
    });

    let cybernetic = use_memo(move || {
        Arc::new(
            techs()
                .iter()
                .filter(|t| {
                    let i = t.info();
                    i.tech_type == TechType::Category(TechCategory::Cybernetic)
                        && i.origin == TechOrigin::Base
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
    });

    rsx! {
        table { class: "card tech-view-table",
            thead {
                tr {
                    {players().into_iter().map(|(id, p)| rsx! {
                        th { key: "{id}", class: "tech-view-table-header",
                            FactionIcon { faction: p.faction }
                        }
                    })}
                }
                tr {
                    {players().into_iter().map(|(id, p)| rsx! {
                        th { key: "{id}", "{p.technologies.len()}" }
                    })}
                }
            }
            tbody {
                // Unit Upgrades
                TableSectionHeading {
                    title: "Unit Upgrade".to_string(),
                    section: TechSection::UnitUpgrade,
                    player_count: players().len(),
                }
                TechRows {
                    techs: unit_upgrades(),
                    player_techs: player_techs(),
                    player_factions: player_factions(),
                    toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                }

                // Warfare
                TableSectionHeading {
                    title: "Warfare".to_string(),
                    section: TechSection::Warfare,
                    player_count: players().len(),
                    icon: TiIconType::WarfareFilled,
                }
                TechRows {
                    techs: warfare(),
                    player_techs: player_techs(),
                    player_factions: player_factions(),
                    toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                }

                // Propulsion
                TableSectionHeading {
                    title: "Propulsion".to_string(),
                    section: TechSection::Propulsion,
                    player_count: players().len(),
                    icon: TiIconType::PropulsionFilled,
                }
                TechRows {
                    techs: propulsion(),
                    player_techs: player_techs(),
                    player_factions: player_factions(),
                    toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                }

                // Cybernetic
                TableSectionHeading {
                    title: "cybernetic".to_string(),
                    section: TechSection::Cybernetic,
                    player_count: players().len(),
                    icon: TiIconType::CyberneticFilled,
                }
                TechRows {
                    techs: cybernetic(),
                    player_techs: player_techs(),
                    player_factions: player_factions(),
                    toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                }

                // Biotic
                TableSectionHeading {
                    title: "Biotic".to_string(),
                    section: TechSection::Biotic,
                    player_count: players().len(),
                    icon: TiIconType::BioticFilled,
                }
                TechRows {
                    techs: biotic(),
                    player_techs: player_techs(),
                    player_factions: player_factions(),
                    toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                }

                // Player Techs
                for (player_id , techs) in player_faction_specific_techs().iter() {
                    Fragment { key: "{player_id}",
                        TableSectionHeading {
                            title: players()
                                .iter()
                                .find(|(id, _)| id.eq(&player_id))
                                .map(|(id, p)| format!("{id} - {}", p.faction.name()))
                                .expect("Player to exist"),
                            section: players()
                                .iter()
                                .find(|(id, _)| id.eq(&player_id))
                                .map(|(_, p)| (&p.color).into())
                                .expect("Player to exist"),
                            player_count: players().len(),
                        }
                        TechRows {
                            techs: techs.clone(),
                            player_techs: player_techs(),
                            player_factions: player_factions(),
                            toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                        }
                    }
                }
            }
        }
    }
}

const ICON_SIZE: u32 = 22;

#[component]
fn TableSectionHeading(
    title: String,
    section: TechSection,
    player_count: usize,
    icon: Option<TiIconType>,
) -> Element {
    let background = format!("{}-background-color", section.to_string());
    let color = format!("{}-color", section.to_string());

    let icon_render = use_memo(move || {
        icon.clone().map(|i| {
            rsx! {
                TiIcon { icon: i, width: ICON_SIZE, height: ICON_SIZE }
            }
        })
    });

    rsx! {
        tr { id: "{section}",
            th { colspan: player_count,
                div { class: "stage-container",
                    div { class: format!("{background} horizontal-line") }
                    h2 { class: format!("{color} tech-group-text"),
                        {icon_render()}
                        "{title}"
                        {icon_render()}
                    }
                    div { class: format!("{background} horizontal-line") }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct TechRowsProps {
    techs: Arc<Vec<Technology>>,
    player_techs: Arc<Vec<(PlayerId, Vec<Technology>)>>,
    player_factions: Arc<HashMap<PlayerId, Faction>>,
    toggle_tech_for_player: EventHandler<(PlayerId, Technology)>,
}

#[component]
fn TechRows(
    TechRowsProps {
        techs,
        player_techs,
        player_factions,
        toggle_tech_for_player,
    }: TechRowsProps,
) -> Element {
    rsx! {
        for (index , tech) in techs.iter().enumerate() {
            tr { key: "{tech}",
                th {
                    colspan: player_techs.len(),
                    class: if index == 0 { "" } else { "border-top" },
                    InfoButton { info: Info::Tech(tech.clone()), visibility: "hidden" }
                    {tech.info().name}
                    InfoButton { info: Info::Tech(tech.clone()) }
                }
            }
            tr {
                for (player_id , pt) in player_techs.iter() {
                    td { key: "{player_id}", class: "align-center",
                        FactionButton {
                            faction: player_factions.get(player_id).cloned().expect("Player to have a faction"),
                            selected: pt.contains(&tech),
                            onclick: {
                                let tech = tech.clone();
                                let player_id = player_id.clone();
                                move |_| toggle_tech_for_player((player_id.clone(), tech.clone()))
                            },
                        }
                    }
                }
            }
        }
    }
}
