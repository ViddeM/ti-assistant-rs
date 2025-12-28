use dioxus::prelude::*;
use strum::IntoEnumIterator;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::tech::{TechCategory, TechOrigin, TechType, Technology},
};

use crate::{
    components::{
        button::Button,
        dropdown::TechDropdown,
        ti_icon::{TiIcon, TiIconType},
    },
    data::game_context::GameContext,
};

const SELECT_TECH_SCSS: Asset = asset!("/assets/styling/views/select_tech.scss");
const ALL_TECH_TYPES: [TechType; 5] = [
    TechType::UnitUpgrade,
    TechType::Category(TechCategory::Biotic),
    TechType::Category(TechCategory::Cybernetic),
    TechType::Category(TechCategory::Propulsion),
    TechType::Category(TechCategory::Warfare),
];

#[component]
pub fn SelectTechView(
    player_id: PlayerId,
    on_select: EventHandler<Technology>,
    filtered_techs: Option<Vec<Technology>>,
) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_tech_type = use_signal(|| None);
    let mut selected_tech = use_signal(|| None);

    let p1 = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .cloned()
            .expect("Current player to exist")
    });
    let player_techs = use_memo(move || player().technologies.iter().cloned().collect::<Vec<_>>());
    let filtered_techs = use_memo(move || filtered_techs.clone().unwrap_or_default());
    let available_techs = use_memo(move || {
        gc.game_options()
            .technologies
            .keys()
            .filter(|&t| player_techs().contains(t))
            .filter(|&t| filtered_techs().contains(t))
            .filter(|&t| match t.info().origin {
                TechOrigin::Base => true,
                TechOrigin::Faction(faction) => player().faction == faction,
            })
            .cloned()
            .collect::<Vec<_>>()
    });
    let selected_techs = use_memo(move || {
        available_techs()
            .iter()
            .filter(|t| selected_tech_type().eq(&Some(&t.info().tech_type)))
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        document::Stylesheet { href: SELECT_TECH_SCSS }

        div { class: "column",
            {
                ALL_TECH_TYPES
                    .iter()
                    .map(|tech_type| {
                        rsx! {
                            TechCategoryButton {
                                tech_type: tech_type.clone(),
                                is_selected: selected_tech_type().as_ref().eq(&Some(&tech_type)),
                                set_selected: move |_| {
                                    selected_tech_type.set(Some(tech_type));
                                    selected_tech.set(None);
                                },
                                selected_tech: selected_tech(),
                                tech_options: selected_techs(),
                                set_selected_tech: move |tech| selected_tech.set(Some(tech)),
                            }
                        }
                    })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct TechCategoryButtonProps {
    tech_type: TechType,
    is_selected: bool,
    set_selected: EventHandler<()>,
    selected_tech: Option<Technology>,
    tech_options: Vec<Technology>,
    set_selected_tech: EventHandler<Technology>,
}

#[component]
fn TechCategoryButton(
    TechCategoryButtonProps {
        tech_type,
        is_selected,
        set_selected,
        selected_tech,
        tech_options,
        set_selected_tech,
    }: TechCategoryButtonProps,
) -> Element {
    let base_style = format!(
        "tech-category-button-container color-{}",
        tech_type.to_type_name()
    );

    if !is_selected {
        return rsx! {
            Button { onclick: move |_| set_selected(()), class: base_style,
                "{tech_type.to_type_name()}"
                {
                    if let TechType::Category(t) = &tech_type {
                        rsx! {
                            TiIcon { icon: get_icon_type(t) }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
        };
    }

    rsx! {
        fieldset { class: format!("{base_style} center-row"),
            legend {
                "{tech_type.to_type_name()}"
                {
                    if let TechType::Category(cat) = &tech_type {
                        rsx! {
                            TiIcon { icon: get_icon_type(&cat) }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
            TechDropdown {
                value:
            }
        }
    }
}

fn get_icon_type(t: &TechCategory) -> TiIconType {
    match t {
        TechCategory::Biotic => TiIconType::BioticFilled,
        TechCategory::Propulsion => TiIconType::PropulsionFilled,
        TechCategory::Cybernetic => TiIconType::CyberneticFilled,
        TechCategory::Warfare => TiIconType::WarfareFilled,
    }
}
