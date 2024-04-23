use crate::data::{common::expansions::Expansion, components::phase::Phase};

use super::{ObjectiveInfo, ObjectiveKind};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

macro_rules! s {
    ($phase:expr, $name:literal, $condition: literal, $expansion: expr) => {
        ObjectiveInfo {
            name: $name.into(),
            condition: $condition.into(),
            kind: ObjectiveKind::Secret { phase: $phase },
            points: 1,
            expansion: $expansion,
        }
    };
}

/// A secret objective.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, EnumIter, TS,
)]
#[ts(export)]
#[allow(missing_docs)]
pub enum SecretObjective {
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
impl SecretObjective {
    /// Get the [ObjectiveInfo] for this secret objective.
    pub fn info(&self) -> ObjectiveInfo {
        match self {
            SecretObjective::DestroyTheirGreatestShip => s!(
                Phase::Action,
                "Destroy Their Greatest Ship",
                "Destroy another player's war sun or flagship.", 
                Expansion::Base
            ),
            SecretObjective::MakeAnExampleOfTheirWorld => s!(
                Phase::Action,
                "Make an Example of Their World",
                "Use BOMBARDMENT to destroy the last of a player's ground forces on a planet.", 
                Expansion::Base
            ),
            SecretObjective::SparkARebellion => s!(
                Phase::Action,
                "Spark a Rebellion",
                "Win a combat against a player who has the most victory points.", 
                Expansion::Base
            ),
            SecretObjective::TurnTheirFleetsToDust => s!(
                Phase::Action,
                "Turn Their Fleets to Dust",
                "Use SPACE CANNON to destroy the last of a player's ships in a system.", 
                Expansion::Base
            ),
            SecretObjective::UnveilFlagship => s!(
                Phase::Action,
                "Unveil Flagship",
                "Win a space combat in a system that contains your flagship. You cannot score this objective if your flagship is destroyed in the combat.", 
                Expansion::Base
            ),
            SecretObjective::BecomeAMartyr => s!(
                Phase::Action,
                "Become a Martyr",
                "Lose control of a planet in a home system.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::BetrayAFriend => s!(
                Phase::Action,
                "Betray a Friend",
                "Win a combat against a player whose promissory note you had in your play area at the start of your tactical action.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::BraveTheVoid => s!(
                Phase::Action,
                "Brave the Void",
                "Win a combat in an anomaly.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::DarkenTheSkies => s!(
                Phase::Action,
                "Darken the Skies",
                "Win a combat in another player's home system.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::DemonstrateYourPower => s!(
                Phase::Action,
                "Demonstrate Your Power",
                "Have 3 or more non-fighter ships in the active system at the end of a space combat.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::FightWithPrecision => s!(
                Phase::Action,
                "Fight With Precision",
                "Use ANTI-FIGHTER BARRAGE to destroy the last of a player's fighters in a system.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::ProveEndurance => s!(
                Phase::Action,
                "Prove Endurance",
                "Be the last player to pass during a game round.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::AdaptNewStrategies => s!(
                Phase::Status,
                "Adapt New Strategies",
                "Own 2 faction technologies. (Valefar Assimilator technologies do not count toward this objective.)", 
                Expansion::Base
            ),
            SecretObjective::BecomeTheGatekeeper => s!(
                Phase::Status,
                "Become the Gatekeeper",
                "Have 1 or more ships in a system that contains an alpha wormhole and 1 or more ships in a system that contains a beta wormhole.", 
                Expansion::Base
            ),
            SecretObjective::ControlTheRegion => s!(
                Phase::Status,
                "Control the Region",
                "Have 1 or more ships in 6 systems.", 
                Expansion::Base
            ),
            SecretObjective::CutSupplyLines => s!(
                Phase::Status,
                "Cut Supply Lines",
                "Have 1 or more ships in the same system as another player's space dock.", 
                Expansion::Base
            ),
            SecretObjective::EstablishAPerimiter => s!(
                Phase::Status,
                "Establish a Perimeter",
                "Have 4 PDS units on the game board.", 
                Expansion::Base
            ),
            SecretObjective::ForgeAnAlliance => s!(
                Phase::Status,
                "Forge an Alliance",
                "Control 4 cultural planets.", 
                Expansion::Base
            ),
            SecretObjective::FormASpyNetwork => s!(
                Phase::Status,
                "Form a Spy Network",
                "Discard 5 Action Cards.", 
                Expansion::Base
            ),
            SecretObjective::FuelTheWarMachine => s!(
                Phase::Status,
                "Fuel the War Machine",
                "Have 3 space docks on the game board.", 
                Expansion::Base
            ),
            SecretObjective::GatherAMightyFleet => s!(
                Phase::Status,
                "Gather a Mighty Fleet",
                "Have 5 dreadnoughts on the game board.", 
                Expansion::Base
            ),
            SecretObjective::LearnTheSecretsOfTheCosmos => s!(
                Phase::Status,
                "Learn the Secrets of the Cosmos",
                "Have 1 or more ships in 3 systems that are each adjacent to an anomaly.", 
                Expansion::Base
            ),
            SecretObjective::MasterTheLawsOfPhysics => s!(
                Phase::Status,
                "Master the Laws of Physics",
                "Own 4 technologies of the same color.", 
                Expansion::Base
            ),
            SecretObjective::MineRateMetals => s!(
                Phase::Status,
                "Mine Rare Metals",
                "Control 4 hazardous planets.", 
                Expansion::Base
            ),
            SecretObjective::MonopolizeProduction => s!(
                Phase::Status,
                "Monopolize Production",
                "Control 4 industrial planets.", 
                Expansion::Base
            ),
            SecretObjective::OccupyTheSeatOfTheEmpire => s!(
                Phase::Status,
                "Occupy the Seat of the Empire",
                "Control Mecatol Rex and have 3 or more ships in its system.", 
                Expansion::Base
            ),
            SecretObjective::ThreatenEnemies => s!(
                Phase::Status,
                "Threaten Enemies",
                "Have 1 or more ships in a system that is adjacent to another player's home system.", 
                Expansion::Base
            ),
            SecretObjective::DefySpaceAndTime => s!(
                Phase::Status,
                "Defy Space and Time",
                "Have units in the wormhole nexus.",
                Expansion::ProphecyOfKings
            ),
            SecretObjective::DestroyHereticalWorks => s!(
                Phase::Status,
                "Destroy Heretical Works",
                "Purge 2 of your relic fragments of any type.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::EstablishHegemony => s!(
                Phase::Status,
                "Establish Hegemony",
                "Control planets that have a combined influence value of at least 12.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::FosterCohesion => s!(
                Phase::Status,
                "Foster Cohesion",
                "Be neighbors with all other players.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::HoardRawMaterials => s!(
                Phase::Status,
                "Hoard Raw Materials",
                "Control planets that have a combined resource value of at least 12.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::MechanizeTheMilitary => s!(
                Phase::Status,
                "Mechanize The Military",
                "Have 1 mech on each of 4 planets.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::OccupyTheFringe => s!(
                Phase::Status,
                "Occupy The Fringe",
                "Have 9 or more ground forces on a planet that does not contain 1 of your space docks.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::ProduceEnMasse => s!(
                Phase::Status,
                "Produce En Masse",
                "Have units with a combined PRODUCTION value of at least 8 in a single system.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::SeizeAnIcon => s!(
                Phase::Status,
                "Seize An Icon",
                "Control a legendary planet.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::StakeYourClaim => s!(
                Phase::Status,
                "Stake your Claim",
                "Control a planet in a system that contains a planet controlled by another player.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::StrengthenBonds => s!(
                Phase::Status,
                "Strengthen Bonds",
                "Have another player's promissory note in your play area.", 
                Expansion::ProphecyOfKings
            ),
            SecretObjective::DictatePolicy => s!(
                Phase::Agenda,
                "Dictate Policy",
                "There are 3 or more laws in play.", 
                Expansion::Base
            ),
            SecretObjective::DriveTheDebate => s!(
                Phase::Agenda,
                "Drive the Debate",
                "You or a planet you control are elected by an agenda.", 
                Expansion::Base
            ),
        }
    }
}
