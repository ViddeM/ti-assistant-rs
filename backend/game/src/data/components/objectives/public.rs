use super::{ObjectiveInfo, ObjectiveKind};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

macro_rules! o {
    ($stage:ident, $name:literal, $condition: literal) => {
        ObjectiveInfo {
            name: $name.into(),
            condition: $condition.into(),
            kind: ObjectiveKind::$stage,
            points: match ObjectiveKind::$stage {
                ObjectiveKind::StageII => 2,
                _ => 1,
            },
        }
    };
}

/// A public objective.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[allow(missing_docs)]
pub enum PublicObjective {
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

impl PublicObjective {
    /// Get the [ObjectiveInfo] for this public objective.
    pub fn get_objective_info(&self) -> ObjectiveInfo {
        match self {
            PublicObjective::CornerTheMarket => o!(
                StageI,
                "Corner the Market",
                "Control 4 planets that each have the same planet trait."
            ),
            PublicObjective::DevelopWeaponry => {
                o!(StageI, "Develop Weaponry", "Own 2 unit upgrade technologies.")
            }
            PublicObjective::DiversifyResearch => {
                o!(
                    StageI,
                    "Diversify Research",
                    "Own 2 technologies in each of 2 colors."
                )
            }
            PublicObjective::ErectAMonument => {
                o!(StageI, "Erect a Monument", "Spend 8 resources.")
            }
            PublicObjective::ExpandBorders => {
                o!(
                    StageI,
                    "Expand Borders",
                    "Control 6 planets in non-home systems."
                )
            }
            PublicObjective::FoundResearchOutposts => o!(
                StageI,
                "Found Research Outposts",
                "Control 3 planets that have technology specialties."
            ),
            PublicObjective::IntimidateCouncil => {
                o!(
                    StageI,
                    "Intimidate Council",
                    "Have 1 or more ships in 2 systems that are adjacent to Mectrol Rex's System."
                )
            }
            PublicObjective::LeadFromTheFront => o!(
                StageI,
                "Lead from the Front",
                "Spend a total of 3 tokens from your tactic and/or strategy pools."
            ),
            PublicObjective::NegotiateTradeRoutes => {
                o!(StageI, "Negotiate Trade Routes", "Spend 5 trade goods.")
            }
            PublicObjective::SwayTheCouncil => o!(StageI, "Sway the Council", "Spend 8 influence."),
            PublicObjective::AmassWealth => o!(
                StageI,
                "Amass wealth",
                "Spend 3 influence, 3 resources, and 3 trade goods."
            ),
            PublicObjective::BuildDefenses => {
                o!(StageI, "Build Defenses", "Have 4 or more structures.")
            }
            PublicObjective::DiscoverLostOutposts => o!(
                StageI,
                "Discover Lost Outposts",
                "Control 2 planets that have attachments."
            ),
            PublicObjective::EngineerAMarvel => o!(
                StageI,
                "Engineer a Marvel",
                "Have your flagship or a war sun on the game board."
            ),
            PublicObjective::ExploreDeepSpace => o!(
                StageI,
                "Explore Deep Space",
                "Have units in 3 systems that do not contain planets."
            ),
            PublicObjective::ImproveInfrastructure => o!(
                StageI,
                "Improve Infrastructure",
                "Have structures on 3 planets outside of your home system."
            ),
            PublicObjective::MakeHistory => o!(
                StageI, "Make History",
                "Have units in 2 systems that contain legendary planets, Mecatol Rex, or anomalies."
            ),
            PublicObjective::PopulateTheOuterRim => o!(
                StageI,
                "Populate the Outer Rim",
                "Have units in 3 systems on the edge of the game board other than your home system."
            ),
            PublicObjective::PushBoundaries => o!(
                StageI,
                "Push Boundaries",
                "Control more planets than each of 2 of your neighbors."
            ),
            PublicObjective::RaiseAFleet => o!(
                StageI,
                "Raise a Fleet",
                "Have 5 or more non-fighter ships in 1 system."
            ),
            PublicObjective::CentralizeGalacticTrade => {
                o!(StageII, "Centralize Galactic Trade", "Spend 10 trade goods.")
            }
            PublicObjective::ConquerTheWeak => o!(
                StageII,
                "Conquer the Weak",
                "Control 1 planet that is in another player's home system."
            ),
            PublicObjective::FormGalacticBrainTrust => o!(
                StageII,
                "Form Galactic Brain Trust",
                "Control 5 planets that have technology specialties."
            ),
            PublicObjective::FoundAGoldenAge => o!(StageII, "Found a Golden Age", "Spend 16 resources."),
            PublicObjective::GalvanizeThePeople => o!(
                StageII,
                "Galvanize the People",
                "Spend a total of 6 tokens from your tactic and/or strategy pools."
            ),
            PublicObjective::ManipulateGalacticLaw => {
                o!(StageII, "Manipulate Galactic Law", "Spend 16 influence")
            }
            PublicObjective::MasterTheSciences => o!(
                StageII,
                "Master the Sciences",
                "Own 2 technologies in each of 4 colors."
            ),
            PublicObjective::RevolutionizeWarfare => o!(
                StageII,
                "Revolutionize Warfare",
                "Own 3 unit upgrade technologies."
            ),
            PublicObjective::SubdueTheGalaxy => o!(
                StageII,
                "Subdue the Galaxy",
                "Control 11 planets in non-home systems."
            ),
            PublicObjective::UnifyTheColonies => o!(
                StageII,
                "Unify the Colonies",
                "Control 6 planets that each have the same planet trait."
            ),
            PublicObjective::AchieveSupremacy => o!(
                StageII,
                "Achieve Supremacy",
                "Have your flagship or a war sun in another player's home system or the Mecatol Rex system."
            ),
            PublicObjective::BecomeALegend => o!(
                StageII,
                "Become a Legend",
                "Have units in 4 systems that contain legendary planets, Mecatol Rex, or anomalies."
            ),
            PublicObjective::CommandAnArmada => o!(
                StageII,
                "Command an Armada", "Have 8 or more non-fighter ships in 1 system."
            ),
            PublicObjective::ConstructMassiveCities => o!(
                StageII,
                "Construct Massive Cities",
                "Have 7 or more structures."
            ),
            PublicObjective::ControlTheBorderlands => o!(
                StageII,
                "Control the Borderlands",
                "Have units in 5 systems on the edge of the game board other than your home system."
            ),
            PublicObjective::HoldVastReserves => o!(
                StageII,
                "Hold Vast Reserves",
                "Spend 6 influence, 6 resources, and 6 trade goods."
            ),
            PublicObjective::PatrolVastTerritories => o!(
                StageII,
                "Patrol Vast Territories",
                "Have units in 5 systems that do not contain planets."
            ),
            PublicObjective::ProtectTheBorder => o!(
                StageII,
                "Protect the Border",
                "Have structures on 5 planets outside of your home system."
            ),
            PublicObjective::ReclaimAncientMonuments => o!(
                StageII,
                "Reclaim Ancient Monuments",
                "Control 3 planets that have attachments."
            ),
            PublicObjective::RuleDistantLands => o!(
                StageII,
                "Rule Distant Lands",
                "Control 2 planets that are each in or adjacent to a different, other player's home system."
            ),
        }
    }
}
