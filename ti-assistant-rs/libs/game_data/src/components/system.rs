use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::{expansions::Expansion, faction::Faction},
    error::GameDataError,
};

use super::planet::Planet;

/// What type of wormhole this is.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(missing_docs)]
pub enum WormHoleType {
    Alpha,
    Beta,
    Gamma,
    Delta,
    Epsilon,
}

/// A type of system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(missing_docs)]
pub enum SystemType {
    Anomaly(Vec<AnomalyType>),
    Hyperlane,
    Normal,
    HomeSystem(Faction),
}

/// A type of anomaly.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(missing_docs)]
pub enum AnomalyType {
    AsteroidField,
    Nebula,
    Supernova,
    MuaatSupernova,
    GravityRift,
    EntropicScar,
}

/// The ID for a system.
pub type SystemId = String;

/// A system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct System {
    /// A unique id for the system.
    pub id: SystemId,
    /// What type of system this is.
    pub system_type: SystemType,
    /// What expansion this system belongs to.
    pub expansion: Expansion,
    /// Which planets exist in this system.
    pub planets: Vec<Planet>,
    /// What wormhole types exists in this system.
    pub wormholes: Vec<WormHoleType>,
}

impl System {
    /// Returns the system that the provide planet belongs to.
    pub fn for_planet(planet: &Planet) -> Result<Self, GameDataError> {
        let systems = systems()
            .values()
            .filter(|s| s.planets.contains(planet))
            .cloned()
            .collect::<Vec<System>>();

        if systems.len() != 1 {
            return Err(GameDataError::PlanetInMoreThanOneSystems {
                planet: planet.clone(),
                num_systems: systems.len(),
            });
        }

        Ok(systems[0].clone())
    }
}

macro_rules! s {
    ($id:literal, $system_type:expr, $expansion:expr) => {
        (
            String::from($id),
            System {
                id: String::from($id),
                system_type: $system_type,
                expansion: $expansion,
                planets: vec![],
                wormholes: vec![],
            },
        )
    };
    ($id:literal, $system_type:expr, $planets:expr, $expansion:expr) => {
        (
            String::from($id),
            System {
                id: String::from($id),
                system_type: $system_type,
                expansion: $expansion,
                planets: $planets,
                wormholes: vec![],
            },
        )
    };
    ($id:literal, $system_type:expr, $planets:expr, $wormholes:expr, $expansion:expr) => {
        (
            String::from($id),
            System {
                id: String::from($id),
                system_type: $system_type,
                expansion: $expansion,
                planets: $planets,
                wormholes: $wormholes,
            },
        )
    };
}

/// Returns a map of from System ID to System for all the systems in the game.
pub fn systems() -> HashMap<SystemId, System> {
    HashMap::from([
        s!(
            "1",
            SystemType::HomeSystem(Faction::FederationOfSol),
            vec![Planet::Jord],
            Expansion::Base
        ),
        s!(
            "2",
            SystemType::HomeSystem(Faction::MentakCoalition),
            vec![Planet::MollPrimus],
            Expansion::Base
        ),
        s!(
            "3",
            SystemType::HomeSystem(Faction::YinBrotherhood),
            vec![Planet::Darien],
            Expansion::Base
        ),
        s!(
            "4",
            SystemType::HomeSystem(Faction::EmbersOfMuaat),
            vec![Planet::Muaat],
            Expansion::Base
        ),
        s!(
            "5",
            SystemType::HomeSystem(Faction::Arborec),
            vec![Planet::Nestphar],
            Expansion::Base
        ),
        s!(
            "6",
            SystemType::HomeSystem(Faction::L1Z1XMindnet),
            vec![Planet::ZeroZeroZero],
            Expansion::Base
        ),
        s!(
            "7",
            SystemType::HomeSystem(Faction::Winnu),
            vec![Planet::Winnu],
            Expansion::Base
        ),
        s!(
            "8",
            SystemType::HomeSystem(Faction::NekroVirus),
            vec![Planet::MordaiII],
            Expansion::Base
        ),
        s!(
            "9",
            SystemType::HomeSystem(Faction::NaaluCollective),
            vec![Planet::Maaluuk, Planet::Druaa],
            Expansion::Base
        ),
        s!(
            "10",
            SystemType::HomeSystem(Faction::BaronyOfLetnev),
            vec![Planet::ArcPrime, Planet::WrenTerra],
            Expansion::Base
        ),
        s!(
            "11",
            SystemType::HomeSystem(Faction::ClanOfSaar),
            vec![Planet::LisisII, Planet::Ragh],
            Expansion::Base
        ),
        s!(
            "12",
            SystemType::HomeSystem(Faction::UniversitiesOfJolNar),
            vec![Planet::Nar, Planet::Jol],
            Expansion::Base
        ),
        s!(
            "13",
            SystemType::HomeSystem(Faction::SardakkNorr),
            vec![Planet::TrenLak, Planet::Quinarra],
            Expansion::Base
        ),
        s!(
            "14",
            SystemType::HomeSystem(Faction::XxchaKingdom),
            vec![Planet::ArchonRen, Planet::ArchonTau],
            Expansion::Base
        ),
        s!(
            "15",
            SystemType::HomeSystem(Faction::YssarilTribes),
            vec![Planet::Retillion, Planet::Shalloq],
            Expansion::Base
        ),
        s!(
            "16",
            SystemType::HomeSystem(Faction::EmiratesOfHacan),
            vec![Planet::Arretze, Planet::Hercant, Planet::Kamdorn],
            Expansion::Base
        ),
        s!(
            "17",
            SystemType::HomeSystem(Faction::GhostsOfCreuss),
            vec![],
            vec![WormHoleType::Delta],
            Expansion::Base
        ),
        s!(
            "18",
            SystemType::Normal,
            vec![Planet::MecatolRex],
            Expansion::Base
        ),
        s!(
            "19",
            SystemType::Normal,
            vec![Planet::Wellon],
            Expansion::Base
        ),
        s!(
            "20",
            SystemType::Normal,
            vec![Planet::VefutII],
            Expansion::Base
        ),
        s!(
            "21",
            SystemType::Normal,
            vec![Planet::Thibah],
            Expansion::Base
        ),
        s!(
            "22",
            SystemType::Normal,
            vec![Planet::TarMann],
            Expansion::Base
        ),
        s!(
            "23",
            SystemType::Normal,
            vec![Planet::Saudor],
            Expansion::Base
        ),
        s!(
            "24",
            SystemType::Normal,
            vec![Planet::MeharXull],
            Expansion::Base
        ),
        s!(
            "25",
            SystemType::Normal,
            vec![Planet::Quann],
            vec![WormHoleType::Beta],
            Expansion::Base
        ),
        s!(
            "26",
            SystemType::Normal,
            vec![Planet::Lodor],
            vec![WormHoleType::Alpha],
            Expansion::Base
        ),
        s!(
            "27",
            SystemType::Normal,
            vec![Planet::NewAlbion, Planet::Starpoint],
            Expansion::Base
        ),
        s!(
            "28",
            SystemType::Normal,
            vec![Planet::TequRan, Planet::Torkan],
            Expansion::Base
        ),
        s!(
            "29",
            SystemType::Normal,
            vec![Planet::Qucenn, Planet::Rarron],
            Expansion::Base
        ),
        s!(
            "30",
            SystemType::Normal,
            vec![Planet::Mellon, Planet::Zohbat],
            Expansion::Base
        ),
        s!(
            "31",
            SystemType::Normal,
            vec![Planet::Lazar, Planet::Sakulag],
            Expansion::Base
        ),
        s!(
            "32",
            SystemType::Normal,
            vec![Planet::DalBootha, Planet::Xxehan],
            Expansion::Base
        ),
        s!(
            "33",
            SystemType::Normal,
            vec![Planet::Corneeq, Planet::Resculon],
            Expansion::Base
        ),
        s!(
            "34",
            SystemType::Normal,
            vec![Planet::Centauri, Planet::Gral],
            Expansion::Base
        ),
        s!(
            "35",
            SystemType::Normal,
            vec![Planet::Bereg, Planet::LirtaIV],
            Expansion::Base
        ),
        s!(
            "36",
            SystemType::Normal,
            vec![Planet::Arnor, Planet::Lor],
            Expansion::Base
        ),
        s!(
            "37",
            SystemType::Normal,
            vec![Planet::Arinam, Planet::Meer],
            Expansion::Base
        ),
        s!(
            "38",
            SystemType::Normal,
            vec![Planet::Abyz, Planet::Fria],
            Expansion::Base
        ),
        s!(
            "39",
            SystemType::Normal,
            vec![],
            vec![WormHoleType::Alpha],
            Expansion::Base
        ),
        s!(
            "40",
            SystemType::Normal,
            vec![],
            vec![WormHoleType::Beta],
            Expansion::Base
        ),
        s!(
            "41",
            SystemType::Anomaly(vec![AnomalyType::GravityRift]),
            Expansion::Base
        ),
        s!(
            "42",
            SystemType::Anomaly(vec![AnomalyType::Nebula]),
            Expansion::Base
        ),
        s!(
            "43",
            SystemType::Anomaly(vec![AnomalyType::Supernova]),
            Expansion::Base
        ),
        s!(
            "44",
            SystemType::Anomaly(vec![AnomalyType::AsteroidField]),
            Expansion::Base
        ),
        s!(
            "45",
            SystemType::Anomaly(vec![AnomalyType::AsteroidField]),
            Expansion::Base
        ),
        s!("46", SystemType::Normal, Expansion::Base),
        s!("47", SystemType::Normal, Expansion::Base),
        s!("48", SystemType::Normal, Expansion::Base),
        s!("49", SystemType::Normal, Expansion::Base),
        s!("50", SystemType::Normal, Expansion::Base),
        s!(
            "51",
            SystemType::HomeSystem(Faction::GhostsOfCreuss),
            vec![Planet::Creuss],
            vec![WormHoleType::Delta],
            Expansion::Base
        ),
        s!(
            "52",
            SystemType::HomeSystem(Faction::MahactGeneSorcerers),
            vec![Planet::Ixth],
            Expansion::ProphecyOfKings
        ),
        s!(
            "53",
            SystemType::HomeSystem(Faction::Nomad),
            vec![Planet::Arcturus],
            Expansion::ProphecyOfKings
        ),
        s!(
            "54",
            SystemType::HomeSystem(Faction::VuilRaithCabal),
            vec![Planet::Acheron],
            Expansion::ProphecyOfKings
        ),
        s!(
            "55",
            SystemType::HomeSystem(Faction::TitansOfUl),
            vec![Planet::Elysium],
            Expansion::ProphecyOfKings
        ),
        s!(
            "56",
            SystemType::HomeSystem(Faction::Empyrean),
            vec![Planet::TheDark],
            Expansion::ProphecyOfKings
        ),
        s!(
            "57",
            SystemType::HomeSystem(Faction::NaazRokhaAlliance),
            vec![Planet::Naazir, Planet::Rokha],
            Expansion::ProphecyOfKings
        ),
        s!(
            "58",
            SystemType::HomeSystem(Faction::ArgentFlight),
            vec![Planet::Valk, Planet::Avar, Planet::Ylir],
            Expansion::ProphecyOfKings
        ),
        s!(
            "59",
            SystemType::Normal,
            vec![Planet::ArchonVail],
            Expansion::ProphecyOfKings
        ),
        s!(
            "60",
            SystemType::Normal,
            vec![Planet::Perimiter],
            Expansion::ProphecyOfKings
        ),
        s!(
            "61",
            SystemType::Normal,
            vec![Planet::Ang],
            Expansion::ProphecyOfKings
        ),
        s!(
            "62",
            SystemType::Normal,
            vec![Planet::SemLore],
            Expansion::ProphecyOfKings
        ),
        s!(
            "63",
            SystemType::Normal,
            vec![Planet::Vorhal],
            Expansion::ProphecyOfKings
        ),
        s!(
            "64",
            SystemType::Normal,
            vec![Planet::Atlas],
            vec![WormHoleType::Beta],
            Expansion::ProphecyOfKings
        ),
        s!(
            "65",
            SystemType::Normal,
            vec![Planet::Primor],
            Expansion::ProphecyOfKings
        ),
        s!(
            "66",
            SystemType::Normal,
            vec![Planet::HopesEnd],
            Expansion::ProphecyOfKings
        ),
        s!(
            "67",
            SystemType::Anomaly(vec![AnomalyType::GravityRift]),
            vec![Planet::Cormund],
            Expansion::ProphecyOfKings
        ),
        s!(
            "68",
            SystemType::Anomaly(vec![AnomalyType::Nebula]),
            vec![Planet::Everra],
            Expansion::ProphecyOfKings
        ),
        s!(
            "69",
            SystemType::Normal,
            vec![Planet::Accoen, Planet::JeolIr],
            Expansion::ProphecyOfKings
        ),
        s!(
            "70",
            SystemType::Normal,
            vec![Planet::Kraag, Planet::Siig],
            Expansion::ProphecyOfKings
        ),
        s!(
            "71",
            SystemType::Normal,
            vec![Planet::Bakal, Planet::AlioPrima],
            Expansion::ProphecyOfKings
        ),
        s!(
            "72",
            SystemType::Normal,
            vec![Planet::Lisis, Planet::Velnor],
            Expansion::ProphecyOfKings
        ),
        s!(
            "73",
            SystemType::Normal,
            vec![Planet::Cealdri, Planet::Xanhact],
            Expansion::ProphecyOfKings
        ),
        s!(
            "74",
            SystemType::Normal,
            vec![Planet::VegaMajor, Planet::VegaMinor],
            Expansion::ProphecyOfKings
        ),
        s!(
            "75",
            SystemType::Normal,
            vec![Planet::Loki, Planet::Abaddon, Planet::Ashtroth],
            Expansion::ProphecyOfKings
        ),
        s!(
            "76",
            SystemType::Normal,
            vec![Planet::RigelI, Planet::RigelII, Planet::RigelIII],
            Expansion::ProphecyOfKings
        ),
        s!("77", SystemType::Normal, Expansion::ProphecyOfKings),
        s!("78", SystemType::Normal, Expansion::ProphecyOfKings),
        s!(
            "79",
            SystemType::Anomaly(vec![AnomalyType::AsteroidField]),
            vec![],
            vec![WormHoleType::Alpha],
            Expansion::ProphecyOfKings
        ),
        s!(
            "80",
            SystemType::Anomaly(vec![AnomalyType::Supernova]),
            Expansion::ProphecyOfKings
        ),
        s!(
            "81",
            SystemType::Anomaly(vec![AnomalyType::MuaatSupernova]),
            Expansion::ProphecyOfKings
        ),
        s!(
            "82",
            SystemType::Normal,
            vec![Planet::Mallice],
            vec![WormHoleType::Alpha, WormHoleType::Beta, WormHoleType::Gamma,],
            Expansion::ProphecyOfKings
        ),
        s!("83A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("83B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("84A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("84B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("85A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("85B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("86A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("86B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("87A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("87B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("88A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("88B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("89A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("89B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("90A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("90B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("91A", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!("91B", SystemType::Hyperlane, Expansion::ProphecyOfKings),
        s!(
            "92",
            SystemType::HomeSystem(Faction::LastBastion),
            vec![Planet::Ordinian, Planet::Revelation],
            vec![],
            Expansion::ThundersEdge
        ),
        s!(
            "93",
            SystemType::HomeSystem(Faction::RalNelConsortium),
            vec![Planet::MezLoOrzFeiZsha, Planet::RepoLoOrzQet],
            vec![],
            Expansion::ThundersEdge
        ),
        s!(
            "94",
            SystemType::HomeSystem(Faction::CrimsonRebellion),
            vec![],
            vec![WormHoleType::Epsilon],
            Expansion::ThundersEdge
        ),
        s!(
            "95",
            SystemType::HomeSystem(Faction::DeepwroughtScholarate),
            vec![Planet::Ikatena],
            vec![],
            Expansion::ThundersEdge
        ),
        s!(
            "96a",
            SystemType::HomeSystem(Faction::FirmamentObsidian),
            vec![Planet::Cronos, Planet::Tallin],
            vec![],
            Expansion::ThundersEdge
        ),
        s!(
            "96b",
            SystemType::HomeSystem(Faction::FirmamentObsidian),
            vec![Planet::CronosHollow, Planet::TallinHollow],
            vec![],
            Expansion::ThundersEdge
        ),
        s!(
            "97",
            SystemType::Normal,
            vec![Planet::Faunus],
            Expansion::ThundersEdge
        ),
        s!(
            "98",
            SystemType::Normal,
            vec![Planet::Garbozia],
            Expansion::ThundersEdge
        ),
        s!(
            "99",
            SystemType::Normal,
            vec![Planet::Emelpar],
            Expansion::ThundersEdge
        ),
        s!(
            "100",
            SystemType::Normal,
            vec![Planet::Tempesta],
            Expansion::ThundersEdge
        ),
        s!(
            "101",
            SystemType::Normal,
            vec![Planet::Olergodt],
            Expansion::ThundersEdge
        ),
        s!(
            "102",
            SystemType::Normal,
            vec![Planet::Andeara],
            vec![WormHoleType::Alpha],
            Expansion::ThundersEdge
        ),
        s!(
            "103",
            SystemType::Normal,
            vec![Planet::ViraPicsIII],
            Expansion::ThundersEdge
        ),
        s!(
            "104",
            SystemType::Normal,
            vec![Planet::Lesab],
            Expansion::ThundersEdge
        ),
        s!(
            "105",
            SystemType::Normal,
            vec![Planet::NewTerra, Planet::Tinnes],
            Expansion::ThundersEdge
        ),
        s!(
            "106",
            SystemType::Normal,
            vec![Planet::Cresius, Planet::LazulRex],
            Expansion::ThundersEdge
        ),
        s!(
            "107",
            SystemType::Normal,
            vec![Planet::Hercalor, Planet::Tiamat],
            Expansion::ThundersEdge
        ),
        s!(
            "108",
            SystemType::Normal,
            vec![Planet::Capha, Planet::Kostboth],
            Expansion::ThundersEdge
        ),
        s!(
            "109",
            SystemType::Normal,
            vec![Planet::Bellatrix, Planet::TsionStation],
            Expansion::ThundersEdge
        ),
        s!(
            "110",
            SystemType::Normal,
            vec![Planet::Elnath, Planet::LuthienVi, Planet::Horizon],
            Expansion::ThundersEdge
        ),
        s!(
            "111",
            SystemType::Normal,
            vec![Planet::Tarana, Planet::OluzStation],
            Expansion::ThundersEdge
        ),
        s!(
            "112",
            SystemType::Normal,
            vec![Planet::MecatolRexOmega],
            Expansion::ThundersEdge
        ),
        s!(
            "113",
            SystemType::Anomaly(vec![AnomalyType::GravityRift]),
            vec![],
            vec![WormHoleType::Beta],
            Expansion::ThundersEdge
        ),
        s!(
            "114",
            SystemType::Anomaly(vec![AnomalyType::EntropicScar]),
            vec![],
            Expansion::ThundersEdge
        ),
        s!(
            "115",
            SystemType::Anomaly(vec![AnomalyType::AsteroidField]),
            vec![Planet::Industrex],
            Expansion::ThundersEdge
        ),
        s!(
            "116",
            SystemType::Anomaly(vec![AnomalyType::EntropicScar]),
            vec![Planet::Lemox],
            Expansion::ThundersEdge
        ),
        s!(
            "117",
            SystemType::Anomaly(vec![AnomalyType::GravityRift, AnomalyType::AsteroidField]),
            vec![Planet::TheWatchtower],
            Expansion::ThundersEdge
        ),
        s!(
            "118",
            SystemType::Normal,
            vec![Planet::AhkCreuxx],
            vec![WormHoleType::Epsilon],
            Expansion::ThundersEdge
        ),
    ])
}
