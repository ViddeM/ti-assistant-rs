use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::data::common::faction::Faction;

/// Information about a commander leader.
#[derive(Clone, Debug, Serialize)]
pub struct CommanderInfo {
    /// [Commander] variant for this commander.
    pub tag: Commander,

    /// Faction that this commander belongs to.
    pub faction: Faction,

    /// Name of the commander.
    pub name: &'static str,

    /// Description of the commanders unlock condition.
    pub unlock: &'static str,

    /// Description of the commanders ability.
    pub description: &'static str,
}

/// A commander leader.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
#[allow(missing_docs)]
pub enum Commander {
    BrotherOmar,
    BrotherOmarCxIII, // patch of Brother Omar from Codex III
    ClaireGibson,
    DartandTai,
    DirzugaRophal,
    ElderQanoj,
    GhomSekkus,
    GilatheSilvertongue,
    IlNaViroset,
    Maban,
    MabanCxIII, // patch of Maban from Codex III
    Magmus,
    NavarchFeng,
    NekroAcidos,
    RearAdmiralFarran,
    RickarRickani,
    RowlSarrig,
    SUlaMentarion,
    SaiSeravus,
    SoAta,
    TaZern,
    ThatWhichMoldsFlesh,
    TrrakanAunZulok,
    Tungstantus,
    Xuange,
    _2RAM,
}
macro_rules! info {
    (tag: $commander:ident, name: $name:expr, unlock: $unlock:expr, faction:  $faction:ident,) => {
        CommanderInfo {
            tag: Commander::$commander,
            faction: Faction::$faction,
            name: $name,
            unlock: $unlock,
            description: include_str!(concat!("description/", stringify!($commander))),
        }
    };
}

impl Commander {
    /// Get the [CommanderInfo] of this [Commander].
    pub fn info(&self) -> CommanderInfo {
        match self {
            Commander::DirzugaRophal => info! {
                tag: DirzugaRophal,
                name: "Dirzuga Rophal",
                unlock: "Have 12 Ground Forces on Planets you control",
                faction: Arborec,
            },
            Commander::TrrakanAunZulok => info! {
                tag: TrrakanAunZulok,
                name: "Trrakan Aun Zulok",
                unlock: "Have 6 units that have ANTI-FIGHTER BARRAGE, SPACE CANNON or BOMBARDMENT on the game board",
                faction: ArgentFlight,
            },
            Commander::RearAdmiralFarran => info! {
                tag: RearAdmiralFarran,
                name: "Rear Admiral Farran",
                unlock: "Have 5 non-fighter ships in 1 system",
                faction: BaronyOfLetnev,
            },
            Commander::RowlSarrig => info! {
                tag: RowlSarrig,
                name: "Rowl Sarrig",
                unlock: "Have 3 space docks on the game board",
                faction: ClanOfSaar,
            },
            Commander::Magmus => info! {
                tag: Magmus,
                name: "Magmus",
                unlock: "Produce a War Sun",
                faction: EmbersOfMuaat,
            },
            Commander::GilatheSilvertongue => info! {
                tag: GilatheSilvertongue,
                name: "Gila the Silvertongue",
                unlock: "Have 10 Trade Goods",
                faction: EmiratesOfHacan,
            },
            Commander::Xuange => info! {
                tag: Xuange,
                name: "Xuange",
                unlock: "Be neighbors with all other players",
                faction: Empyrean,
            },
            Commander::ClaireGibson => info! {
                tag: ClaireGibson,
                name: "Claire Gibson",
                unlock: "Control planets that have a combined total of at least 12 resources",
                faction: FederationOfSol,
            },
            Commander::SaiSeravus => info! {
                tag: SaiSeravus,
                name: "Sai Seravus",
                unlock: "Have units in 3 systems that contain alpha or beta wormholes",
                faction: GhostsOfCreuss,
            },
            Commander::_2RAM => info! {
                tag: _2RAM,
                name: "2RAM",
                unlock: "Have 4 Dreadnoughts on the Board",
                faction: L1Z1XMindnet,
            },
            Commander::IlNaViroset => info! {
                tag: IlNaViroset,
                name: "Il Na Viroset",
                unlock: "Have 2 other factions command tokens in your fleet pool",
                faction: MahactGeneSorcerers,
            },
            Commander::SUlaMentarion => info! {
                tag: SUlaMentarion,
                name: "S'Ula Mentarion",
                unlock: "Have 4 cruisers on the game board",
                faction: MentakCoalition,
            },
            Commander::Maban => info! {
                tag: Maban,
                name: "M'aban",
                unlock: "Have 12 fighters on the game board",
                faction: NaaluCollective,
            },
            Commander::MabanCxIII => info! {
                tag: MabanCxIII,
                name: "M'aban Ω",
                unlock: "Have ground units in or adjacent to the Mexatol Rex system",
                faction: NaaluCollective,
            },
            Commander::DartandTai => info! {
                tag: DartandTai,
                name: "Dart and Tai",
                unlock: "Have 3 mechs in 3 systems",
                faction: NaazRokhaAlliance,
            },
            Commander::NekroAcidos => info! {
                tag: NekroAcidos,
                name: "Nekro Acidos",
                unlock: "Own 3 technologies. A \"Valefar Assimilator\" technology counts only if its X or Y token is on a technology",
                faction: NekroVirus,
            },
            Commander::GhomSekkus => info! {
                tag: GhomSekkus,
                name: "G'hom Sek'kus",
                unlock: "Control 5 planets in non-home systems",
                faction: SardakkNorr,
            },
            Commander::Tungstantus => info! {
                tag: Tungstantus,
                name: "Tungstantus",
                unlock: "Have 5 Structures on the game board",
                faction: TitansOfUl,
            },
            Commander::TaZern => info! {
                tag: TaZern,
                name: "Ta-Zern",
                unlock: "Own 8 technologies",
                faction: UniversitiesOfJolNar,
            },
            Commander::ThatWhichMoldsFlesh => info! {
                tag: ThatWhichMoldsFlesh,
                name: "That Which Molds Flesh",
                unlock: "Have units in 3 Gravity Rifts",
                faction: VuilRaithCabal,
            },
            Commander::RickarRickani => info! {
                tag: RickarRickani,
                name: "Rickar Rickani",
                unlock: "Control Mecatol Rex or enter into a combat in the Mecatol Rex system",
                faction: Winnu,
            },
            Commander::ElderQanoj => info! {
                tag: ElderQanoj,
                name: "Elder Qanoj",
                unlock: "Control planets that have a combined total of at least 12 influence",
                faction: XxchaKingdom,
            },
            Commander::BrotherOmar => info! {
                tag: BrotherOmar,
                name: "Brother Omar",
                unlock: "Use your Indoctrination faction ability",
                faction: YinBrotherhood,
            },
            Commander::BrotherOmarCxIII => info! {
                tag: BrotherOmarCxIII,
                name: "Brother Omar Ω",
                unlock: "Use one of your faction abilities",
                faction: YinBrotherhood,
            },
            Commander::SoAta => info! {
                tag: SoAta,
                name: "So Ata",
                unlock: "Have 7 Action cards",
                faction:  YssarilTribes,
            },
            Commander::NavarchFeng => info! {
                tag: NavarchFeng,
                name: "Navarch Feng",
                unlock: "Have 1 scored secret objective",
                faction:  Nomad,
            },
        }
    }
}
