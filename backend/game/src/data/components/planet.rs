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
    /// Which, if any, planet trait the planet has.
    pub planet_trait: Option<PlanetTrait>,
    /// Which, if any, technology bonus the planet has.
    pub tech_speciality: Option<TechCategory>,
    /// How many resources the planet has.
    pub resources: u32,
    /// How much influence the planet provides.
    pub influence: u32,
    /// Which expansion the planet belongs to.
    pub expansion: Expansion,
}

macro_rules! p {
    ($trait:expr, $tech_spec:expr, $resources: literal, $influence: literal, $expansion:expr) => {
        PlanetInfo {
            planet_trait: $trait,
            tech_speciality: $tech_spec,
            resources: $resources,
            influence: $influence,
            expansion: $expansion,
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
    pub fn info(&self) -> PlanetInfo {
        match self {
            Planet::Nestphar => p!(None, None, 3, 2, Expansion::Base),
            Planet::ArcPrime => p!(None, None, 4, 0, Expansion::Base),
            Planet::WrenTerra => p!(None, None, 2, 1, Expansion::Base),
            Planet::LisisII => p!(None, None, 1, 0, Expansion::Base),
            Planet::Ragh => p!(None, None, 2, 1, Expansion::Base),
            Planet::Muaat => p!(None, None, 4, 1, Expansion::Base),
            Planet::Hercant => p!(None, None, 1, 1, Expansion::Base),
            Planet::Arretze => p!(None, None, 2, 0, Expansion::Base),
            Planet::Kamdorn => p!(None, None, 0, 1, Expansion::Base),
            Planet::Jord => p!(None, None, 4, 2, Expansion::Base),
            Planet::Creuss => p!(None, None, 4, 2, Expansion::Base),
            Planet::ZeroZeroZero => p!(None, None, 5, 0, Expansion::Base),
            Planet::MollPrimus => p!(None, None, 4, 1, Expansion::Base),
            Planet::Druaa => p!(None, None, 3, 1, Expansion::Base),
            Planet::Maaluuk => p!(None, None, 0, 2, Expansion::Base),
            Planet::MordaiII => p!(None, None, 4, 0, Expansion::Base),
            Planet::TrenLak => p!(None, None, 1, 0, Expansion::Base),
            Planet::Quinarra => p!(None, None, 3, 1, Expansion::Base),
            Planet::Jol => p!(None, None, 1, 2, Expansion::Base),
            Planet::Nar => p!(None, None, 2, 3, Expansion::Base),
            Planet::Winnu => p!(None, None, 3, 4, Expansion::Base),
            Planet::ArchonRen => p!(None, None, 2, 3, Expansion::Base),
            Planet::ArchonTau => p!(None, None, 1, 1, Expansion::Base),
            Planet::Darien => p!(None, None, 4, 4, Expansion::Base),
            Planet::Retillion => p!(None, None, 2, 3, Expansion::Base),
            Planet::Shalloq => p!(None, None, 1, 2, Expansion::Base),
            Planet::MecatolRex => p!(None, None, 1, 6, Expansion::Base),
            Planet::Abyz => p!(Some(PlanetTrait::Hazardous), None, 3, 0, Expansion::Base),
            Planet::Fria => p!(Some(PlanetTrait::Hazardous), None, 2, 0, Expansion::Base),
            Planet::Arinam => p!(Some(PlanetTrait::Industrial), None, 1, 2, Expansion::Base),
            Planet::Meer => p!(
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Warfare),
                0,
                4,
                Expansion::Base
            ),
            Planet::Arnor => p!(Some(PlanetTrait::Industrial), None, 2, 1, Expansion::Base),
            Planet::Lor => p!(Some(PlanetTrait::Industrial), None, 1, 2, Expansion::Base),
            Planet::Bereg => p!(Some(PlanetTrait::Hazardous), None, 3, 1, Expansion::Base),
            Planet::LirtaIV => p!(Some(PlanetTrait::Hazardous), None, 2, 3, Expansion::Base),
            Planet::Centauri => p!(Some(PlanetTrait::Cultural), None, 1, 3, Expansion::Base),
            Planet::Gral => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Propulsion),
                1,
                1,
                Expansion::Base
            ),
            Planet::Coorneeq => p!(Some(PlanetTrait::Cultural), None, 1, 2, Expansion::Base),
            Planet::Resculon => p!(Some(PlanetTrait::Cultural), None, 2, 0, Expansion::Base),
            Planet::DalBootha => p!(Some(PlanetTrait::Cultural), None, 0, 2, Expansion::Base),
            Planet::Xxehan => p!(Some(PlanetTrait::Cultural), None, 1, 1, Expansion::Base),
            Planet::Lazar => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Cybernetic),
                1,
                0,
                Expansion::Base
            ),
            Planet::Sakulag => p!(Some(PlanetTrait::Hazardous), None, 2, 1, Expansion::Base),
            Planet::Lodor => p!(Some(PlanetTrait::Cultural), None, 3, 1, Expansion::Base),
            Planet::MeharXull => p!(
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Warfare),
                1,
                3,
                Expansion::Base
            ),
            Planet::Mellon => p!(Some(PlanetTrait::Cultural), None, 0, 2, Expansion::Base),
            Planet::Zohbat => p!(Some(PlanetTrait::Hazardous), None, 3, 1, Expansion::Base),
            Planet::NewAlbion => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1,
                Expansion::Base
            ),
            Planet::Starpoint => p!(Some(PlanetTrait::Hazardous), None, 3, 1, Expansion::Base),
            Planet::Quann => p!(Some(PlanetTrait::Cultural), None, 2, 1, Expansion::Base),
            Planet::Qucenn => p!(Some(PlanetTrait::Industrial), None, 1, 2, Expansion::Base),
            Planet::Rarron => p!(Some(PlanetTrait::Cultural), None, 0, 3, Expansion::Base),
            Planet::Saudor => p!(Some(PlanetTrait::Industrial), None, 2, 2, Expansion::Base),
            Planet::TarMann => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1,
                Expansion::Base
            ),
            Planet::TequRan => p!(Some(PlanetTrait::Industrial), None, 2, 0, Expansion::Base),
            Planet::Torkan => p!(Some(PlanetTrait::Cultural), None, 0, 3, Expansion::Base),
            Planet::Thibah => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Propulsion),
                1,
                1,
                Expansion::Base
            ),
            Planet::VefutII => p!(Some(PlanetTrait::Hazardous), None, 2, 2, Expansion::Base),
            Planet::Wellon => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Cybernetic),
                1,
                2,
                Expansion::Base
            ),
            Planet::ArchonVail => p!(
                Some(PlanetTrait::Hazardous),
                Some(TechCategory::Propulsion),
                1,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Perimiter => p!(
                Some(PlanetTrait::Industrial),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Ang => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Warfare),
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::SemLore => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Cybernetic),
                3,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Vorhal => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Biotic),
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Atlas => p!(
                Some(PlanetTrait::Hazardous),
                None,
                3,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Primor => p!(
                Some(PlanetTrait::Cultural),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::HopesEnd => p!(
                Some(PlanetTrait::Hazardous),
                None,
                3,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Cormund => p!(
                Some(PlanetTrait::Hazardous),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Everra => p!(
                Some(PlanetTrait::Cultural),
                None,
                3,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::JoelIr => p!(
                Some(PlanetTrait::Industrial),
                None,
                2,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Accoen => p!(
                Some(PlanetTrait::Industrial),
                None,
                2,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Kraag => p!(
                Some(PlanetTrait::Hazardous),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Siig => p!(
                Some(PlanetTrait::Hazardous),
                None,
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Bakal => p!(
                Some(PlanetTrait::Industrial),
                None,
                3,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::AlioPrima => p!(
                Some(PlanetTrait::Cultural),
                None,
                1,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Lisis => p!(
                Some(PlanetTrait::Industrial),
                None,
                2,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Velnor => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Warfare),
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Cealdri => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Cybernetic),
                0,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Xanhact => p!(
                Some(PlanetTrait::Hazardous),
                None,
                0,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::VegaMajor => p!(
                Some(PlanetTrait::Cultural),
                None,
                2,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::VegaMinor => p!(
                Some(PlanetTrait::Cultural),
                Some(TechCategory::Propulsion),
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::Abaddon => p!(
                Some(PlanetTrait::Cultural),
                None,
                1,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Ashtroth => p!(
                Some(PlanetTrait::Hazardous),
                None,
                2,
                0,
                Expansion::ProphecyOfKings
            ),
            Planet::Loki => p!(
                Some(PlanetTrait::Cultural),
                None,
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelI => p!(
                Some(PlanetTrait::Hazardous),
                None,
                0,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelII => p!(
                Some(PlanetTrait::Industrial),
                None,
                1,
                2,
                Expansion::ProphecyOfKings
            ),
            Planet::RigelIII => p!(
                Some(PlanetTrait::Industrial),
                Some(TechCategory::Biotic),
                1,
                1,
                Expansion::ProphecyOfKings
            ),
            Planet::Valk => p!(None, None, 2, 0, Expansion::ProphecyOfKings),
            Planet::Avar => p!(None, None, 1, 1, Expansion::ProphecyOfKings),
            Planet::Ylir => p!(None, None, 0, 2, Expansion::ProphecyOfKings),
            Planet::TheDark => p!(None, None, 3, 4, Expansion::ProphecyOfKings),
            Planet::Ixth => p!(None, None, 3, 5, Expansion::ProphecyOfKings),
            Planet::Naazir => p!(None, None, 2, 1, Expansion::ProphecyOfKings),
            Planet::Rokha => p!(None, None, 1, 2, Expansion::ProphecyOfKings),
            Planet::Arcturus => p!(None, None, 4, 4, Expansion::ProphecyOfKings),
            Planet::Elysium => p!(None, None, 4, 1, Expansion::ProphecyOfKings),
            Planet::Acheron => p!(None, None, 4, 0, Expansion::ProphecyOfKings),
            Planet::Mallice => p!(
                Some(PlanetTrait::Cultural),
                None,
                0,
                3,
                Expansion::ProphecyOfKings
            ),
            Planet::Mirage => p!(
                Some(PlanetTrait::Cultural),
                None,
                1,
                2,
                Expansion::ProphecyOfKings
            ),
        }
    }
}
