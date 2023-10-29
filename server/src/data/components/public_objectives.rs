use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicObjectiveInfo {
    pub points: u8,
    pub name: String,
    pub condition: String,
}

macro_rules! o {
    ($points:literal, $name:literal, $condition: literal) => {
        PublicObjectiveInfo {
            points: $points,
            name: $name.into(),
            condition: $condition.into(),
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter)]
pub enum PublicObjectives {
    // Base 1 point objectives
    CornerTheMarket,
    DevelopWeaponry,
    DiversifyResearch,
    ErectAMonument,
    ExpandBorders,
    FoundResearchOutposts,
    IntimidateCouncil,
    LeadFromTheFront,
    NegotiateTradeRoutes,
    SwayTheCouncil,

    // PoK 1 point objectives
    AmassWealth,
    BuildDefenses,
    DiscoverLostOutposts,
    EngineerAMarvel,
    ExploreDeepSpace,
    ImproveInfrastructure,
    MakeHistory,
    PopulateTheOuterRim,
    PushBoundaries,
    RaiseAFleet,

    // Base 2 point objectives
    CentralizeGalacticTrade,
    ConquerTheWeak,
    FormGalacticBrainTrust,
    FoundAGoldenAge,
    GalvanizeThePeople,
    ManipulateGalacticLaw,
    MasterTheSciences,
    RevolutionizeWarfare,
    SubdueTheGalaxy,
    UnifyTheColonies,

    // PoK 2 point objectives
    AchieveSupremacy,
    BecomeALegend,
    CommandAnArmada,
    ConstructMassiveCities,
    ControlTheBorderlands,
    HoldVastReserves,
    PatrolVastTerritories,
    ProtectTheBorder,
    ReclaimAncientMonuments,
    RuleDistantLands,
}

impl PublicObjectives {
    pub fn get_objective_info(&self) -> PublicObjectiveInfo {
        match self {
            PublicObjectives::CornerTheMarket => o!(
                1,
                "Corner the Market",
                "Control 4 planets that each have the same planet trait."
            ),
            PublicObjectives::DevelopWeaponry => {
                o!(1, "Develop Weaponry", "Own 2 unit upgrade technologies.")
            }
            PublicObjectives::DiversifyResearch => {
                o!(
                    1,
                    "Diversify Research",
                    "Own 2 technologies in each of 2 colors."
                )
            }
            PublicObjectives::ErectAMonument => {
                o!(1, "Erect a Monument", "Spend 8 resources.")
            }
            PublicObjectives::ExpandBorders => {
                o!(
                    1,
                    "Expand Borders",
                    "Control 6 planets in non-home systems."
                )
            }
            PublicObjectives::FoundResearchOutposts => o!(
                1,
                "Found Research Outposts",
                "Control 3 planets that have technology specialties."
            ),
            PublicObjectives::IntimidateCouncil => {
                o!(
                    1,
                    "Intimidate Council",
                    "Have 1 or more ships in 2 systems that are adjacent to Mectrol Rex's System."
                )
            }
            PublicObjectives::LeadFromTheFront => o!(
                1,
                "Lead from the Front",
                "Spend a total of 3 tokens from your tactic and/or strategy pools."
            ),
            PublicObjectives::NegotiateTradeRoutes => {
                o!(1, "Negotiate Trade Routes", "Spend 5 trade goods.")
            }
            PublicObjectives::SwayTheCouncil => o!(1, "Sway the Council", "Spend 8 influence."),
            PublicObjectives::AmassWealth => o!(
                1,
                "Amass wealth",
                "Spend 3 influence, 3 resources, and 3 trade goods."
            ),
            PublicObjectives::BuildDefenses => {
                o!(1, "Build Defenses", "Have 4 or more structures.")
            }
            PublicObjectives::DiscoverLostOutposts => o!(
                1,
                "Discover Lost Outposts",
                "Control 2 planets that have attachments."
            ),
            PublicObjectives::EngineerAMarvel => o!(
                1,
                "Engineer a Marvel",
                "Have your flagship or a war sun on the game board."
            ),
            PublicObjectives::ExploreDeepSpace => o!(
                1,
                "Explore Deep Space",
                "Have units in 3 systems that do not contain planets."
            ),
            PublicObjectives::ImproveInfrastructure => o!(
                1,
                "Improve Infrastructure",
                "Have structures on 3 planets outside of your home system."
            ),
            PublicObjectives::MakeHistory => o!(
                1, "Make History",
                "Have units in 2 systems that contain legendary planets, Mecatol Rex, or anomalies."
            ),
            PublicObjectives::PopulateTheOuterRim => o!(
                1,
                "Populate the Outer Rim",
                "Have units in 3 systems on the edge of the game board other than your home system."
            ),
            PublicObjectives::PushBoundaries => o!(
                1,
                "Push Boundaries",
                "Control more planets than each of 2 of your neighbors."
            ),
            PublicObjectives::RaiseAFleet => o!(
                1,
                "Raise a Fleet",
                "Have 5 or more non-fighter ships in 1 system."
            ),
            PublicObjectives::CentralizeGalacticTrade => {
                o!(2, "Centralize Galactic Trade", "Spend 10 trade goods.")
            }
            PublicObjectives::ConquerTheWeak => o!(
                2,
                "Conquer the Weak",
                "Control 1 planet that is in another player's home system."
            ),
            PublicObjectives::FormGalacticBrainTrust => o!(
                2,
                "Form Galactic Brain Trust",
                "Control 5 planets that have technology specialties."
            ),
            PublicObjectives::FoundAGoldenAge => o!(2, "Found a Golden Age", "Spend 16 resources."),
            PublicObjectives::GalvanizeThePeople => o!(
                2,
                "Galvanize the People",
                "Spend a total of 6 tokens from your tactic and/or strategy pools."
            ),
            PublicObjectives::ManipulateGalacticLaw => {
                o!(2, "Manipulate Galactic Law", "Spend 16 influence")
            }
            PublicObjectives::MasterTheSciences => o!(
                2,
                "Master the Sciences",
                "Own 2 technologies in each of 4 colors."
            ),
            PublicObjectives::RevolutionizeWarfare => o!(
                2,
                "Revolutionize Warfare",
                "Own 3 unit upgrade technologies."
            ),
            PublicObjectives::SubdueTheGalaxy => o!(
                2,
                "Subdue the Galaxy",
                "Control 11 planets in non-home systems."
            ),
            PublicObjectives::UnifyTheColonies => o!(
                2,
                "Unify the Colonies",
                "Control 6 planets that each have the same planet trait."
            ),
            PublicObjectives::AchieveSupremacy => o!(
                2,
                "Achieve Supremacy",
                "Have your flagship or a war sun in another player's home system or the Mecatol Rex system."
            ),
            PublicObjectives::BecomeALegend => o!(
                2,
                "Become a Legend",
                "Have units in 4 systems that contain legendary planets, Mecatol Rex, or anomalies."
            ),
            PublicObjectives::CommandAnArmada => o!(
                2,
                "Command an Armada", "Have 8 or more non-fighter ships in 1 system."
            ),
            PublicObjectives::ConstructMassiveCities => o!(
                2,
                "Construct Massive Cities",
                "Have 7 or more structures."
            ),
            PublicObjectives::ControlTheBorderlands => o!(
                2,
                "Control the Borderlands",
                "Have units in 5 systems on the edge of the game board other than your home system."
            ),
            PublicObjectives::HoldVastReserves => o!(
                2,
                "Hold Vast Reserves",
                "Spend 6 influence, 6 resources, and 6 trade goods."
            ),
            PublicObjectives::PatrolVastTerritories => o!(
                2,
                "Patrol Vast Territories",
                "Have units in 5 systems that do not contain planets."
            ),
            PublicObjectives::ProtectTheBorder => o!(
                2,
                "Protect the Border",
                "Have structures on 5 planets outside of your home system."
            ),
            PublicObjectives::ReclaimAncientMonuments => o!(
                2,
                "Reclaim Ancient Monuments",
                "Control 3 planets that have attachments."
            ),
            PublicObjectives::RuleDistantLands => o!(
                2,
                "Rule Distant Lands",
                "Control 2 planets that are each in or adjacent to a different, other player's home system."
            ),
        }
    }
}
