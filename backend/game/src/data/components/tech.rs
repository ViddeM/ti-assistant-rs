use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::{
    data::common::{expansions::Expansion, faction::Faction},
    gameplay::game_settings::Expansions,
};

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
    X89BacterialWeaponOmega,
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
    YinSpinnerOmega,
    Neuroglaive,
    PreFabArcologies,
    MageonImplants,
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
    SalvageOperations,
    InheritanceSystems,
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
    MagmusReactorOmega,
    ValkyrieParticleWeave,
}

/// All relevant information about a tech.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TechInfo {
    /// The name of the tech in 'pretty' format.
    pub name: &'static str,
    /// What type of tech this is.
    pub tech_type: TechType,
    /// Weather the tech is general or belongs to a faction.
    pub origin: TechOrigin,
    /// What requirements there are for the technology.
    pub requirements: HashMap<TechCategory, u32>,
    /// Which expansion this tech belongs to.
    pub expansion: Expansion,
    /// The effects of the technology. Each element corresponds to a s
    pub effects: &'static [&'static str],
}

macro_rules! t {
    ($name: expr, $t:expr, $orig:expr, $reqs: expr, $expansion: expr,) => {
        TechInfo {
            name: $name,
            tech_type: $t,
            origin: $orig,
            requirements: $reqs,
            expansion: $expansion,
            effects: &[],
        }
    };
    ($name: expr, $t:expr, $orig:expr, $reqs: expr, $expansion: expr, $effects: expr,) => {
        TechInfo {
            name: $name,
            tech_type: $t,
            origin: $orig,
            requirements: $reqs,
            expansion: $expansion,
            effects: $effects,
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
    /// Is this tech enabled for the given [Expansions]?
    pub fn is_enabled_in(&self, expansions: &Expansions) -> bool {
        use Technology::*;

        // check if the expansion that adds this tech is enabled.
        let faction_source = self.info().expansion;
        if !expansions.is_enabled(&faction_source) {
            return false;
        }

        // check if tech is patched in codex 1

        let removed_in_codex_1 = &[
            MagenDefenceGrid,
            X89BacterialWeapon,
            MagmusReactor,
            WormholeGenerator,
            YinSpinner,
        ];

        let added_in_codex_1 = &[
            MagenDefenceGridOmega,
            X89BacterialWeaponOmega,
            MagmusReactorOmega,
            WormholeGeneratorOmega,
            YinSpinnerOmega,
        ];

        let is_removed_in_codex_1 = removed_in_codex_1.contains(self);
        let is_added_in_codex_1 = added_in_codex_1.contains(self);

        if expansions.codex_1 && is_removed_in_codex_1 {
            return false;
        }

        if !expansions.codex_1 && is_added_in_codex_1 {
            return false;
        }

        true
    }

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
                &["After you win a ground combat, you may place 1 infantry from your reinforcements on that planet."],
            ),
            Technology::BioStims => t!(
                "Bio-Stims",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ProphecyOfKings,
                &["You may exhaust this card at the end of your turn to ready 1 of your planets that has a technology specialty or 1 of your other technologies."],
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
                &["ACTION: Exhaust this card and choose 1 planet in a system that contains 1 or more of your ships that have BOMBARDMENT; destroy all infantry on that planet."],
            ),
            Technology::X89BacterialWeaponOmega => t!(
                "X-89 Bacterial Weapon Ω",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::Codex,
                &["After 1 or more of your units use BOMBARDMENT against a planet, if at least 1 of your opponent's infantry was destroyed, you may destroy all of your opponent's infantry on that planet."],
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
                &["After you activate a system, apply +1 to the move value of 1 of your ships during this tactical action."],
            ),
            Technology::SlingRelay => t!(
                "Sling Relay",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 1)]),
                Expansion::ProphecyOfKings,
                &["ACTION: Exhaust this card to produce 1 ship in any system that contains one of your space docks"],
            ),
            Technology::FleetLogistics => t!(
                "Fleet Logistics",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["During each of your turns of the action phase, you may perform 2 actions instead of 1."],
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
                &["When 1 or more of your units use Production, reduce the combined cost of the produced units by 1"],
            ),
            Technology::ScanlinkDroneNetwork => t!(
                "Scanlink Drone Network",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!(),
                Expansion::ProphecyOfKings,
                &["When you activate a system, you may explore 1 planet in that system which contains 1 or more of your units."],
            ),
            Technology::GravitonLaserSystem => t!(
                "Graviton Laser System",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &["You may exhaust this card before 1 or more of your units uses Space Cannon; hits produced by those units must be assigned to non-fighter ships if able."],
            ),
            Technology::PredictiveIntelligence => t!(
                "Predictive Intelligence",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ProphecyOfKings,
                &["At the end of your turn, you may exhaust this card to redistribute your command tokens."],
            ),
            Technology::TransitDiodes => t!(
                "Transit Diodes",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["You may exhaust this card at the start of your turn during the action phase; remove up to 4 of your ground forces from the game board and place them on 1 or more planets you control."],
            ),
            Technology::IntegratedEconomy => t!(
                "Integrated Economy",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 3)]),
                Expansion::Base,
                &["After you gain control of a planet, you may produce any number of units on that planet that have a combined cost equal to or less than that planet’s resource value."],
            ),
            Technology::PlasmaScoring => t!(
                "Plasma Scoring",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!(),
                Expansion::Base,
                &["When 1 or more of your units use Bombardment or Space Cannon, 1 of those units may roll 1 additional die."],
            ),
            Technology::AiDevelopmentAlgorithm => t!(
                "AI Development Algorithm",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!(),
                Expansion::ProphecyOfKings,
                &["When you research a unit upgrade technology, you may exhaust this card to ignore any 1 prerequisite."],
            ),
            Technology::MagenDefenceGrid => t!(
                "Magen Defense Grid",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::Base,
                &["You may exhaust this card at the start of a round of ground combat on a planet that contains 1 or more of your units that have Planetary Shield; your opponent cannot make combat rolls this combat round."],
            ),
            Technology::MagenDefenceGridOmega => t!(
                "Magen Defense Grid Ω",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::Base,
                &["At the start of ground combat on a planet that contains 1 or more of your structures, you may produce 1 hit and assign it to 1 of your opponent's ground forces."],
            ),
            Technology::SelfAssemblyRoutines => t!(
                "Self Assembly Routines",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &["After 1 or more of your units use PRODUCTION, you may exhaust this card to place 1 mech from your reinforcements on a planet you control in that system."],
            ),
            Technology::DuraniumArmor => t!(
                "Duranium Armor",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &["During each combat round, after you assign hits to your units, repair 1 of your damaged units that did not use Sustain Damage during this combat round."],
            ),
            Technology::AssaultCannon => t!(
                "Assault Cannon",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 3)]),
                Expansion::Base,
                &["At the start of a space combat in a system that contains 3 or more of your non-fighter ships, your opponent must destroy 1 of their non-fighter ships."],
            ),
            Technology::InfantryII => t!(
                "Infantry II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["Upgraded infantry are more effective at ground combat and gain the ability to return to home system at the next turn when destroyed."],
            ),
            Technology::DreadnoughtII => t!(
                "Dreadnought II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &["Upgraded Dreadnoughts are faster and cannot be targeted by Direct Hit action cards."],
            ),
            Technology::CarrierII => t!(
                "Carrier II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["Upgraded Carriers are faster and have larger capacity."],
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
                &["Upgraded Cruisers are faster, perform better in combat and gain capacity."],
            ),
            Technology::SpaceDockII => t!(
                "Space Dock II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["Upgraded Space Docks have a higher production value and allow up to 3 fighters (per space dock) to remain with them in excess of the system's capacity."],
            ),
            Technology::WarSun => t!(
                "War Sun",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)]),
                Expansion::Base,
                &["Allows the construction of War Suns"],
            ),
            Technology::DestroyerII => t!(
                "Destroyer II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &["Upgraded Destroyers are better in combat and have significantly improved Anti-Fighter Barrage."],
            ),
            Technology::FighterII => t!(
                "Fighter II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &["Upgraded Fighters are better in combat and gain the ability to move without being transported."],
            ),
            Technology::PdsII => t!(
                "PDS II",
                TechType::UnitUpgrade,
                TechOrigin::Base,
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)]),
                Expansion::Base,
                &["Upgraded PDS have improved Space Cannon and are able to fire into adjacent systems."],
            ),
            Technology::SpecOpsII => t!(
                "Spec Ops II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["Upgraded Sol Infantry are stronger than regular upgraded infantry and have a higher likelihood of returning back to the home system."],
            ),
            Technology::LetaniWarriorII => t!(
                "Letani Warrior II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["Upgraded Arborec Infantry gain an extra production value and are better in combat compared to the non-upgraded variant."],
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
                &["Upgraded Titans of Ul Cruisers have more capacity compared to regular upgraded cruisers and gain Sustain Damage."],
            ),
            Technology::SuperDreadnoughtII => t!(
                "Super Dreadnought II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &["Upgraded L1Z1X Dreadnoughts are stronger than regular Dreadnoughts, have greater capacity and better Bombardment."],
            ),
            Technology::ExotriremeII => t!(
                "Exotrireme II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::SardakkNorr),
                tr!([(TechCategory::Propulsion, 2), (TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &["Upgraded Sardakk N'orr Dreadnoughts have better bombardment than all other Dreadnoughts and gain the ability to directly destroy ships after a round of space combat."],
            ),
            Technology::AdvancedCarrierII => t!(
                "Advanced Carrier II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::FederationOfSol),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["Upgraded Sol Carriers have greater capacity compared to regular upgraded carriers and gain Sustain Damage."],
            ),
            Technology::CrimsonLegionnaireII => t!(
                "Crimson Legionnaire II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::ProphecyOfKings,
                &["Upgraded Mahact Infantry automatically return to the home system the next turn after being destroyed."],
            ),
            Technology::FloatingFactoryII => t!(
                "Floating Factory II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["Upgraded Saar space docks are faster, have more capacity, and are able to produce more units."],
            ),
            Technology::DimensionalTearII => t!(
                "Dimensional Tear II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::ProphecyOfKings,
                &["Upgraded Vuil'Raith Space Docks have increased production and allow double the amount of fighters in the system to be in excess of capacity."],
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
                &["The Nomad is the only faction capable of upgrading their Flagship. When upgraded, it gains improved speed, combat performance and capacity as well as improved anti-fighter barrage."],
            ),
            Technology::PrototypeWarSunII => t!(
                "Prototype War Sun II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::EmbersOfMuaat),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 3)]),
                Expansion::Base,
                &["Upgraded Muaat War Suns are faster and cheaper than regular War Suns."],
            ),
            Technology::StrikeWingAlphaII => t!(
                "Strike Wing Alpha II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::ProphecyOfKings,
            ),
            Technology::HybridCrystalFighterII => t!(
                "Hybrid Crystal Fighter II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 1), (TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &["Upgraded Naalu Fighters are better in combat compared to regular upgraded Fighters and only count as 1/2 to the fleet supply if not carried."],
            ),
            Technology::HelTitanII => t!(
                "Hel Titan II",
                TechType::UnitUpgrade,
                TechOrigin::Faction(Faction::TitansOfUl),
                tr!([(TechCategory::Cybernetic, 1), (TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &["Upgraded Titans of Ul PDS have improved Space cannon and are able to fire into adjacent systems."],
            ),
            Technology::Voidwatch => t!(
                "Voidwatch",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ProphecyOfKings,
                &["After a player moves ships into a system that contains 1 or more of your units, they must give you 1 promissory note from their hand, if able."],
            ),
            Technology::InstinctTraining => t!(
                "Instinct Training",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::Base,
                &["You may exhaust this card and spend 1 token from your strategy pool when another player plays an action card; cancel that action card."],
            ),
            Technology::TransparasteelPlating => t!(
                "Transparasteel Plating",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YssarilTribes),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::Base,
                &["During your turn of the action phase, players that have passed cannot play action cards."],
            ),
            Technology::GeneticRecombination => t!(
                "Genetic Recombination",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::MahactGeneSorcerers),
                tr!([(TechCategory::Biotic, 1)]),
                Expansion::ProphecyOfKings,
                &["You may exhaust this card before a player casts votes; that player must cast at least 1 vote for an outcome of your choice or remove 1 token from their fleet pool and return it to their reinforcements."],
            ),
            Technology::Bioplasmosis => t!(
                "Bioplasmosis",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::Arborec),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["At the end of the status phase, you may remove any number of infantry from planets you control and place them on 1 or more planets you control in the same or adjacent systems."],
            ),
            Technology::ProductionBiomes => t!(
                "Production Biomes",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["ACTION: Exhaust this card and spend 1 token from your strategy pool to gain 4 trade goods and choose 1 other player; that player gains 2 trade goods."],
            ),
            Technology::YinSpinner => t!(
                "Yin Spinner",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Base,
                &["After 1 or more of your units use Production, place 1 infantry from your reinforcements on a planet you control in that system."],
            ),
            Technology::YinSpinnerOmega => t!(
                "Yin Spinner Ω",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Biotic, 2)]),
                Expansion::Codex,
                &["After you produce units, place up to 2 infantry from your reinforcements on any planet you control or in any space area that contains 1 or more of your ships."],
            ),
            Technology::Neuroglaive => t!(
                "Neuroglaive",
                TechType::Category(TechCategory::Biotic),
                TechOrigin::Faction(Faction::NaaluCollective),
                tr!([(TechCategory::Biotic, 3)]),
                Expansion::Base,
                &["After another player activates a system that contains 1 or more of your ships, that player removes 1 token from their fleet pool and returns it to their reinforcements."],
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
                &["ACTION: Exhaust this card to look at another player's hand of action cards. Choose 1 of those cards and add it to your hand."],
            ),
            Technology::ChaosMapping => t!(
                "Chaos Mapping",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::ClanOfSaar),
                tr!([(TechCategory::Propulsion, 1)]),
                Expansion::Base,
                &["Other players cannot activate asteroid fields that contain 1 or more of your ships."],
            ),
            Technology::SpacialConduitCylinder => t!(
                "Spatial Conduit Cylinder",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["You may exhaust this card after you activate a system that contains 1 or more of your units; that system is adjacent to all other systems that contain 1 or more of your units during this activation."],
            ),
            Technology::Aetherstream => t!(
                "Aetherstream",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Empyrean),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::ProphecyOfKings,
                &["After you or one of your neighbors activates a system that is adjacent to an anomaly, you may apply +1 to the move value of all of that player's ships during this tactical action"],
            ),
            Technology::WormholeGenerator => t!(
                "Wormhole Generator",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["At the start of the status phase, place or move a Creuss wormhole token into either a system that contains a planet you control or a non-home system that does not contain another player's ships."],
            ),
            Technology::WormholeGeneratorOmega => t!(
                "Wormhole Generator Ω",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Codex,
                &["ACTION: Exhaust this card to place or move a Creuss wormhole token into either a system that contains a planet you control or a non-home system that does not contain another player's ships."],
            ),
            Technology::LazaxGateFolding => t!(
                "Lazax Gate Folding",
                TechType::Category(TechCategory::Propulsion),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Propulsion, 2)]),
                Expansion::Base,
                &["During your tactical actions, if you do not control Mecatol Rex, treat its system as if it has both an α and β wormhole. ACTION: If you control Mecatol Rex, exhaust this card to place 1 infantry from your reinforcements on Mecatol Rex."],
            ),
            Technology::AerieHololattice => t!(
                "Aerie Hololattice",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::ArgentFlight),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::ProphecyOfKings,
                &["Other players cannot move ships through systems that contain your structures"],
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
                &["After any player's agent becomes exhausted, you may exhaust this card to ready that agent; if you ready another player's agent, you may perform a transaction with that player."],
            ),
            Technology::IIHQModernization => t!(
                "I.I.H.Q Modernization",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 1)]),
                Expansion::Base,
                &["You are neighbors with all players that have units or control planets in or adjacent to the Mecatol Rex system."],
            ),
            Technology::SalvageOperations => t!(
                "Salvage Operations",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["After you win or lose a space combat, gain 1 trade good; if you won the combat, you may also produce 1 ship in that system of any ship type that was destroyed during the combat."],
            ),
            Technology::InheritanceSystems => t!(
                "Inheritance Systems",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::L1Z1XMindnet),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["You may exhaust this card and spend 2 resources when you research a technology; ignore all of that technology's prerequisites."],
            ),
            Technology::EResSiphons => t!(
                "E-Res Siphons",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::UniversitiesOfJolNar),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["After another player activates a system that contains 1 or more of your ships, gain 4 trade goods."],
            ),
            Technology::HegemonicTradePolicy => t!(
                "Hegemonic Trade Policy",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::Winnu),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["Exhaust this card when 1 or more of your units use PRODUCTION; swap the resource and influence values of 1 planet you control until the end of your turn."],
            ),
            Technology::NullificationField => t!(
                "Nullification Field",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::XxchaKingdom),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["After another player activates a system that contains 1 or more of your ships, you may exhaust this card and spend 1 token from your strategy pool; immediately end that player's turn."],
            ),
            Technology::CoreImpulse => t!(
                "Impulse Core",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::YinBrotherhood),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["At the start of a space combat, you may destroy 1 of your cruisers or destroyers in the active system to produce 1 hit against your opponent's ships; that hit must be assigned by your opponent to 1 of their non-fighters ships if able."],
            ),
            Technology::AgencySupplyNetwork => t!(
                "Agency Supply Network",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::CouncilKeleres),
                tr!([(TechCategory::Cybernetic, 2)]),
                Expansion::Base,
                &["Whenever you resolve one of your PRODUCTION abilities, you may resolve an additional one of your PRODUCTION abilities in any system; the additional use does not trigger this ability."],
            ),
            Technology::QuantumDatahubNode => t!(
                "Quantum Datahub Node",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::EmiratesOfHacan),
                tr!([(TechCategory::Cybernetic, 3)]),
                Expansion::Base,
                &["At the end of the strategy phase, you may spend 1 token from your strategy pool and give another player 3 of your trade goods. If you do, give 1 of your strategy cards to that player and take 1 of their strategy cards."],
            ),
            Technology::MirrorComputing => t!(
                "Mirror Computing",
                TechType::Category(TechCategory::Cybernetic),
                TechOrigin::Faction(Faction::MentakCoalition),
                tr!([(TechCategory::Cybernetic, 3)]),
                Expansion::Base,
                &["When you spend trade goods, each trade good is worth 2 resources or influence instead of 1."],
            ),
            Technology::DimensionalSplicer => t!(
                "Dimensional Splicer",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::GhostsOfCreuss),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::Base,
                &["At the start of space combat in a system that contains a wormhole and 1 or more of your ships, you may produce 1 hit and assign it to 1 of your opponent's ships."],
            ),
            Technology::Supercharge => t!(
                "Supercharge",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::NaazRokhaAlliance),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &["At the start of a combat round, you may exhaust this card to apply +1 to the result of each of your unit's combat rolls during this combat round."],
            ),
            Technology::Vortex => t!(
                "Vortex",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::VuilRaithCabal),
                tr!([(TechCategory::Warfare, 1)]),
                Expansion::ProphecyOfKings,
                &["ACTION: Exhaust this card to choose another player's non-structure unit in a system that is adjacent to 1 or more of your space docks. Capture 1 unit of that type from that player's reinforcements."],
            ),
            Technology::NonEuclidianShielding => t!(
                "Non-Euclidian Shielding",
                TechType::Category(TechCategory::Warfare),
                TechOrigin::Faction(Faction::BaronyOfLetnev),
                tr!([(TechCategory::Warfare, 2)]),
                Expansion::Base,
                &["When 1 of your units uses Sustain Damage, cancel 2 hits instead of 1."],
            ),
            Technology::MagmusReactor => t!(
                "Magmus Reactor/ Magmus Reactor Ω",
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
                Expansion::Base,
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
                &["After making combat rolls during a round of ground combat, if your opponent produced 1 or more hits, you produce 1 additional hit."],
            ),
        }
    }
}
