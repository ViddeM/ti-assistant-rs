use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::common::{expansions::Expansion, faction::Faction};

use super::LeaderAbilityKind;

/// Information about a hero leader.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HeroInfo {
    /// [Hero] variant for this hero.
    pub tag: Hero,

    /// Faction that this hero belongs to.
    pub faction: Faction,

    /// Expansion that this hero comes with.
    pub expansion: Expansion,

    /// Name of the hero.
    pub name: String,

    /// Name of the heros ability.
    pub ability: String,

    /// Description of the heros ability.
    pub description: String,

    /// The kind of ability, i.e. whether it's an action or something else.
    pub kind: LeaderAbilityKind,
}

/// A hero leader.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
#[allow(missing_docs)]
pub enum Hero {
    AdjudicatorBaal,
    AhkSylSiven,
    AiroShirAur,
    ConservatorProcyon,
    DannelOfTheTenth,
    DannelOfTheTenthCxIII, // patch of Dannel from Codex III
    DarktalonTreilla,
    GurnoAggero,
    HarkaLeeds,
    HarrughGefhara,
    HeshAndPrit,
    IpswitchLooseCannon,
    ItFeedsOnCarrion,
    JaceX4thAirLegion,
    KuuasiAunJalatai,
    KyverBladeAndKey,
    LetaniMiasmiala,
    MathisMathinus,
    MirikAunSissiri,
    OdlynnMyrr,
    RiftwalkerMeian,
    RinTheMastersLegacy,
    ShvalHarbinger,
    TheHelmsman,
    TheOracle,
    UlTheProgenitor,
    UnitDsgnFlayesh,
    XxekirGrom,
    XxekirGromCxIII, // patch of Xxekir Grom from Codex III
    // Thunder's Edge
    DirectorNel,
    TaZernTE, // Name collision with PoK commander for Universities of Jol-Nar.
    HomesickPhantom,
    Sharsiss,
    SharsissHollow,
    LyraKeen,
}

macro_rules! info {
    (tag: $tag:ident, name: $name:expr, ability: $ability:expr, faction: $faction:ident, expansion: $expansion:ident, kind: $kind:ident,) => {
        HeroInfo {
            tag: Hero::$tag,
            faction: Faction::$faction,
            expansion: Expansion::$expansion,
            name: $name.to_string(),
            ability: $ability.to_string(),
            description: include_str!(concat!("description/", stringify!($tag))).to_string(),
            kind: LeaderAbilityKind::$kind,
        }
    };
}

impl Hero {
    /// Get the [HeroInfo] of this [Hero].
    pub fn info(&self) -> HeroInfo {
        match self {
            Hero::LetaniMiasmiala => info! {
                tag: LetaniMiasmiala,
                name: "Letani Miasmiala",
                ability: "Ultrasonic Emitter",
                faction: Arborec,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::MirikAunSissiri => info! {
                tag: MirikAunSissiri,
                name: "Mirik Aun Sissiri",
                ability: "Helix Protocol",
                faction: ArgentFlight,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::DarktalonTreilla => info! {
                tag: DarktalonTreilla,
                name: "Darktalon Treilla",
                ability: "Dark Matter Affinity",
                faction: BaronyOfLetnev,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::GurnoAggero => info! {
                tag: GurnoAggero,
                name: "Gurno Aggero",
                ability: "Armageddon Relay",
                faction: ClanOfSaar,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::AdjudicatorBaal => info! {
                tag: AdjudicatorBaal,
                name: "Adjudicator Ba'al",
                ability: "Nova Seed",
                faction: EmbersOfMuaat,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Hero::HarrughGefhara => info! {
                tag: HarrughGefhara,
                name: "Harrugh Gefhara",
                ability: "Galactic Securities Net",
                faction: EmiratesOfHacan,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Hero::ConservatorProcyon => info! {
                tag: ConservatorProcyon,
                name: "Conservator Procyon",
                ability: "Multiverse Shift",
                faction: Empyrean,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::JaceX4thAirLegion => info! {
                tag: JaceX4thAirLegion,
                name: "Jace X. 4th Air Legion",
                ability: "Helio Command Array",
                faction: FederationOfSol,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::RiftwalkerMeian => info! {
                tag: RiftwalkerMeian,
                name: "Riftwalker Meian",
                ability: "Singularity Reactor",
                faction: GhostsOfCreuss,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::TheHelmsman => info! {
                tag: TheHelmsman,
                name: "The Helmsman",
                ability: "Dark Space Navigation",
                faction: L1Z1XMindnet,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::AiroShirAur => info! {
                tag: AiroShirAur,
                name: "Airo Shir Aur",
                ability: "Benediction",
                faction: MahactGeneSorcerers,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::IpswitchLooseCannon => info! {
                tag: IpswitchLooseCannon,
                name: "Ipswitch, Loose Cannon",
                ability: "Sleeper Cell",
                faction: MentakCoalition,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Hero::TheOracle => info! {
                tag: TheOracle,
                name: "The Oracle",
                ability: "C-Radium Geometry",
                faction: NaaluCollective,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Hero::HeshAndPrit => info! {
                tag: HeshAndPrit,
                name: "Hesh and Prit",
                ability: "Perfect Synthesis",
                faction: NaazRokhaAlliance,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::UnitDsgnFlayesh => info! {
                tag: UnitDsgnFlayesh,
                name: "UNIT.DSGN.FLAYESH",
                ability: "Polymorphic Algorithm",
                faction: NekroVirus,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::ShvalHarbinger => info! {
                tag: ShvalHarbinger,
                name: "Sh'val, Harbinger",
                ability: "Tekklar Conditioning",
                faction: SardakkNorr,
                expansion: ProphecyOfKings,
                kind: Other,
            },
            Hero::UlTheProgenitor => info! {
                tag: UlTheProgenitor,
                name: "Ul the Progenitor",
                ability: "Geoform",
                faction: TitansOfUl,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::RinTheMastersLegacy => info! {
                tag: RinTheMastersLegacy,
                name: "Rin, the Master's Legacy",
                ability: "Genetic Memory",
                faction: UniversitiesOfJolNar,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::ItFeedsOnCarrion => info! {
                tag: ItFeedsOnCarrion,
                name: "It Feeds on Carrion",
                ability: "Dimensional Anchor",
                faction: VuilRaithCabal,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            // TODO: Implement
            Hero::MathisMathinus => info! {
                tag: MathisMathinus,
                name: "Mathis Mathinus",
                ability: "Imperial Seal",
                faction: Winnu,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::XxekirGrom => info! {
                tag: XxekirGrom,
                name: "Xxekir Grom",
                ability: "Political Data Nexus",
                faction: XxchaKingdom,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::XxekirGromCxIII => info! {
                tag: XxekirGromCxIII,
                name: "Xxekir Grom Ω",
                ability: "Political Data Nexus Ω",
                faction: XxchaKingdom,
                expansion: CodexIII,
                kind: Other,
            },
            Hero::DannelOfTheTenth => info! {
                tag: DannelOfTheTenth,
                name: "Dannel of the Tenth",
                ability: "Spinner Overdrive",
                faction: YinBrotherhood,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::DannelOfTheTenthCxIII => info! {
                tag: DannelOfTheTenthCxIII,
                name: "Dannel of the Tenth Ω",
                ability: "Quantum Dissemination",
                faction: YinBrotherhood,
                expansion: CodexIII,
                kind: Action,
            },
            Hero::KyverBladeAndKey => info! {
                tag: KyverBladeAndKey,
                name: "Kyver, Blade and Key",
                ability: "Guild of Spies",
                faction: YssarilTribes,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::AhkSylSiven => info! {
                tag: AhkSylSiven,
                name: "Ahk-Syl Siven",
                ability: "Probability Matrix",
                faction: Nomad,
                expansion: ProphecyOfKings,
                kind: Action,
            },
            Hero::KuuasiAunJalatai => info! {
                tag: KuuasiAunJalatai,
                name: "Kuuasi Aun Jalatai",
                ability: "Overwing Zeta",
                faction: CouncilKeleres,
                expansion: CodexIII,
                kind: Other,
            },
            Hero::OdlynnMyrr => info! {
                tag: OdlynnMyrr,
                name: "Odlynn Myrr",
                ability: "Operation Archon",
                faction: CouncilKeleres,
                expansion: CodexIII,
                kind: Other,
            },
            Hero::HarkaLeeds => info! {
                tag: HarkaLeeds,
                name: "Harka Leeds",
                ability: "Erwan's Covenant",
                faction: CouncilKeleres,
                expansion: CodexIII,
                kind: Action,
            },
            Hero::DirectorNel => info! {
                tag: DirectorNel,
                name: "Director Nel",
                ability: "Signal Intrusion",
                faction: RalNelConsortium,
                expansion: ThundersEdge,
                kind: Other,
            },
            Hero::TaZernTE => info! {
                tag: TaZernTE,
                name: "Ta Zern",
                ability: "Wave Function Collapse",
                faction: DeepwroughtScholarate,
                expansion: ThundersEdge,
                kind: Action,
            },
            Hero::HomesickPhantom => info! {
                tag: HomesickPhantom,
                name: "Homesick Phantom",
                ability: "Fragment Reality",
                faction: CrimsonRebellion,
                expansion: ThundersEdge,
                kind: Other,
            },
            Hero::Sharsiss => info! {
                tag: Sharsiss,
                name: "Sharsiss",
                ability: "The Blade Beckons",
                faction: FirmamentObsidian,
                expansion: ThundersEdge,
                kind: Action,
            },
            Hero::SharsissHollow => info! {
                tag: SharsissHollow,
                name: "Sharsiss Hollow",
                ability: "The Blade Revealed",
                faction: FirmamentObsidian,
                expansion: ThundersEdge,
                kind: Action,
            },
            Hero::LyraKeen => info! {
                tag: LyraKeen,
                name: "Lyra Keen",
                ability: "Entity 4X41A \"Apollo\"",
                faction: LastBastion,
                expansion: ThundersEdge,
                kind: Other,
            },
        }
    }
}
