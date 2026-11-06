use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

use crate::common::{
    component::{Component, replacements},
    expansions::Expansion,
    faction::Faction,
};

/// What category the tech belongs to.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Display,
)]
#[allow(missing_docs)]
pub enum TechCategory {
    Biotic,
    Propulsion,
    Cybernetic,
    Warfare,
}

/// What type of tech this is.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TechType {
    Category(TechCategory),
    UnitUpgrade,
    Colorless,
}

impl TechType {
    pub fn to_type_name(&self) -> String {
        match self {
            TechType::Category(tech_category) => tech_category.to_string().to_lowercase(),
            TechType::UnitUpgrade => "unit-upgrade".to_string(),
            TechType::Colorless => "colorless".to_string(),
        }
    }

    pub fn to_display_name(&self) -> String {
        match self {
            TechType::Category(tech_category) => tech_category.to_string(),
            TechType::UnitUpgrade => String::from("Unit Upgrade"),
            TechType::Colorless => String::from("Colorless"),
        }
    }
}

impl Display for TechType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TechType::UnitUpgrade => "Unit Upgrade".to_string(),
                TechType::Colorless => "Technology".to_string(),
                TechType::Category(tech_category) => format!("Technology, {}", tech_category),
            }
        )
    }
}

/// Weather the game is general or faction specific.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TechOrigin {
    Base,
    Faction(Faction),
}

/// Technologies in the game.
#[derive(
    Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, EnumString, Display,
)]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
pub enum Technology {
    // Biotic
    NeuralMotivator,
    Psychoarchaeology,
    DacxiveAnimators,
    BioStims,
    HyperMetabolism,
    X89BacterialWeapon,
    X89BacterialWeaponOmega,
    X89BacterialWeaponOmegaOmega,
    // Propulsion
    AntimassDeflectors,
    DarkEnergyTap,
    GravityDrive,
    SlingRelay,
    FleetLogistics,
    LightWaveDeflector,
    // Cybernetic
    SarweenTools,
    ScanlinkDroneNetwork,
    GravitonLaserSystem,
    PredictiveIntelligence,
    TransitDiodes,
    IntegratedEconomy,
    // Warfare
    PlasmaScoring,
    AiDevelopmentAlgorithm,
    MagenDefenceGrid,
    MagenDefenceGridOmega,
    MagenDefenceGridOmegaOmega,
    SelfAssemblyRoutines,
    DuraniumArmor,
    AssaultCannon,
    // Unit upgrades
    InfantryII,
    DreadnoughtII,
    CarrierII,
    CruiserII,
    SpaceDockII,
    WarSun,
    DestroyerII,
    FighterII,
    PdsII,
    // Faction techs - upgrades
    SpecOpsII,
    LetaniWarriorII,
    SaturnEngineII,
    SuperDreadnoughtII,
    ExotriremeII,
    AdvancedCarrierII,
    CrimsonLegionnaireII,
    FloatingFactoryII,
    DimensionalTearII,
    MemoriaII,
    PrototypeWarSunII,
    StrikeWingAlphaII,
    HybridCrystalFighterII,
    HelTitanII,
    HeliosV2,
    LinkshipII,
    ExileII,
    // Faction techs - Biotic
    Voidwatch,
    InstinctTraining,
    TransparasteelPlating,
    GeneticRecombination,
    Bioplasmosis,
    ProductionBiomes,
    YinSpinner,
    YinSpinnerOmega,
    Neuroglaive,
    PreFabArcologies,
    MageonImplants,
    RadicalAdvancement,
    HydrothermalMining,
    NeuralParasite,
    // Faction techs - Propulsion
    ChaosMapping,
    SpacialConduitCylinder,
    Aetherstream,
    WormholeGenerator,
    WormholeGeneratorOmega,
    LazaxGateFolding,
    // Faction techs - Cybernetic
    AerieHololattice,
    L4Disruptors,
    TemporalCommandSuite,
    IIHQModernization,
    ExecutiveOrder,
    SalvageOperations,
    InheritanceSystems,
    EResSiphons,
    HegemonicTradePolicy,
    NullificationField,
    CoreImpulse,
    AgencySupplyNetwork,
    AgencySupplyNetworkOmega,
    QuantumDatahubNode,
    MirrorComputing,
    SubatomicSplicer,
    PlaneSplitter,
    // Faction techs - Warfare
    DimensionalSplicer,
    Supercharge,
    Vortex,
    NonEuclidianShielding,
    MagmusReactor,
    MagmusReactorOmega,
    ValkyrieParticleWeave,
    ProximaTargetingVI,
    Nanomachines,
    // Faction techs - Colorless
    ValefarAssimilatorX,
    ValefarAssimilatorY,
}

impl PartialOrd for Technology {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Technology {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.info()
            .name
            .to_lowercase()
            .cmp(&other.info().name.to_lowercase())
    }
}

/// All relevant information about a tech.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TechInfo {
    /// The name of the tech in 'pretty' format.
    pub name: String,
    /// What type of tech this is.
    pub tech_type: TechType,
    /// Weather the tech is general or belongs to a faction.
    pub origin: TechOrigin,
    /// What requirements there are for the technology.
    pub requirements: HashMap<TechCategory, u32>,
    /// Which expansion this tech belongs to.
    pub expansion: Expansion,
    /// The effects of the technology. Each element corresponds to an effect of the technology.
    pub effects: Vec<String>,
}

macro_rules! t {
    ($name: expr, $t:expr, $orig:expr, $reqs: expr, $expansion: expr,) => {
        TechInfo {
            name: $name.to_string(),
            tech_type: $t,
            origin: $orig,
            requirements: $reqs,
            expansion: $expansion,
            effects: Vec::new(),
        }
    };
    ($name: expr, $t:expr, $orig:expr, $reqs: expr, $expansion: expr, $effects: expr,) => {
        TechInfo {
            name: $name.to_string(),
            tech_type: $t,
            origin: $orig,
            requirements: $reqs,
            expansion: $expansion,
            effects: $effects.into_iter().map(|s| s.to_string()).collect(),
        }
    };
}

macro_rules! tr {
    () => {
        Default::default()
    };
    ($e:expr) => {
        From::from($e)
    };
}

impl Component for Technology {
    replacements! {
        MagenDefenceGrid -> MagenDefenceGridOmega -> MagenDefenceGridOmegaOmega,
        X89BacterialWeapon -> X89BacterialWeaponOmega -> X89BacterialWeaponOmegaOmega,
        MagmusReactor -> MagmusReactorOmega,
        WormholeGenerator -> WormholeGeneratorOmega,
        YinSpinner -> YinSpinnerOmega,
        IIHQModernization -> ExecutiveOrder,
        AgencySupplyNetwork -> AgencySupplyNetworkOmega,
    }

    fn expansion(&self) -> Expansion {
        self.info().expansion
    }
}

impl Technology {
    /// Returns the [TechInfo] for this technology.
    pub fn info(&self) -> TechInfo {
        match self {
            Technology::NeuralMotivator => t!(
                "Neural Motivator",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!(),
                Expansion::Base,
                &["During the status phase, draw 2 action cards instead of 1."],
            ),
            Technology::Psychoarchaeology => t!(
                "Psychoarchaeology",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!(),
                Expansion::ProphecyOfKings,
                &[
                    "You can use technology specialties on planets you control without exhausting them, even if those planets are exhausted.",
                    "During the Action Phase, you can exhaust planets you control that have technology specialties to gain 1 Trade Good.",
                ],
            ),
            Technology::DacxiveAnimators => t!(
                "Dacxive Animators",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::Base,
                &[
                    "After you win a ground combat, you may place 1 infantry from your reinforcements on that planet."
                ],
            ),
            Technology::BioStims => t!(
                "Bio-Stims",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "You may exhaust this card at the end of your turn to ready 1 of your planets that has a technology specialty or 1 of your other technologies."
                ],
            ),
            Technology::HyperMetabolism => t!(
                "Hyper Metabolism",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["During the status phase, gain 3 command tokens instead of 2."],
            ),
            Technology::X89BacterialWeapon => t!(
                "X-89 Bacterial Weapon",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::Base,
                &[
                    "ACTION: Exhaust this card and choose 1 planet in a system that contains 1 or more of your ships that have BOMBARDMENT; destroy all infantry on that planet."
                ],
            ),
            Technology::X89BacterialWeaponOmega => t!(
                "X-89 Bacterial Weapon Ω",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::Codex,
                &[
                    "After 1 or more of your units use BOMBARDMENT against a planet, if at least 1 of your opponent's infantry was destroyed, you may destroy all of your opponent's infantry on that planet."
                ],
            ),
            Technology::X89BacterialWeaponOmegaOmega => t!(
                "X-89 Bacterial Weapon ΩΩ",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::CodexIV,
                &[
                    "Double the hits produced by your units' BOMBARDMENT and ground combat rolls. Exhaust each planet you use BOMBARDMENT against."
                ],
            ),
            Technology::AntimassDeflectors => t!(
                "Antimass Deflectors",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!(),
                Expansion::Base,
                &[
                    "Your ships can move into and through asteroid fields.",
                    "When other players’ units use SPACE CANNON against your units, apply -1 to the result of each die roll.",
                ],
            ),
            Technology::DarkEnergyTap => t!(
                "Dark Energy Tap",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!(),
                Expansion::ProphecyOfKings,
                &[
                    "After you perform a tactical action in a system that contains a frontier token, if you have 1 or more ships in that system, explore that token.",
                    "Your ships can retreat into adjacent systems that do not contain other players' units, even if you do not have units or control planets in that system."
                ],
            ),
            Technology::GravityDrive => t!(
                "Gravity Drive",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &[
                    "After you activate a system, apply +1 to the move value of 1 of your ships during this tactical action."
                ],
            ),
            Technology::SlingRelay => t!(
                "Sling Relay",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "ACTION: Exhaust this card to produce 1 ship in any system that contains one of your space docks"
                ],
            ),
            Technology::FleetLogistics => t!(
                "Fleet Logistics",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &[
                    "During each of your turns of the action phase, you may perform 2 actions instead of 1."
                ],
            ),
            Technology::LightWaveDeflector => t!(
                "Light/Wave Deflector",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 3)]),
                Expansion::Base,
                &["Your ships can move through systems that contain other players' ships."],
            ),
            Technology::SarweenTools => t!(
                "Sarween Tools",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!(),
                Expansion::Base,
                &[
                    "When 1 or more of your units use Production, reduce the combined cost of the produced units by 1"
                ],
            ),
            Technology::ScanlinkDroneNetwork => t!(
                "Scanlink Drone Network",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!(),
                Expansion::ProphecyOfKings,
                &[
                    "When you activate a system, you may explore 1 planet in that system which contains 1 or more of your units."
                ],
            ),
            Technology::GravitonLaserSystem => t!(
                "Graviton Laser System",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &[
                    "You may exhaust this card before 1 or more of your units uses Space Cannon; hits produced by those units must be assigned to non-fighter ships if able."
                ],
            ),
            Technology::PredictiveIntelligence => t!(
                "Predictive Intelligence",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "At the end of your turn, you may exhaust this card to redistribute your command tokens.",
                    "When you cast votes during the agenda phase, you may cast 3 additional votes; if you do, and the outcome you voted for is not resolved, exhaust this card.",
                ],
            ),
            Technology::TransitDiodes => t!(
                "Transit Diodes",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "You may exhaust this card at the start of your turn during the action phase; remove up to 4 of your ground forces from the game board and place them on 1 or more planets you control."
                ],
            ),
            Technology::IntegratedEconomy => t!(
                "Integrated Economy",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 3)]),
                Expansion::Base,
                &[
                    "After you gain control of a planet, you may produce any number of units on that planet that have a combined cost equal to or less than that planet’s resource value."
                ],
            ),
            Technology::PlasmaScoring => t!(
                "Plasma Scoring",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!(),
                Expansion::Base,
                &[
                    "When 1 or more of your units use Bombardment or Space Cannon, 1 of those units may roll 1 additional die."
                ],
            ),
            Technology::AiDevelopmentAlgorithm => t!(
                "AI Development Algorithm",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!(),
                Expansion::ProphecyOfKings,
                &[
                    "When you research a unit upgrade technology, you may exhaust this card to ignore any 1 prerequisite.",
                    "When 1 or more of your units use PRODUCTION, you may exhaust this card to reduce the combined cost of the produced units by the number of unit upgrade technologies that you own.",
                ],
            ),
            Technology::MagenDefenceGrid => t!(
                "Magen Defense Grid",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::Base,
                &[
                    "You may exhaust this card at the start of a round of ground combat on a planet that contains 1 or more of your units that have Planetary Shield; your opponent cannot make combat rolls this combat round."
                ],
            ),
            Technology::MagenDefenceGridOmega => t!(
                "Magen Defense Grid Ω",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::Codex,
                &[
                    "At the start of ground combat on a planet that contains 1 or more of your structures, you may produce 1 hit and assign it to 1 of your opponent's ground forces."
                ],
            ),
            Technology::MagenDefenceGridOmegaOmega => t!(
                "Magen Defense Grid ΩΩ",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::CodexIV,
                &[
                    "When any player activates a system that contains 1 or more of your structures, place 1 infantry from your reinforcements with each of those structures.",
                    "At the start of ground combat on a planet that contains 1 or more of your structures, produce 1 hit and assign it to 1 of your opponent's ground forces.",
                ],
            ),
            Technology::SelfAssemblyRoutines => t!(
                "Self Assembly Routines",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "After 1 or more of your units use PRODUCTION, you may exhaust this card to place 1 mech from your reinforcements on a planet you control in that system.",
                    "After 1 of your mechs is destroyed, gain 1 trade good.",
                ],
            ),
            Technology::DuraniumArmor => t!(
                "Duranium Armor",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &[
                    "During each combat round, after you assign hits to your units, repair 1 of your damaged units that did not use Sustain Damage during this combat round."
                ],
            ),
            Technology::AssaultCannon => t!(
                "Assault Cannon",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 3)]),
                Expansion::Base,
                &[
                    "At the start of a space combat in a system that contains 3 or more of your non-fighter ships, your opponent must destroy 1 of their non-fighter ships."
                ],
            ),
            Technology::InfantryII => t!(
                "Infantry II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &[
                    "After this unit is destroyed, roll 1 die. If the result is 6 or greater, place the unit on this card. At the start of your next turn, place each unit that is on this card on a planet you control in your home system.",
                ],
            ),
            Technology::DreadnoughtII => t!(
                "Dreadnought II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &[
                    "This unit cannot be destroyed by \"Direct Hit\" action cards.",
                    "SUSTAIN DAMAGE",
                    "BOMBARDMENT 5",
                ],
            ),
            Technology::CarrierII => t!(
                "Carrier II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
            ),
            Technology::CruiserII => t!(
                "Cruiser II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Cybernetic, 1),
                    (TechCategory::Warfare, 1)
                ]),
                Expansion::Base,
            ),
            Technology::SpaceDockII => t!(
                "Space Dock II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "This unit's PRODUCTION value is equal to 4 more than the resource value of this planet.",
                    "Up to 3 fighters in this system do not count against your ships' capacity.",
                    "PRODUCTION X",
                ],
            ),
            Technology::WarSun => t!(
                "War Sun",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)]),
                Expansion::Base,
                &[
                    "Other players' units in this system lose their PLANETARY SHIELD ability.",
                    "SUSTAIN DAMAGE",
                    "BOMBARDMENT 3 (x3)",
                ],
            ),
            Technology::DestroyerII => t!(
                "Destroyer II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &["ANTI-FIGHTER BARRAGE 6 (x3)",],
            ),
            Technology::FighterII => t!(
                "Fighter II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &[
                    "This unit may move without being transported.",
                    "Fighters in excess of your ships' capacity count against your fleet pool.",
                ],
            ),
            Technology::PdsII => t!(
                "PDS II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)]),
                Expansion::Base,
                &[
                    "You may use this unit's SPACE CANNON against ships that are adjacent to this unit's system.",
                    "PLANETARY SHIELD",
                    "SPACE CANNON 5",
                ],
            ),
            Technology::SpecOpsII => t!(
                "Spec Ops II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &[
                    "After this unit is destroyed, roll 1 die. If the result is 5 or greater, place the unit on this card. At the start of your next turn, place each unit that is on this card on a planet you control in your home system.",
                ],
            ),
            Technology::LetaniWarriorII => t!(
                "Letani Warrior II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &[
                    "After this unit is destroyed, roll 1 die. If the result is 6 or greater, place the unit on this card. At the start of your next turn, place each unit that is on this card on a planet you control in your home system.",
                    "PRODUCTION 2",
                ],
            ),
            Technology::SaturnEngineII => t!(
                "Saturn Engine II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Cybernetic, 1),
                    (TechCategory::Warfare, 1)
                ]),
                Expansion::ProphecyOfKings,
                &["SUSTAIN DAMAGE",],
            ),
            Technology::SuperDreadnoughtII => t!(
                "Super-Dreadnought II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &[
                    "This unit cannot be destroyed by \"Direct Hit\" action cards.",
                    "SUSTAIN DAMAGE",
                    "BOMBARDMENT 4",
                ],
            ),
            Technology::ExotriremeII => t!(
                "Exotrireme II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &[
                    "This unit cannot be destroyed by \"Direct Hit\" action cards.",
                    "After a round of space combat, you may destroy this unit to destroy up to 2 ships in this system.",
                    "SUSTAIN DAMAGE",
                    "BOMBARDMENT 4 (x2)",
                ],
            ),
            Technology::AdvancedCarrierII => t!(
                "Advanced Carrier II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["SUSTAIN DAMAGE",],
            ),
            Technology::CrimsonLegionnaireII => t!(
                "Crimson Legionnaire II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::ProphecyOfKings,
                &[
                    "After this unit is destroyed, gain 1 commodity or convert 1 of your commodities to a trade good. Then, place the unit on this card. At the start of your next turn, place each unit that is on this card on a planet you control in your home system.",
                ],
            ),
            Technology::FloatingFactoryII => t!(
                "Floating Factory II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "This unit is placed in the space area instead of on a planet.",
                    "This unit can move and retreat as if it were a ship.",
                    "If this unit is blockaded, it is destroyed.",
                    "PRODUCTION 7",
                ],
            ),
            Technology::DimensionalTearII => t!(
                "Dimensional Tear II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::ProphecyOfKings,
                &[
                    "This system is a gravity rift; your ships do not roll for this gravity rift.",
                    "Place a dimensional tear token beneath this unit as a reminder.",
                    "Up to 12 fighters in this system do not count against your ships' capacity.",
                    "PRODUCTION 7",
                ],
            ),
            Technology::MemoriaII => t!(
                "Memoria II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Nomad),
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Propulsion, 1),
                    (TechCategory::Cybernetic, 1)
                ]),
                Expansion::ProphecyOfKings,
                &[
                    "You may treat this unit as if it were adjacent to systems that contain one or more of your mechs.",
                    "SUSTAIN DAMAGE",
                    "ANTI-FIGHTER BARRAGE 5 (x3)",
                ],
            ),
            Technology::PrototypeWarSunII => t!(
                "Prototype War Sun II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)]),
                Expansion::Base,
                &[
                    "Other players' units in this system lose their PLANETARY SHIELD ability.",
                    "SUSTAIN DAMAGE",
                    "BOMBARDMENT 3 (x3)",
                ],
            ),
            Technology::StrikeWingAlphaII => t!(
                "Strike Wing Alpha II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::ProphecyOfKings,
                &[
                    "When this unit uses ANTI-FIGHTER BARRAGE, each result of 9 or 10 also destroys 1 of your opponent's infantry in the space area of the active system.",
                    "ANTI-FIGHTER BARRAGE 6 (x3)",
                ],
            ),
            Technology::HybridCrystalFighterII => t!(
                "Hybrid Crystal Fighter II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &[
                    "This unit may move without being transported.",
                    "Each fighter in excess of your ships' capacity counts as 1/2 of a ship against your fleet pool.",
                ],
            ),
            Technology::HelTitanII => t!(
                "Hel Titan II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "This unit is treated as both a structure and a ground force. It cannot be transported.",
                    "You may use this unit's SPACE CANNON against ships that are adjacent to this unit's system.",
                    "PLANETARY SHIELD",
                    "SPACE CANNON 5",
                    "SUSTAIN DAMAGE",
                    "PRODUCTION 1",
                ],
            ),
            Technology::HeliosV2 => t!(
                "4X41C \"Helios\" V2",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::LastBastion),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::ThundersEdge,
                &[
                    "This unit's PRODUCTION value is equal to 4 more than the resource value of this planet.",
                    "The resource value of this planet is increased by 2.",
                    "Up to 3 fighters in this system do not count against your ships' capacity.",
                    "PRODUCTION X",
                ],
            ),
            Technology::LinkshipII => t!(
                "Linkship II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::RalNelConsortium),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::ThundersEdge,
                &[
                    "This unit can use the SPACE CANNON ability of one of your structures in its space area; each linkship can trigger the same structure.",
                    "ANTI-FIGHTER BARRAGE 6 (x3)",
                ],
            ),
            Technology::ExileII => t!(
                "Exile II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::CrimsonRebellion),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::ThundersEdge,
                &[
                    "At the end of any player's combat in this unit's system or up to 2 systems away, you may place 1 active or inactive breach in that system.",
                    "ANTI-FIGHTER BARRAGE 6 (x3)",
                ],
            ),
            Technology::Voidwatch => t!(
                "Voidwatch",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "After a player moves ships into a system that contains 1 or more of your units, they must give you 1 promissory note from their hand, if able."
                ],
            ),
            Technology::InstinctTraining => t!(
                "Instinct Training",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::Base,
                &[
                    "You may exhaust this card and spend 1 token from your strategy pool when another player plays an action card; cancel that action card."
                ],
            ),
            Technology::TransparasteelPlating => t!(
                "Transparasteel Plating",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::Base,
                &[
                    "During your turn of the action phase, players that have passed cannot play action cards."
                ],
            ),
            Technology::GeneticRecombination => t!(
                "Genetic Recombination",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "You may exhaust this card before a player casts votes; that player must cast at least 1 vote for an outcome of your choice or remove 1 token from their fleet pool and return it to their reinforcements."
                ],
            ),
            Technology::Bioplasmosis => t!(
                "Bioplasmosis",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &[
                    "At the end of the status phase, you may remove any number of infantry from planets you control and place them on 1 or more planets you control in the same or adjacent systems."
                ],
            ),
            Technology::ProductionBiomes => t!(
                "Production Biomes",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &[
                    "ACTION: Exhaust this card and spend 1 token from your strategy pool to gain 4 trade goods and choose 1 other player; that player gains 2 trade goods."
                ],
            ),
            Technology::YinSpinner => t!(
                "Yin Spinner",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &[
                    "After 1 or more of your units use Production, place 1 infantry from your reinforcements on a planet you control in that system."
                ],
            ),
            Technology::YinSpinnerOmega => t!(
                "Yin Spinner Ω",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Codex,
                &[
                    "After you produce units, place up to 2 infantry from your reinforcements on any planet you control or in any space area that contains 1 or more of your ships."
                ],
            ),
            Technology::Neuroglaive => t!(
                "Neuroglaive",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::Base,
                &[
                    "After another player activates a system that contains 1 or more of your ships, that player removes 1 token from their fleet pool and returns it to their reinforcements."
                ],
            ),
            Technology::PreFabArcologies => t!(
                "Pre-Fab Arcologies",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::ProphecyOfKings,
                &["After you explore a planet, ready that planet."],
            ),
            Technology::MageonImplants => t!(
                "Mageon Implants",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::Base,
                &[
                    "ACTION: Exhaust this card to look at another player's hand of action cards. Choose 1 of those cards and add it to your hand."
                ],
            ),
            Technology::ChaosMapping => t!(
                "Chaos Mapping",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &[
                    "Other players cannot activate asteroid fields that contain 1 or more of your ships.",
                    "At the start of your turn during the action phase, you may produce 1 unit in a system that contains at least 1 of your units that has PRODUCTION.",
                ],
            ),
            Technology::SpacialConduitCylinder => t!(
                "Spatial Conduit Cylinder",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &[
                    "You may exhaust this card after you activate a system that contains 1 or more of your units; that system is adjacent to all other systems that contain 1 or more of your units during this activation."
                ],
            ),
            Technology::Aetherstream => t!(
                "Aetherstream",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::ProphecyOfKings,
                &[
                    "After you or one of your neighbors activates a system that is adjacent to an anomaly, you may apply +1 to the move value of all of that player's ships during this tactical action"
                ],
            ),
            Technology::WormholeGenerator => t!(
                "Wormhole Generator",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &[
                    "At the start of the status phase, place or move a Creuss wormhole token into either a system that contains a planet you control or a non-home system that does not contain another player's ships."
                ],
            ),
            Technology::WormholeGeneratorOmega => t!(
                "Wormhole Generator Ω",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Codex,
                &[
                    "ACTION: Exhaust this card to place or move a Creuss wormhole token into either a system that contains a planet you control or a non-home system that does not contain another player's ships."
                ],
            ),
            Technology::LazaxGateFolding => t!(
                "Lazax Gate Folding",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &[
                    "During your tactical actions, if you do not control Mecatol Rex, treat its system as if it has both an α and β wormhole. ACTION: If you control Mecatol Rex, exhaust this card to place 1 infantry from your reinforcements on Mecatol Rex."
                ],
            ),
            Technology::AerieHololattice => t!(
                "Aerie Hololattice",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "Other players cannot move ships through systems that contain your structures. Each planet that contains 1 or more of your structures gains the PRODUCTION 1 ability as if it were a unit."
                ],
            ),
            Technology::L4Disruptors => t!(
                "L4 Disruptors",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &["During an invasion, units cannot use Space Cannon against your units."],
            ),
            Technology::TemporalCommandSuite => t!(
                "Temporal Command Suite",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Nomad),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "After any player's agent becomes exhausted, you may exhaust this card to ready that agent; if you ready another player's agent, you may perform a transaction with that player."
                ],
            ),
            Technology::IIHQModernization => t!(
                "I.I.H.Q. Modernization",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::CodexIII,
                &[
                    "You are neighbors with all players that have units or control planets in or adjacent to the Mecatol Rex system."
                ],
            ),
            Technology::ExecutiveOrder => t!(
                "Executive Order",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ThundersEdge,
                &[
                    "ACTION: Exhaust this card and draw the top or bottom card of the agenda deck. Players immediately vote on this agenda as if you were the speaker; you can spend trade goods and resources on this agenda as if they were votes."
                ],
            ),
            Technology::SalvageOperations => t!(
                "Salvage Operations",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "After you win or lose a space combat, gain 1 trade good; if you won the combat, you may also produce 1 ship in that system of any ship type that was destroyed during the combat."
                ],
            ),
            Technology::InheritanceSystems => t!(
                "Inheritance Systems",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "You may exhaust this card and spend 2 resources when you research a technology; ignore all of that technology's prerequisites."
                ],
            ),
            Technology::EResSiphons => t!(
                "E-Res Siphons",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "After another player activates a system that contains 1 or more of your ships, gain 4 trade goods."
                ],
            ),
            Technology::HegemonicTradePolicy => t!(
                "Hegemonic Trade Policy",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "Exhaust this card when 1 or more of your units use PRODUCTION; swap the resource and influence values of 1 planet you control during that use of PRODUCTION."
                ],
            ),
            Technology::NullificationField => t!(
                "Nullification Field",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "After another player activates a system that contains 1 or more of your ships, you may exhaust this card and spend 1 token from your strategy pool; immediately end that player's turn."
                ],
            ),
            Technology::CoreImpulse => t!(
                "Impulse Core",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &[
                    "At the start of a space combat, you may destroy 1 of your cruisers or destroyers in the active system to produce 1 hit against your opponent's ships; that hit must be assigned by your opponent to 1 of their non-fighters ships if able."
                ],
            ),
            Technology::AgencySupplyNetwork => t!(
                "Agency Supply Network",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::CodexIII,
                &[
                    "Whenever you resolve one of your PRODUCTION abilities, you may resolve an additional one of your PRODUCTION abilities in any system; the additional use does not trigger this ability."
                ],
            ),
            Technology::AgencySupplyNetworkOmega => t!(
                "Agency Supply Network Ω",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::ThundersEdge,
                &[
                    "Once per action, when you resolve a unit's PRODUCTION ability, you may resolve another of your unit's PRODUCTION abilities in any system."
                ],
            ),
            Technology::QuantumDatahubNode => t!(
                "Quantum Datahub Node",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Cybernetic, 3)]),
                Expansion::Base,
                &[
                    "At the end of the strategy phase, you may spend 1 token from your strategy pool and give another player 3 of your trade goods. If you do, give 1 of your strategy cards to that player and take 1 of their strategy cards."
                ],
            ),
            Technology::MirrorComputing => t!(
                "Mirror Computing",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 3)]),
                Expansion::Base,
                &[
                    "When you spend trade goods, each trade good is worth 2 resources or influence instead of 1."
                ],
            ),
            Technology::DimensionalSplicer => t!(
                "Dimensional Splicer",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::Base,
                &[
                    "At the start of space combat in a system that contains a wormhole and 1 or more of your ships, you may produce 1 hit and assign it to 1 of your opponent's ships."
                ],
            ),
            Technology::Supercharge => t!(
                "Supercharge",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "At the start of a combat round, you may exhaust this card to apply +1 to the result of each of your unit's combat rolls during this combat round."
                ],
            ),
            Technology::Vortex => t!(
                "Vortex",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &[
                    "ACTION: Exhaust this card to choose another player's non-structure unit in a system that is adjacent to 1 or more of your space docks. Capture 1 unit of that type from that player's reinforcements."
                ],
            ),
            Technology::NonEuclidianShielding => t!(
                "Non-Euclidean Shielding",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &["When 1 of your units uses Sustain Damage, cancel 2 hits instead of 1."],
            ),
            Technology::MagmusReactor => t!(
                "Magmus Reactor",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &[
                    "Your ships can move into supernovas.",
                    "After 1 or more of your units use Production in a system that either contains a war sun or is adjacent to a supernova, gain 1 trade good.",
                ],
            ),
            Technology::MagmusReactorOmega => t!(
                "Magmus Reactor Ω",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Codex,
                &[
                    "Your ships can move into supernovas.",
                    "Each supernova that contains 1 or more of your units gains the PRODUCTION 5 ability as if it were 1 of your units.",
                ],
            ),
            Technology::ValkyrieParticleWeave => t!(
                "Valkyrie Particle Weave",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &[
                    "After making combat rolls during a round of ground combat, if your opponent produced 1 or more hits, you produce 1 additional hit."
                ],
            ),
            Technology::RadicalAdvancement => t!(
                "Radical Advancement",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::DeepwroughtScholarate),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ThundersEdge,
                &[
                    "At the start of the status phase, you may replace one of your non-unit upgrade technologies with a technology of the same color that has exactly 1 more prerequisite."
                ],
            ),
            Technology::HydrothermalMining => t!(
                "Hydrothermal Mining",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::DeepwroughtScholarate),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ThundersEdge,
                &[
                    "At the start of the status phase, gain 1 trade good for each ocean card in play."
                ],
            ),
            Technology::NeuralParasite => t!(
                "Neural Parasite",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::FirmamentObsidian),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::ThundersEdge,
                &[
                    "At the start of the status phase, you may place 1 infantry from your reinforcements on a planet you control in your home system.",
                    "Flip this card if the Obsidian faction is in play."
                ],
            ),
            Technology::PlaneSplitter => t!(
                "Plane Splitter",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::FirmamentObsidian),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::ThundersEdge,
                &[
                    "When you gain this card, put The Fracture into play.",
                    "Flip this card if the Obsidian faction is in play."
                ],
            ),
            Technology::SubatomicSplicer => t!(
                "Subatomic Splicer",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CrimsonRebellion),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ThundersEdge,
                &[
                    "When one of your ships is destroyed, you may produce a ship of the same type at a space dock in your home system.",
                ],
            ),
            Technology::ProximaTargetingVI => t!(
                "Proxima Targeting VI",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::LastBastion),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ThundersEdge,
                &[
                    "Cancel 1 hit produced by BOMBARDMENT rolls made against your ground forces for each of your galvanized units present.",
                    "At the start of a round of ground combat, you may resolve BOMBARDMENT 8 (x3) against your opponents ground forces; if you do, make an identical roll against your own ground forces."
                ],
            ),
            Technology::Nanomachines => t!(
                "Nanomachines",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::RalNelConsortium),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ThundersEdge,
                &[
                    "ACTION: Exhaust this card to place 1 PDS on a planet you control.",
                    "ACTION: Exhaust this card to repair all of your damaged units.",
                    "ACTION: Exhaust this card and discard 1 action card to draw 1 action card."
                ],
            ),
            Technology::ValefarAssimilatorX => t!(
                "Valefar Assimilator X",
                TechType::Colorless,
                TechOrigin::Faction(Faction::NekroVirus),
                tr!(),
                Expansion::Base,
                &[
                    "When you would gain another player's technology using 1 of your faction abilities, you may place the \"X\" assimilator token on a faction technology owned by that player instead. While that token is on a technology, this card gains that technology's text. You cannot place an assimilator token on a technology that already has an assimilator token."
                ],
            ),
            Technology::ValefarAssimilatorY => t!(
                "Valefar Assimilator Y",
                TechType::Colorless,
                TechOrigin::Faction(Faction::NekroVirus),
                tr!(),
                Expansion::Base,
                &[
                    "When you would gain another player's technology using 1 of your faction abilities, you may place the \"Y\" assimilator token on a faction technology owned by that player instead. While that token is on a technology, this card gains that technology's text. You cannot place an assimilator token on a technology that already has an assimilator token."
                ],
            ),
        }
    }

    /// The [TechInfo] of the side the tech is flipped to once the Obsidian is in play, for the
    /// Firmament techs that have one.
    pub fn obsidian_info(&self) -> Option<TechInfo> {
        match self {
            Technology::NeuralParasite => Some(t!(
                "Neural Parasite",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::FirmamentObsidian),
                tr!(),
                Expansion::ThundersEdge,
                &[
                    "At the start of your turn, destroy 1 of another player's infantry in or adjacent to a system that contains your infantry.",
                    "This technology cannot be researched.",
                ],
            )),
            Technology::PlaneSplitter => Some(t!(
                "Plane Splitter",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::FirmamentObsidian),
                tr!(),
                Expansion::ThundersEdge,
                &[
                    "At the start of your strategic actions, you may move an ingress token into a system that contains or is adjacent to your units.",
                    "This technology cannot be researched.",
                ],
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::game_settings::Expansions;

    use super::*;

    #[test]
    fn exactly_one_revision_of_codex_iv_techs_is_enabled() {
        use Technology::*;

        let cases = [
            (false, false, X89BacterialWeapon, MagenDefenceGrid),
            (true, false, X89BacterialWeaponOmega, MagenDefenceGridOmega),
            (
                false,
                true,
                X89BacterialWeaponOmegaOmega,
                MagenDefenceGridOmegaOmega,
            ),
            (
                true,
                true,
                X89BacterialWeaponOmegaOmega,
                MagenDefenceGridOmegaOmega,
            ),
        ];

        let x89s = [
            X89BacterialWeapon,
            X89BacterialWeaponOmega,
            X89BacterialWeaponOmegaOmega,
        ];
        let magens = [
            MagenDefenceGrid,
            MagenDefenceGridOmega,
            MagenDefenceGridOmegaOmega,
        ];

        for (codex_1, codex_4, expected_x89, expected_magen) in cases {
            let expansions = Expansions {
                codex_1,
                codex_4,
                ..Default::default()
            };

            for (variants, expected) in [(&x89s, expected_x89), (&magens, expected_magen)] {
                let enabled = variants
                    .iter()
                    .filter(|t| t.is_enabled_in(&expansions))
                    .cloned()
                    .collect::<Vec<_>>();
                assert_eq!(
                    enabled,
                    vec![expected],
                    "codex_1: {codex_1}, codex_4: {codex_4}"
                );
            }
        }
    }
}
