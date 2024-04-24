use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::data::common::expansions::Expansion;

use super::tech::TechCategory;

/// A planetary trait.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum PlanetTrait {
    Cultural,
    Hazardous,
    Industrial,
}

/// All relevant information for a planet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanetInfo {
    /// The name of the planet.
    pub name: String,
    /// Which, if any, planet trait the planet has.
    pub planet_trait: Option<PlanetTrait>,
    /// Which, if any, technology bonus the planet has.
    pub tech_specialty: Option<TechCategory>,
    /// How many resources the planet has.
    pub resources: u32,
    /// How much influence the planet provides.
    pub influence: u32,
    /// Which expansion the planet belongs to.
    pub expansion: Expansion,
    /// Weather the planet is legendary or not.
    pub is_legendary: bool,
}

macro_rules! p {
    ($name: literal, $trait:expr, $tech_spec:expr, $resources: literal, $influence: literal, $expansion:expr) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_trait: $trait,
            tech_specialty: $tech_spec,
            resources: $resources,
            influence: $influence,
            expansion: $expansion,
            is_legendary: false,
        }
    };
    ($name: literal, $trait:expr, $tech_spec:expr, $resources: literal, $influence: literal, $expansion: expr, $legendary: literal) => {
        PlanetInfo {
            name: $name.to_string(),
            planet_trait: $trait,
            tech_specialty: $tech_spec,
            resources: $resources,
            influence: $influence,
            expansion: $expansion,
            is_legendary: $legendary,
        }
    };
}

/// A planet
#[derive(Debug, Clone, Serialize, Deserialize, EnumIter, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}

impl Planet {
    /// Returns the [PlanetInfo] for this planet.
    pub fn info(&self) -> PlanetInfo {
        match self {
            Planet::Nestphar => p!("Nestphar", None, None, 3, 2, Expansion::Base),
            Planet::ArcPrime => p!("Arc Prime", None, None, 4, 0, Expansion::Base),
            Planet::WrenTerra => p!("Wren Terra", None, None, 2, 1, Expansion::Base),
            Planet::LisisII => p!("Lisis II", None, None, 1, 0, Expansion::Base),
            Planet::Ragh => p!("Ragh", None, None, 2, 1, Expansion::Base),
            Planet::Muaat => p!("Muaat", None, None, 4, 1, Expansion::Base),
            Planet::Hercant => p!("Hercant", None, None, 1, 1, Expansion::Base),
            Planet::Arretze => p!("Arretze", None, None, 2, 0, Expansion::Base),
            Planet::Kamdorn => p!("Kamdorn", None, None, 0, 1, Expansion::Base),
            Planet::Jord => p!("Jord", None, None, 4, 2, Expansion::Base),
            Planet::Creuss => p!("Creuss", None, None, 4, 2, Expansion::Base),
            Planet::ZeroZeroZero => p!("[0.0.0]", None, None, 5, 0, Expansion::Base),
            Planet::MollPrimus => p!("Moll Primus", None, None, 4, 1, Expansion::Base),
            Planet::Druaa => p!("Druaa", None, None, 3, 1, Expansion::Base),
            Planet::Maaluuk => p!("Maaluuk", None, None, 0, 2, Expansion::Base),
            Planet::MordaiII => p!("Mordai II", None, None, 4, 0, Expansion::Base),
            Planet::TrenLak => p!("Tren'Lak", None, None, 1, 0, Expansion::Base),
            Planet::Quinarra => p!("Quinarra", None, None, 3, 1, Expansion::Base),
            Planet::Jol => p!("Jol", None, None, 1, 2, Expansion::Base),
            Planet::Nar => p!("Nar", None, None, 2, 3, Expansion::Base),
            Planet::Winnu => p!("Winnu", None, None, 3, 4, Expansion::Base),
            Planet::ArchonRen => p!("Archon Ren", None, None, 2, 3, Expansion::Base),
            Planet::ArchonTau => p!("Archon Tau", None, None, 1, 1, Expansion::Base),
            Planet::Darien => p!("Darien", None, None, 4, 4, Expansion::Base),
            Planet::Retillion => p!("Retillion", None, None, 2, 3, Expansion::Base),
            Planet::Shalloq => p!("Shalloq", None, None, 1, 2, Expansion::Base),
            Planet::MecatolRex => p!("Mecatol Rex", None, None, 1, 6, Expansion::Base),
            Planet::Abyz => p!(
                "Abyz",
                Some(PlanetTrait::Hazardous),
                None,
                3,
                0,
                Expansion::Base
            ),
            Planet::Fria => p!(
                "Fria",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                0,
                Expansion::Base
            ),
            Planet::Arinam => p!(
                "Arinam",
                Some(PlanetTrait::Industrial),
                None,
                1,
                2,
                Expansion::Base
            ),
            Planet::Meer => p!(
                "Meer",
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Warfare),
                0,
                4,
                Expansion::Base
            ),
            Planet::Arnor => p!(
                "Arnor",
                Some(PlanetTrait::Industrial),
                None,
                2,
                1,
                Expansion::Base
            ),
            Planet::Lor => p!(
                "Lor",
                Some(PlanetTrait::Industrial),
                None,
                1,
                2,
                Expansion::Base
            ),
            Planet::Bereg => p!(
                "Bereg",
                Some(PlanetTrait::Hazardous),
                None,
                3,
                1,
                Expansion::Base
            ),
            Planet::LirtaIV => p!(
                "Lirta IV",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                3,
                Expansion::Base
            ),
            Planet::Centauri => p!(
                "Centauri",
                Some(PlanetTrait::Cultural),
                None,
                1,
                3,
                Expansion::Base
            ),
            Planet::Gral => p!(
                "Gral",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Propulsion),
                1,
                1,
                Expansion::Base
            ),
            Planet::Corneeq => p!(
                "Corneeq",
                Some(PlanetTrait::Cultural),
                None,
                1,
                2,
                Expansion::Base
            ),
            Planet::Resculon => p!(
                "Resculon",
                Some(PlanetTrait::Cultural),
                None,
                2,
                0,
                Expansion::Base
            ),
            Planet::DalBootha => p!(
                "Dal Bootha",
                Some(PlanetTrait::Cultural),
                None,
                0,
                2,
                Expansion::Base
            ),
            Planet::Xxehan => p!(
                "Xxehan",
                Some(PlanetTrait::Cultural),
                None,
                1,
                1,
                Expansion::Base
            ),
            Planet::Lazar => p!(
                "Lazar",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Cybernetic),
                1,
                0,
                Expansion::Base
            ),
            Planet::Sakulag => p!(
                "Sakulag",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                1,
                Expansion::Base
            ),
            Planet::Lodor => p!(
                "Lodor",
                Some(PlanetTrait::Cultural),
                None,
                3,
                1,
                Expansion::Base
            ),
            Planet::MeharXull => p!(
                "Mehar Xull",
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Warfare),
                1,
                3,
                Expansion::Base
            ),
            Planet::Mellon => p!(
                "Mellon",
                Some(PlanetTrait::Cultural),
                None,
                0,
                2,
                Expansion::Base
            ),
            Planet::Zohbat => p!(
                "Zohbat",
                Some(PlanetTrait::Hazardous),
                None,
                3,
                1,
                Expansion::Base
            ),
            Planet::NewAlbion => p!(
                "New Albion",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1,
                Expansion::Base
            ),
            Planet::Starpoint => p!(
                "Starpoint",
                Some(PlanetTrait::Hazardous),
                None,
                3,
                1,
                Expansion::Base
            ),
            Planet::Quann => p!(
                "Quann",
                Some(PlanetTrait::Cultural),
                None,
                2,
                1,
                Expansion::Base
            ),
            Planet::Qucenn => p!(
                "Qucen'n",
                Some(PlanetTrait::Industrial),
                None,
                1,
                2,
                Expansion::Base
            ),
            Planet::Rarron => p!(
                "Rarron",
                Some(PlanetTrait::Cultural),
                None,
                0,
                3,
                Expansion::Base
            ),
            Planet::Saudor => p!(
                "Saudor",
                Some(PlanetTrait::Industrial),
                None,
                2,
                2,
                Expansion::Base
            ),
            Planet::TarMann => p!(
                "Tar'Mann",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1,
                Expansion::Base
            ),
            Planet::TequRan => p!(
                "Tequ'ran",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                0,
                Expansion::Base
            ),
            Planet::Torkan => p!(
                "Torkan",
                Some(PlanetTrait::Cultural),
                None,
                0,
                3,
                Expansion::Base
            ),
            Planet::Thibah => p!(
                "Thibah",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Propulsion),
                1,
                1,
                Expansion::Base
            ),
            Planet::VefutII => p!(
                "Vefut II",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                2,
                Expansion::Base
            ),
            Planet::Wellon => p!(
                "Wellon",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Cybernetic),
                1,
                2,
                Expansion::Base
            ),
            Planet::ArchonVail => p!(
                "Archon Vail",
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Propulsion),
                1,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Perimiter => p!(
                "Perimeter",
                Some(PlanetTrait::Industrial),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Ang => p!(
                "Ang",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Warfare),
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::SemLore => p!(
                "Sem-Lore",
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Cybernetic),
                3,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Vorhal => p!(
                "Vorhal",
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Biotic),
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Atlas => p!(
                "Atlas",
                Some(PlanetTrait::Hazardous),
                None,
                3,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Primor => p!(
                "Primor",
                Some(PlanetTrait::Cultural),
                None,
                2,
                1,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::HopesEnd => p!(
                "Hope's End",
                Some(PlanetTrait::Hazardous),
                None,
                3,
                0,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::Cormund => p!(
                "Cormund",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Everra => p!(
                "Everra",
                Some(PlanetTrait::Cultural),
                None,
                3,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::JeolIr => p!(
                "Jeol Ir",
                Some(PlanetTrait::Industrial),
                None,
                2,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Accoen => p!(
                "Accoen",
                Some(PlanetTrait::Industrial),
                None,
                2,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Kraag => p!(
                "Kraag",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Siig => p!(
                "Siig",
                Some(PlanetTrait::Hazardous),
                None,
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Bakal => p!(
                "Ba'kal",
                Some(PlanetTrait::Industrial),
                None,
                3,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::AlioPrima => p!(
                "Alio Prima",
                Some(PlanetTrait::Cultural),
                None,
                1,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Lisis => p!(
                "Lisis",
                Some(PlanetTrait::Industrial),
                None,
                2,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Velnor => p!(
                "Velnor",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Warfare),
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Cealdri => p!(
                "Cealdri",
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Cybernetic),
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Xanhact => p!(
                "Xanhact",
                Some(PlanetTrait::Hazardous),
                None,
                0,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::VegaMajor => p!(
                "Vega Major",
                Some(PlanetTrait::Cultural),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::VegaMinor => p!(
                "Vega Minor",
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Propulsion),
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Abaddon => p!(
                "Abaddon",
                Some(PlanetTrait::Cultural),
                None,
                1,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Ashtroth => p!(
                "Ashtroth",
                Some(PlanetTrait::Hazardous),
                None,
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Loki => p!(
                "Loki",
                Some(PlanetTrait::Cultural),
                None,
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelI => p!(
                "Rigel I",
                Some(PlanetTrait::Hazardous),
                None,
                0,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelII => p!(
                "Rigel II",
                Some(PlanetTrait::Industrial),
                None,
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelIII => p!(
                "Rigel III",
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Valk => p!("Valk", None, None, 2, 0, Expansion::ProphecyOfKings),
            Planet::Avar => p!("Avar", None, None, 1, 1, Expansion::ProphecyOfKings),
            Planet::Ylir => p!("Ylir", None, None, 0, 2, Expansion::ProphecyOfKings),
            Planet::TheDark => p!("The Dark", None, None, 3, 4, Expansion::ProphecyOfKings),
            Planet::Ixth => p!("Ixth", None, None, 3, 5, Expansion::ProphecyOfKings),
            Planet::Naazir => p!("Naazir", None, None, 2, 1, Expansion::ProphecyOfKings),
            Planet::Rokha => p!("Rokha", None, None, 1, 2, Expansion::ProphecyOfKings),
            Planet::Arcturus => p!("Arcturus", None, None, 4, 4, Expansion::ProphecyOfKings),
            Planet::Elysium => p!("Elysium", None, None, 4, 1, Expansion::ProphecyOfKings),
            Planet::Acheron => p!("Acheron", None, None, 4, 0, Expansion::ProphecyOfKings),
            Planet::Mallice => p!(
                "Mallice",
                Some(PlanetTrait::Cultural),
                None,
                0,
                3,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::Mirage => p!(
                "Mirage",
                Some(PlanetTrait::Cultural),
                None,
                1,
                2,
                Expansion::ProphecyOfKings,
                true
            ),
            Planet::CustodiaVigilia => {
                p!(
                    "Custodia Vigilia",
                    None,
                    None,
                    2,
                    3,
                    Expansion::CodexIII,
                    true
                )
            }
        }
    }
}
