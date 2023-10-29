use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::phases::Phase;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretObjectiveInfo {
    pub phase: Phase,
    pub name: String,
    pub condition: String,
}

macro_rules! s {
    ($phase:expr, $name:literal, $condition: literal) => {
        SecretObjectiveInfo {
            phase: $phase,
            name: $name.into(),
            condition: $condition.into(),
        }
    };
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumIter)]
pub enum SecretObjectives {
    // Action phase base cards
    DestroyTheirGreatestShip,
    MakeAnExampleOfTheirWorld,
    SparkARebellion,
    TurnTheirFleetsToDust,
    UnveilFlagship,
    // Action phase PoK cards
    BecomeAMartyr,
    BetrayAFriend,
    BraveTheVoid,
    DarkenTheSkies,
    DemonstrateYourPower,
    FightWithPrecision,
    ProveEndurance,
    // Status phase base cards
    AdaptNewStrategies,
    BecomeTheGatekeeper,
    ControlTheRegion,
    CutSupplyLines,
    EstablishAPerimiter,
    ForgeAnAlliance,
    FormASpyNetwork,
    FuelTheWarMachine,
    GatherAMightyFleet,
    LearnTheSecretsOfTheCosmos,
    MasterTheLawsOfPhysics,
    MineRateMetals,
    MonopolizeProduction,
    OccupyTheSeatOfTheEmpire,
    ThreatenEnemies,
    // Status phase PoK cards
    DefySpaceAndTime,
    DestroyHereticalWorks,
    EstablishHegemony,
    FosterCohesion,
    HoardRawMaterials,
    MechanizeTheMilitary,
    OccupyTheFringe,
    ProduceEnMasse,
    SeizeAnIcon,
    StakeYourClaim,
    StrengthenBonds,
    // Agenda phase cards
    DictatePolicy,
    DriveTheDebate,
}

// TODO: Look over codex updates for these.
impl SecretObjectives {
    pub fn get_objective_info(&self) -> SecretObjectiveInfo {
        match self {
            SecretObjectives::DestroyTheirGreatestShip => s!(
                Phase::Action,
                "Destroy Their Greatest Ship",
                "Destroy another player's war sun or flagship."
            ),
            SecretObjectives::MakeAnExampleOfTheirWorld => s!(
                Phase::Action,
                "Make an Example of Their World",
                "Use BOMBARDMENT to destroy the last of a player's ground forces on a planet."
            ),
            SecretObjectives::SparkARebellion => s!(
                Phase::Action,
                "Spark a Rebellion",
                "Win a combat against a player who has the most victory points."
            ),
            SecretObjectives::TurnTheirFleetsToDust => s!(
                Phase::Action,
                "Turn Their Fleets to Dust",
                "Use SPACE CANNON to destroy the last of a player's ships in a system."
            ),
            SecretObjectives::UnveilFlagship => s!(
                Phase::Action,
                "Unveil Flagship",
                "Win a space combat in a system that contains your flagship. You cannot score this objective if your flagship is destroyed in the combat."
            ),
            SecretObjectives::BecomeAMartyr => s!(
                Phase::Action,
                "Become a Martyr",
                "Lose control of a planet in a home system."
            ),
            SecretObjectives::BetrayAFriend => s!(
                Phase::Action,
                "Betray a Friend",
                "Win a combat against a player whose promissory note you had in your play area at the start of your tactical action."
            ),
            SecretObjectives::BraveTheVoid => s!(
                Phase::Action,
                "Brave the Void",
                "Win a combat in an anomaly."
            ),
            SecretObjectives::DarkenTheSkies => s!(
                Phase::Action,
                "Darken the Skies",
                "Win a combat in another player's home system."
            ),
            SecretObjectives::DemonstrateYourPower => s!(
                Phase::Action,
                "Demonstrate Your Power",
                "Have 3 or more non-fighter ships in the active system at the end of a space combat."
            ),
            SecretObjectives::FightWithPrecision => s!(
                Phase::Action,
                "Fight With Precision",
                "Use ANTI-FIGHTER BARRAGE to destroy the last of a player's fighters in a system."
            ),
            SecretObjectives::ProveEndurance => s!(
                Phase::Action,
                "Prove Endurance",
                "Be the last player to pass during a game round."
            ),
            SecretObjectives::AdaptNewStrategies => s!(
                Phase::Status,
                "Adapt New Strategies",
                "Own 2 faction technologies. (Valefar Assimilator technologies do not count toward this objective.)"
            ),
            SecretObjectives::BecomeTheGatekeeper => s!(
                Phase::Status,
                "Become the Gatekeeper",
                "Have 1 or more ships in a system that contains an alpha wormhole and 1 or more ships in a system that contains a beta wormhole."
            ),
            SecretObjectives::ControlTheRegion => s!(
                Phase::Status,
                "Control the Region",
                "Have 1 or more ships in 6 systems."
            ),
            SecretObjectives::CutSupplyLines => s!(
                Phase::Status,
                "Cut Supply Lines",
                "Have 1 or more ships in the same system as another player's space dock."
            ),
            SecretObjectives::EstablishAPerimiter => s!(
                Phase::Status,
                "Establish a Perimeter",
                "Have 4 PDS units on the game board."
            ),
            SecretObjectives::ForgeAnAlliance => s!(
                Phase::Status,
                "Forge an Alliance",
                "Control 4 cultural planets."
            ),
            SecretObjectives::FormASpyNetwork => s!(
                Phase::Status,
                "Form a Spy Network",
                "Discard 5 Action Cards."
            ),
            SecretObjectives::FuelTheWarMachine => s!(
                Phase::Status,
                "Fuel the War Machine",
                "Have 3 space docks on the game board."
            ),
            SecretObjectives::GatherAMightyFleet => s!(
                Phase::Status,
                "Gather a Mighty Fleet",
                "Have 5 dreadnoughts on the game board."
            ),
            SecretObjectives::LearnTheSecretsOfTheCosmos => s!(
                Phase::Status,
                "Learn the Secrets of the Cosmos",
                "Have 1 or more ships in 3 systems that are each adjacent to an anomaly."
            ),
            SecretObjectives::MasterTheLawsOfPhysics => s!(
                Phase::Status,
                "Master the Laws of Physics",
                "Own 4 technologies of the same color."
            ),
            SecretObjectives::MineRateMetals => s!(
                Phase::Status,
                "Mine Rare Metals",
                "Control 4 hazardous planets."
            ),
            SecretObjectives::MonopolizeProduction => s!(
                Phase::Status,
                "Monopolize Production",
                "Control 4 industrial planets."
            ),
            SecretObjectives::OccupyTheSeatOfTheEmpire => s!(
                Phase::Status,
                "Occupy the Seat of the Empire",
                "Control Mecatol Rex and have 3 or more ships in its system."
            ),
            SecretObjectives::ThreatenEnemies => s!(
                Phase::Status,
                "Threaten Enemies",
                "Have 1 or more ships in a system that is adjacent to another player's home system."
            ),
            SecretObjectives::DefySpaceAndTime => s!(
                Phase::Status,
                "Defy Space and Time",
                "Have units in the wormhole nexus."  
            ),
            SecretObjectives::DestroyHereticalWorks => s!(
                Phase::Status,
                "Destroy Heretical Works",
                "Purge 2 of your relic fragments of any type."
            ),
            SecretObjectives::EstablishHegemony => s!(
                Phase::Status,
                "Establish Hegemony",
                "Control planets that have a combined influence value of at least 12."
            ),
            SecretObjectives::FosterCohesion => s!(
                Phase::Status,
                "Foster Cohesion",
                "Be neighbors with all other players."
            ),
            SecretObjectives::HoardRawMaterials => s!(
                Phase::Status,
                "Hoard Raw Materials",
                "Control planets that have a combined resource value of at least 12."
            ),
            SecretObjectives::MechanizeTheMilitary => s!(
                Phase::Status,
                "Mechanize The Military",
                "Have 1 mech on each of 4 planets."
            ),
            SecretObjectives::OccupyTheFringe => s!(
                Phase::Status,
                "Occupy The Fringe",
                "Have 9 or more ground forces on a planet that does not contain 1 of your space docks."
            ),
            SecretObjectives::ProduceEnMasse => s!(
                Phase::Status,
                "Produce En Masse",
                "Have units with a combined PRODUCTION value of at least 8 in a single system."
            ),
            SecretObjectives::SeizeAnIcon => s!(
                Phase::Status,
                "Seize An Icon",
                "Control a legendary planet."
            ),
            SecretObjectives::StakeYourClaim => s!(
                Phase::Status,
                "Stake your Claim",
                "Control a planet in a system that contains a planet controlled by another player."
            ),
            SecretObjectives::StrengthenBonds => s!(
                Phase::Status,
                "Strengthen Bonds",
                "Have another player's promissory note in your play area."
            ),
            SecretObjectives::DictatePolicy => s!(
                Phase::Agenda,
                "Dictate Policy",
                "There are 3 or more laws in play."
            ),
            SecretObjectives::DriveTheDebate => s!(
                Phase::Agenda,
                "Drive the Debate",
                "You or a planet you control are elected by an agenda."
            ),
        }
    }
}
