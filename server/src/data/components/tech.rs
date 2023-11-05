use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::common::faction::Faction;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum TechCategory {
    Biotic,
    Propulsion,
    Cybernetic,
    Warfare,
}

pub enum TechType {
    Category(TechCategory),
    UnitUpgrade,
}

pub enum TechOrigin {
    Base,
    Faction(Faction),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
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

pub struct TechInfo {
    tech_type: TechType,
    origin: TechOrigin,
    requirements: HashMap<TechCategory, u32>,
}

macro_rules! t {
    ($t:expr, $orig:expr, $reqs: expr) => {
        TechInfo {
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
    pub fn info(&self) -> TechInfo {
        match self {
            Technology::NeuralMotivator => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::Psychoarchaeology => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::DacxiveAnimators => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::BioStims => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::HyperMetabolism => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::X89BacterialWeapon => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::AntimassDeflectors => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!()
            ),
            Technology::DarkEnergyTap => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!()
            ),
            Technology::GravityDrive => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)])
            ),
            Technology::SlingRelay => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)])
            ),
            Technology::FleetLogistics => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::LightWaveDeflector => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 3)])
            ),
            Technology::SarweenTools => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::ScanlinkDroneNetwork => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!()
            ),
            Technology::GravitonLaserSystem => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::PredictiveIntelligence => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::TransitDiodes => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::IntegratedEconomy => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 3)])
            ),
            Technology::PlasmaScoring => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!()
            ),
            Technology::AiDevelopmentAlgorithm => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!()
            ),
            Technology::MagenDefenceGrid => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::SelfAssemblyRoutines => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::DuraniumArmor => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::AssaultCannon => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 3)])
            ),
            Technology::InfantryII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::DreadnoughtII => t!(TechType::UnitUpgrade, TechOrigin::Base, {
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)])
            }),
            Technology::CarrierII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::CruiserII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Cybernetic, 1),
                    (TechCategory::Warfare, 1)
                ])
            ),
            Technology::SpaceDockII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::WarSun => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)])
            ),
            Technology::DestroyerII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::FighterII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)])
            ),
            Technology::PdsII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)])
            ),
            Technology::SpecOpsII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::LetaniWarriorII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::SaturnEngineII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Cybernetic, 1),
                    (TechCategory::Warfare, 1)
                ])
            ),
            Technology::SuperDreadnoughtII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)])
            ),
            Technology::ExotriremeII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)])
            ),
            Technology::AdvancedCarrierII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::CrimsonLegionnaireII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::FloatingFactoryII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::DimensionalTearII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::MemoriaII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Nomad),
                tr!([
                    (TechCategory::Biotic, 1),
                    (TechCategory::Propulsion, 1),
                    (TechCategory::Cybernetic, 1)
                ])
            ),
            Technology::PrototypeWarSunII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)])
            ),
            Technology::StrikeWingAlphaII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::HybridCrystalFighterII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)])
            ),
            Technology::HelTitanII => t!(
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)])
            ),
            Technology::Voidwatch => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::InstinctTraining => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::TransparasteelPlating => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::GeneticRecombination => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 1)])
            ),
            Technology::Bioplasmosis => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::ProductionBiomes => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::YinSpinner => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Biotic, 2)])
            ),
            Technology::Neuroglaive => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::PreFabArcologies => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::MageonImplants => t!(
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 3)])
            ),
            Technology::ChaosMapping => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Propulsion, 1)])
            ),
            Technology::SpacialConduitCylinder => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::Aetherstream => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::WormholeGenerator => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::LazaxGateFolding => t!(
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Propulsion, 2)])
            ),
            Technology::AerieHololattice => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::L4Disruptors => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::TemporalCommandSuite => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Nomad),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::IIHQModernization => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 1)])
            ),
            Technology::SalvageOperations => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::InheritenceSystems => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::EResSiphons => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::HegemonicTradePolicy => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::NullificationField => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::CoreImpulse => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::AgencySupplyNetwork => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 2)])
            ),
            Technology::QuantumDatahubNode => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Cybernetic, 3)])
            ),
            Technology::MirrorComputing => t!(
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 3)])
            ),
            Technology::DimensionalSplicer => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::Supercharge => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::Vortex => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Warfare, 1)])
            ),
            Technology::NonEuclidianShielding => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::MagmusReactor => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Warfare, 2)])
            ),
            Technology::ValkyrieParticleWeave => t!(
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Warfare, 2)])
            ),
        }
    }
}
