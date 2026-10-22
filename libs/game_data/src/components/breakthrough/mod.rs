use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use strum_macros::EnumIter;

use crate::{
    common::faction::Faction::{self},
    components::tech::TechCategory,
};

/// Breakthroughs.
#[derive(
    Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter, EnumString, Display,
)]
#[strum(serialize_all = "snake_case")]
#[allow(missing_docs)]
pub enum Breakthrough {
    Psychospore,
    WingTransfer,
    GravleashManeuvers,
    DeorbitBarrage,
    IIHQModernization,
    ResonanceGenerator,
    VisionariaSelect,
    StellarGenesis,
    AutoFactories,
    VoidTether,
    BellumGloriosum,
    TheSowing,
    TheReaping,
    ParticleSynthesis,
    FealtyUplink,
    TheIcon,
    VaultsOfTheHeir,
    TheTablesGrace,
    Corsair,
    Mindsieve,
    AbsoluteSynergy,
    EidolonMaximum,
    ValefarAssimilatorZ,
    ThundersParadox,
    DataSkimmer,
    NorrSupremacy,
    SlumberstateComputing,
    SpecializedCompounds,
    AlRaithIxIanovar,
    Imperator,
    ArchonsGift,
    YinAscendant,
    DeepgloomExecutable,
}

/// Information about a breakthrough, i.e. if it's an Action, etc.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BreakthroughKind {
    /// Breakthrough has an Action.
    Action,
    /// Breakthrough is not one of the other alternatives.
    Other,
}

/// All relevant information for a breakthrough.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BreakthroughInfo {
    pub breakthrough: Breakthrough,
    pub name: String,
    pub description: String,
    pub faction: Faction,
    pub synergy: Option<(TechCategory, TechCategory)>,
    pub kind: BreakthroughKind,
}

macro_rules! info {
    (tag: $breakthrough:ident, name: $name:expr, faction: $faction:ident, synergies: $synergies:expr, kind: $kind:ident) => {
        BreakthroughInfo {
            breakthrough: Breakthrough::$breakthrough,
            name: $name.to_string(),
            faction: Faction::$faction,
            description: include_str!(concat!("description/", stringify!($breakthrough)))
                .to_string(),
            synergy: $synergies,
            kind: BreakthroughKind::$kind
        }
    };
    (tag: $breakthrough:ident, name: $name:expr, faction: $faction:ident, syn_a: $synergy_a:ident, syn_b: $synergy_b:ident, kind: $kind:ident) => {
        info! { tag: $breakthrough, name: $name, faction: $faction, synergies: Some((TechCategory::$synergy_a, TechCategory::$synergy_b)), kind: $kind }
    };
    (tag: $breakthrough:ident, name: $name:expr, faction: $faction:ident, syn_a: $synergy_a:ident, syn_b: $synergy_b:ident) => {
        info! { tag: $breakthrough, name: $name, faction: $faction, syn_a: $synergy_a, syn_b: $synergy_b, kind: Other }
    };
    (tag: $breakthrough:ident, name: $name:expr, faction: $faction:ident) => {
        info! { tag: $breakthrough, name: $name, faction: $faction, synergies: None, kind: Other }
    };
}

impl Breakthrough {
    /// Get the [BreakthroughInfo] of this [Breakthrough].
    pub fn info(&self) -> BreakthroughInfo {
        match self {
            Breakthrough::Psychospore => info! {
                tag: Psychospore,
                name: "Psychospore",
                faction: Arborec,
                syn_a: Warfare,
                syn_b: Biotic,
                kind: Action
            },
            Breakthrough::WingTransfer => info! {
                tag: WingTransfer,
                name: "Wing Transfer",
                faction: ArgentFlight,
                syn_a: Propulsion,
                syn_b: Cybernetic
            },
            Breakthrough::GravleashManeuvers => info! {
                tag: GravleashManeuvers,
                name: "Gravleash Maneuvers",
                faction: BaronyOfLetnev,
                syn_a: Propulsion,
                syn_b: Warfare
            },
            Breakthrough::DeorbitBarrage => info! {
                tag: DeorbitBarrage,
                name: "Deorbit Barrage",
                faction: ClanOfSaar,
                syn_a: Propulsion,
                syn_b: Warfare
            },
            Breakthrough::IIHQModernization => info! {
                tag: IIHQModernization,
                name: "I.I.H.Q. Modernization",
                faction: CouncilKeleres,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::ResonanceGenerator => info! {
                tag: ResonanceGenerator,
                name: "Resonance Generator",
                faction: CrimsonRebellion,
                syn_a: Propulsion,
                syn_b: Warfare,
                kind: Action
            },
            Breakthrough::VisionariaSelect => info! {
                tag: VisionariaSelect,
                name: "Visionaria Select",
                faction: DeepwroughtScholarate,
                syn_a: Cybernetic,
                syn_b: Biotic,
                kind: Action
            },
            Breakthrough::StellarGenesis => info! {
                tag: StellarGenesis,
                name: "Stellar Genesis",
                faction: EmbersOfMuaat,
                syn_a: Warfare,
                syn_b: Cybernetic
            },
            Breakthrough::AutoFactories => info! {
                tag: AutoFactories,
                name: "Auto-Factories",
                faction: EmiratesOfHacan,
                syn_a: Warfare,
                syn_b: Cybernetic
            },
            Breakthrough::VoidTether => info! {
                tag: VoidTether,
                name: "Void Tether",
                faction: Empyrean,
                syn_a: Biotic,
                syn_b: Propulsion
            },
            Breakthrough::BellumGloriosum => info! {
                tag: BellumGloriosum,
                name: "Bellum Gloriosum",
                faction: FederationOfSol,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::TheSowing => info! {
                tag: TheSowing,
                name: "The Sowing",
                faction: FirmamentObsidian,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::TheReaping => info! {
                tag: TheReaping,
                name: "The Reaping",
                faction: FirmamentObsidian,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::ParticleSynthesis => info! {
                tag: ParticleSynthesis,
                name: "Particle Synthesis",
                faction: GhostsOfCreuss,
                syn_a: Propulsion,
                syn_b: Cybernetic
            },
            Breakthrough::FealtyUplink => info! {
                tag: FealtyUplink,
                name: "Fealty Uplink",
                faction: L1Z1XMindnet,
                syn_a: Warfare,
                syn_b: Biotic
            },
            Breakthrough::TheIcon => info! {
                tag: TheIcon,
                name: "The Icon",
                faction: LastBastion,
                syn_a: Warfare,
                syn_b: Cybernetic
            },
            Breakthrough::VaultsOfTheHeir => info! {
                tag: VaultsOfTheHeir,
                name: "Vaults of the Heir",
                faction: MahactGeneSorcerers,
                syn_a: Cybernetic,
                syn_b: Biotic,
                kind: Action
            },
            Breakthrough::TheTablesGrace => info! {
                tag: TheTablesGrace,
                name: "The Table's Grace",
                faction: MentakCoalition,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::Corsair => info! {
                tag: Corsair,
                name: "Corsair",
                faction: MentakCoalition,
                syn_a: Biotic,
                syn_b: Cybernetic
            },
            Breakthrough::Mindsieve => info! {
                tag: Mindsieve,
                name: "Mindsieve",
                faction: NaaluCollective,
                syn_a: Warfare,
                syn_b: Biotic
            },
            Breakthrough::AbsoluteSynergy => info! {
                tag: AbsoluteSynergy,
                name: "Absolute Synergy",
                faction: NaazRokhaAlliance,
                syn_a: Biotic,
                syn_b: Propulsion
            },
            Breakthrough::EidolonMaximum => info! {
                tag: EidolonMaximum,
                name: "Eidolon Maximum",
                faction: NaazRokhaAlliance,
                syn_a: Biotic,
                syn_b: Propulsion
            },
            Breakthrough::ValefarAssimilatorZ => info! {
                tag: ValefarAssimilatorZ,
                name: "Valefar Assimilator Z",
                faction: NekroVirus
            },
            Breakthrough::ThundersParadox => info! {
                tag: ThundersParadox,
                name: "Thunder's Paradox",
                faction: Nomad,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::DataSkimmer => info! {
                tag: DataSkimmer,
                name: "Data Skimmer",
                faction: RalNelConsortium,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::NorrSupremacy => info! {
                tag: NorrSupremacy,
                name: "Norr Supremacy",
                faction: SardakkNorr,
                syn_a: Propulsion,
                syn_b: Warfare
            },
            Breakthrough::SlumberstateComputing => info! {
                tag: SlumberstateComputing,
                name: "Slumberstate Computing",
                faction: TitansOfUl,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::SpecializedCompounds => info! {
                tag: SpecializedCompounds,
                name: "Specialized Compounds",
                faction: UniversitiesOfJolNar,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::AlRaithIxIanovar => info! {
                tag: AlRaithIxIanovar,
                name: "Al'Raith Ix Ianovar",
                faction: VuilRaithCabal,
                syn_a: Warfare,
                syn_b: Biotic
            },
            Breakthrough::Imperator => info! {
                tag: Imperator,
                name: "Imperator",
                faction: Winnu,
                syn_a: Propulsion,
                syn_b: Warfare
            },
            Breakthrough::ArchonsGift => info! {
                tag: ArchonsGift,
                name: "Archon's Gift",
                faction: XxchaKingdom,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::YinAscendant => info! {
                tag: YinAscendant,
                name: "Yin Ascendant",
                faction: YinBrotherhood,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
            Breakthrough::DeepgloomExecutable => info! {
                tag: DeepgloomExecutable,
                name: "Deepgloom Executable",
                faction: YssarilTribes,
                syn_a: Cybernetic,
                syn_b: Biotic
            },
        }
    }
}
