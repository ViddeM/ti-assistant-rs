use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::common::expansions::Expansion;

use super::tech::TechCategory;

/// A planetary trait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[allow(missing_docs)]
pub enum PlanetTrait {
    Cultural,
    Hazardous,
    Industrial,
}

/// All relevant information for a planet.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
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
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub enum OrbitalBodyType {
    Planet,
    SpaceStation,
}

macro_rules! planet {
    ($name: literal, $traits:expr, $tech_specs:expr, $resources: literal, $influence: literal, $expansion:expr) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_traits: $traits,
            tech_specialities: $tech_specs,
            resources: $resources,
            influence: $influence,
            expansion: $expansion,
            is_legendary: false,
            body_type: OrbitalBodyType::Planet,
        }
    };
    ($name: literal, $traits:expr, $tech_specs:expr, $resources: literal, $influence: literal, $expansion: expr, $legendary: literal) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_traits: $traits,
            tech_specialities: $tech_specs,
            resources: $resources,
            influence: $influence,
            expansion: $expansion,
            is_legendary: $legendary,
            body_type: OrbitalBodyType::Planet,
        }
    };
}

macro_rules! space_station {
    ($name: literal, $resources:literal, $influence:literal, $expansion:expr) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_traits: vec![],
            tech_specialities: vec![],
            resources: $resources,
            influence: $influence,
            expansion: $expansion,
            is_legendary: false,
            body_type: OrbitalBodyType::SpaceStation,
        }
    };
}

/// A planet
#[derive(
    Debug, Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord, Hash, TS,
)]
#[ts(export)]
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
            Planet::Nestphar => planet!("Nestphar", vec![], vec![], 3, 2, Expansion::Base),
            Planet::ArcPrime => planet!("Arc Prime", vec![], vec![], 4, 0, Expansion::Base),
            Planet::WrenTerra => planet!("Wren Terra", vec![], vec![], 2, 1, Expansion::Base),
            Planet::LisisII => planet!("Lisis II", vec![], vec![], 1, 0, Expansion::Base),
            Planet::Ragh => planet!("Ragh", vec![], vec![], 2, 1, Expansion::Base),
            Planet::Muaat => planet!("Muaat", vec![], vec![], 4, 1, Expansion::Base),
            Planet::Hercant => planet!("Hercant", vec![], vec![], 1, 1, Expansion::Base),
            Planet::Arretze => planet!("Arretze", vec![], vec![], 2, 0, Expansion::Base),
            Planet::Kamdorn => planet!("Kamdorn", vec![], vec![], 0, 1, Expansion::Base),
            Planet::Jord => planet!("Jord", vec![], vec![], 4, 2, Expansion::Base),
            Planet::Creuss => planet!("Creuss", vec![], vec![], 4, 2, Expansion::Base),
            Planet::ZeroZeroZero => planet!("[0.0.0]", vec![], vec![], 5, 0, Expansion::Base),
            Planet::MollPrimus => planet!("Moll Primus", vec![], vec![], 4, 1, Expansion::Base),
            Planet::Druaa => planet!("Druaa", vec![], vec![], 3, 1, Expansion::Base),
            Planet::Maaluuk => planet!("Maaluuk", vec![], vec![], 0, 2, Expansion::Base),
            Planet::MordaiII => planet!("Mordai II", vec![], vec![], 4, 0, Expansion::Base),
            Planet::TrenLak => planet!("Tren'Lak", vec![], vec![], 1, 0, Expansion::Base),
            Planet::Quinarra => planet!("Quinarra", vec![], vec![], 3, 1, Expansion::Base),
            Planet::Jol => planet!("Jol", vec![], vec![], 1, 2, Expansion::Base),
            Planet::Nar => planet!("Nar", vec![], vec![], 2, 3, Expansion::Base),
            Planet::Winnu => planet!("Winnu", vec![], vec![], 3, 4, Expansion::Base),
            Planet::ArchonRen => planet!("Archon Ren", vec![], vec![], 2, 3, Expansion::Base),
            Planet::ArchonTau => planet!("Archon Tau", vec![], vec![], 1, 1, Expansion::Base),
            Planet::Darien => planet!("Darien", vec![], vec![], 4, 4, Expansion::Base),
            Planet::Retillion => planet!("Retillion", vec![], vec![], 2, 3, Expansion::Base),
            Planet::Shalloq => planet!("Shalloq", vec![], vec![], 1, 2, Expansion::Base),
            Planet::MecatolRex => planet!("Mecatol Rex", vec![], vec![], 1, 6, Expansion::Base),
            Planet::Abyz => planet!(
                "Abyz",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                0,
                Expansion::Base
            ),
            Planet::Fria => planet!(
                "Fria",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                0,
                Expansion::Base
            ),
            Planet::Arinam => planet!(
                "Arinam",
                vec![PlanetTrait::Industrial],
                vec![],
                1,
                2,
                Expansion::Base
            ),
            Planet::Meer => planet!(
                "Meer",
                vec![PlanetTrait::Hazardous],
                vec![TechCategory::Warfare],
                0,
                4,
                Expansion::Base
            ),
            Planet::Arnor => planet!(
                "Arnor",
                vec![PlanetTrait::Industrial],
                vec![],
                2,
                1,
                Expansion::Base
            ),
            Planet::Lor => planet!(
                "Lor",
                vec![PlanetTrait::Industrial],
                vec![],
                1,
                2,
                Expansion::Base
            ),
            Planet::Bereg => planet!(
                "Bereg",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                1,
                Expansion::Base
            ),
            Planet::LirtaIV => planet!(
                "Lirta IV",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                3,
                Expansion::Base
            ),
            Planet::Centauri => planet!(
                "Centauri",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                3,
                Expansion::Base
            ),
            Planet::Gral => planet!(
                "Gral",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Propulsion],
                1,
                1,
                Expansion::Base
            ),
            Planet::Corneeq => planet!(
                "Corneeq",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                2,
                Expansion::Base
            ),
            Planet::Resculon => planet!(
                "Resculon",
                vec![PlanetTrait::Cultural],
                vec![],
                2,
                0,
                Expansion::Base
            ),
            Planet::DalBootha => planet!(
                "Dal Bootha",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                2,
                Expansion::Base
            ),
            Planet::Xxehan => planet!(
                "Xxehan",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                1,
                Expansion::Base
            ),
            Planet::Lazar => planet!(
                "Lazar",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Cybernetic],
                1,
                0,
                Expansion::Base
            ),
            Planet::Sakulag => planet!(
                "Sakulag",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                1,
                Expansion::Base
            ),
            Planet::Lodor => planet!(
                "Lodor",
                vec![PlanetTrait::Cultural],
                vec![],
                3,
                1,
                Expansion::Base
            ),
            Planet::MeharXull => planet!(
                "Mehar Xull",
                vec![PlanetTrait::Hazardous],
                vec![TechCategory::Warfare],
                1,
                3,
                Expansion::Base
            ),
            Planet::Mellon => planet!(
                "Mellon",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                2,
                Expansion::Base
            ),
            Planet::Zohbat => planet!(
                "Zohbat",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                1,
                Expansion::Base
            ),
            Planet::NewAlbion => planet!(
                "New Albion",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Biotic],
                1,
                1,
                Expansion::Base
            ),
            Planet::Starpoint => planet!(
                "Starpoint",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                1,
                Expansion::Base
            ),
            Planet::Quann => planet!(
                "Quann",
                vec![PlanetTrait::Cultural],
                vec![],
                2,
                1,
                Expansion::Base
            ),
            Planet::Qucenn => planet!(
                "Qucen'n",
                vec![PlanetTrait::Industrial],
                vec![],
                1,
                2,
                Expansion::Base
            ),
            Planet::Rarron => planet!(
                "Rarron",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                3,
                Expansion::Base
            ),
            Planet::Saudor => planet!(
                "Saudor",
                vec![PlanetTrait::Industrial],
                vec![],
                2,
                2,
                Expansion::Base
            ),
            Planet::TarMann => planet!(
                "Tar'Mann",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Biotic],
                1,
                1,
                Expansion::Base
            ),
            Planet::TequRan => planet!(
                "Tequ'ran",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                0,
                Expansion::Base
            ),
            Planet::Torkan => planet!(
                "Torkan",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                3,
                Expansion::Base
            ),
            Planet::Thibah => planet!(
                "Thibah",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Propulsion],
                1,
                1,
                Expansion::Base
            ),
            Planet::VefutII => planet!(
                "Vefut II",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                2,
                Expansion::Base
            ),
            Planet::Wellon => planet!(
                "Wellon",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Cybernetic],
                1,
                2,
                Expansion::Base
            ),
            Planet::ArchonVail => planet!(
                "Archon Vail",
                vec![PlanetTrait::Hazardous],
                vec![TechCategory::Propulsion],
                1,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Perimiter => planet!(
                "Perimeter",
                vec![PlanetTrait::Industrial],
                vec![],
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Ang => planet!(
                "Ang",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Warfare],
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::SemLore => planet!(
                "Sem-Lore",
                vec![PlanetTrait::Cultural],
                vec![TechCategory::Cybernetic],
                3,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Vorhal => planet!(
                "Vorhal",
                vec![PlanetTrait::Cultural],
                vec![TechCategory::Biotic],
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Atlas => planet!(
                "Atlas",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Primor => planet!(
                "Primor",
                vec![PlanetTrait::Cultural],
                vec![],
                2,
                1,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::HopesEnd => planet!(
                "Hope's End",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                0,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::Cormund => planet!(
                "Cormund",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Everra => planet!(
                "Everra",
                vec![PlanetTrait::Cultural],
                vec![],
                3,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::JeolIr => planet!(
                "Jeol Ir",
                vec![PlanetTrait::Industrial],
                vec![],
                2,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Accoen => planet!(
                "Accoen",
                vec![PlanetTrait::Industrial],
                vec![],
                2,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Kraag => planet!(
                "Kraag",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Siig => planet!(
                "Siig",
                vec![PlanetTrait::Hazardous],
                vec![],
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Bakal => planet!(
                "Ba'kal",
                vec![PlanetTrait::Industrial],
                vec![],
                3,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::AlioPrima => planet!(
                "Alio Prima",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Lisis => planet!(
                "Lisis",
                vec![PlanetTrait::Industrial],
                vec![],
                2,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Velnor => planet!(
                "Velnor",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Warfare],
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Cealdri => planet!(
                "Cealdri",
                vec![PlanetTrait::Cultural],
                vec![TechCategory::Cybernetic],
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Xanhact => planet!(
                "Xanhact",
                vec![PlanetTrait::Hazardous],
                vec![],
                0,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::VegaMajor => planet!(
                "Vega Major",
                vec![PlanetTrait::Cultural],
                vec![],
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::VegaMinor => planet!(
                "Vega Minor",
                vec![PlanetTrait::Cultural],
                vec![TechCategory::Propulsion],
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Abaddon => planet!(
                "Abaddon",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Ashtroth => planet!(
                "Ashtroth",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Loki => planet!(
                "Loki",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelI => planet!(
                "Rigel I",
                vec![PlanetTrait::Hazardous],
                vec![],
                0,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelII => planet!(
                "Rigel II",
                vec![PlanetTrait::Industrial],
                vec![],
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelIII => planet!(
                "Rigel III",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Biotic],
                1,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Valk => planet!("Valk", vec![], vec![], 2, 0, Expansion::ProphecyOfKings),
            Planet::Avar => planet!("Avar", vec![], vec![], 1, 1, Expansion::ProphecyOfKings),
            Planet::Ylir => planet!("Ylir", vec![], vec![], 0, 2, Expansion::ProphecyOfKings),
            Planet::TheDark => {
                planet!("The Dark", vec![], vec![], 3, 4, Expansion::ProphecyOfKings)
            }
            Planet::Ixth => planet!("Ixth", vec![], vec![], 3, 5, Expansion::ProphecyOfKings),
            Planet::Naazir => planet!("Naazir", vec![], vec![], 2, 1, Expansion::ProphecyOfKings),
            Planet::Rokha => planet!("Rokha", vec![], vec![], 1, 2, Expansion::ProphecyOfKings),
            Planet::Arcturus => {
                planet!("Arcturus", vec![], vec![], 4, 4, Expansion::ProphecyOfKings)
            }
            Planet::Elysium => planet!("Elysium", vec![], vec![], 4, 1, Expansion::ProphecyOfKings),
            Planet::Acheron => planet!("Acheron", vec![], vec![], 4, 0, Expansion::ProphecyOfKings),
            Planet::Mallice => planet!(
                "Mallice",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                3,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::Mirage => planet!(
                "Mirage",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                2,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::CustodiaVigilia => {
                planet!(
                    "Custodia Vigilia",
                    vec![],
                    vec![],
                    2,
                    3,
                    Expansion::CodexIII,
                    true
                )
            }
            Planet::Lesab => planet!(
                "Lesab",
                vec![PlanetTrait::Industrial, PlanetTrait::Hazardous],
                vec![],
                2,
                1,
                Expansion::ThundersEdge
            ),
            Planet::Olergodt => planet!(
                "Olergodt",
                vec![PlanetTrait::Cultural, PlanetTrait::Hazardous],
                vec![TechCategory::Cybernetic, TechCategory::Warfare],
                2,
                1,
                Expansion::ThundersEdge
            ),
            Planet::ViraPicsIII => planet!(
                "Vira-Pics III",
                vec![PlanetTrait::Cultural, PlanetTrait::Hazardous],
                vec![],
                2,
                3,
                Expansion::ThundersEdge
            ),
            Planet::Andeara => planet!(
                "Andeara",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Propulsion],
                1,
                1,
                Expansion::ThundersEdge
            ),
            Planet::Lemox => planet!(
                "Lemox",
                vec![PlanetTrait::Industrial],
                vec![],
                0,
                3,
                Expansion::ThundersEdge
            ),
            Planet::TheWatchtower => {
                space_station!("The Watchtower", 1, 1, Expansion::ThundersEdge)
            }
            Planet::Emelpar => planet!(
                "Emelpar",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                2,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Faunus => planet!(
                "Faunus",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Biotic],
                1,
                3,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Garbozia => planet!(
                "Garbozia",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                1,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Tempesta => planet!(
                "Tempesta",
                vec![PlanetTrait::Hazardous],
                vec![TechCategory::Propulsion],
                1,
                1,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Industrex => planet!(
                "Industrex",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Warfare],
                2,
                0,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Capha => planet!(
                "Capha",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                0,
                Expansion::ThundersEdge
            ),
            Planet::Kostboth => planet!(
                "Kostboth",
                vec![PlanetTrait::Cultural],
                vec![],
                0,
                1,
                Expansion::ThundersEdge
            ),
            Planet::Cresius => planet!(
                "Cresius",
                vec![PlanetTrait::Hazardous],
                vec![],
                0,
                1,
                Expansion::ThundersEdge
            ),
            Planet::LazulRex => planet!(
                "Lazul Rex",
                vec![PlanetTrait::Cultural, PlanetTrait::Industrial],
                vec![],
                2,
                2,
                Expansion::ThundersEdge
            ),
            Planet::Hercalor => planet!(
                "Hercalor",
                vec![PlanetTrait::Industrial],
                vec![],
                1,
                0,
                Expansion::ThundersEdge
            ),
            Planet::Tiamat => planet!(
                "Tiamat",
                vec![PlanetTrait::Cultural],
                vec![TechCategory::Cybernetic],
                1,
                2,
                Expansion::ThundersEdge
            ),
            Planet::NewTerra => planet!(
                "New Terra",
                vec![PlanetTrait::Industrial],
                vec![TechCategory::Biotic],
                1,
                1,
                Expansion::ThundersEdge
            ),
            Planet::Tinnes => planet!(
                "Tinnes",
                vec![PlanetTrait::Industrial, PlanetTrait::Hazardous],
                vec![TechCategory::Biotic],
                2,
                1,
                Expansion::ThundersEdge
            ),
            Planet::Bellatrix => planet!(
                "Bellatrix",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                2,
                Expansion::ThundersEdge
            ),
            Planet::TsionStation => space_station!("Tsion Station", 1, 1, Expansion::ThundersEdge),
            Planet::Tarana => planet!(
                "Tarana",
                vec![PlanetTrait::Cultural, PlanetTrait::Industrial],
                vec![],
                1,
                2,
                Expansion::ThundersEdge
            ),
            Planet::OluzStation => space_station!("Oluz Station", 1, 1, Expansion::ThundersEdge),
            Planet::Cocytus => planet!("Cocytus", vec![], vec![], 3, 0, Expansion::ThundersEdge),
            Planet::Styx => planet!("Styx", vec![], vec![], 3, 0, Expansion::ThundersEdge, true),
            Planet::Lethe => planet!("Lethe", vec![], vec![], 0, 2, Expansion::ThundersEdge),
            Planet::Phlegethon => {
                planet!("Phlegethon", vec![], vec![], 1, 2, Expansion::ThundersEdge)
            }
            Planet::MecatolRexOmega => planet!(
                "Mecatol Rex Ω",
                vec![],
                vec![],
                1,
                6,
                Expansion::ThundersEdge,
                true
            ),
            Planet::ThundersEdge => planet!(
                "Thunder's Edge",
                vec![],
                vec![],
                5,
                1,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Ordinian => planet!(
                "Ordinian",
                vec![],
                vec![],
                0,
                0,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Avernus => planet!(
                "Avernus",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                0,
                Expansion::ThundersEdge,
                true
            ),
            Planet::Cronos => planet!("Cronos", vec![], vec![], 2, 1, Expansion::ThundersEdge),
            Planet::CronosHollow => planet!(
                "Cronos Hollow",
                vec![],
                vec![],
                3,
                0,
                Expansion::ThundersEdge
            ),
            Planet::Tallin => planet!(
                "Tallin Hollow",
                vec![],
                vec![],
                1,
                2,
                Expansion::ThundersEdge
            ),
            Planet::TallinHollow => planet!(
                "Tallin Hollow",
                vec![],
                vec![],
                3,
                0,
                Expansion::ThundersEdge
            ),
            Planet::Revelation => space_station!("Revelation", 1, 2, Expansion::ThundersEdge),
            Planet::MezLoOrzFeiZsha => planet!(
                "Mez Lo Orz Fei Zsha",
                vec![],
                vec![],
                2,
                1,
                Expansion::ThundersEdge
            ),
            Planet::RepoLoOrzQet => planet!(
                "Rep Lo Orz Qet",
                vec![],
                vec![],
                1,
                3,
                Expansion::ThundersEdge
            ),
            Planet::Ikatena => planet!("Ikatena", vec![], vec![], 4, 4, Expansion::ThundersEdge),
            Planet::AhkCreuxx => {
                planet!("Ahk Creuxx", vec![], vec![], 4, 2, Expansion::ThundersEdge)
            }
            Planet::Elnath => planet!(
                "Elnath",
                vec![PlanetTrait::Hazardous],
                vec![],
                2,
                0,
                Expansion::ThundersEdge
            ),
            Planet::Horizon => planet!(
                "Horizon",
                vec![PlanetTrait::Cultural],
                vec![],
                1,
                2,
                Expansion::ThundersEdge
            ),
            Planet::LuthienVi => planet!(
                "Luthien VI",
                vec![PlanetTrait::Hazardous],
                vec![],
                3,
                1,
                Expansion::ThundersEdge
            ),
        }
    }
}
