use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    components::{
        faction_icon::FactionIcon,
        ti_icon::{TiIcon, TiIconType},
    },
    data::game_context::GameContext,
    views::tech_view_mode::TechSection,
};

#[component]
pub fn TechLedger() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.faction.clone()))
            .collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| a.cmp(b));
        players
    });
    let player_color = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.color.clone()))
            .collect::<HashMap<_, _>>()
    });

    rsx! {
        div { class: "card",
            h2 { "Shortcuts" }
            ul { class: "tech-ledger-list",
                li {
                    // TODO: What icon do we use here? xd
                    TiIcon { icon: TiIconType::LegendaryPlanetFilled }
                    a { href: TechSection::UnitUpgrade.to_tag(), "Unit Upgrades" }
                }
                li {
                    TiIcon { icon: TiIconType::WarfareFilled }
                    a { href: TechSection::Warfare.to_tag(), "Warfare" }
                }
                li {
                    TiIcon { icon: TiIconType::PropulsionFilled }
                    a { href: TechSection::Propulsion.to_tag(), "Propulsion" }
                }
                li {
                    TiIcon { icon: TiIconType::CyberneticFilled }
                    a { href: TechSection::Cybernetic.to_tag(), "Cybernetic" }
                }
                li {
                    TiIcon { icon: TiIconType::BioticFilled }
                    a { href: TechSection::Biotic.to_tag(), "Biotic" }
                }
                {
                    players()
                        .into_iter()
                        .map(|(p, f)| {
                            let color = player_color()
                                .get(&p)
                                .cloned()
                                .expect("Player to have color");
                            let name = format!("{} - {}", p, f.name());
                            rsx! {
                                li { key: "{p}",
                                    FactionIcon { faction: f, width: 16, height: 16 }
                                    a { href: TechSection::from(&color).to_tag(), "{name}" }
                                }
                            }
                        })
                }
            }
        }
    }
}
