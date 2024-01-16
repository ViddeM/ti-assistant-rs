use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::data::common::faction::Faction;

/// Information about a hero leader.
#[derive(Clone, Debug, Serialize)]
pub struct HeroInfo {
    /// [Hero] variant for this hero.
    pub hero: Hero,

    /// Faction that this hero belongs to.
    pub faction: Faction,

    /// Name of the hero.
    pub name: &'static str,

    /// Name of the heros ability.
    pub ability: &'static str,
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
    DarktalonTreilla,
    GurnoAggero,
    HarrughGefhara,
    HeshAndPrit,
    IpswitchLooseCannon,
    ItFeedsOnCarrion,
    JaceX4thAirLegion,
    KyverBladeAndKey,
    LetaniMiasmiala,
    MathisMathinus,
    MirikAunSissiri,
    RiftwalkerMeian,
    RinTheMastersLegacy,
    ShvalHarbinger,
    TheHelmsman,
    TheOracle,
    UlTheProgenitor,
    UnitDsgnFlayesh,
    XxekirGrom,
}

macro_rules! info {
    (hero: $hero:ident, name: $name:expr, ability: $ability:expr, faction: $faction:ident,) => {
        HeroInfo {
            hero: Hero::$hero,
            faction: Faction::$faction,
            name: $name,
            ability: $ability,
        }
    };
}

impl Hero {
    /// Get the [HeroInfo] of this [Hero].
    pub fn info(&self) -> HeroInfo {
        match self {
            Hero::LetaniMiasmiala => info! {
                hero: LetaniMiasmiala,
                name: "Letani Miasmiala",
                ability: "Ultrasonic Emitter",
                faction: Arborec,
            },
            Hero::MirikAunSissiri => info! {
                hero: MirikAunSissiri,
                name: "Mirik Aun Sissiri",
                ability: "Helix Protocol",
                faction: ArgentFlight,
            },
            Hero::DarktalonTreilla => info! {
                hero: DarktalonTreilla,
                name: "Darktalon Treilla",
                ability: "Dark Matter Affinity",
                faction: BaronyOfLetnev,
            },
            Hero::GurnoAggero => info! {
                hero: GurnoAggero,
                name: "Gurno Aggero",
                ability: "Armageddon Relay",
                faction: ClanOfSaar,
            },
            Hero::AdjudicatorBaal => info! {
                hero: AdjudicatorBaal,
                name: "Adjudicator Ba'al",
                ability: "Nova Seed",
                faction: EmbersOfMuaat,
            },
            Hero::HarrughGefhara => info! {
                hero: HarrughGefhara,
                name: "Harrugh Gefhara",
                ability: "Galactic Securities Net",
                faction: EmiratesOfHacan,
            },
            Hero::ConservatorProcyon => info! {
                hero: ConservatorProcyon,
                name: "Conservator Procyon",
                ability: "Multiverse Shift",
                faction: Empyrean,
            },
            Hero::JaceX4thAirLegion => info! {
                hero: JaceX4thAirLegion,
                name: "Jace X. 4th Air Legion",
                ability: "Helio Command Array",
                faction: FederationOfSol,
            },
            Hero::RiftwalkerMeian => info! {
                hero: RiftwalkerMeian,
                name: "Riftwalker Meian",
                ability: "Singularity Reactor",
                faction: GhostsOfCreuss,
            },
            Hero::TheHelmsman => info! {
                hero: TheHelmsman,
                name: "The Helmsman",
                ability: "Dark Space Navigation",
                faction: L1Z1XMindnet,
            },
            Hero::AiroShirAur => info! {
                hero: AiroShirAur,
                name: "Airo Shir Aur",
                ability: "Benediction",
                faction: MahactGeneSorcerers,
            },
            Hero::IpswitchLooseCannon => info! {
                hero: IpswitchLooseCannon,
                name: "Ipswitch, Loose Cannon",
                ability: "Sleeper Cell",
                faction: MentakCoalition,
            },
            Hero::TheOracle => info! {
                hero: TheOracle,
                name: "The Oracle",
                ability: "C-Radium Geometry",
                faction: NaaluCollective,
            },
            Hero::HeshAndPrit => info! {
                hero: HeshAndPrit,
                name: "Hesh and Prit",
                ability: "Perfect Synthesis",
                faction: NaazRokhaAlliance,
            },
            Hero::UnitDsgnFlayesh => info! {
                hero: UnitDsgnFlayesh,
                name: "UNIT.DSGN.FLAYESH",
                ability: "Polymorphic Algorithm",
                faction: NekroVirus,
            },
            Hero::ShvalHarbinger => info! {
                hero: ShvalHarbinger,
                name: "Sh'val, Harbinger",
                ability: "Tekklar Conditioning",
                faction: SardakkNorr,
            },
            Hero::UlTheProgenitor => info! {
                hero: UlTheProgenitor,
                name: "Ul the Progenitor",
                ability: "Geoform",
                faction: TitansOfUl,
            },
            Hero::RinTheMastersLegacy => info! {
                hero: RinTheMastersLegacy,
                name: "Rin, the Master's Legacy",
                ability: "Genetic Memory",
                faction: UniversitiesOfJolNar,
            },
            Hero::ItFeedsOnCarrion => info! {
                hero: ItFeedsOnCarrion,
                name: "It Feeds on Carrion",
                ability: "Dimensional Anchor",
                faction: VuilRaithCabal,
            },
            Hero::MathisMathinus => info! {
                hero: MathisMathinus,
                name: "Mathis Mathinus",
                ability: "Imperial Seal",
                faction: Winnu,
            },
            Hero::XxekirGrom => info! {
                hero: XxekirGrom,
                name: "Xxekir Grom",
                ability: "Political Data Nexus",
                faction: XxchaKingdom,
            },
            Hero::DannelOfTheTenth => info! {
                hero: DannelOfTheTenth,
                name: "Dannel of the Tenth",
                ability: "Spinner Overdrive",
                faction: YinBrotherhood,
            },
            Hero::KyverBladeAndKey => info! {
                hero: KyverBladeAndKey,
                name: "Kyver, Blade and Key",
                ability: "Guild of Spies",
                faction: YssarilTribes,
            },
            Hero::AhkSylSiven => info! {
                hero: AhkSylSiven,
                name: "Ahk-Syl Siven",
                ability: "Probability Matrix",
                faction: Nomad,
            },
        }
    }
}
