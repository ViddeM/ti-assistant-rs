use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::data::common::faction::Faction;

use super::LeaderAbilityKind;

/// Information about a hero leader.
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export)]
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

    /// The kind of ability, i.e. whether it's an action or something else.
    pub kind: LeaderAbilityKind,
}

/// A hero leader.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter,
)]
#[allow(missing_docs)]
#[derive(ts_rs::TS)]
#[ts(export)]
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
    (tag: $tag:ident, name: $name:expr, ability: $ability:expr, faction: $faction:ident, kind: $kind:ident,) => {
        HeroInfo {
            tag: Hero::$tag,
            faction: Faction::$faction,
            name: $name,
            ability: $ability,
            description: include_str!(concat!("description/", stringify!($tag))),
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
                kind: Action,
            },
            Hero::MirikAunSissiri => info! {
                tag: MirikAunSissiri,
                name: "Mirik Aun Sissiri",
                ability: "Helix Protocol",
                faction: ArgentFlight,
                kind: Action,
            },
            Hero::DarktalonTreilla => info! {
                tag: DarktalonTreilla,
                name: "Darktalon Treilla",
                ability: "Dark Matter Affinity",
                faction: BaronyOfLetnev,
                kind: Action,
            },
            Hero::GurnoAggero => info! {
                tag: GurnoAggero,
                name: "Gurno Aggero",
                ability: "Armageddon Relay",
                faction: ClanOfSaar,
                kind: Action,
            },
            Hero::AdjudicatorBaal => info! {
                tag: AdjudicatorBaal,
                name: "Adjudicator Ba'al",
                ability: "Nova Seed",
                faction: EmbersOfMuaat,
                kind: Other,
            },
            Hero::HarrughGefhara => info! {
                tag: HarrughGefhara,
                name: "Harrugh Gefhara",
                ability: "Galactic Securities Net",
                faction: EmiratesOfHacan,
                kind: Other,
            },
            Hero::ConservatorProcyon => info! {
                tag: ConservatorProcyon,
                name: "Conservator Procyon",
                ability: "Multiverse Shift",
                faction: Empyrean,
                kind: Action,
            },
            Hero::JaceX4thAirLegion => info! {
                tag: JaceX4thAirLegion,
                name: "Jace X. 4th Air Legion",
                ability: "Helio Command Array",
                faction: FederationOfSol,
                kind: Action,
            },
            Hero::RiftwalkerMeian => info! {
                tag: RiftwalkerMeian,
                name: "Riftwalker Meian",
                ability: "Singularity Reactor",
                faction: GhostsOfCreuss,
                kind: Action,
            },
            Hero::TheHelmsman => info! {
                tag: TheHelmsman,
                name: "The Helmsman",
                ability: "Dark Space Navigation",
                faction: L1Z1XMindnet,
                kind: Action,
            },
            Hero::AiroShirAur => info! {
                tag: AiroShirAur,
                name: "Airo Shir Aur",
                ability: "Benediction",
                faction: MahactGeneSorcerers,
                kind: Action,
            },
            Hero::IpswitchLooseCannon => info! {
                tag: IpswitchLooseCannon,
                name: "Ipswitch, Loose Cannon",
                ability: "Sleeper Cell",
                faction: MentakCoalition,
                kind: Other,
            },
            Hero::TheOracle => info! {
                tag: TheOracle,
                name: "The Oracle",
                ability: "C-Radium Geometry",
                faction: NaaluCollective,
                kind: Other,
            },
            Hero::HeshAndPrit => info! {
                tag: HeshAndPrit,
                name: "Hesh and Prit",
                ability: "Perfect Synthesis",
                faction: NaazRokhaAlliance,
                kind: Action,
            },
            Hero::UnitDsgnFlayesh => info! {
                tag: UnitDsgnFlayesh,
                name: "UNIT.DSGN.FLAYESH",
                ability: "Polymorphic Algorithm",
                faction: NekroVirus,
                kind: Action,
            },
            Hero::ShvalHarbinger => info! {
                tag: ShvalHarbinger,
                name: "Sh'val, Harbinger",
                ability: "Tekklar Conditioning",
                faction: SardakkNorr,
                kind: Other,
            },
            Hero::UlTheProgenitor => info! {
                tag: UlTheProgenitor,
                name: "Ul the Progenitor",
                ability: "Geoform",
                faction: TitansOfUl,
                kind: Action,
            },
            Hero::RinTheMastersLegacy => info! {
                tag: RinTheMastersLegacy,
                name: "Rin, the Master's Legacy",
                ability: "Genetic Memory",
                faction: UniversitiesOfJolNar,
                kind: Action,
            },
            Hero::ItFeedsOnCarrion => info! {
                tag: ItFeedsOnCarrion,
                name: "It Feeds on Carrion",
                ability: "Dimensional Anchor",
                faction: VuilRaithCabal,
                kind: Action,
            },
            Hero::MathisMathinus => info! {
                tag: MathisMathinus,
                name: "Mathis Mathinus",
                ability: "Imperial Seal",
                faction: Winnu,
                kind: Action,
            },
            Hero::XxekirGrom => info! {
                tag: XxekirGrom,
                name: "Xxekir Grom",
                ability: "Political Data Nexus",
                faction: XxchaKingdom,
                kind: Action,
            },
            Hero::XxekirGromCxIII => info! {
                tag: XxekirGromCxIII,
                name: "Xxekir Grom Ω",
                ability: "Political Data Nexus Ω",
                faction: XxchaKingdom,
                kind: Other,
            },
            Hero::DannelOfTheTenth => info! {
                tag: DannelOfTheTenth,
                name: "Dannel of the Tenth",
                ability: "Spinner Overdrive",
                faction: YinBrotherhood,
                kind: Action,
            },
            Hero::DannelOfTheTenthCxIII => info! {
                tag: DannelOfTheTenthCxIII,
                name: "Dannel of the Tenth Ω",
                ability: "Spinner Overdrive Ω",
                faction: YinBrotherhood,
                kind: Action,
            },
            Hero::KyverBladeAndKey => info! {
                tag: KyverBladeAndKey,
                name: "Kyver, Blade and Key",
                ability: "Guild of Spies",
                faction: YssarilTribes,
                kind: Action,
            },
            Hero::AhkSylSiven => info! {
                tag: AhkSylSiven,
                name: "Ahk-Syl Siven",
                ability: "Probability Matrix",
                faction: Nomad,
                kind: Action,
            },
            Hero::KuuasiAunJalatai => info! {
                tag: KuuasiAunJalatai,
                name: "Kuuasi Aun Jalatai",
                ability: "Overwing Zeta",
                faction: CouncilKeleres,
                kind: Other,
            },
            Hero::OdlynnMyrr => info! {
                tag: OdlynnMyrr,
                name: "Odlynn Myrr",
                ability: "Operation Archon",
                faction: CouncilKeleres,
                kind: Other,
            },
            Hero::HarkaLeeds => info! {
                tag: HarkaLeeds,
                name: "Harka Leeds",
                ability: "Erwan's Covenant",
                faction: CouncilKeleres,
                kind: Action,
            },
        }
    }
}
