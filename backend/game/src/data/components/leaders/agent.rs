use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::data::common::faction::Faction;

/// Information about an agent leader.
#[derive(Clone, Debug, Serialize)]
pub struct AgentInfo {
    /// [Agent] variant for this agent.
    pub agent: Agent,

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
pub enum Agent {
    Acamar,
    ArtunoTheBetrayer,
    BerekarBerekon,
    BrotherMilor,
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
}

macro_rules! info {
    (agent: $agent:ident, name: $name:expr, faction: $faction:ident,) => {
        AgentInfo {
            agent: Agent::$agent,
            faction: Faction::$faction,
            name: $name,
            description: include_str!(concat!("description/", $name)),
        }
    };
}

impl Agent {
    /// Get the [AgentInfo] of this [Agent].
    pub fn info(&self) -> AgentInfo {
        match self {
            Agent::LetaniOspha => info! {
                agent: LetaniOspha,
                name: "Letani Ospha",
                faction: Arborec,
            },
            Agent::TrillossaAunMirik => info! {
                agent: TrillossaAunMirik,
                name: "Trillossa Aun Mirik",
                faction: ArgentFlight,
            },
            Agent::ViscountUnlenn => info! {
                agent: ViscountUnlenn,
                name: "Viscount Unlenn",
                faction: BaronyOfLetnev,
            },
            Agent::CaptainMendosa => info! {
                agent: CaptainMendosa,
                name: "Captain Mendosa",
                faction: ClanOfSaar,
            },
            Agent::Umbat => info! {
                agent: Umbat,
                name: "Umbat",
                faction: EmbersOfMuaat,
            },
            Agent::CarthOfGoldenSands => info! {
                agent: CarthOfGoldenSands,
                name: "Carth of Golden Sands",
                faction: EmiratesOfHacan,
            },
            Agent::Acamar => info! {
                agent: Acamar,
                name: "Acamar",
                faction: Empyrean,
            },
            Agent::EvelynDelouis => info! {
                agent: EvelynDelouis,
                name: "Evelyn Delouis",
                faction: FederationOfSol,
            },
            Agent::EmissaryTaivra => info! {
                agent: EmissaryTaivra,
                name: "Emissary Taivra",
                faction: GhostsOfCreuss,
            },
            Agent::I48S => info! {
                agent: I48S,
                name: "I48S",
                faction: L1Z1XMindnet,
            },
            Agent::JaeMirKan => info! {
                agent: JaeMirKan,
                name: "Jae Mir Kan",
                faction: MahactGeneSorcerers,
            },
            Agent::SuffiAn => info! {
                agent: SuffiAn,
                name: "Suffi An",
                faction: MentakCoalition,
            },
            Agent::Zeu => info! {
                agent: Zeu,
                name: "Z'eu",
                faction: NaaluCollective,
            },
            Agent::GarvAndGunn => info! {
                agent: GarvAndGunn,
                name: "Garv and Gunn",
                faction: NaazRokhaAlliance,
            },
            Agent::NekroMalleon => info! {
                agent: NekroMalleon,
                name: "Nekro Malleon",
                faction: NekroVirus,
            },
            Agent::Tro => info! {
                agent: Tro,
                name: "T'ro",
                faction: SardakkNorr,
            },
            Agent::Tellurian => info! {
                agent: Tellurian,
                name: "Tellurian",
                faction: TitansOfUl,
            },
            Agent::DoctorSucaban => info! {
                agent: DoctorSucaban,
                name: "Doctor Sucaban",
                faction: UniversitiesOfJolNar,
            },
            Agent::TheStillnessOfStars => info! {
                agent: TheStillnessOfStars,
                name: "The Stillness of Stars",
                faction: VuilRaithCabal,
            },
            Agent::BerekarBerekon => info! {
                agent: BerekarBerekon,
                name: "Berekar Berekon",
                faction: Winnu,
            },
            Agent::GgrucotoRinn => info! {
                agent: GgrucotoRinn,
                name: "Ggrucoto Rinn",
                faction: XxchaKingdom,
            },
            Agent::BrotherMilor => info! {
                agent: BrotherMilor,
                name: "Brother Milor",
                faction: YinBrotherhood,
            },
            Agent::CleverCleverSsruu => info! {
                agent: CleverCleverSsruu,
                name: "Clever Clever Ssruu",
                faction: YssarilTribes,
            },
            Agent::ArtunoTheBetrayer => info! {
                agent: ArtunoTheBetrayer,
                name: "Artuno the Betrayer",
                faction: Nomad,
            },
            Agent::FieldMarshallMercer => info! {
                agent: FieldMarshallMercer,
                name: "Field Marshall Mercer",
                faction: Nomad,
            },
            Agent::TheThundarian => info! {
                agent: TheThundarian,
                name: "The Thundarian",
                faction: Nomad,
            },
        }
    }
}
