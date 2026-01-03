use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::common::faction::Faction;

use super::LeaderAbilityKind;

/// Information about an agent leader.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentInfo {
    /// [Agent] variant for this agent.
    pub tag: Agent,

    /// Faction that this agent belongs to.
    pub faction: Faction,

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
}

macro_rules! info {
    (tag: $agent:ident, name: $name:expr, faction: $faction:ident, kind: $kind:ident,) => {
        AgentInfo {
            tag: Agent::$agent,
            faction: Faction::$faction,
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
                kind: Action,
            },
            Agent::TrillossaAunMirik => info! {
                tag: TrillossaAunMirik,
                name: "Trillossa Aun Mirik",
                faction: ArgentFlight,
                kind: Other,
            },
            Agent::ViscountUnlenn => info! {
                tag: ViscountUnlenn,
                name: "Viscount Unlenn",
                faction: BaronyOfLetnev,
                kind: Other,
            },
            Agent::CaptainMendosa => info! {
                tag: CaptainMendosa,
                name: "Captain Mendosa",
                faction: ClanOfSaar,
                kind: Other,
            },
            Agent::Umbat => info! {
                tag: Umbat,
                name: "Umbat",
                faction: EmbersOfMuaat,
                kind: Action,
            },
            Agent::CarthOfGoldenSands => info! {
                tag: CarthOfGoldenSands,
                name: "Carth of Golden Sands",
                faction: EmiratesOfHacan,
                kind: Other,
            },
            Agent::Acamar => info! {
                tag: Acamar,
                name: "Acamar",
                faction: Empyrean,
                kind: Other,
            },
            Agent::EvelynDelouis => info! {
                tag: EvelynDelouis,
                name: "Evelyn Delouis",
                faction: FederationOfSol,
                kind: Other,
            },
            Agent::EmissaryTaivra => info! {
                tag: EmissaryTaivra,
                name: "Emissary Taivra",
                faction: GhostsOfCreuss,
                kind: Other,
            },
            Agent::I48S => info! {
                tag: I48S,
                name: "I48S",
                faction: L1Z1XMindnet,
                kind: Other,
            },
            Agent::JaeMirKan => info! {
                tag: JaeMirKan,
                name: "Jae Mir Kan",
                faction: MahactGeneSorcerers,
                kind: Other,
            },
            Agent::SuffiAn => info! {
                tag: SuffiAn,
                name: "Suffi An",
                faction: MentakCoalition,
                kind: Other,
            },
            Agent::Zeu => info! {
                tag: Zeu,
                name: "Z'eu",
                faction: NaaluCollective,
                kind: Other,
            },
            Agent::ZeuCxIII => info! {
                tag: ZeuCxIII,
                name: "Z'eu Ω",
                faction: NaaluCollective,
                kind: Action,
            },
            Agent::GarvAndGunn => info! {
                tag: GarvAndGunn,
                name: "Garv and Gunn",
                faction: NaazRokhaAlliance,
                kind: Other,
            },
            Agent::NekroMalleon => info! {
                tag: NekroMalleon,
                name: "Nekro Malleon",
                faction: NekroVirus,
                kind: Other,
            },
            Agent::Tro => info! {
                tag: Tro,
                name: "T'ro",
                faction: SardakkNorr,
                kind: Other,
            },
            Agent::Tellurian => info! {
                tag: Tellurian,
                name: "Tellurian",
                faction: TitansOfUl,
                kind: Other,
            },
            Agent::DoctorSucaban => info! {
                tag: DoctorSucaban,
                name: "Doctor Sucaban",
                faction: UniversitiesOfJolNar,
                kind: Other,
            },
            Agent::TheStillnessOfStars => info! {
                tag: TheStillnessOfStars,
                name: "The Stillness of Stars",
                faction: VuilRaithCabal,
                kind: Other,
            },
            Agent::BerekarBerekon => info! {
                tag: BerekarBerekon,
                name: "Berekar Berekon",
                faction: Winnu,
                kind: Other,
            },
            Agent::GgrucotoRinn => info! {
                tag: GgrucotoRinn,
                name: "Ggrucoto Rinn",
                faction: XxchaKingdom,
                kind: Action,
            },
            Agent::BrotherMilor => info! {
                tag: BrotherMilor,
                name: "Brother Milor",
                faction: YinBrotherhood,
                kind: Other,
            },
            Agent::BrotherMilorCxIII => info! {
                tag: BrotherMilorCxIII,
                name: "Brother Milor Ω",
                faction: YinBrotherhood,
                kind: Other,
            },
            Agent::CleverCleverSsruu => info! {
                tag: CleverCleverSsruu,
                name: "Clever Clever Ssruu",
                faction: YssarilTribes,
                kind: Other,
            },
            Agent::ArtunoTheBetrayer => info! {
                tag: ArtunoTheBetrayer,
                name: "Artuno the Betrayer",
                faction: Nomad,
                kind: Other,
            },
            Agent::FieldMarshallMercer => info! {
                tag: FieldMarshallMercer,
                name: "Field Marshall Mercer",
                faction: Nomad,
                kind: Other,
            },
            Agent::TheThundarian => info! {
                tag: TheThundarian,
                name: "The Thundarian",
                faction: Nomad,
                kind: Other,
            },
            Agent::XanderAlexinVictoriIII => info! {
                tag: XanderAlexinVictoriIII,
                name: "Xander Alexin Victori III",
                faction: CouncilKeleres,
                kind: Other,
            },
        }
    }
}
