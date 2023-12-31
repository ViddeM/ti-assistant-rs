use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

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
    /// Which, if any, planet trait the planet has.
    pub planet_trait: Option<PlanetTrait>,
    /// Which, if any, technology bonus the planet has.
    pub tech_speciality: Option<TechCategory>,
    /// How many resources the planet has.
    pub resources: u32,
    /// How much influence the planet provides.
    pub influence: u32,
}

macro_rules! p {
    ($trait:expr, $tech_spec:expr, $resources: literal, $influence: literal) => {
        PlanetInfo {
            planet_trait: $trait,
            tech_speciality: $tech_spec,
            resources: $resources,
            influence: $influence,
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
    Coorneeq,
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
    JoelIr,
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
}

impl Planet {
    /// Returns the [PlanetInfo] for this planet.
    pub fn planet_info(&self) -> PlanetInfo {
        match self {
            Planet::Nestphar => p!(None, None, 3, 2),
            Planet::ArcPrime => p!(None, None, 4, 0),
            Planet::WrenTerra => p!(None, None, 2, 1),
            Planet::LisisII => p!(None, None, 1, 0),
            Planet::Ragh => p!(None, None, 2, 1),
            Planet::Muaat => p!(None, None, 4, 1),
            Planet::Hercant => p!(None, None, 1, 1),
            Planet::Arretze => p!(None, None, 2, 0),
            Planet::Kamdorn => p!(None, None, 0, 1),
            Planet::Jord => p!(None, None, 4, 2),
            Planet::Creuss => p!(None, None, 4, 2),
            Planet::ZeroZeroZero => p!(None, None, 5, 0),
            Planet::MollPrimus => p!(None, None, 4, 1),
            Planet::Druaa => p!(None, None, 3, 1),
            Planet::Maaluuk => p!(None, None, 0, 2),
            Planet::MordaiII => p!(None, None, 4, 0),
            Planet::TrenLak => p!(None, None, 1, 0),
            Planet::Quinarra => p!(None, None, 3, 1),
            Planet::Jol => p!(None, None, 1, 2),
            Planet::Nar => p!(None, None, 2, 3),
            Planet::Winnu => p!(None, None, 3, 4),
            Planet::ArchonRen => p!(None, None, 2, 3),
            Planet::ArchonTau => p!(None, None, 1, 1),
            Planet::Darien => p!(None, None, 4, 4),
            Planet::Retillion => p!(None, None, 2, 3),
            Planet::Shalloq => p!(None, None, 1, 2),
            Planet::MecatolRex => p!(None, None, 1, 6),
            Planet::Abyz => p!(Some(PlanetTrait::Hazardous), None, 3, 0),
            Planet::Fria => p!(Some(PlanetTrait::Hazardous), None, 2, 0),
            Planet::Arinam => p!(Some(PlanetTrait::Industrial), None, 1, 2),
            Planet::Meer => p!(
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Warfare),
                0,
                4
            ),
            Planet::Arnor => p!(Some(PlanetTrait::Industrial), None, 2, 1),
            Planet::Lor => p!(Some(PlanetTrait::Industrial), None, 1, 2),
            Planet::Bereg => p!(Some(PlanetTrait::Hazardous), None, 3, 1),
            Planet::LirtaIV => p!(Some(PlanetTrait::Hazardous), None, 2, 3),
            Planet::Centauri => p!(Some(PlanetTrait::Cultural), None, 1, 3),
            Planet::Gral => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Propulsion),
                1,
                1
            ),
            Planet::Coorneeq => p!(Some(PlanetTrait::Cultural), None, 1, 2),
            Planet::Resculon => p!(Some(PlanetTrait::Cultural), None, 2, 0),
            Planet::DalBootha => p!(Some(PlanetTrait::Cultural), None, 0, 2),
            Planet::Xxehan => p!(Some(PlanetTrait::Cultural), None, 1, 1),
            Planet::Lazar => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Cybernetic),
                1,
                0
            ),
            Planet::Sakulag => p!(Some(PlanetTrait::Hazardous), None, 2, 1),
            Planet::Lodor => p!(Some(PlanetTrait::Cultural), None, 3, 1),
            Planet::MeharXull => p!(
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Warfare),
                1,
                3
            ),
            Planet::Mellon => p!(Some(PlanetTrait::Cultural), None, 0, 2),
            Planet::Zohbat => p!(Some(PlanetTrait::Hazardous), None, 3, 1),
            Planet::NewAlbion => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1
            ),
            Planet::Starpoint => p!(Some(PlanetTrait::Hazardous), None, 3, 1),
            Planet::Quann => p!(Some(PlanetTrait::Cultural), None, 2, 1),
            Planet::Qucenn => p!(Some(PlanetTrait::Industrial), None, 1, 2),
            Planet::Rarron => p!(Some(PlanetTrait::Cultural), None, 0, 3),
            Planet::Saudor => p!(Some(PlanetTrait::Industrial), None, 2, 2),
            Planet::TarMann => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1
            ),
            Planet::TequRan => p!(Some(PlanetTrait::Industrial), None, 2, 0),
            Planet::Torkan => p!(Some(PlanetTrait::Cultural), None, 0, 3),
            Planet::Thibah => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Propulsion),
                1,
                1
            ),
            Planet::VefutII => p!(Some(PlanetTrait::Hazardous), None, 2, 2),
            Planet::Wellon => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Cybernetic),
                1,
                2
            ),
            Planet::ArchonVail => p!(
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Propulsion),
                1,
                3
            ),
            Planet::Perimiter => p!(Some(PlanetTrait::Industrial), None, 2, 1),
            Planet::Ang => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Warfare),
                2,
                0
            ),
            Planet::SemLore => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Cybernetic),
                3,
                2
            ),
            Planet::Vorhal => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Biotic),
                0,
                2
            ),
            Planet::Atlas => p!(Some(PlanetTrait::Hazardous), None, 3, 1),
            Planet::Primor => p!(Some(PlanetTrait::Cultural), None, 2, 1),
            Planet::HopesEnd => p!(Some(PlanetTrait::Hazardous), None, 3, 0),
            Planet::Cormund => p!(Some(PlanetTrait::Hazardous), None, 2, 1),
            Planet::Everra => p!(Some(PlanetTrait::Cultural), None, 3, 1),
            Planet::JoelIr => p!(Some(PlanetTrait::Industrial), None, 2, 3),
            Planet::Accoen => p!(Some(PlanetTrait::Industrial), None, 2, 3),
            Planet::Kraag => p!(Some(PlanetTrait::Hazardous), None, 2, 1),
            Planet::Siig => p!(Some(PlanetTrait::Hazardous), None, 0, 2),
            Planet::Bakal => p!(Some(PlanetTrait::Industrial), None, 3, 2),
            Planet::AlioPrima => p!(Some(PlanetTrait::Cultural), None, 1, 1),
            Planet::Lisis => p!(Some(PlanetTrait::Industrial), None, 2, 2),
            Planet::Velnor => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Warfare),
                2,
                1
            ),
            Planet::Cealdri => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Cybernetic),
                0,
                2
            ),
            Planet::Xanhact => p!(Some(PlanetTrait::Hazardous), None, 0, 1),
            Planet::VegaMajor => p!(Some(PlanetTrait::Cultural), None, 2, 1),
            Planet::VegaMinor => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Propulsion),
                1,
                2
            ),
            Planet::Abaddon => p!(Some(PlanetTrait::Cultural), None, 1, 0),
            Planet::Ashtroth => p!(Some(PlanetTrait::Hazardous), None, 2, 0),
            Planet::Loki => p!(Some(PlanetTrait::Cultural), None, 1, 2),
            Planet::RigelI => p!(Some(PlanetTrait::Hazardous), None, 0, 1),
            Planet::RigelII => p!(Some(PlanetTrait::Industrial), None, 1, 2),
            Planet::RigelIII => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1
            ),
            Planet::Valk => p!(None, None, 2, 0),
            Planet::Avar => p!(None, None, 1, 1),
            Planet::Ylir => p!(None, None, 0, 2),
            Planet::TheDark => p!(None, None, 3, 4),
            Planet::Ixth => p!(None, None, 3, 5),
            Planet::Naazir => p!(None, None, 2, 1),
            Planet::Rokha => p!(None, None, 1, 2),
            Planet::Arcturus => p!(None, None, 4, 4),
            Planet::Elysium => p!(None, None, 4, 1),
            Planet::Acheron => p!(None, None, 4, 0),
            Planet::Mallice => p!(Some(PlanetTrait::Cultural), None, 0, 3),
            Planet::Mirage => p!(Some(PlanetTrait::Cultural), None, 1, 2),
        }
    }
}
