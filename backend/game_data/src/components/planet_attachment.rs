use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::common::expansions::Expansion;

use super::{
    planet::{PlanetInfo, PlanetTrait},
    tech::TechCategory,
};

/// A planet attachment.
#[derive(
    Debug, Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord, Hash, TS,
)]
#[ts(export)]
#[allow(missing_docs)]
pub enum PlanetAttachment {
    DemilitarizedZone,
    DysonSphere,
    ParadiseWorld,
    TombOfEmphidia,
    BioticResearchFacility,
    CyberneticResearchFacility,
    PropulsionResearchFacility,
    WarfareResearchFacility,
    /* Next 4 are made up to account for the alternative function of the research facility (provides resources if the planet already or as the trait). */
    BioticResearchFacilityResources,
    CyberneticResearchFacilityResources,
    PropulsionResearchFacilityResources,
    WarfareResearchFacilityResources,
    LasaxSurvivors,
    MiningWorld,
    RichWorld,
    UITheProgenitor,
    Terraform,
    NanoForge,
}

/// All relevant information about this planet attachment.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct PlanetAttachmentInfo {
    /// The 'pretty' name of the attachment.
    pub name: String,
    /// The expansion this attachment comes from.
    pub expansion: Expansion,
    /// The planet trait required (if any) for this attachment to be attached to it.
    pub planet_trait: Option<PlanetTrait>,
    /// The amount of resources gained from the attachment.
    pub resources: u32,
    /// The amount of influence gained from the attachment.
    pub influence: u32,
    /// Tech specialty (if any) gained from this attachment.
    pub tech_specialty: Option<TechCategory>,
    /// The planet traits provided from this attachment.
    pub added_planet_traits: Vec<PlanetTrait>,
    /// Weather this attachment makes the planet a legendary planet or not.
    pub set_legendary: bool,
}

macro_rules! a {
    ($name: literal, $expansion: ident, $trait: ident) => {
        PlanetAttachmentInfo {
            name: $name.to_string(),
            expansion: Expansion::$expansion,
            planet_trait: Some(PlanetTrait::$trait),
            resources: 0,
            influence: 0,
            tech_specialty: None,
            added_planet_traits: vec![],
            set_legendary: false,
        }
    };
    ($name: literal, $expansion: ident, $resources: literal, $influence: literal) => {
        PlanetAttachmentInfo {
            name: $name.to_string(),
            expansion: Expansion::$expansion,
            planet_trait: None,
            resources: $resources,
            influence: $influence,
            tech_specialty: None,
            added_planet_traits: vec![],
            set_legendary: false,
        }
    };
    ($name: literal, $expansion: ident, $resources: literal, $influence: literal, $added_planet_traits: expr) => {
        PlanetAttachmentInfo {
            name: $name.to_string(),
            expansion: Expansion::$expansion,
            planet_trait: None,
            resources: $resources,
            influence: $influence,
            tech_specialty: None,
            added_planet_traits: $added_planet_traits,
            set_legendary: false,
        }
    };
    ($name: literal, $expansion: ident, $trait: ident, $resources: literal, $influence: literal) => {
        PlanetAttachmentInfo {
            name: $name.to_string(),
            expansion: Expansion::$expansion,
            planet_trait: Some(PlanetTrait::$trait),
            resources: $resources,
            influence: $influence,
            tech_specialty: None,
            added_planet_traits: vec![],
            set_legendary: false,
        }
    };
    ($name: literal, $expansion: ident, $trait: ident, $resources: literal, $influence: literal, $tech_spec: ident) => {
        PlanetAttachmentInfo {
            name: $name.to_string(),
            expansion: Expansion::$expansion,
            planet_trait: Some(PlanetTrait::$trait),
            resources: $resources,
            influence: $influence,
            tech_specialty: Some(TechCategory::$tech_spec),
            added_planet_traits: vec![],
            set_legendary: false,
        }
    };
}

macro_rules! al {
    ($name: literal, $expansion: ident, $resources: literal, $influence: literal) => {
        PlanetAttachmentInfo {
            name: $name.to_string(),
            expansion: Expansion::$expansion,
            planet_trait: None,
            resources: $resources,
            influence: $influence,
            tech_specialty: None,
            added_planet_traits: vec![],
            set_legendary: true,
        }
    };
}

impl PlanetAttachment {
    /// Returns the [PlanetAttachmentInfo] for this [PlanetAttachment].
    pub fn info(&self) -> PlanetAttachmentInfo {
        match self {
            PlanetAttachment::DemilitarizedZone => {
                a!("Demilitarized Zone", ProphecyOfKings, Cultural)
            }
            PlanetAttachment::DysonSphere => a!("Dyson Sphere", ProphecyOfKings, Cultural, 2, 1),
            PlanetAttachment::ParadiseWorld => {
                a!("Paradise World", ProphecyOfKings, Cultural, 0, 2)
            }
            PlanetAttachment::TombOfEmphidia => {
                a!("Tomb of Emphidia", ProphecyOfKings, Cultural, 0, 1)
            }

            PlanetAttachment::BioticResearchFacility => a!(
                "Biotic Research Facility",
                ProphecyOfKings,
                Industrial,
                0,
                0,
                Biotic
            ),
            PlanetAttachment::BioticResearchFacilityResources => a!(
                "Biotic Research Facility",
                ProphecyOfKings,
                Industrial,
                1,
                1
            ),
            PlanetAttachment::CyberneticResearchFacility => a!(
                "Cybernetic Research Facility",
                ProphecyOfKings,
                Industrial,
                0,
                0,
                Cybernetic
            ),
            PlanetAttachment::CyberneticResearchFacilityResources => a!(
                "Cybernetic Research Facility",
                ProphecyOfKings,
                Industrial,
                1,
                1
            ),
            PlanetAttachment::PropulsionResearchFacility => a!(
                "Propulsion Research Facility",
                ProphecyOfKings,
                Industrial,
                0,
                0,
                Propulsion
            ),
            PlanetAttachment::PropulsionResearchFacilityResources => a!(
                "Propulsion Research Facility",
                ProphecyOfKings,
                Industrial,
                1,
                1
            ),
            PlanetAttachment::LasaxSurvivors => {
                a!("Lasax Survivors", ProphecyOfKings, Hazardous, 1, 2)
            }
            PlanetAttachment::MiningWorld => a!("Mining world", ProphecyOfKings, Hazardous, 2, 0),
            PlanetAttachment::RichWorld => a!("Rich world", ProphecyOfKings, Hazardous, 1, 0),
            // Note, provides resources & influence (1, 1) if the planet it is attached to already have the planet trait.
            PlanetAttachment::WarfareResearchFacility => {
                a!(
                    "Warfare Research Facility",
                    ProphecyOfKings,
                    Hazardous,
                    0,
                    0,
                    Warfare
                )
            }
            PlanetAttachment::WarfareResearchFacilityResources => {
                a!(
                    "Warfare Research Facility",
                    ProphecyOfKings,
                    Hazardous,
                    1,
                    1
                )
            }
            PlanetAttachment::UITheProgenitor => {
                a!("UI, the Progenitor (Geoform)", ProphecyOfKings, 3, 3)
            }
            PlanetAttachment::Terraform => a!(
                "Terraform",
                ProphecyOfKings,
                1,
                1,
                vec![
                    PlanetTrait::Cultural,
                    PlanetTrait::Hazardous,
                    PlanetTrait::Industrial
                ]
            ),
            PlanetAttachment::NanoForge => al!("Nano-Forge", CodexII, 2, 2),
        }
    }

    /// Returns the ''correct'' variant of this attachment for the provided PlanetInfo.
    pub fn match_planet(self, planet: &PlanetInfo) -> Self {
        let handle_alt = |tech_category: TechCategory, a: Self, b: Self| {
            if planet.tech_specialty == Some(tech_category) {
                a
            } else {
                b
            }
        };

        match self {
            PlanetAttachment::BioticResearchFacility => handle_alt(
                TechCategory::Biotic,
                Self::BioticResearchFacilityResources,
                self,
            ),
            PlanetAttachment::CyberneticResearchFacility => handle_alt(
                TechCategory::Cybernetic,
                Self::CyberneticResearchFacilityResources,
                self,
            ),
            PlanetAttachment::PropulsionResearchFacility => handle_alt(
                TechCategory::Propulsion,
                Self::PropulsionResearchFacilityResources,
                self,
            ),
            PlanetAttachment::WarfareResearchFacility => handle_alt(
                TechCategory::Warfare,
                Self::WarfareResearchFacilityResources,
                self,
            ),
            PlanetAttachment::BioticResearchFacilityResources => {
                handle_alt(TechCategory::Biotic, self, Self::BioticResearchFacility)
            }
            PlanetAttachment::CyberneticResearchFacilityResources => handle_alt(
                TechCategory::Cybernetic,
                self,
                Self::CyberneticResearchFacility,
            ),
            PlanetAttachment::PropulsionResearchFacilityResources => handle_alt(
                TechCategory::Propulsion,
                self,
                Self::PropulsionResearchFacility,
            ),
            PlanetAttachment::WarfareResearchFacilityResources => {
                handle_alt(TechCategory::Warfare, self, Self::WarfareResearchFacility)
            }
            _ => self,
        }
    }
}
