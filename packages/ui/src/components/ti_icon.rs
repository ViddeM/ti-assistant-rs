use dioxus::prelude::*;
use ti_helper_game_data::components::{planet::PlanetTrait, tech::TechCategory};

const BIOTIC_FILLED: Asset = asset!("/assets/icons/resources/webp/biotic_filled.webp");
const BIOTIC: Asset = asset!("/assets/icons/resources/webp/biotic.webp");
const CULTURAL: Asset = asset!("/assets/icons/resources/webp/cultural.webp");
const CUSTODIANS: Asset = asset!("/assets/icons/resources/webp/custodians.webp");
const CYBERNETIC_FILLED: Asset = asset!("/assets/icons/resources/webp/cybernetic_filled.webp");
const CYBERNETIC: Asset = asset!("/assets/icons/resources/webp/cybernetic.webp");
const DEMILITARIZED_ZONE: Asset = asset!("/assets/icons/resources/webp/demilitarized_zone.webp");
const HAZARDOUS: Asset = asset!("/assets/icons/resources/webp/hazardous.webp");
const INDUSTRIAL: Asset = asset!("/assets/icons/resources/webp/industrial.webp");
const INFLUENCE_FILLED: Asset = asset!("/assets/icons/resources/webp/influence_filled.webp");
const INFLUENCE: Asset = asset!("/assets/icons/resources/webp/influence.webp");
const LEGENDARY_FILLED: Asset = asset!("/assets/icons/resources/webp/legendary_filled.webp");
const LEGENDARY_PLANET_CIRCLED: Asset =
    asset!("/assets/icons/resources/webp/legendary_planet_circled.webp");
const LEGENDARY_PLANET_FILLED: Asset =
    asset!("/assets/icons/resources/webp/legendary_planet_filled.webp");
const LEGENDARY_PLANET: Asset = asset!("/assets/icons/resources/webp/legendary_planet.webp");
const LEGENDARY: Asset = asset!("/assets/icons/resources/webp/legendary.webp");
const NAALU_0_TOKEN: Asset = asset!("/assets/icons/resources/webp/naalu_0_token.webp");
const PROPULSION_FILLED: Asset = asset!("/assets/icons/resources/webp/propulsion_filled.webp");
const PROPULSION: Asset = asset!("/assets/icons/resources/webp/propulsion.webp");
const RESOURCE_FILLED: Asset = asset!("/assets/icons/resources/webp/resource_filled.webp");
const RESOURCE: Asset = asset!("/assets/icons/resources/webp/resource.webp");
const TOMB_OF_EMPHIDA: Asset = asset!("/assets/icons/resources/webp/tomb_of_emphida.webp");
const WARFARE_FILLED: Asset = asset!("/assets/icons/resources/webp/warfare_filled.webp");
const WARFARE: Asset = asset!("/assets/icons/resources/webp/warfare.webp");

#[derive(Debug, Clone, PartialEq)]
pub enum TiIconType {
    BioticFilled,
    Biotic,
    Cultural,
    Custodians,
    CyberneticFilled,
    Cybernetic,
    DemilitarizedZone,
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
    TombOfEmphidia,
    WarfareFilled,
    Warfare,
}

impl TiIconType {
    fn get_asset(&self) -> Asset {
        match self {
            TiIconType::BioticFilled => BIOTIC_FILLED,
            TiIconType::Biotic => BIOTIC,
            TiIconType::Cultural => CULTURAL,
            TiIconType::Custodians => CUSTODIANS,
            TiIconType::CyberneticFilled => CYBERNETIC_FILLED,
            TiIconType::Cybernetic => CYBERNETIC,
            TiIconType::DemilitarizedZone => DEMILITARIZED_ZONE,
            TiIconType::Hazardous => HAZARDOUS,
            TiIconType::Industrial => INDUSTRIAL,
            TiIconType::InfluenceFilled => INFLUENCE_FILLED,
            TiIconType::Influence => INFLUENCE,
            TiIconType::LegendaryFilled => LEGENDARY_FILLED,
            TiIconType::LegendaryPlanetCircled => LEGENDARY_PLANET_CIRCLED,
            TiIconType::LegendaryPlanetFilled => LEGENDARY_PLANET_FILLED,
            TiIconType::LegendaryPlanet => LEGENDARY_PLANET,
            TiIconType::Legendary => LEGENDARY,
            TiIconType::Naalu0Token => NAALU_0_TOKEN,
            TiIconType::PropulsionFilled => PROPULSION_FILLED,
            TiIconType::Propulsion => PROPULSION,
            TiIconType::ResourceFilled => RESOURCE_FILLED,
            TiIconType::Resource => RESOURCE,
            TiIconType::TombOfEmphidia => TOMB_OF_EMPHIDA,
            TiIconType::WarfareFilled => WARFARE_FILLED,
            TiIconType::Warfare => WARFARE,
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
