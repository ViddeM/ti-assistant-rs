use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::data::common::faction::Faction;

/// What category the tech belongs to.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
}

/// Weather the game is general or faction specific.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TechOrigin {
    Base,
    Faction(Faction),
}

/// Technologies in the game.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[allow(missing_docs)]
pub enum Technology {
    // Biotic
    NeuralMotivator,
    Psychoarchaeology,
    DacxiveAnimators,
    BioStims,
    HyperMetabolism,
    X89BacterialWeapon,
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
    // Faction techs - Biotic
    Voidwatch,
    InstinctTraining,
    TransparasteelPlating,
    GeneticRecombination,
    Bioplasmosis,
    ProductionBiomes,
    YinSpinner,
    Neuroglaive,
    PreFabArcologies,
    MageonImplants,
    // Faction techs - Propulsion
    ChaosMapping,
    SpacialConduitCylinder,
    Aetherstream,
    WormholeGenerator,
    LazaxGateFolding,
    // Faction techs - Cybernetic
    AerieHololattice,
    L4Disruptors,
    TemporalCommandSuite,
    IIHQModernization,
    SalvageOperations,
    InheritenceSystems,
    EResSiphons,
    HegemonicTradePolicy,
    NullificationField,
    CoreImpulse,
    AgencySupplyNetwork,
    QuantumDatahubNode,
    MirrorComputing,
    // Faction techs - Warfare
    DimensionalSplicer,
    Supercharge,
    Vortex,
    NonEuclidianShielding,
    MagmusReactor,
    ValkyrieParticleWeave,
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
}

macro_rules! t {
    ($name: expr, $t:expr, $orig:expr, $reqs: expr) => {
        TechInfo {
            name: $name.to_string(),
            tech_type: $t,
            origin: $orig,
            requirements: $reqs,
        }
    };
}

macro_rules! tr {
    () => {
        HashMap::new()
    };
    ($e:expr) => {
        HashMap::from($e)
    };
}

impl Technology {
    /// Returns the [TechInfo] for this technology.
    pub fn info(&self) -> TechInfo {
        match self {
            Technology::NeuralMotivator => t!(
                "Neural Motivator",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::Psychoarchaeology => t!(
                "Psychoarchaeology",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::DacxiveAnimators => t!(
                "Dacxive Animators",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::BioStims => t!(
                "Bio Stims",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::HyperMetabolism => t!(
                "Hyper Metabolism",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::X89BacterialWeapon => t!(
                "X-89 Bacterial Weapon/X-89 Bacterial Weapon Ω",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::AntimassDeflectors => t!(
                "Antimass Deflectors",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!()
            ),
            Technology::DarkEnergyTap => t!(
                "Dark Energy Tap",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!()
            ),
            Technology::GravityDrive => t!(
                "Gravity Drive",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)])
            ),
            Technology::SlingRelay => t!(
                "Sling Relay",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)])
            ),
            Technology::FleetLogistics => t!(
                "Fleet Logistics",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::LightWaveDeflector => t!(
                "Light/Wave Deflector",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 3)])
            ),
            Technology::SarweenTools => t!(
                "Sarween Tools",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::ScanlinkDroneNetwork => t!(
                "Scanlink Drone Network",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::GravitonLaserSystem => t!(
                "Graviton Laser System",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::PredictiveIntelligence => t!(
                "Predictive Intelligence",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::TransitDiodes => t!(
                "Transit Diodes",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::IntegratedEconomy => t!(
                "Integrated Economy",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 3)])
            ),
            Technology::PlasmaScoring => t!(
                "Plasma Scoring",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!()
            ),
            Technology::AiDevelopmentAlgorithm => t!(
                "AI Development Algorithm",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!()
            ),
            Technology::MagenDefenceGrid => t!(
                "Magen Defense Grid/Magen Defense Grid Ω",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::SelfAssemblyRoutines => t!(
                "Self Assembly Routines",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::DuraniumArmor => t!(
                "Duranium Armor",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::AssaultCannon => t!(
                "Assault Cannon",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 3)])
            ),
            Technology::InfantryII => t!(
                "Infantry II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::DreadnoughtII => {
                t!("Dreadnought II", TechType::UnitUpgrade, TechOrigin::Base, {
                    tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)])
                })
            }
            Technology::CarrierII => t!(
                "Carrier II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::CruiserII => t!(
                "Cruiser II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Cybernetic, 1),
                    (TechCategory::Warfare, 1)
                ])
            ),
            Technology::SpaceDockII => t!(
                "Space Dock II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::WarSun => t!(
                "War Sun",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)])
            ),
            Technology::DestroyerII => t!(
                "Destroyer II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::FighterII => t!(
                "Fighter II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)])
            ),
            Technology::PdsII => t!(
                "PDS II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)])
            ),
            Technology::SpecOpsII => t!(
                "Spec Ops II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::LetaniWarriorII => t!(
                "Letani Warrior II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::SaturnEngineII => t!(
                "Saturn Engine II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Cybernetic, 1),
                    (TechCategory::Warfare, 1)
                ])
            ),
            Technology::SuperDreadnoughtII => t!(
                "Super Dreadnought II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)])
            ),
            Technology::ExotriremeII => t!(
                "Exotrireme II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)])
            ),
            Technology::AdvancedCarrierII => t!(
                "Advanced Carrier II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::CrimsonLegionnaireII => t!(
                "Crimson Legionnaire II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::FloatingFactoryII => t!(
                "Floating Factory II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::DimensionalTearII => t!(
                "Dimensional Tear II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::MemoriaII => t!(
                "Memoria II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Nomad),
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Propulsion, 1),
                    (TechCategory::Cybernetic, 1)
                ])
            ),
            Technology::PrototypeWarSunII => t!(
                "Prototype War Sun II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)])
            ),
            Technology::StrikeWingAlphaII => t!(
                "Strike Wing Alpha II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::HybridCrystalFighterII => t!(
                "Hybrid Crystal Fighter II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)])
            ),
            Technology::HelTitanII => t!(
                "Hel Titan II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)])
            ),
            Technology::Voidwatch => t!(
                "Voidwatch",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::InstinctTraining => t!(
                "Instinct Training",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::TransparasteelPlating => t!(
                "Transparasteel Plating",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::GeneticRecombination => t!(
                "Genetic Recombination",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::Bioplasmosis => t!(
                "Bioplasmosis",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::ProductionBiomes => t!(
                "Production Biomes",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::YinSpinner => t!(
                "Yin Spinner/ Yin Spinner Ω",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::Neuroglaive => t!(
                "Neuroglaive",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::PreFabArcologies => t!(
                "Pre-Fab Arcologies",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::MageonImplants => t!(
                "Mageon Implants",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::ChaosMapping => t!(
                "Chaos Mapping",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Propulsion, 1)])
            ),
            Technology::SpacialConduitCylinder => t!(
                "Spatial Conduit Cylinder",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::Aetherstream => t!(
                "Aetherstream",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::WormholeGenerator => t!(
                "Wormhole Generator/ Wormhole Generator Ω",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::LazaxGateFolding => t!(
                "Lazax Gate Folding",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::AerieHololattice => t!(
                "Aerie Hololattice",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::L4Disruptors => t!(
                "L4 Disruptors",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::TemporalCommandSuite => t!(
                "Temporal Command Suite",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Nomad),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::IIHQModernization => t!(
                "I.I.H.Q Modernization",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::SalvageOperations => t!(
                "Salvage Operations",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::InheritenceSystems => t!(
                "Inheritance Systems",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::EResSiphons => t!(
                "E-Res Siphons",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::HegemonicTradePolicy => t!(
                "Hegemonic Trade Policy",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::NullificationField => t!(
                "Nullification Field",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::CoreImpulse => t!(
                "Core Impulse",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::AgencySupplyNetwork => t!(
                "Agency Supply Network",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::QuantumDatahubNode => t!(
                "Quantum Datahub Node",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Cybernetic, 3)])
            ),
            Technology::MirrorComputing => t!(
                "Mirror Computing",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 3)])
            ),
            Technology::DimensionalSplicer => t!(
                "Dimensional Splicer",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::Supercharge => t!(
                "Supercharge",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::Vortex => t!(
                "Vortex",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::NonEuclidianShielding => t!(
                "Non-Euclidian Shielding",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::MagmusReactor => t!(
                "Magmus Reactor/ Magmus Reactor Ω",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::ValkyrieParticleWeave => t!(
                "Valkyrie Particle Weave",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Warfare, 2)])
            ),
        }
    }
}
