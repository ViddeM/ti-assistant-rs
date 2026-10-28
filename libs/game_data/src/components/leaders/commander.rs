use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::common::{expansions::Expansion, faction::Faction};

use super::LeaderAbilityKind;

/// Information about a commander leader.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CommanderInfo {
    /// [Commander] variant for this commander.
    pub tag: Commander,

    /// Faction that this commander belongs to.
    pub faction: Faction,

    /// Expansion that this commander comes with.
    pub expansion: Expansion,

    /// Name of the commander.
    pub name: String,

    /// Description of the commanders unlock condition.
    pub unlock: String,

    /// Description of the commanders ability.
    pub description: String,

    /// The kind of ability, i.e. whether it's an action or something else.
    pub kind: LeaderAbilityKind,
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
    SuffiAnKeleres,
    TaZern,
    ThatWhichMoldsFlesh,
    TrrakanAunZulok,
    Tungstantus,
    Xuange,
    _2RAM,
    // Thunder's Edge
    WatchfulOjz,
    Aello,
    AhkSiever,
    CaptainAroz,
    ArozHollow,
    NipAndTuck,
}
macro_rules! info {
    (tag: $commander:ident, name: $name:expr, unlock: $unlock:expr, faction:  $faction:ident, expansion: $expansion:ident,) => {
        CommanderInfo {
            tag: Commander::$commander,
            faction: Faction::$faction,
            expansion: Expansion::$expansion,
            name: $name.to_string(),
            unlock: $unlock.to_string(),
            description: include_str!(concat!("description/", stringify!($commander))).to_string(),
            kind: LeaderAbilityKind::Other, // No commanders have an ACTION ability.
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
                expansion: ProphecyOfKings,
            },
            Commander::TrrakanAunZulok => info! {
                tag: TrrakanAunZulok,
                name: "Trrakan Aun Zulok",
                unlock: "Have 6 units that have ANTI-FIGHTER BARRAGE, SPACE CANNON or BOMBARDMENT on the game board",
                faction: ArgentFlight,
                expansion: ProphecyOfKings,
            },
            Commander::RearAdmiralFarran => info! {
                tag: RearAdmiralFarran,
                name: "Rear Admiral Farran",
                unlock: "Have 5 non-fighter ships in 1 system",
                faction: BaronyOfLetnev,
                expansion: ProphecyOfKings,
            },
            Commander::RowlSarrig => info! {
                tag: RowlSarrig,
                name: "Rowl Sarrig",
                unlock: "Have 3 space docks on the game board",
                faction: ClanOfSaar,
                expansion: ProphecyOfKings,
            },
            Commander::Magmus => info! {
                tag: Magmus,
                name: "Magmus",
                unlock: "Produce a War Sun",
                faction: EmbersOfMuaat,
                expansion: ProphecyOfKings,
            },
            Commander::GilatheSilvertongue => info! {
                tag: GilatheSilvertongue,
                name: "Gila the Silvertongue",
                unlock: "Have 10 Trade Goods",
                faction: EmiratesOfHacan,
                expansion: ProphecyOfKings,
            },
            Commander::Xuange => info! {
                tag: Xuange,
                name: "Xuange",
                unlock: "Be neighbors with all other players",
                faction: Empyrean,
                expansion: ProphecyOfKings,
            },
            Commander::ClaireGibson => info! {
                tag: ClaireGibson,
                name: "Claire Gibson",
                unlock: "Control planets that have a combined total of at least 12 resources",
                faction: FederationOfSol,
                expansion: ProphecyOfKings,
            },
            Commander::SaiSeravus => info! {
                tag: SaiSeravus,
                name: "Sai Seravus",
                unlock: "Have units in 3 systems that contain alpha or beta wormholes",
                faction: GhostsOfCreuss,
                expansion: ProphecyOfKings,
            },
            Commander::_2RAM => info! {
                tag: _2RAM,
                name: "2RAM",
                unlock: "Have 4 Dreadnoughts on the Board",
                faction: L1Z1XMindnet,
                expansion: ProphecyOfKings,
            },
            Commander::IlNaViroset => info! {
                tag: IlNaViroset,
                name: "Il Na Viroset",
                unlock: "Have 2 other factions command tokens in your fleet pool",
                faction: MahactGeneSorcerers,
                expansion: ProphecyOfKings,
            },
            Commander::SUlaMentarion => info! {
                tag: SUlaMentarion,
                name: "S'Ula Mentarion",
                unlock: "Have 4 cruisers on the game board",
                faction: MentakCoalition,
                expansion: ProphecyOfKings,
            },
            Commander::Maban => info! {
                tag: Maban,
                name: "M'aban",
                unlock: "Have 12 fighters on the game board",
                faction: NaaluCollective,
                expansion: ProphecyOfKings,
            },
            Commander::MabanCxIII => info! {
                tag: MabanCxIII,
                name: "M'aban Ω",
                unlock: "Have ground units in or adjacent to the Mexatol Rex system",
                faction: NaaluCollective,
                expansion: CodexIII,
            },
            Commander::DartandTai => info! {
                tag: DartandTai,
                name: "Dart and Tai",
                unlock: "Have 3 mechs in 3 systems",
                faction: NaazRokhaAlliance,
                expansion: ProphecyOfKings,
            },
            Commander::NekroAcidos => info! {
                tag: NekroAcidos,
                name: "Nekro Acidos",
                unlock: "Own 3 technologies. A \"Valefar Assimilator\" technology counts only if its X or Y token is on a technology",
                faction: NekroVirus,
                expansion: ProphecyOfKings,
            },
            Commander::GhomSekkus => info! {
                tag: GhomSekkus,
                name: "G'hom Sek'kus",
                unlock: "Control 5 planets in non-home systems",
                faction: SardakkNorr,
                expansion: ProphecyOfKings,
            },
            Commander::Tungstantus => info! {
                tag: Tungstantus,
                name: "Tungstantus",
                unlock: "Have 5 Structures on the game board",
                faction: TitansOfUl,
                expansion: ProphecyOfKings,
            },
            Commander::TaZern => info! {
                tag: TaZern,
                name: "Ta-Zern",
                unlock: "Own 8 technologies",
                faction: UniversitiesOfJolNar,
                expansion: ProphecyOfKings,
            },
            Commander::ThatWhichMoldsFlesh => info! {
                tag: ThatWhichMoldsFlesh,
                name: "That Which Molds Flesh",
                unlock: "Have units in 3 Gravity Rifts",
                faction: VuilRaithCabal,
                expansion: ProphecyOfKings,
            },
            Commander::RickarRickani => info! {
                tag: RickarRickani,
                name: "Rickar Rickani",
                unlock: "Control Mecatol Rex or enter into a combat in the Mecatol Rex system",
                faction: Winnu,
                expansion: ProphecyOfKings,
            },
            Commander::ElderQanoj => info! {
                tag: ElderQanoj,
                name: "Elder Qanoj",
                unlock: "Control planets that have a combined total of at least 12 influence",
                faction: XxchaKingdom,
                expansion: ProphecyOfKings,
            },
            Commander::BrotherOmar => info! {
                tag: BrotherOmar,
                name: "Brother Omar",
                unlock: "Use your Indoctrination faction ability",
                faction: YinBrotherhood,
                expansion: ProphecyOfKings,
            },
            Commander::BrotherOmarCxIII => info! {
                tag: BrotherOmarCxIII,
                name: "Brother Omar Ω",
                unlock: "Use one of your faction abilities",
                faction: YinBrotherhood,
                expansion: CodexIII,
            },
            Commander::SoAta => info! {
                tag: SoAta,
                name: "So Ata",
                unlock: "Have 7 Action cards",
                faction:  YssarilTribes,
                expansion: ProphecyOfKings,
            },
            Commander::NavarchFeng => info! {
                tag: NavarchFeng,
                name: "Navarch Feng",
                unlock: "Have 1 scored secret objective",
                faction:  Nomad,
                expansion: ProphecyOfKings,
            },
            Commander::SuffiAnKeleres => info! {
                tag: SuffiAnKeleres,
                name: "Suffi An",
                unlock: "Spend 1 trade good after you play an action card that has a component action",
                faction: CouncilKeleres,
                expansion: CodexIII,
            },
            Commander::WatchfulOjz => info! {
                tag: WatchfulOjz,
                name: "Watchful Ojz",
                unlock: "Be the last person to pass during the Action Phase.",
                faction: RalNelConsortium,
                expansion: ThundersEdge,
            },
            Commander::Aello => info! {
                tag: Aello,
                name: "Aello",
                unlock: "Have an ocean card in play.",
                faction: DeepwroughtScholarate,
                expansion: ThundersEdge,
            },
            Commander::AhkSiever => info! {
                tag: AhkSiever,
                name: "Ahk Siever",
                unlock: "Place a breach token in a system that contains another player's unit.",
                faction: CrimsonRebellion,
                expansion: ThundersEdge,
            },
            Commander::CaptainAroz => info! {
                tag: CaptainAroz,
                name: "Captain Aroz",
                unlock: "Have one plot card in play.",
                faction: FirmamentObsidian,
                expansion: ThundersEdge,
            },
            Commander::ArozHollow => info! {
                tag: ArozHollow,
                name: "Aroz Hollow",
                unlock: "Have units in The Fracture .",
                faction: FirmamentObsidian,
                expansion: ThundersEdge,
            },
            Commander::NipAndTuck => info! {
                tag: NipAndTuck,
                name: "Nip and Tuck",
                unlock: "There are 3 galvanized units on the game board.",
                faction: LastBastion,
                expansion: ThundersEdge,
            },
        }
    }
}
