use dioxus::prelude::*;
use ti_helper_game_data::components::planet_attachment::PlanetAttachment;

const DEMILITARIZED_ZONE: Asset = asset!("/assets/icons/tokens/webp/demilitarized_zone_token.webp");
const DYSON_SPHERE: Asset = asset!("/assets/icons/tokens/webp/dyson_sphere_token.webp");
const PARADISE_WORLD: Asset = asset!("/assets/icons/tokens/webp/paradise_world_token.webp");
const TOMB_OF_EMPHIDIA: Asset = asset!("/assets/icons/tokens/webp/tomb_of_emphidia_token.webp");
const BIOTIC_TOKEN: Asset = asset!("/assets/icons/tokens/webp/biotic_token.webp");
const PROPULSION_TOKEN: Asset = asset!("/assets/icons/tokens/webp/propulsion_token.webp");
const CYBERNETIC_TOKEN: Asset = asset!("/assets/icons/tokens/webp/cybernetic_token.webp");
const WARFARE_TOKEN: Asset = asset!("/assets/icons/tokens/webp/warfare_token.webp");
const BIOTIC_RESOURCES_TOKEN: Asset =
    asset!("/assets/icons/tokens/webp/biotic_resources_token.webp");
const CYBERNETIC_RESOURCES_TOKEN: Asset =
    asset!("/assets/icons/tokens/webp/cybernetic_resources_token.webp");
const PROPULSION_RESOURCES_TOKEN: Asset =
    asset!("/assets/icons/tokens/webp/propulsion_resources_token.webp");
const WARFARE_RESOURCES_TOKEN: Asset =
    asset!("/assets/icons/tokens/webp/warfare_resources_token.webp");
const LASAX_SURVIVORS: Asset = asset!("/assets/icons/tokens/webp/lasax_survivors_token.webp");
const MINING_WORLD: Asset = asset!("/assets/icons/tokens/webp/mining_world_token.webp");
const RICH_WORLD: Asset = asset!("/assets/icons/tokens/webp/rich_world_token.webp");
const GEOFORM: Asset = asset!("/assets/icons/tokens/webp/geoform_token.webp");
const TERRAFORM: Asset = asset!("/assets/icons/tokens/webp/terraform_token.webp");
const NANO_FORGE: Asset = asset!("/assets/icons/tokens/webp/nano_forge_token.webp");
const MIRAGE: Asset = asset!("/assets/icons/tokens/webp/mirage_token.webp");
const DESTROYED_PLANET: Asset = asset!("/assets/icons/tokens/webp/destroyed_planet_token.webp");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiToken {
    DemilitarizedZone,
    DysonSphere,
    ParadiseWorld,
    TombOfEmphidia,
    BioticResearchFacility,
    CyberneticResearchFacility,
    PropulsionResearchFacility,
    WarfareResearchFacility,
    BioticResearchFacilityResources,
    CyberneticResearchFacilityResources,
    PropulsionResearchFacilityResources,
    WarfareResearchFacilityResources,
    LasaxSurvivors,
    MiningWorld,
    RichWorld,
    Geoform,
    Terraform,
    NanoForge,
    Mirage,
    DestroyedPlanet,
}

impl TiToken {
    pub fn get_asset(&self) -> Asset {
        match self {
            TiToken::DemilitarizedZone => DEMILITARIZED_ZONE,
            TiToken::DysonSphere => DYSON_SPHERE,
            TiToken::ParadiseWorld => PARADISE_WORLD,
            TiToken::TombOfEmphidia => TOMB_OF_EMPHIDIA,
            TiToken::BioticResearchFacility => BIOTIC_TOKEN,
            TiToken::CyberneticResearchFacility => CYBERNETIC_TOKEN,
            TiToken::PropulsionResearchFacility => PROPULSION_TOKEN,
            TiToken::WarfareResearchFacility => WARFARE_TOKEN,
            TiToken::BioticResearchFacilityResources => BIOTIC_RESOURCES_TOKEN,
            TiToken::CyberneticResearchFacilityResources => CYBERNETIC_RESOURCES_TOKEN,
            TiToken::PropulsionResearchFacilityResources => PROPULSION_RESOURCES_TOKEN,
            TiToken::WarfareResearchFacilityResources => WARFARE_RESOURCES_TOKEN,
            TiToken::LasaxSurvivors => LASAX_SURVIVORS,
            TiToken::MiningWorld => MINING_WORLD,
            TiToken::RichWorld => RICH_WORLD,
            TiToken::Geoform => GEOFORM,
            TiToken::Terraform => TERRAFORM,
            TiToken::NanoForge => NANO_FORGE,
            TiToken::Mirage => MIRAGE,
            TiToken::DestroyedPlanet => DESTROYED_PLANET,
        }
    }
}

impl From<&PlanetAttachment> for TiToken {
    fn from(attachment: &PlanetAttachment) -> Self {
        match attachment {
            PlanetAttachment::DemilitarizedZone => TiToken::DemilitarizedZone,
            PlanetAttachment::DysonSphere => TiToken::DysonSphere,
            PlanetAttachment::ParadiseWorld => TiToken::ParadiseWorld,
            PlanetAttachment::TombOfEmphidia => TiToken::TombOfEmphidia,
            PlanetAttachment::BioticResearchFacility => TiToken::BioticResearchFacility,
            PlanetAttachment::CyberneticResearchFacility => TiToken::CyberneticResearchFacility,
            PlanetAttachment::PropulsionResearchFacility => TiToken::PropulsionResearchFacility,
            PlanetAttachment::WarfareResearchFacility => TiToken::WarfareResearchFacility,
            PlanetAttachment::BioticResearchFacilityResources => {
                TiToken::BioticResearchFacilityResources
            }
            PlanetAttachment::CyberneticResearchFacilityResources => {
                TiToken::CyberneticResearchFacilityResources
            }
            PlanetAttachment::PropulsionResearchFacilityResources => {
                TiToken::PropulsionResearchFacilityResources
            }
            PlanetAttachment::WarfareResearchFacilityResources => {
                TiToken::WarfareResearchFacilityResources
            }
            PlanetAttachment::LasaxSurvivors => TiToken::LasaxSurvivors,
            PlanetAttachment::MiningWorld => TiToken::MiningWorld,
            PlanetAttachment::RichWorld => TiToken::RichWorld,
            PlanetAttachment::Geoform => TiToken::Geoform,
            PlanetAttachment::Terraform => TiToken::Terraform,
            PlanetAttachment::NanoForge => TiToken::NanoForge,
        }
    }
}
