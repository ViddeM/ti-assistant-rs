use std::collections::HashMap;

use eyre::ensure;
use serde::{Deserialize, Serialize};

use crate::data::common::faction::Faction;

use super::planet::Planet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WormHoleType {
    Alpha,
    Beta,
    Gamma,
    Delta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemType {
    Anomaly(AnomalyType),
    Hyperlane,
    Normal,
    HomeSystem(Faction),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnomalyType {
    AsteroidField,
    Nebula,
    Supernova,
    MuaatSupernova,
    GravityRift,
}

pub type SystemId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub id: SystemId,
    pub system_type: SystemType,
    pub planets: Vec<Planet>,
    pub wormholes: Vec<WormHoleType>,
}

impl System {
    pub fn for_planet(planet: &Planet) -> Result<Self, eyre::Error> {
        let systems = systems()
            .values()
            .filter(|s| s.planets.contains(planet))
            .cloned()
            .collect::<Vec<System>>();

        ensure!(
            systems.len() != 1,
            "A planet should only be a part of one system"
        );

        Ok(systems[0].clone())
    }
}

macro_rules! s {
    ($id:literal, $system_type:expr) => {
        (
            String::from($id),
            System {
                id: String::from($id),
                system_type: $system_type,
                planets: vec![],
                wormholes: vec![],
            },
        )
    };
    ($id:literal, $system_type:expr, $planets:expr) => {
        (
            String::from($id),
            System {
                id: String::from($id),
                system_type: $system_type,
                planets: $planets,
                wormholes: vec![],
            },
        )
    };
    ($id:literal, $system_type:expr, $planets:expr, $wormholes:expr) => {
        (
            String::from($id),
            System {
                id: String::from($id),
                system_type: $system_type,
                planets: $planets,
                wormholes: $wormholes,
            },
        )
    };
}

pub fn systems() -> HashMap<String, System> {
    HashMap::from([
        s!(
            "1",
            SystemType::HomeSystem(Faction::FederationOfSol),
            vec![Planet::Jord]
        ),
        s!(
            "2",
            SystemType::HomeSystem(Faction::MentakCoalition),
            vec![Planet::MollPrimus]
        ),
        s!(
            "3",
            SystemType::HomeSystem(Faction::YinBrotherhood),
            vec![Planet::Darien]
        ),
        s!(
            "4",
            SystemType::HomeSystem(Faction::EmbersOfMuaat),
            vec![Planet::Muaat]
        ),
        s!(
            "5",
            SystemType::HomeSystem(Faction::Arborec),
            vec![Planet::Nestphar]
        ),
        s!(
            "6",
            SystemType::HomeSystem(Faction::L1Z1XMindnet),
            vec![Planet::ZeroZeroZero]
        ),
        s!(
            "7",
            SystemType::HomeSystem(Faction::Winnu),
            vec![Planet::Winnu]
        ),
        s!(
            "8",
            SystemType::HomeSystem(Faction::NekroVirus),
            vec![Planet::MordaiII]
        ),
        s!(
            "9",
            SystemType::HomeSystem(Faction::NaaluCollective),
            vec![Planet::Maaluuk, Planet::Druaa]
        ),
        s!(
            "10",
            SystemType::HomeSystem(Faction::BaronyOfLetnev),
            vec![Planet::ArcPrime, Planet::WrenTerra]
        ),
        s!(
            "11",
            SystemType::HomeSystem(Faction::ClanOfSaar),
            vec![Planet::LisisII, Planet::Ragh]
        ),
        s!(
            "12",
            SystemType::HomeSystem(Faction::UniversitiesOfJolNar),
            vec![Planet::Nar, Planet::Jol]
        ),
        s!(
            "13",
            SystemType::HomeSystem(Faction::SardakkNorr),
            vec![Planet::TrenLak, Planet::Quinarra]
        ),
        s!(
            "14",
            SystemType::HomeSystem(Faction::XxchaKingdom),
            vec![Planet::ArchonRen, Planet::ArchonTau]
        ),
        s!(
            "15",
            SystemType::HomeSystem(Faction::YssarilTribes),
            vec![Planet::Retillion, Planet::Shalloq]
        ),
        s!(
            "16",
            SystemType::HomeSystem(Faction::EmiratesOfHacan),
            vec![Planet::Arretze, Planet::Hercant, Planet::Kamdorn]
        ),
        s!(
            "17",
            SystemType::HomeSystem(Faction::GhostsOfCreuss),
            vec![],
            vec![WormHoleType::Delta]
        ),
        s!("18", SystemType::Normal, vec![Planet::MecatolRex]),
        s!("19", SystemType::Normal, vec![Planet::Wellon]),
        s!("20", SystemType::Normal, vec![Planet::VefutII]),
        s!("21", SystemType::Normal, vec![Planet::Thibah]),
        s!("22", SystemType::Normal, vec![Planet::TarMann]),
        s!("23", SystemType::Normal, vec![Planet::Saudor]),
        s!("24", SystemType::Normal, vec![Planet::MeharXull]),
        s!(
            "25",
            SystemType::Normal,
            vec![Planet::Quann],
            vec![WormHoleType::Beta]
        ),
        s!(
            "26",
            SystemType::Normal,
            vec![Planet::Lodor],
            vec![WormHoleType::Alpha]
        ),
        s!(
            "27",
            SystemType::Normal,
            vec![Planet::NewAlbion, Planet::Starpoint]
        ),
        s!(
            "28",
            SystemType::Normal,
            vec![Planet::TequRan, Planet::Torkan]
        ),
        s!(
            "29",
            SystemType::Normal,
            vec![Planet::Qucenn, Planet::Rarron]
        ),
        s!(
            "30",
            SystemType::Normal,
            vec![Planet::Mellon, Planet::Zohbat]
        ),
        s!(
            "31",
            SystemType::Normal,
            vec![Planet::Lazar, Planet::Sakulag]
        ),
        s!(
            "32",
            SystemType::Normal,
            vec![Planet::DalBootha, Planet::Xxehan]
        ),
        s!(
            "33",
            SystemType::Normal,
            vec![Planet::Coorneeq, Planet::Resculon]
        ),
        s!(
            "34",
            SystemType::Normal,
            vec![Planet::Centauri, Planet::Gral]
        ),
        s!(
            "35",
            SystemType::Normal,
            vec![Planet::Bereg, Planet::LirtaIV]
        ),
        s!("36", SystemType::Normal, vec![Planet::Arnor, Planet::Lor]),
        s!("37", SystemType::Normal, vec![Planet::Arinam, Planet::Meer]),
        s!("38", SystemType::Normal, vec![Planet::Abyz, Planet::Fria]),
        s!("39", SystemType::Normal, vec![], vec![WormHoleType::Alpha]),
        s!("40", SystemType::Normal, vec![], vec![WormHoleType::Beta]),
        s!("41", SystemType::Anomaly(AnomalyType::GravityRift)),
        s!("42", SystemType::Anomaly(AnomalyType::Nebula)),
        s!("43", SystemType::Anomaly(AnomalyType::Supernova)),
        s!("44", SystemType::Anomaly(AnomalyType::AsteroidField)),
        s!("45", SystemType::Anomaly(AnomalyType::AsteroidField)),
        s!("46", SystemType::Normal),
        s!("47", SystemType::Normal),
        s!("48", SystemType::Normal),
        s!("49", SystemType::Normal),
        s!("50", SystemType::Normal),
        s!(
            "51",
            SystemType::HomeSystem(Faction::GhostsOfCreuss),
            vec![Planet::Creuss],
            vec![WormHoleType::Delta]
        ),
        s!(
            "52",
            SystemType::HomeSystem(Faction::MahactGeneSorcerers),
            vec![Planet::Ixth]
        ),
        s!(
            "53",
            SystemType::HomeSystem(Faction::Nomad),
            vec![Planet::Arcturus]
        ),
        s!(
            "54",
            SystemType::HomeSystem(Faction::VuilRaithCabal),
            vec![Planet::Acheron]
        ),
        s!(
            "55",
            SystemType::HomeSystem(Faction::TitansOfUl),
            vec![Planet::Elysium]
        ),
        s!(
            "56",
            SystemType::HomeSystem(Faction::Empyrean),
            vec![Planet::TheDark]
        ),
        s!(
            "57",
            SystemType::HomeSystem(Faction::NaazRokhaAlliance),
            vec![Planet::Naazir, Planet::Rokha]
        ),
        s!(
            "58",
            SystemType::HomeSystem(Faction::ArgentFlight),
            vec![Planet::Valk, Planet::Avar, Planet::Ylir]
        ),
        s!("59", SystemType::Normal, vec![Planet::ArchonVail]),
        s!("60", SystemType::Normal, vec![Planet::Perimiter]),
        s!("61", SystemType::Normal, vec![Planet::Ang]),
        s!("62", SystemType::Normal, vec![Planet::SemLore]),
        s!("63", SystemType::Normal, vec![Planet::Vorhal]),
        s!(
            "64",
            SystemType::Normal,
            vec![Planet::Atlas],
            vec![WormHoleType::Beta]
        ),
        s!("65", SystemType::Normal, vec![Planet::Primor]),
        s!("66", SystemType::Normal, vec![Planet::HopesEnd]),
        s!(
            "67",
            SystemType::Anomaly(AnomalyType::GravityRift),
            vec![Planet::Cormund]
        ),
        s!(
            "68",
            SystemType::Anomaly(AnomalyType::Nebula),
            vec![Planet::Everra]
        ),
        s!(
            "69",
            SystemType::Normal,
            vec![Planet::Accoen, Planet::JoelIr]
        ),
        s!("70", SystemType::Normal, vec![Planet::Kraag, Planet::Siig]),
        s!(
            "71",
            SystemType::Normal,
            vec![Planet::Bakal, Planet::AlioPrima]
        ),
        s!(
            "72",
            SystemType::Normal,
            vec![Planet::Lisis, Planet::Velnor]
        ),
        s!(
            "73",
            SystemType::Normal,
            vec![Planet::Cealdri, Planet::Xanhact]
        ),
        s!(
            "74",
            SystemType::Normal,
            vec![Planet::VegaMajor, Planet::VegaMinor]
        ),
        s!(
            "75",
            SystemType::Normal,
            vec![Planet::Loki, Planet::Abaddon, Planet::Ashtroth]
        ),
        s!(
            "76",
            SystemType::Normal,
            vec![Planet::RigelI, Planet::RigelII, Planet::RigelIII]
        ),
        s!("77", SystemType::Normal),
        s!("78", SystemType::Normal),
        s!("79", SystemType::Anomaly(AnomalyType::AsteroidField)),
        s!("80", SystemType::Anomaly(AnomalyType::Supernova)),
        s!("81", SystemType::Anomaly(AnomalyType::MuaatSupernova)),
        s!(
            "82",
            SystemType::Normal,
            vec![Planet::Mallice],
            vec![WormHoleType::Alpha, WormHoleType::Beta, WormHoleType::Gamma,]
        ),
        s!("83A", SystemType::Hyperlane),
        s!("83B", SystemType::Hyperlane),
        s!("84A", SystemType::Hyperlane),
        s!("84B", SystemType::Hyperlane),
        s!("85A", SystemType::Hyperlane),
        s!("85B", SystemType::Hyperlane),
        s!("86A", SystemType::Hyperlane),
        s!("86B", SystemType::Hyperlane),
        s!("87A", SystemType::Hyperlane),
        s!("87B", SystemType::Hyperlane),
        s!("88A", SystemType::Hyperlane),
        s!("88B", SystemType::Hyperlane),
        s!("89A", SystemType::Hyperlane),
        s!("89B", SystemType::Hyperlane),
        s!("90A", SystemType::Hyperlane),
        s!("90B", SystemType::Hyperlane),
        s!("91A", SystemType::Hyperlane),
        s!("91B", SystemType::Hyperlane),
    ])
}
