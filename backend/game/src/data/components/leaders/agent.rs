use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::data::common::faction::Faction;

/// Information about an agent leader.
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
pub struct AgentInfo {
    /// [Agent] variant for this agent.
    pub tag: Agent,

    /// Faction that this agent belongs to.
    pub faction: Faction,

    /// Name of the agent.
    pub name: &'static str,

    /// Description of the agents ability.
    pub description: &'static str,
}

/// An agent leader.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
#[allow(missing_docs)]
#[derive(ts_rs::TS)]
#[ts(export)]
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
    (tag: $agent:ident, name: $name:expr, faction: $faction:ident,) => {
        AgentInfo {
            tag: Agent::$agent,
            faction: Faction::$faction,
            name: $name,
            description: include_str!(concat!("description/", stringify!($agent))),
        }
    };
}

impl Agent {
    /// Get the [AgentInfo] of this [Agent].
    pub const fn info(&self) -> AgentInfo {
        match self {
            Agent::LetaniOspha => info! {
                tag: LetaniOspha,
                name: "Letani Ospha",
                faction: Arborec,
            },
            Agent::TrillossaAunMirik => info! {
                tag: TrillossaAunMirik,
                name: "Trillossa Aun Mirik",
                faction: ArgentFlight,
            },
            Agent::ViscountUnlenn => info! {
                tag: ViscountUnlenn,
                name: "Viscount Unlenn",
                faction: BaronyOfLetnev,
            },
            Agent::CaptainMendosa => info! {
                tag: CaptainMendosa,
                name: "Captain Mendosa",
                faction: ClanOfSaar,
            },
            Agent::Umbat => info! {
                tag: Umbat,
                name: "Umbat",
                faction: EmbersOfMuaat,
            },
            Agent::CarthOfGoldenSands => info! {
                tag: CarthOfGoldenSands,
                name: "Carth of Golden Sands",
                faction: EmiratesOfHacan,
            },
            Agent::Acamar => info! {
                tag: Acamar,
                name: "Acamar",
                faction: Empyrean,
            },
            Agent::EvelynDelouis => info! {
                tag: EvelynDelouis,
                name: "Evelyn Delouis",
                faction: FederationOfSol,
            },
            Agent::EmissaryTaivra => info! {
                tag: EmissaryTaivra,
                name: "Emissary Taivra",
                faction: GhostsOfCreuss,
            },
            Agent::I48S => info! {
                tag: I48S,
                name: "I48S",
                faction: L1Z1XMindnet,
            },
            Agent::JaeMirKan => info! {
                tag: JaeMirKan,
                name: "Jae Mir Kan",
                faction: MahactGeneSorcerers,
            },
            Agent::SuffiAn => info! {
                tag: SuffiAn,
                name: "Suffi An",
                faction: MentakCoalition,
            },
            Agent::Zeu => info! {
                tag: Zeu,
                name: "Z'eu",
                faction: NaaluCollective,
            },
            Agent::ZeuCxIII => info! {
                tag: ZeuCxIII,
                name: "Z'eu Ω",
                faction: NaaluCollective,
            },
            Agent::GarvAndGunn => info! {
                tag: GarvAndGunn,
                name: "Garv and Gunn",
                faction: NaazRokhaAlliance,
            },
            Agent::NekroMalleon => info! {
                tag: NekroMalleon,
                name: "Nekro Malleon",
                faction: NekroVirus,
            },
            Agent::Tro => info! {
                tag: Tro,
                name: "T'ro",
                faction: SardakkNorr,
            },
            Agent::Tellurian => info! {
                tag: Tellurian,
                name: "Tellurian",
                faction: TitansOfUl,
            },
            Agent::DoctorSucaban => info! {
                tag: DoctorSucaban,
                name: "Doctor Sucaban",
                faction: UniversitiesOfJolNar,
            },
            Agent::TheStillnessOfStars => info! {
                tag: TheStillnessOfStars,
                name: "The Stillness of Stars",
                faction: VuilRaithCabal,
            },
            Agent::BerekarBerekon => info! {
                tag: BerekarBerekon,
                name: "Berekar Berekon",
                faction: Winnu,
            },
            Agent::GgrucotoRinn => info! {
                tag: GgrucotoRinn,
                name: "Ggrucoto Rinn",
                faction: XxchaKingdom,
            },
            Agent::BrotherMilor => info! {
                tag: BrotherMilor,
                name: "Brother Milor",
                faction: YinBrotherhood,
            },
            Agent::BrotherMilorCxIII => info! {
                tag: BrotherMilorCxIII,
                name: "Brother Milor Ω",
                faction: YinBrotherhood,
            },
            Agent::CleverCleverSsruu => info! {
                tag: CleverCleverSsruu,
                name: "Clever Clever Ssruu",
                faction: YssarilTribes,
            },
            Agent::ArtunoTheBetrayer => info! {
                tag: ArtunoTheBetrayer,
                name: "Artuno the Betrayer",
                faction: Nomad,
            },
            Agent::FieldMarshallMercer => info! {
                tag: FieldMarshallMercer,
                name: "Field Marshall Mercer",
                faction: Nomad,
            },
            Agent::TheThundarian => info! {
                tag: TheThundarian,
                name: "The Thundarian",
                faction: Nomad,
            },
            Agent::XanderAlexinVictoriIII => info! {
                tag: XanderAlexinVictoriIII,
                name: "Xander Alexin Victori III",
                faction: CouncilKeleres,
            },
        }
    }
}
