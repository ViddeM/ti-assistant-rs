use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{
        planet::PlanetTrait,
        tech::{TechCategory, TechType},
    },
};

use crate::{
    components::ti_icon::{TiIcon, TiIconType},
    data::game_context::GameContext,
};

const PLAYER_RESOURCES_SCSS: Asset =
    asset!("/assets/styling/views/players_sidebar/parts/player_resources.scss");

#[component]
pub fn PlayerResources(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    let p1 = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .expect("Player to exist")
            .clone()
    });
    let num_planets = use_memo(move || player().planets.len());
    let planets = use_memo(move || {
        player()
            .planets
            .iter()
            .map(|(p, attachments)| (p.clone(), p.info(), attachments.clone()))
            .collect::<Vec<_>>()
    });
    let num_resources = use_memo(move || {
        planets()
            .iter()
            .map(|(_, info, attachments)| {
                info.resources + attachments.iter().map(|a| a.info().resources).sum::<u32>()
            })
            .sum::<u32>()
    });
    let num_influence = use_memo(move || {
        planets()
            .iter()
            .map(|(_, info, attachments)| {
                info.influence + attachments.iter().map(|a| a.info().influence).sum::<u32>()
            })
            .sum::<u32>()
    });

    let planet_traits = use_memo(move || {
        planets()
            .iter()
            .map(|(_, info, attachments)| {
                let mut traits = info.planet_traits.clone();
                for t in attachments
                    .iter()
                    .map(|a| a.info().added_planet_traits.clone())
                    .flatten()
                {
                    traits.push(t);
                }
                traits
            })
            .flatten()
            .collect::<Vec<_>>()
    });

    let num_cultural = use_memo(move || {
        planet_traits()
            .iter()
            .filter(|&t| t.eq(&PlanetTrait::Cultural))
            .count()
    });
    let num_hazardous = use_memo(move || {
        planet_traits()
            .iter()
            .filter(|&t| t.eq(&PlanetTrait::Hazardous))
            .count()
    });
    let num_industrial = use_memo(move || {
        planet_traits()
            .iter()
            .filter(|&t| t.eq(&PlanetTrait::Industrial))
            .count()
    });

    let num_techs = use_memo(move || player().technologies.len());
    let techs = use_memo(move || {
        player()
            .technologies
            .iter()
            .map(|t| t.info())
            .collect::<Vec<_>>()
    });
    let num_biotic = use_memo(move || {
        techs()
            .iter()
            .filter(|t| matches!(t.tech_type, TechType::Category(TechCategory::Biotic)))
            .count()
    });
    let num_cybernetic = use_memo(move || {
        techs()
            .iter()
            .filter(|t| matches!(t.tech_type, TechType::Category(TechCategory::Cybernetic)))
            .count()
    });
    let num_propulsion = use_memo(move || {
        techs()
            .iter()
            .filter(|t| matches!(t.tech_type, TechType::Category(TechCategory::Propulsion)))
            .count()
    });
    let num_warfare = use_memo(move || {
        techs()
            .iter()
            .filter(|t| matches!(t.tech_type, TechType::Category(TechCategory::Warfare)))
            .count()
    });

    rsx! {
        document::Stylesheet { href: PLAYER_RESOURCES_SCSS }

        div { class: "planet-tech-container",
            div { class: "planet-content",
                div { class: "resource-row",
                    div { class: "planets-count",
                        p { "{num_planets()}" }
                    }
                    p { "{num_resources()}" }
                    TiIcon { icon: TiIconType::Resource }
                    TiIcon { icon: TiIconType::Influence }
                    p { "{num_influence()}" }
                }
                div { class: "resource-row",
                    "{num_cultural()}"
                    TiIcon { icon: TiIconType::Cultural }
                    "{num_industrial()}"
                    TiIcon { icon: TiIconType::Industrial }
                    "{num_hazardous()}"
                    TiIcon { icon: TiIconType::Hazardous }
                }
            }
            div { class: "tech-content",
                div { class: "resource-row",
                    p { "{num_techs()}" }
                    p { class: "tech-icon", "T" }
                }
                div { class: "resource-row",
                    "{num_biotic()}"
                    TiIcon { icon: TiIconType::BioticFilled }
                    "{num_cybernetic()}"
                    TiIcon { icon: TiIconType::CyberneticFilled }
                    "{num_propulsion()}"
                    TiIcon { icon: TiIconType::PropulsionFilled }
                    "{num_warfare()}"
                    TiIcon { icon: TiIconType::WarfareFilled }
                }
            }
        }
    }
}
