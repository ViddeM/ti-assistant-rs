use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::data::common::faction::Faction;

/// Information about a hero leader.
#[derive(Clone, Debug, Serialize)]
pub struct HeroInfo {
    /// [Hero] variant for this hero.
    pub tag: Hero,

    /// Faction that this hero belongs to.
    pub faction: Faction,

    /// Name of the hero.
    pub name: &'static str,

    /// Name of the heros ability.
    pub ability: &'static str,

    /// Description of the heros ability.
    pub description: &'static str,
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
}

macro_rules! info {
    (tag: $tag:ident, name: $name:expr, ability: $ability:expr, faction: $faction:ident,) => {
        HeroInfo {
            tag: Hero::$tag,
            faction: Faction::$faction,
            name: $name,
            ability: $ability,
            description: include_str!(concat!("description/", stringify!($tag))),
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
            },
            Hero::MirikAunSissiri => info! {
                tag: MirikAunSissiri,
                name: "Mirik Aun Sissiri",
                ability: "Helix Protocol",
                faction: ArgentFlight,
            },
            Hero::DarktalonTreilla => info! {
                tag: DarktalonTreilla,
                name: "Darktalon Treilla",
                ability: "Dark Matter Affinity",
                faction: BaronyOfLetnev,
            },
            Hero::GurnoAggero => info! {
                tag: GurnoAggero,
                name: "Gurno Aggero",
                ability: "Armageddon Relay",
                faction: ClanOfSaar,
            },
            Hero::AdjudicatorBaal => info! {
                tag: AdjudicatorBaal,
                name: "Adjudicator Ba'al",
                ability: "Nova Seed",
                faction: EmbersOfMuaat,
            },
            Hero::HarrughGefhara => info! {
                tag: HarrughGefhara,
                name: "Harrugh Gefhara",
                ability: "Galactic Securities Net",
                faction: EmiratesOfHacan,
            },
            Hero::ConservatorProcyon => info! {
                tag: ConservatorProcyon,
                name: "Conservator Procyon",
                ability: "Multiverse Shift",
                faction: Empyrean,
            },
            Hero::JaceX4thAirLegion => info! {
                tag: JaceX4thAirLegion,
                name: "Jace X. 4th Air Legion",
                ability: "Helio Command Array",
                faction: FederationOfSol,
            },
            Hero::RiftwalkerMeian => info! {
                tag: RiftwalkerMeian,
                name: "Riftwalker Meian",
                ability: "Singularity Reactor",
                faction: GhostsOfCreuss,
            },
            Hero::TheHelmsman => info! {
                tag: TheHelmsman,
                name: "The Helmsman",
                ability: "Dark Space Navigation",
                faction: L1Z1XMindnet,
            },
            Hero::AiroShirAur => info! {
                tag: AiroShirAur,
                name: "Airo Shir Aur",
                ability: "Benediction",
                faction: MahactGeneSorcerers,
            },
            Hero::IpswitchLooseCannon => info! {
                tag: IpswitchLooseCannon,
                name: "Ipswitch, Loose Cannon",
                ability: "Sleeper Cell",
                faction: MentakCoalition,
            },
            Hero::TheOracle => info! {
                tag: TheOracle,
                name: "The Oracle",
                ability: "C-Radium Geometry",
                faction: NaaluCollective,
            },
            Hero::HeshAndPrit => info! {
                tag: HeshAndPrit,
                name: "Hesh and Prit",
                ability: "Perfect Synthesis",
                faction: NaazRokhaAlliance,
            },
            Hero::UnitDsgnFlayesh => info! {
                tag: UnitDsgnFlayesh,
                name: "UNIT.DSGN.FLAYESH",
                ability: "Polymorphic Algorithm",
                faction: NekroVirus,
            },
            Hero::ShvalHarbinger => info! {
                tag: ShvalHarbinger,
                name: "Sh'val, Harbinger",
                ability: "Tekklar Conditioning",
                faction: SardakkNorr,
            },
            Hero::UlTheProgenitor => info! {
                tag: UlTheProgenitor,
                name: "Ul the Progenitor",
                ability: "Geoform",
                faction: TitansOfUl,
            },
            Hero::RinTheMastersLegacy => info! {
                tag: RinTheMastersLegacy,
                name: "Rin, the Master's Legacy",
                ability: "Genetic Memory",
                faction: UniversitiesOfJolNar,
            },
            Hero::ItFeedsOnCarrion => info! {
                tag: ItFeedsOnCarrion,
                name: "It Feeds on Carrion",
                ability: "Dimensional Anchor",
                faction: VuilRaithCabal,
            },
            Hero::MathisMathinus => info! {
                tag: MathisMathinus,
                name: "Mathis Mathinus",
                ability: "Imperial Seal",
                faction: Winnu,
            },
            Hero::XxekirGrom => info! {
                tag: XxekirGrom,
                name: "Xxekir Grom",
                ability: "Political Data Nexus",
                faction: XxchaKingdom,
            },
            Hero::XxekirGromCxIII => info! {
                tag: XxekirGromCxIII,
                name: "Xxekir Grom Ω",
                ability: "Political Data Nexus Ω",
                faction: XxchaKingdom,
            },
            Hero::DannelOfTheTenth => info! {
                tag: DannelOfTheTenth,
                name: "Dannel of the Tenth",
                ability: "Spinner Overdrive",
                faction: YinBrotherhood,
            },
            Hero::DannelOfTheTenthCxIII => info! {
                tag: DannelOfTheTenthCxIII,
                name: "Dannel of the Tenth Ω",
                ability: "Spinner Overdrive Ω",
                faction: YinBrotherhood,
            },
            Hero::KyverBladeAndKey => info! {
                tag: KyverBladeAndKey,
                name: "Kyver, Blade and Key",
                ability: "Guild of Spies",
                faction: YssarilTribes,
            },
            Hero::AhkSylSiven => info! {
                tag: AhkSylSiven,
                name: "Ahk-Syl Siven",
                ability: "Probability Matrix",
                faction: Nomad,
            },
            Hero::KuuasiAunJalatai => info! {
                tag: KuuasiAunJalatai,
                name: "Kuuasi Aun Jalatai",
                ability: "Overwing Zeta",
                faction: CouncilKeleres,
            },
            Hero::OdlynnMyrr => info! {
                tag: OdlynnMyrr,
                name: "Odlynn Myrr",
                ability: "Operation Archon",
                faction: CouncilKeleres,
            },
            Hero::HarkaLeeds => info! {
                tag: HarkaLeeds,
                name: "Harka Leeds",
                ability: "Erwan's Covenant",
                faction: CouncilKeleres,
            },
        }
    }
}
