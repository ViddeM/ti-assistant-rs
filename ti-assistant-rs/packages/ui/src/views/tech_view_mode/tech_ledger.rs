use dioxus::prelude::*;

use crate::{
    components::{
        faction_icon::FactionIcon,
        ti_icon::{TiIcon, TiIconType},
    },
    data::game_context::GameContext,
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

    rsx! {
        div { class: "card",
            h2 { "Shortcuts" }
            ul { class: "tech-ledger-list",
                li {
                    // TODO: What icon do we use here? xd
                    TiIcon { icon: TiIconType::LegendaryPlanetFilled }
                    a { href: "#UnitUpgrades", "Unit Upgrades" }
                }
                li {
                    TiIcon { icon: TiIconType::WarfareFilled }
                    a { href: "#Warfare", "Warfare" }
                }
                li {
                    TiIcon { icon: TiIconType::PropulsionFilled }
                    a { href: "#Propulsion", "Propulsion" }
                }
                li {
                    TiIcon { icon: TiIconType::CyberneticFilled }
                    a { href: "#Cybernetic", "Cybernetic" }
                }
                li {
                    TiIcon { icon: TiIconType::BioticFilled }
                    a { href: "#Biotic", "Biotic" }
                }
                {players().into_iter().map(|(p, f)| rsx! {
                    li { key: "{p}",
                        FactionIcon { faction: f, width: 16, height: 16 }
                        a { href: format!("#{p}"), "{p} - {f}" }
                    }
                })}
            }
        }
    }
}
