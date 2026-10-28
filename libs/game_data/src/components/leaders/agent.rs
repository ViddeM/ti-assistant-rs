use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::common::{expansions::Expansion, faction::Faction};

use super::LeaderAbilityKind;

/// Information about an agent leader.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentInfo {
    /// [Agent] variant for this agent.
    pub tag: Agent,

    /// Faction that this agent belongs to.
    pub faction: Faction,

    /// Expansion that this agent comes with.
    pub expansion: Expansion,

    /// Name of the agent.
    pub name: String,

    /// Description of the agents ability.
    pub description: String,

    /// The kind of ability, i.e. whether it's an action or something else.
    pub kind: LeaderAbilityKind,
}

/// An agent leader.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
#[allow(missing_docs)]
pub enum Agent {
    Acamar,
    ArtunoTheBetrayer,
    BerekarBerekon,
    BrotherMilor,
    BrotherMilorCxIII, // patch of Brother Milor from Codex III
    CaptainMendosa,
    CarthOfGoldenSands,
    CleverCleverSsruu,
    DoctorSucaban,
    EmissaryTaivra,
    EvelynDelouis,
    FieldMarshallMercer,
    GarvAndGunn,
    GgrucotoRinn,
    I48S,
    JaeMirKan,
    LetaniOspha,
    NekroMalleon,
    SuffiAn,
    Tellurian,
    TheStillnessOfStars,
    TheThundarian,
    TrillossaAunMirik,
    Tro,
    Umbat,
    ViscountUnlenn,
    Zeu,
    ZeuCxIII, // patch of Zeu from Codex III
    XanderAlexinVictoriIII,
    /// Thunder's Edge
    DameBriar,
    KanKipRel,
    DoctorCarrina,
    AhkRavin,
    MyruVos,
    VosHollow,
}

macro_rules! info {
    (tag: $agent:ident, name: $name:expr, faction: $faction:ident, expansion: $expansion:ident, kind: $kind:ident,) => {
        AgentInfo {
            tag: Agent::$agent,
            faction: Faction::$faction,
            expansion: Expansion::$expansion,
            name: $name.to_string(),
            description: include_str!(concat!("description/", stringify!($agent))).to_string(),
            kind: LeaderAbilityKind::$kind,
        }
    };
}

impl Agent {
    /// Get the [AgentInfo] of this [Agent].
    pub fn info(&self) -> AgentInfo {
        match self {
            Agent::LetaniOspha => info! {
                tag: LetaniOspha,
                name: "Letani Ospha",
                faction: Arborec,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Agent::TrillossaAunMirik => info! {
                tag: TrillossaAunMirik,
                name: "Trillossa Aun Mirik",
                faction: ArgentFlight,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::ViscountUnlenn => info! {
                tag: ViscountUnlenn,
                name: "Viscount Unlenn",
                faction: BaronyOfLetnev,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::CaptainMendosa => info! {
                tag: CaptainMendosa,
                name: "Captain Mendosa",
                faction: ClanOfSaar,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::Umbat => info! {
                tag: Umbat,
                name: "Umbat",
                faction: EmbersOfMuaat,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Agent::CarthOfGoldenSands => info! {
                tag: CarthOfGoldenSands,
                name: "Carth of Golden Sands",
                faction: EmiratesOfHacan,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::Acamar => info! {
                tag: Acamar,
                name: "Acamar",
                faction: Empyrean,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::EvelynDelouis => info! {
                tag: EvelynDelouis,
                name: "Evelyn Delouis",
                faction: FederationOfSol,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::EmissaryTaivra => info! {
                tag: EmissaryTaivra,
                name: "Emissary Taivra",
                faction: GhostsOfCreuss,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::I48S => info! {
                tag: I48S,
                name: "I48S",
                faction: L1Z1XMindnet,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::JaeMirKan => info! {
                tag: JaeMirKan,
                name: "Jae Mir Kan",
                faction: MahactGeneSorcerers,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::SuffiAn => info! {
                tag: SuffiAn,
                name: "Suffi An",
                faction: MentakCoalition,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::Zeu => info! {
                tag: Zeu,
                name: "Z'eu",
                faction: NaaluCollective,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::ZeuCxIII => info! {
                tag: ZeuCxIII,
                name: "Z'eu Ω",
                faction: NaaluCollective,
                expansion: CodexIII,
                kind: Action,
            },
            Agent::GarvAndGunn => info! {
                tag: GarvAndGunn,
                name: "Garv and Gunn",
                faction: NaazRokhaAlliance,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::NekroMalleon => info! {
                tag: NekroMalleon,
                name: "Nekro Malleon",
                faction: NekroVirus,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::Tro => info! {
                tag: Tro,
                name: "T'ro",
                faction: SardakkNorr,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::Tellurian => info! {
                tag: Tellurian,
                name: "Tellurian",
                faction: TitansOfUl,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::DoctorSucaban => info! {
                tag: DoctorSucaban,
                name: "Doctor Sucaban",
                faction: UniversitiesOfJolNar,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::TheStillnessOfStars => info! {
                tag: TheStillnessOfStars,
                name: "The Stillness of Stars",
                faction: VuilRaithCabal,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::BerekarBerekon => info! {
                tag: BerekarBerekon,
                name: "Berekar Berekon",
                faction: Winnu,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::GgrucotoRinn => info! {
                tag: GgrucotoRinn,
                name: "Ggrucoto Rinn",
                faction: XxchaKingdom,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Agent::BrotherMilor => info! {
                tag: BrotherMilor,
                name: "Brother Milor",
                faction: YinBrotherhood,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::BrotherMilorCxIII => info! {
                tag: BrotherMilorCxIII,
                name: "Brother Milor Ω",
                faction: YinBrotherhood,
                expansion: CodexIII,
                kind: Other,
            },
            Agent::CleverCleverSsruu => info! {
                tag: CleverCleverSsruu,
                name: "Clever Clever Ssruu",
                faction: YssarilTribes,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::ArtunoTheBetrayer => info! {
                tag: ArtunoTheBetrayer,
                name: "Artuno the Betrayer",
                faction: Nomad,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::FieldMarshallMercer => info! {
                tag: FieldMarshallMercer,
                name: "Field Marshal Mercer",
                faction: Nomad,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::TheThundarian => info! {
                tag: TheThundarian,
                name: "The Thundarian",
                faction: Nomad,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Agent::XanderAlexinVictoriIII => info! {
                tag: XanderAlexinVictoriIII,
                name: "Xander Alexin Victori III",
                faction: CouncilKeleres,
                expansion: CodexIII,
                kind: Other,
            },
            Agent::DameBriar => info! {
                tag: DameBriar,
                name: "Dame Briar",
                faction: LastBastion,
                expansion: ThundersEdge,
                kind: Other,
            },
            Agent::KanKipRel => info! {
                tag: KanKipRel,
                name: "Kan Kip Rel",
                faction: RalNelConsortium,
                expansion: ThundersEdge,
                kind: Action,
            },
            Agent::DoctorCarrina => info! {
                tag: DoctorCarrina,
                name: "Doctor Carrina",
                faction: DeepwroughtScholarate,
                expansion: ThundersEdge,
                kind: Other,
            },
            Agent::AhkRavin => info! {
                tag: AhkRavin,
                name: "Ahk Ravin",
                faction: CrimsonRebellion,
                expansion: ThundersEdge,
                kind: Action,
            },
            Agent::MyruVos => info! {
                tag: MyruVos,
                name: "Myru Vos",
                faction: FirmamentObsidian,
                expansion: ThundersEdge,
                kind: Other,
            },
            Agent::VosHollow => info! {
                tag: VosHollow,
                name: "Vos Hollow",
                faction: FirmamentObsidian,
                expansion: ThundersEdge,
                kind: Other,
            },
        }
    }
}
