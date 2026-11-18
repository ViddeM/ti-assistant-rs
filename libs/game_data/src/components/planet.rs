use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use strum_macros::EnumIter;

use crate::common::{
    component::{Component, replacements},
    expansions::Expansion,
    faction::Faction,
};

use super::{
    system::{SystemType, systems},
    tech::TechCategory,
};

/// A planetary trait.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum PlanetTrait {
    Cultural,
    Hazardous,
    Industrial,
}

/// All relevant information for a planet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PlanetInfo {
    /// The name of the planet.
    pub name: String,
    /// Which, if any, planet traits the planet has.
    pub planet_traits: Vec<PlanetTrait>,
    /// Which, if any, technology bonuses the planet has.
    pub tech_specialities: Vec<TechCategory>,
    /// How many resources the planet has.
    pub resources: u32,
    /// How much influence the planet provides.
    pub influence: u32,
    /// Which expansion the planet belongs to.
    pub expansion: Expansion,
    /// Weather the planet is legendary or not.
    pub is_legendary: bool,
    /// Weather this is a planet or a space station.
    pub body_type: OrbitalBodyType,
}

/// What type of orbital body type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrbitalBodyType {
    Planet,
    SpaceStation,
}

macro_rules! planet {
    (
        name: $name:literal,
        $(planet_traits: [$($trait:ident),*],)?
        $(tech_specialities: [$($tech:ident),*],)?
        resources: $resources:literal,
        influence: $influence:literal,
        expansion: $expansion:ident
        $(, is_legendary: $legendary:literal)?
    ) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_traits: vec![$($(PlanetTrait::$trait),*)?],
            tech_specialities: vec![$($(TechCategory::$tech),*)?],
            resources: $resources,
            influence: $influence,
            expansion: Expansion::$expansion,
            is_legendary: planet!(@legendary $($legendary)?),
            body_type: OrbitalBodyType::Planet,
        }
    };
    (@legendary) => {
        false
    };
    (@legendary $legendary:literal) => {
        $legendary
    };
}

macro_rules! space_station {
    (name: $name:literal, resources: $resources:literal, influence: $influence:literal, expansion: $expansion:ident) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_traits: vec![],
            tech_specialities: vec![],
            resources: $resources,
            influence: $influence,
            expansion: Expansion::$expansion,
            is_legendary: false,
            body_type: OrbitalBodyType::SpaceStation,
        }
    };
}

/// A planet
#[derive(
    Debug, Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq, Hash, EnumString, Display,
)]
#[strum(serialize_all = "kebab-case")]
#[allow(missing_docs)]
pub enum Planet {
    Nestphar,
    ArcPrime,
    WrenTerra,
    LisisII,
    Ragh,
    Muaat,
    Hercant,
    Arretze,
    Kamdorn,
    Jord,
    Creuss,
    ZeroZeroZero,
    MollPrimus,
    Druaa,
    Maaluuk,
    MordaiII,
    TrenLak,
    Quinarra,
    Jol,
    Nar,
    Winnu,
    ArchonRen,
    ArchonTau,
    Darien,
    Retillion,
    Shalloq,
    MecatolRex,
    Abyz,
    Fria,
    Arinam,
    Meer,
    Arnor,
    Lor,
    Bereg,
    LirtaIV,
    Centauri,
    Gral,
    Corneeq,
    Resculon,
    DalBootha,
    Xxehan,
    Lazar,
    Sakulag,
    Lodor,
    MeharXull,
    Mellon,
    Zohbat,
    NewAlbion,
    Starpoint,
    Quann,
    Qucenn,
    Rarron,
    Saudor,
    TarMann,
    TequRan,
    Torkan,
    Thibah,
    VefutII,
    Wellon,
    ArchonVail,
    Perimiter,
    Ang,
    SemLore,
    Vorhal,
    Atlas,
    Primor,
    HopesEnd,
    Cormund,
    Everra,
    JeolIr,
    Accoen,
    Kraag,
    Siig,
    Bakal,
    AlioPrima,
    Lisis,
    Velnor,
    Cealdri,
    Xanhact,
    VegaMajor,
    VegaMinor,
    Abaddon,
    Ashtroth,
    Loki,
    RigelI,
    RigelII,
    RigelIII,
    Valk,
    Avar,
    Ylir,
    TheDark,
    Ixth,
    Naazir,
    Rokha,
    Arcturus,
    Elysium,
    Acheron,
    Mallice,
    Mirage,
    CustodiaVigilia,
    CustodiaVigiliaOmega,
    Lesab,
    Olergodt,
    ViraPicsIII,
    Andeara,
    Lemox,
    TheWatchtower,
    Emelpar,
    Faunus,
    Garbozia,
    Tempesta,
    Industrex,
    Capha,
    Kostboth,
    Cresius,
    LazulRex,
    Hercalor,
    Tiamat,
    NewTerra,
    Tinnes,
    Bellatrix,
    TsionStation,
    Tarana,
    OluzStation,
    Cocytus,
    Styx,
    Lethe,
    Phlegethon,
    MecatolRexOmega,
    ThundersEdge,
    Ordinian,
    Avernus,
    Cronos,
    CronosHollow,
    Tallin,
    TallinHollow,
    Revelation,
    MezLoOrzFeiZsha,
    RepoLoOrzQet,
    Ikatena,
    AhkCreuxx,
    Elnath,
    Horizon,
    LuthienVi,
}

impl Planet {
    /// Returns the [PlanetInfo] for this planet.
    pub fn info(&self) -> PlanetInfo {
        match self {
            Planet::Nestphar => planet! {
                name: "Nestphar",
                resources: 3,
                influence: 2,
                expansion: Base
            },
            Planet::ArcPrime => planet! {
                name: "Arc Prime",
                resources: 4,
                influence: 0,
                expansion: Base
            },
            Planet::WrenTerra => planet! {
                name: "Wren Terra",
                resources: 2,
                influence: 1,
                expansion: Base
            },
            Planet::LisisII => planet! {
                name: "Lisis II",
                resources: 1,
                influence: 0,
                expansion: Base
            },
            Planet::Ragh => planet! {
                name: "Ragh",
                resources: 2,
                influence: 1,
                expansion: Base
            },
            Planet::Muaat => planet! {
                name: "Muaat",
                resources: 4,
                influence: 1,
                expansion: Base
            },
            Planet::Hercant => planet! {
                name: "Hercant",
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::Arretze => planet! {
                name: "Arretze",
                resources: 2,
                influence: 0,
                expansion: Base
            },
            Planet::Kamdorn => planet! {
                name: "Kamdorn",
                resources: 0,
                influence: 1,
                expansion: Base
            },
            Planet::Jord => planet! {
                name: "Jord",
                resources: 4,
                influence: 2,
                expansion: Base
            },
            Planet::Creuss => planet! {
                name: "Creuss",
                resources: 4,
                influence: 2,
                expansion: Base
            },
            Planet::ZeroZeroZero => planet! {
                name: "[0.0.0]",
                resources: 5,
                influence: 0,
                expansion: Base
            },
            Planet::MollPrimus => planet! {
                name: "Moll Primus",
                resources: 4,
                influence: 1,
                expansion: Base
            },
            Planet::Druaa => planet! {
                name: "Druaa",
                resources: 3,
                influence: 1,
                expansion: Base
            },
            Planet::Maaluuk => planet! {
                name: "Maaluuk",
                resources: 0,
                influence: 2,
                expansion: Base
            },
            Planet::MordaiII => planet! {
                name: "Mordai II",
                resources: 4,
                influence: 0,
                expansion: Base
            },
            Planet::TrenLak => planet! {
                name: "Tren'Lak",
                resources: 1,
                influence: 0,
                expansion: Base
            },
            Planet::Quinarra => planet! {
                name: "Quinarra",
                resources: 3,
                influence: 1,
                expansion: Base
            },
            Planet::Jol => planet! {
                name: "Jol",
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::Nar => planet! {
                name: "Nar",
                resources: 2,
                influence: 3,
                expansion: Base
            },
            Planet::Winnu => planet! {
                name: "Winnu",
                resources: 3,
                influence: 4,
                expansion: Base
            },
            Planet::ArchonRen => planet! {
                name: "Archon Ren",
                resources: 2,
                influence: 3,
                expansion: Base
            },
            Planet::ArchonTau => planet! {
                name: "Archon Tau",
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::Darien => planet! {
                name: "Darien",
                resources: 4,
                influence: 4,
                expansion: Base
            },
            Planet::Retillion => planet! {
                name: "Retillion",
                resources: 2,
                influence: 3,
                expansion: Base
            },
            Planet::Shalloq => planet! {
                name: "Shalloq",
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::MecatolRex => planet! {
                name: "Mecatol Rex",
                resources: 1,
                influence: 6,
                expansion: Base
            },
            Planet::Abyz => planet! {
                name: "Abyz",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 0,
                expansion: Base
            },
            Planet::Fria => planet! {
                name: "Fria",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 0,
                expansion: Base
            },
            Planet::Arinam => planet! {
                name: "Arinam",
                planet_traits: [Industrial],
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::Meer => planet! {
                name: "Meer",
                planet_traits: [Hazardous],
                tech_specialities: [Warfare],
                resources: 0,
                influence: 4,
                expansion: Base
            },
            Planet::Arnor => planet! {
                name: "Arnor",
                planet_traits: [Industrial],
                resources: 2,
                influence: 1,
                expansion: Base
            },
            Planet::Lor => planet! {
                name: "Lor",
                planet_traits: [Industrial],
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::Bereg => planet! {
                name: "Bereg",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 1,
                expansion: Base
            },
            Planet::LirtaIV => planet! {
                name: "Lirta IV",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 3,
                expansion: Base
            },
            Planet::Centauri => planet! {
                name: "Centauri",
                planet_traits: [Cultural],
                resources: 1,
                influence: 3,
                expansion: Base
            },
            Planet::Gral => planet! {
                name: "Gral",
                planet_traits: [Industrial],
                tech_specialities: [Propulsion],
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::Corneeq => planet! {
                name: "Corneeq",
                planet_traits: [Cultural],
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::Resculon => planet! {
                name: "Resculon",
                planet_traits: [Cultural],
                resources: 2,
                influence: 0,
                expansion: Base
            },
            Planet::DalBootha => planet! {
                name: "Dal Bootha",
                planet_traits: [Cultural],
                resources: 0,
                influence: 2,
                expansion: Base
            },
            Planet::Xxehan => planet! {
                name: "Xxehan",
                planet_traits: [Cultural],
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::Lazar => planet! {
                name: "Lazar",
                planet_traits: [Industrial],
                tech_specialities: [Cybernetic],
                resources: 1,
                influence: 0,
                expansion: Base
            },
            Planet::Sakulag => planet! {
                name: "Sakulag",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 1,
                expansion: Base
            },
            Planet::Lodor => planet! {
                name: "Lodor",
                planet_traits: [Cultural],
                resources: 3,
                influence: 1,
                expansion: Base
            },
            Planet::MeharXull => planet! {
                name: "Mehar Xull",
                planet_traits: [Hazardous],
                tech_specialities: [Warfare],
                resources: 1,
                influence: 3,
                expansion: Base
            },
            Planet::Mellon => planet! {
                name: "Mellon",
                planet_traits: [Cultural],
                resources: 0,
                influence: 2,
                expansion: Base
            },
            Planet::Zohbat => planet! {
                name: "Zohbat",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 1,
                expansion: Base
            },
            Planet::NewAlbion => planet! {
                name: "New Albion",
                planet_traits: [Industrial],
                tech_specialities: [Biotic],
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::Starpoint => planet! {
                name: "Starpoint",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 1,
                expansion: Base
            },
            Planet::Quann => planet! {
                name: "Quann",
                planet_traits: [Cultural],
                resources: 2,
                influence: 1,
                expansion: Base
            },
            Planet::Qucenn => planet! {
                name: "Qucen'n",
                planet_traits: [Industrial],
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::Rarron => planet! {
                name: "Rarron",
                planet_traits: [Cultural],
                resources: 0,
                influence: 3,
                expansion: Base
            },
            Planet::Saudor => planet! {
                name: "Saudor",
                planet_traits: [Industrial],
                resources: 2,
                influence: 2,
                expansion: Base
            },
            Planet::TarMann => planet! {
                name: "Tar'Mann",
                planet_traits: [Industrial],
                tech_specialities: [Biotic],
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::TequRan => planet! {
                name: "Tequ'ran",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 0,
                expansion: Base
            },
            Planet::Torkan => planet! {
                name: "Torkan",
                planet_traits: [Cultural],
                resources: 0,
                influence: 3,
                expansion: Base
            },
            Planet::Thibah => planet! {
                name: "Thibah",
                planet_traits: [Industrial],
                tech_specialities: [Propulsion],
                resources: 1,
                influence: 1,
                expansion: Base
            },
            Planet::VefutII => planet! {
                name: "Vefut II",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 2,
                expansion: Base
            },
            Planet::Wellon => planet! {
                name: "Wellon",
                planet_traits: [Industrial],
                tech_specialities: [Cybernetic],
                resources: 1,
                influence: 2,
                expansion: Base
            },
            Planet::ArchonVail => planet! {
                name: "Archon Vail",
                planet_traits: [Hazardous],
                tech_specialities: [Propulsion],
                resources: 1,
                influence: 3,
                expansion: ProphecyOfKings
            },
            Planet::Perimiter => planet! {
                name: "Perimeter",
                planet_traits: [Industrial],
                resources: 2,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Ang => planet! {
                name: "Ang",
                planet_traits: [Industrial],
                tech_specialities: [Warfare],
                resources: 2,
                influence: 0,
                expansion: ProphecyOfKings
            },
            Planet::SemLore => planet! {
                name: "Sem-Lore",
                planet_traits: [Cultural],
                tech_specialities: [Cybernetic],
                resources: 3,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Vorhal => planet! {
                name: "Vorhal",
                planet_traits: [Cultural],
                tech_specialities: [Biotic],
                resources: 0,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Atlas => planet! {
                name: "Atlas",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Primor => planet! {
                name: "Primor",
                planet_traits: [Cultural],
                resources: 2,
                influence: 1,
                expansion: ProphecyOfKings,
                is_legendary: true
            },
            Planet::HopesEnd => planet! {
                name: "Hope's End",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 0,
                expansion: ProphecyOfKings,
                is_legendary: true
            },
            Planet::Cormund => planet! {
                name: "Cormund",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 0,
                expansion: ProphecyOfKings
            },
            Planet::Everra => planet! {
                name: "Everra",
                planet_traits: [Cultural],
                resources: 3,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::JeolIr => planet! {
                name: "Jeol Ir",
                planet_traits: [Industrial],
                resources: 2,
                influence: 3,
                expansion: ProphecyOfKings
            },
            Planet::Accoen => planet! {
                name: "Accoen",
                planet_traits: [Industrial],
                resources: 2,
                influence: 3,
                expansion: ProphecyOfKings
            },
            Planet::Kraag => planet! {
                name: "Kraag",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Siig => planet! {
                name: "Siig",
                planet_traits: [Hazardous],
                resources: 0,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Bakal => planet! {
                name: "Ba'kal",
                planet_traits: [Industrial],
                resources: 3,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::AlioPrima => planet! {
                name: "Alio Prima",
                planet_traits: [Cultural],
                resources: 1,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Lisis => planet! {
                name: "Lisis",
                planet_traits: [Industrial],
                resources: 2,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Velnor => planet! {
                name: "Velnor",
                planet_traits: [Industrial],
                tech_specialities: [Warfare],
                resources: 2,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Cealdri => planet! {
                name: "Cealdri",
                planet_traits: [Cultural],
                tech_specialities: [Cybernetic],
                resources: 0,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Xanhact => planet! {
                name: "Xanhact",
                planet_traits: [Hazardous],
                resources: 0,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::VegaMajor => planet! {
                name: "Vega Major",
                planet_traits: [Cultural],
                resources: 2,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::VegaMinor => planet! {
                name: "Vega Minor",
                planet_traits: [Cultural],
                tech_specialities: [Propulsion],
                resources: 1,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Abaddon => planet! {
                name: "Abaddon",
                planet_traits: [Cultural],
                resources: 1,
                influence: 0,
                expansion: ProphecyOfKings
            },
            Planet::Ashtroth => planet! {
                name: "Ashtroth",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 0,
                expansion: ProphecyOfKings
            },
            Planet::Loki => planet! {
                name: "Loki",
                planet_traits: [Cultural],
                resources: 1,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::RigelI => planet! {
                name: "Rigel I",
                planet_traits: [Hazardous],
                resources: 0,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::RigelII => planet! {
                name: "Rigel II",
                planet_traits: [Industrial],
                resources: 1,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::RigelIII => planet! {
                name: "Rigel III",
                planet_traits: [Industrial],
                tech_specialities: [Biotic],
                resources: 1,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Valk => planet! {
                name: "Valk",
                resources: 2,
                influence: 0,
                expansion: ProphecyOfKings
            },
            Planet::Avar => planet! {
                name: "Avar",
                resources: 1,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Ylir => planet! {
                name: "Ylir",
                resources: 0,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::TheDark => planet! {
                name: "The Dark",
                resources: 3,
                influence: 4,
                expansion: ProphecyOfKings
            },
            Planet::Ixth => planet! {
                name: "Ixth",
                resources: 3,
                influence: 5,
                expansion: ProphecyOfKings
            },
            Planet::Naazir => planet! {
                name: "Naazir",
                resources: 2,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Rokha => planet! {
                name: "Rokha",
                resources: 1,
                influence: 2,
                expansion: ProphecyOfKings
            },
            Planet::Arcturus => planet! {
                name: "Arcturus",
                resources: 4,
                influence: 4,
                expansion: ProphecyOfKings
            },
            Planet::Elysium => planet! {
                name: "Elysium",
                resources: 4,
                influence: 1,
                expansion: ProphecyOfKings
            },
            Planet::Acheron => planet! {
                name: "Acheron",
                resources: 4,
                influence: 0,
                expansion: ProphecyOfKings
            },
            Planet::Mallice => planet! {
                name: "Mallice",
                planet_traits: [Cultural],
                resources: 0,
                influence: 3,
                expansion: ProphecyOfKings,
                is_legendary: true
            },
            Planet::Mirage => planet! {
                name: "Mirage",
                planet_traits: [Cultural],
                resources: 1,
                influence: 2,
                expansion: ProphecyOfKings,
                is_legendary: true
            },
            Planet::CustodiaVigilia => planet! {
                name: "Custodia Vigilia",
                resources: 2,
                influence: 3,
                expansion: CodexIII,
                is_legendary: true
            },
            Planet::CustodiaVigiliaOmega => planet! {
                name: "Custodia Vigilia Ω",
                resources: 2,
                influence: 2,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Lesab => planet! {
                name: "Lesab",
                planet_traits: [Industrial, Hazardous],
                resources: 2,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Olergodt => planet! {
                name: "Olergodt",
                planet_traits: [Cultural, Hazardous],
                tech_specialities: [Cybernetic, Warfare],
                resources: 2,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::ViraPicsIII => planet! {
                name: "Vira-Pics III",
                planet_traits: [Cultural, Hazardous],
                resources: 2,
                influence: 3,
                expansion: ThundersEdge
            },
            Planet::Andeara => planet! {
                name: "Andeara",
                planet_traits: [Industrial],
                tech_specialities: [Propulsion],
                resources: 1,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Lemox => planet! {
                name: "Lemox",
                planet_traits: [Industrial],
                resources: 0,
                influence: 3,
                expansion: ThundersEdge
            },
            Planet::TheWatchtower => space_station! {
                name: "The Watchtower",
                resources: 1,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Emelpar => planet! {
                name: "Emelpar",
                planet_traits: [Cultural],
                resources: 0,
                influence: 2,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Faunus => planet! {
                name: "Faunus",
                planet_traits: [Industrial],
                tech_specialities: [Biotic],
                resources: 1,
                influence: 3,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Garbozia => planet! {
                name: "Garbozia",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 1,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Tempesta => planet! {
                name: "Tempesta",
                planet_traits: [Hazardous],
                tech_specialities: [Propulsion],
                resources: 1,
                influence: 1,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Industrex => planet! {
                name: "Industrex",
                planet_traits: [Industrial],
                tech_specialities: [Warfare],
                resources: 2,
                influence: 0,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Capha => planet! {
                name: "Capha",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 0,
                expansion: ThundersEdge
            },
            Planet::Kostboth => planet! {
                name: "Kostboth",
                planet_traits: [Cultural],
                resources: 0,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Cresius => planet! {
                name: "Cresius",
                planet_traits: [Hazardous],
                resources: 0,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::LazulRex => planet! {
                name: "Lazul Rex",
                planet_traits: [Cultural, Industrial],
                resources: 2,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::Hercalor => planet! {
                name: "Hercalor",
                planet_traits: [Industrial],
                resources: 1,
                influence: 0,
                expansion: ThundersEdge
            },
            Planet::Tiamat => planet! {
                name: "Tiamat",
                planet_traits: [Cultural],
                tech_specialities: [Cybernetic],
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::NewTerra => planet! {
                name: "New Terra",
                planet_traits: [Industrial],
                tech_specialities: [Biotic],
                resources: 1,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Tinnes => planet! {
                name: "Tinnes",
                planet_traits: [Industrial, Hazardous],
                tech_specialities: [Biotic],
                resources: 2,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Bellatrix => planet! {
                name: "Bellatrix",
                planet_traits: [Cultural],
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::TsionStation => space_station! {
                name: "Tsion Station",
                resources: 1,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Tarana => planet! {
                name: "Tarana",
                planet_traits: [Cultural, Industrial],
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::OluzStation => space_station! {
                name: "Oluz Station",
                resources: 1,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::Cocytus => planet! {
                name: "Cocytus",
                resources: 3,
                influence: 0,
                expansion: ThundersEdge
            },
            Planet::Styx => planet! {
                name: "Styx",
                resources: 4,
                influence: 0,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Lethe => planet! {
                name: "Lethe",
                resources: 0,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::Phlegethon => planet! {
                name: "Phlegethon",
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::MecatolRexOmega => planet! {
                name: "Mecatol Rex Ω",
                resources: 1,
                influence: 6,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::ThundersEdge => planet! {
                name: "Thunder's Edge",
                resources: 5,
                influence: 1,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Ordinian => planet! {
                name: "Ordinian",
                resources: 0,
                influence: 0,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Avernus => planet! {
                name: "Avernus",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 0,
                expansion: ThundersEdge,
                is_legendary: true
            },
            Planet::Cronos => planet! {
                name: "Cronos",
                resources: 2,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::CronosHollow => planet! {
                name: "Cronos Hollow",
                resources: 3,
                influence: 0,
                expansion: ThundersEdge
            },
            Planet::Tallin => planet! {
                name: "Tallin",
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::TallinHollow => planet! {
                name: "Tallin Hollow",
                resources: 3,
                influence: 0,
                expansion: ThundersEdge
            },
            Planet::Revelation => space_station! {
                name: "Revelation",
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::MezLoOrzFeiZsha => planet! {
                name: "Mez Lo Orz Fei Zsha",
                resources: 2,
                influence: 1,
                expansion: ThundersEdge
            },
            Planet::RepoLoOrzQet => planet! {
                name: "Rep Lo Orz Qet",
                resources: 1,
                influence: 3,
                expansion: ThundersEdge
            },
            Planet::Ikatena => planet! {
                name: "Ikatena",
                resources: 4,
                influence: 4,
                expansion: ThundersEdge
            },
            Planet::AhkCreuxx => planet! {
                name: "Ahk Creuxx",
                resources: 4,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::Elnath => planet! {
                name: "El'Nath",
                planet_traits: [Hazardous],
                resources: 2,
                influence: 0,
                expansion: ThundersEdge
            },
            Planet::Horizon => planet! {
                name: "Horizon",
                planet_traits: [Cultural],
                resources: 1,
                influence: 2,
                expansion: ThundersEdge
            },
            Planet::LuthienVi => planet! {
                name: "Luthien VI",
                planet_traits: [Hazardous],
                resources: 3,
                influence: 1,
                expansion: ThundersEdge
            },
        }
    }

    pub fn is_mecatol_rex(&self) -> bool {
        matches!(self, Planet::MecatolRex | Planet::MecatolRexOmega)
    }

    /// Whether the planet is in a home system.
    pub fn is_in_home_system(&self) -> bool {
        systems().values().any(|system| {
            matches!(system.system_type, SystemType::HomeSystem(..))
                && system.planets.contains(self)
        })
    }

    /// Whether the planet is in the home system of `faction`.
    pub fn is_in_home_system_of(&self, faction: Faction) -> bool {
        systems().values().any(|system| {
            matches!(system.system_type, SystemType::HomeSystem(f) if f == faction)
                && system.planets.contains(self)
        })
    }

    /// False for planets that only exist as a token or a card, i.e. that aren't in any pre-defined system.
    pub fn has_system_tile(&self) -> bool {
        !matches!(
            self,
            Planet::Mirage
                | Planet::CustodiaVigilia
                | Planet::CustodiaVigiliaOmega
                | Planet::ThundersEdge
                | Planet::Avernus
        )
    }
}

impl Component for Planet {
    replacements! {
        MecatolRex -> MecatolRexOmega,
        CustodiaVigilia -> CustodiaVigiliaOmega,
    }

    fn expansion(&self) -> Expansion {
        self.info().expansion
    }
}

impl PartialOrd for Planet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Planet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.info()
            .name
            .to_lowercase()
            .cmp(&other.info().name.to_lowercase())
    }
}
