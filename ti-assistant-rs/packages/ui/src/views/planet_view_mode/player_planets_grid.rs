use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_regular_icons::FaTrashCan,
        fa_solid_icons::{FaArrowTurnUp, FaTrash},
    },
    Icon,
};
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{planet::PlanetTrait, planet_attachment::PlanetAttachment, tech::TechCategory},
    state::player::Player,
};

use crate::{
    components::{
        button::Button,
        faction_icon::FactionIcon,
        ti_icon::{TiIcon, TiIconType},
    },
    data::{event_context::EventContext, game_context::GameContext},
};

const NUM_COLUMNS: usize = 6;

#[component]
pub fn PlayerPlanetsGrid() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "player-planet-cards-container",
            for player_id in players().iter() {
                PlayerPlanetsCard { key: "{player_id}", player_id: player_id.clone() }
            }
        }
    }
}

const NAME_COL_ALIGN: &'static str = "align-left";
const TRAIT_COL_ALIGN: &'static str = "align-center";
const RESOURCE_COL_ALIGN: &'static str = "align-center";
const INFLUENCE_COL_ALIGN: &'static str = "align-center";
const TECH_COL_ALIGN: &'static str = "align-center";
const DELETE_COL_ALIGN: &'static str = "align-right";

#[component]
fn PlayerPlanetsCard(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let p1 = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .cloned()
            .expect("Player to exist")
    });

    let summary = use_memo(move || PlayerSummary::from_player(&player()));
    let planets = use_memo(move || {
        let mut planets = player()
            .planets
            .iter()
            .map(|(p, a)| {
                let mut a = a.iter().cloned().collect::<Vec<_>>();
                a.sort();
                (p.clone(), p.info(), a)
            })
            .collect::<Vec<_>>();
        planets.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
        planets
    });

    rsx! {
        div { class: "card player-planets-table-container",
            table { class: "player-planets-table",
                thead {
                    tr {
                        th { colspan: NUM_COLUMNS,
                            div { class: "player-name-row",
                                FactionIcon { faction: player().faction }
                                h2 { "{player().name}" }
                                FactionIcon { faction: player().faction }
                            }
                        }
                    }
                    tr {
                        th { colspan: NUM_COLUMNS,
                            div {
                                "{summary().cultural}"
                                TiIcon { icon: TiIconType::Cultural }

                                "{summary().industrial}"
                                TiIcon { icon: TiIconType::Industrial }

                                "{summary().hazardous}"
                                TiIcon { icon: TiIconType::Hazardous }
                            }
                            div {
                                "{summary().warfare}"
                                TiIcon { icon: TiIconType::WarfareFilled }

                                "{summary().propulsion}"
                                TiIcon { icon: TiIconType::PropulsionFilled }

                                "{summary().cybernetic}"
                                TiIcon { icon: TiIconType::CyberneticFilled }

                                "{summary().biotic}"
                                TiIcon { icon: TiIconType::BioticFilled }
                            }
                        }
                    }
                    tr {
                        th { class: format!("{NAME_COL_ALIGN} border-bottom-row"),
                            "Name"
                        }
                        th { class: format!("{TRAIT_COL_ALIGN} border-bottom-row"),
                            "Type"
                        }
                        th { class: format!("{RESOURCE_COL_ALIGN} border-bottom-row"),
                            TiIcon { icon: TiIconType::ResourceFilled }
                        }
                        th { class: format!("{INFLUENCE_COL_ALIGN} border-bottom-row"),
                            TiIcon { icon: TiIconType::InfluenceFilled }
                        }
                        th { class: format!("{TECH_COL_ALIGN} border-bottom-row"),
                            TiIcon { icon: TiIconType::LegendaryPlanetFilled }
                        }
                        th { class: format!("{DELETE_COL_ALIGN} border-bottom-row") }
                    }
                    tr {
                        th { class: format!("{NAME_COL_ALIGN} border-bottom-row"),
                            "Total"
                        }
                        th { class: format!("{TRAIT_COL_ALIGN} border-bottom-row"),
                            "{summary().cultural + summary().industrial + summary().hazardous}"
                        }
                        th { class: format!("{RESOURCE_COL_ALIGN} border-bottom-row"),
                            "{summary().resources}"
                        }
                        th { class: format!("{INFLUENCE_COL_ALIGN} border-bottom-row"),
                            "{summary().influence}"
                        }
                        th { class: format!("{TECH_COL_ALIGN} border-bottom-row"),
                            "{summary().warfare + summary().propulsion + summary().cybernetic + summary().biotic}"
                        }
                        th { class: format!("{DELETE_COL_ALIGN} border-bottom-row") }
                    }
                }
                tbody {
                    for (planet , info , attachments) in planets().iter() {
                        tr { class: "planet-row",
                            td { class: NAME_COL_ALIGN, "{info.name}" }
                            td { class: TRAIT_COL_ALIGN,
                                for t in info.planet_traits.iter() {
                                    TiIcon { icon: TiIconType::from(t) }
                                }
                            }
                            td { class: RESOURCE_COL_ALIGN, "{info.resources}" }
                            td { class: INFLUENCE_COL_ALIGN, "{info.influence}" }
                            td { class: TECH_COL_ALIGN,
                                for t in info.tech_specialities.iter() {
                                    TiIcon { icon: TiIconType::from(t) }
                                }
                            }
                            td { class: DELETE_COL_ALIGN,
                                Button {
                                    class: "unclaim-planet-button",
                                    onclick: {
                                        let p = planet.clone();
                                        move |_| {
                                            event
                                                .send_event(Event::SetPlanetOwner {
                                                    player: None,
                                                    planet: p.clone(),
                                                })
                                        }
                                    },
                                    Icon { class: "inline-icon", icon: FaTrash }
                                }
                            }
                        }
                        for (attachment , info) in attachments.iter().map(|a| (a, a.info())) {
                            tr {
                                td { class: "align-center",
                                    Icon {
                                        icon: FaArrowTurnUp,
                                        class: "rotate90",
                                    }
                                }

                                td { class: "align-right attachment-row-text",
                                    AttachmentIcon { attachment: attachment.clone() }
                                }

                                td { class: format!("attachment-row-text {RESOURCE_COL_ALIGN}"),
                                    "{info.resources}"
                                }

                                td { class: format!("attachment-row-text {INFLUENCE_COL_ALIGN}"),
                                    "{info.influence}"
                                }

                                td { class: TECH_COL_ALIGN,
                                    {info.tech_specialty.as_ref().map(|spec| rsx! {
                                        TiIcon { icon: TiIconType::from(spec) }
                                    })}
                                }

                                td { class: DELETE_COL_ALIGN,
                                    Button {
                                        class: "unclaim-planet-button",
                                        onclick: {
                                            let player = player_id.clone();
                                            let p = planet.clone();
                                            let a = attachment.clone();
                                            move |_| {
                                                event
                                                    .send_event(Event::RemovePlanetAttachment {
                                                        player: player.clone(),
                                                        planet: p.clone(),
                                                        attachment: a.clone(),
                                                    })
                                            }
                                        },
                                        Icon {
                                            class: "inline-icon",
                                            icon: FaTrashCan,
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

#[derive(Debug, Clone, Default, PartialEq)]
struct PlayerSummary {
    pub cultural: usize,
    pub industrial: usize,
    pub hazardous: usize,
    pub resources: usize,
    pub influence: usize,
    pub warfare: usize,
    pub propulsion: usize,
    pub cybernetic: usize,
    pub biotic: usize,
}

impl PlayerSummary {
    fn from_player(value: &Player) -> Self {
        let mut player_info = PlayerSummary::default();

        for (planet, attachment) in value.planets.iter() {
            let planet_info = planet.info();
            let attachment_infos = attachment
                .iter()
                .map(|a| (a.clone(), a.info()))
                .collect::<HashMap<_, _>>();

            let mut distinct_traits = HashSet::new();
            for t in planet_info.planet_traits.iter() {
                distinct_traits.insert(t);
            }

            for info in attachment_infos
                .values()
                .flat_map(|i| i.added_planet_traits.iter())
            {
                distinct_traits.insert(info);
            }

            for t in distinct_traits {
                match t {
                    PlanetTrait::Cultural => player_info.cultural += 1,
                    PlanetTrait::Hazardous => player_info.hazardous += 1,
                    PlanetTrait::Industrial => player_info.industrial += 1,
                }
            }

            player_info.resources += planet_info.resources as usize;
            player_info.influence += planet_info.influence as usize;

            for info in attachment_infos.values() {
                player_info.resources += info.resources as usize;
                player_info.influence += info.resources as usize;
            }

            let mut distinct_tech_specs = HashSet::new();
            for spec in planet_info.tech_specialities.iter() {
                distinct_tech_specs.insert(spec);
            }

            for spec in attachment_infos
                .values()
                .filter_map(|i| i.tech_specialty.as_ref())
            {
                distinct_tech_specs.insert(spec);
            }

            for spec in distinct_tech_specs.iter() {
                match spec {
                    TechCategory::Biotic => player_info.biotic += 1,
                    TechCategory::Propulsion => player_info.propulsion += 1,
                    TechCategory::Cybernetic => player_info.cybernetic += 1,
                    TechCategory::Warfare => player_info.warfare += 1,
                }
            }
        }

        player_info
    }
}

#[component]
fn AttachmentIcon(attachment: PlanetAttachment) -> Element {
    match attachment {
        PlanetAttachment::DemilitarizedZone => rsx! {
            TiIcon { icon: TiIconType::Demilitarized }
        },
        PlanetAttachment::TombOfEmphidia => rsx! {
            TiIcon { icon: TiIconType::TombOfEmphida }
        },
        PlanetAttachment::UITheProgenitor => rsx! {
            p { class: "white-text", "✹✹✹" }
        },
        PlanetAttachment::BioticResearchFacility
        | PlanetAttachment::CyberneticResearchFacility
        | PlanetAttachment::PropulsionResearchFacility
        | PlanetAttachment::WarfareResearchFacility => rsx! {
            div { class: "gray-text",
                "( 1 "
                TiIcon { icon: TiIconType::ResourceFilled }
                " 1 "
                TiIcon { icon: TiIconType::InfluenceFilled }
                ")"
            }
        },
        PlanetAttachment::BioticResearchFacilityResources => rsx! {
            div { class: "gray-text",
                "("
                TiIcon { icon: TiIconType::BioticFilled }
                ")"
            }
        },
        PlanetAttachment::CyberneticResearchFacilityResources => rsx! {
            div { class: "gray-text",
                "("
                TiIcon { icon: TiIconType::CyberneticFilled }
                ")"
            }
        },
        PlanetAttachment::PropulsionResearchFacilityResources => rsx! {
            div { class: "gray-text",
                "("
                TiIcon { icon: TiIconType::PropulsionFilled }
                ")"
            }
        },
        PlanetAttachment::WarfareResearchFacilityResources => rsx! {
            div { class: "gray-text",
                "("
                TiIcon { icon: TiIconType::WarfareFilled }
                ")"
            }
        },
        PlanetAttachment::NanoForge => rsx! {
            TiIcon { icon: TiIconType::LegendaryPlanetCircled }
        },
        PlanetAttachment::Terraform => rsx! {
            div {
                TiIcon { icon: TiIconType::Industrial }
                TiIcon { icon: TiIconType::Hazardous }
                TiIcon { icon: TiIconType::Cultural }
            }
        },
        _ => rsx! {},
    }
}
