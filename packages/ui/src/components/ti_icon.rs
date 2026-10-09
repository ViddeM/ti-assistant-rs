use dioxus::prelude::*;
use ti_helper_game_data::components::{
    planet::PlanetTrait, planet_attachment::PlanetAttachment, tech::TechCategory,
};

const BIOTIC_FILLED_PNG: Asset = asset!("/assets/icons/resources/biotic_filled.png");
const BIOTIC_PNG: Asset = asset!("/assets/icons/resources/biotic.png");
const CULTURAL_PNG: Asset = asset!("/assets/icons/resources/cultural.png");
const CUSTODIANS_PNG: Asset = asset!("/assets/icons/resources/custodians.png");
const CYBERNETIC_FILLED_PNG: Asset = asset!("/assets/icons/resources/cybernetic_filled.png");
const CYBERNETIC_PNG: Asset = asset!("/assets/icons/resources/cybernetic.png");
const DEMILITARIZED_ZONE_SVG: Asset = asset!("/assets/icons/resources/demilitarized_zone.svg");
const DEMILITARIZED_PNG: Asset = asset!("/assets/icons/resources/demilitarized.png");
const HAZARDOUS_PNG: Asset = asset!("/assets/icons/resources/hazardous.png");
const INDUSTRIAL_PNG: Asset = asset!("/assets/icons/resources/industrial.png");
const INFLUENCE_FILLED_PNG: Asset = asset!("/assets/icons/resources/influence_filled.png");
const INFLUENCE_PNG: Asset = asset!("/assets/icons/resources/influence.png");
const LEGENDARY_FILLED_PNG: Asset = asset!("/assets/icons/resources/legendary_filled.png");
const LEGENDARY_PLANET_CIRCLED_PNG: Asset =
    asset!("/assets/icons/resources/legendary_planet_circled.png");
const LEGENDARY_PLANET_FILLED_PNG: Asset =
    asset!("/assets/icons/resources/legendary_planet_filled.png");
const LEGENDARY_PLANET_PNG: Asset = asset!("/assets/icons/resources/legendary_planet.png");
const LEGENDARY_PNG: Asset = asset!("/assets/icons/resources/legendary.png");
const NAALU_0_TOKEN_WEBP: Asset = asset!("/assets/icons/resources/naalu_0_token.webp");
const PROPULSION_FILLED_PNG: Asset = asset!("/assets/icons/resources/propulsion_filled.png");
const PROPULSION_PNG: Asset = asset!("/assets/icons/resources/propulsion.png");
const RESOURCE_FILLED_PNG: Asset = asset!("/assets/icons/resources/resource_filled.png");
const RESOURCE_PNG: Asset = asset!("/assets/icons/resources/resource.png");
const TOMB_OF_EMPHIDA_PNG: Asset = asset!("/assets/icons/resources/tomb_of_emphida.png");
const TOMB_OF_EMPHIDA_WEBP: Asset = asset!("/assets/icons/resources/tomb_of_emphida.webp");
const WARFARE_FILLED_PNG: Asset = asset!("/assets/icons/resources/warfare_filled.png");
const WARFARE_PNG: Asset = asset!("/assets/icons/resources/warfare.png");

#[derive(Debug, Clone, PartialEq)]
pub enum TiIconType {
    BioticFilled,
    Biotic,
    Cultural,
    Custodians,
    CyberneticFilled,
    Cybernetic,
    DemilitarizedZone,
    Demilitarized,
    Hazardous,
    Industrial,
    InfluenceFilled,
    Influence,
    LegendaryFilled,
    LegendaryPlanetCircled,
    LegendaryPlanetFilled,
    LegendaryPlanet,
    Legendary,
    Naalu0Token,
    PropulsionFilled,
    Propulsion,
    ResourceFilled,
    Resource,
    TombOfEmphida,
    WarfareFilled,
    Warfare,
}

impl TiIconType {
    fn get_asset(&self) -> Asset {
        match self {
            TiIconType::BioticFilled => BIOTIC_FILLED_PNG,
            TiIconType::Biotic => BIOTIC_PNG,
            TiIconType::Cultural => CULTURAL_PNG,
            TiIconType::Custodians => CUSTODIANS_PNG,
            TiIconType::CyberneticFilled => CYBERNETIC_FILLED_PNG,
            TiIconType::Cybernetic => CYBERNETIC_PNG,
            TiIconType::DemilitarizedZone => DEMILITARIZED_ZONE_SVG,
            TiIconType::Demilitarized => DEMILITARIZED_PNG,
            TiIconType::Hazardous => HAZARDOUS_PNG,
            TiIconType::Industrial => INDUSTRIAL_PNG,
            TiIconType::InfluenceFilled => INFLUENCE_FILLED_PNG,
            TiIconType::Influence => INFLUENCE_PNG,
            TiIconType::LegendaryFilled => LEGENDARY_FILLED_PNG,
            TiIconType::LegendaryPlanetCircled => LEGENDARY_PLANET_CIRCLED_PNG,
            TiIconType::LegendaryPlanetFilled => LEGENDARY_PLANET_FILLED_PNG,
            TiIconType::LegendaryPlanet => LEGENDARY_PLANET_PNG,
            TiIconType::Legendary => LEGENDARY_PNG,
            TiIconType::Naalu0Token => NAALU_0_TOKEN_WEBP,
            TiIconType::PropulsionFilled => PROPULSION_FILLED_PNG,
            TiIconType::Propulsion => PROPULSION_PNG,
            TiIconType::ResourceFilled => RESOURCE_FILLED_PNG,
            TiIconType::Resource => RESOURCE_PNG,
            TiIconType::TombOfEmphida => TOMB_OF_EMPHIDA_PNG,
            TiIconType::WarfareFilled => WARFARE_FILLED_PNG,
            TiIconType::Warfare => WARFARE_PNG,
        }
    }
}

impl From<&PlanetTrait> for TiIconType {
    fn from(value: &PlanetTrait) -> Self {
        match value {
            PlanetTrait::Cultural => Self::Cultural,
            PlanetTrait::Hazardous => Self::Hazardous,
            PlanetTrait::Industrial => Self::Industrial,
        }
    }
}

impl From<&TechCategory> for TiIconType {
    fn from(value: &TechCategory) -> Self {
        match value {
            TechCategory::Biotic => Self::BioticFilled,
            TechCategory::Propulsion => Self::PropulsionFilled,
            TechCategory::Cybernetic => Self::CyberneticFilled,
            TechCategory::Warfare => Self::WarfareFilled,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Props)]
pub struct TiIconProps {
    icon: TiIconType,
    width: Option<u32>,
    height: Option<u32>,
    #[props(default, into)]
    class: String,
}

#[component]
pub fn TiIcon(
    TiIconProps {
        icon,
        width,
        height,
        class,
    }: TiIconProps,
) -> Element {
    rsx! {
        img {
            src: icon.get_asset(),
            alt: format!("Icon {icon:?}"),
            width: width.unwrap_or(16),
            height: height.unwrap_or(16),
            class,
        }
    }
}
