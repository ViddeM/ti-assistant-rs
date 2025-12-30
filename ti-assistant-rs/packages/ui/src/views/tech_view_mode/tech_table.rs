use std::collections::HashMap;

use dioxus::prelude::*;
use strum::Display;
use ti_helper_game_data::{
    actions::event::Event,
    common::{color::Color, faction::Faction, player_id::PlayerId},
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

    let player_ids = use_memo(move || {
        players()
            .iter()
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>()
    });
    let player_techs = use_memo(move || {
        players()
            .iter()
            .map(|(id, p)| {
                (
                    id.clone(),
                    p.technologies.iter().cloned().collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>()
    });
    let player_factions = use_memo(move || {
        players()
            .iter()
            .map(|(id, p)| (id.clone(), p.faction.clone()))
            .collect::<HashMap<_, _>>()
    });

    let toggle_tech_for_player = move |player_id: PlayerId, technology: Technology| {
        if player_techs()
            .get(&player_id)
            .expect("player to be in tech maps")
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
        techs()
            .iter()
            .filter(|t| {
                let i = t.info();
                i.tech_type == TechType::UnitUpgrade && i.origin == TechOrigin::Base
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let warfare = use_memo(move || {
        techs()
            .iter()
            .filter(|t| {
                let i = t.info();
                i.tech_type == TechType::Category(TechCategory::Warfare)
                    && i.origin == TechOrigin::Base
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let biotic = use_memo(move || {
        techs()
            .iter()
            .filter(|t| {
                let i = t.info();
                i.tech_type == TechType::Category(TechCategory::Biotic)
                    && i.origin == TechOrigin::Base
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let propulsion = use_memo(move || {
        techs()
            .iter()
            .filter(|t| {
                let i = t.info();
                i.tech_type == TechType::Category(TechCategory::Propulsion)
                    && i.origin == TechOrigin::Base
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let cybernetic = use_memo(move || {
        techs()
            .iter()
            .filter(|t| {
                let i = t.info();
                i.tech_type == TechType::Category(TechCategory::Cybernetic)
                    && i.origin == TechOrigin::Base
            })
            .cloned()
            .collect::<Vec<_>>()
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
                    player_count: players().len(),
                    title: "Unit Upgrades",
                    styling_prefix: StylingPrefix::UnitUpgrade,
                }
                TechRows {
                    techs: unit_upgrades(),
                    players: player_ids(),
                    player_techs: player_techs(),
                    player_factions: player_factions(),
                    toggle_tech_for_player: move |(a, b)| toggle_tech_for_player(a, b),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Display, PartialEq)]
#[strum(serialize_all = "kebab-case")]
enum StylingPrefix {
    UnitUpgrade,
    Warfare,
    Propulsion,
    Biotic,
    Cybernetic,
    PlayerBlack,
    PlayerBlue,
    PlayerGreen,
    PlayerRed,
    PlayerYellow,
    PlayerPurple,
    PlayerOrange,
    PlayerPink,
}

impl From<&Color> for StylingPrefix {
    fn from(value: &Color) -> Self {
        match value {
            Color::Pink => Self::PlayerPink,
            Color::Orange => Self::PlayerOrange,
            Color::Green => Self::PlayerGreen,
            Color::Red => Self::PlayerRed,
            Color::Yellow => Self::PlayerYellow,
            Color::Black => Self::PlayerBlack,
            Color::Purple => Self::PlayerPurple,
            Color::Blue => Self::PlayerBlue,
        }
    }
}

const ICON_SIZE: u32 = 22;

#[component]
fn TableSectionHeading(
    player_count: usize,
    title: &'static str,
    styling_prefix: StylingPrefix,
    icon: Option<TiIconType>,
) -> Element {
    let background = format!("{}-background-color", styling_prefix.to_string());
    let color = format!("{}-color", styling_prefix.to_string());

    let icon_render = use_memo(move || {
        icon.clone().map(|i| {
            rsx! {
                TiIcon { icon: i, width: ICON_SIZE, height: ICON_SIZE }

            }
        })
    });

    rsx! {
        tr {
            th { colspan: player_count,
                div { class: "stage-container",
                    div { class: format!("{background} horizontal-line"),
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
}

#[derive(Debug, Clone, PartialEq, Props)]
struct TechRowsProps {
    techs: Vec<Technology>,
    players: Vec<PlayerId>,
    player_techs: HashMap<PlayerId, Vec<Technology>>,
    player_factions: HashMap<PlayerId, Faction>,
    toggle_tech_for_player: EventHandler<(PlayerId, Technology)>,
}

#[component]
fn TechRows(
    TechRowsProps {
        techs,
        players,
        player_techs,
        player_factions,
        toggle_tech_for_player,
    }: TechRowsProps,
) -> Element {
    rsx! {
        {techs.iter().enumerate().map(|(i, t)| rsx! {
            tr { key: "{t}",
                th { colspan: players.len(), class: if i == 0 { "" } else { "border-top" },
                    InfoButton { info: Info::Tech(t.clone()) }
                    {t.info().name}
                    InfoButton { info: Info::Tech(t.clone()) }
                }
            }
            tr {
                {
                    players
                        .clone()
                        .into_iter()
                        .map(|player_id| {
                            let has_tech = player_techs
                                .get(&player_id)
                                .map(|techs| techs.contains(&t))
                                .unwrap_or(false);
                            let faction = player_factions
                                .get(&player_id)
                                .cloned()
                                .expect("Player to have a faction");
                            let tech = t.clone();
                            rsx! {
                                td { key: "{player_id}", class: "align-center",
                                    FactionButton {
                                        faction,
                                        selected: has_tech,
                                        onclick: move |_| toggle_tech_for_player((player_id.clone(), tech.clone())),
                                    }
                                }
                            }
                        })
                }
            }
        })}
    }
}
