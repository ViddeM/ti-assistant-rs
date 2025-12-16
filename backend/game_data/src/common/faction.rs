use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::components::{
    planet::Planet,
    system::{systems, SystemType},
    tech::Technology,
};

use super::{expansions::Expansion, game_settings::Expansions};

/// A playable faction in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, TS)]
#[ts(export)]
#[allow(missing_docs)]
pub enum Faction {
    // Vanilla
    Arborec,
    BaronyOfLetnev,
    ClanOfSaar,
    EmbersOfMuaat,
    EmiratesOfHacan,
    FederationOfSol,
    GhostsOfCreuss,
    L1Z1XMindnet,
    MentakCoalition,
    NaaluCollective,
    NekroVirus,
    SardakkNorr,
    UniversitiesOfJolNar,
    Winnu,
    XxchaKingdom,
    YinBrotherhood,
    YssarilTribes,
    // PoK expansion
    ArgentFlight,
    Empyrean,
    MahactGeneSorcerers,
    NaazRokhaAlliance,
    Nomad,
    TitansOfUl,
    VuilRaithCabal,
    // Codex
    CouncilKeleres,
    // Thunder's Edge expansion
    LastBastion,
    RalNelConsortium,
    DeepwroughtScolorate,
    CrimsonRebellion,
    FirmamentObsidian,
}

impl Faction {
    /// Returns which expansion the faction belongs to.
    pub fn expansion(&self) -> Expansion {
        match self {
            Faction::Arborec => Expansion::Base,
            Faction::BaronyOfLetnev => Expansion::Base,
            Faction::ClanOfSaar => Expansion::Base,
            Faction::EmbersOfMuaat => Expansion::Base,
            Faction::EmiratesOfHacan => Expansion::Base,
            Faction::FederationOfSol => Expansion::Base,
            Faction::GhostsOfCreuss => Expansion::Base,
            Faction::L1Z1XMindnet => Expansion::Base,
            Faction::MentakCoalition => Expansion::Base,
            Faction::NaaluCollective => Expansion::Base,
            Faction::NekroVirus => Expansion::Base,
            Faction::SardakkNorr => Expansion::Base,
            Faction::UniversitiesOfJolNar => Expansion::Base,
            Faction::Winnu => Expansion::Base,
            Faction::XxchaKingdom => Expansion::Base,
            Faction::YinBrotherhood => Expansion::Base,
            Faction::YssarilTribes => Expansion::Base,
            Faction::ArgentFlight => Expansion::ProphecyOfKings,
            Faction::Empyrean => Expansion::ProphecyOfKings,
            Faction::MahactGeneSorcerers => Expansion::ProphecyOfKings,
            Faction::NaazRokhaAlliance => Expansion::ProphecyOfKings,
            Faction::Nomad => Expansion::ProphecyOfKings,
            Faction::TitansOfUl => Expansion::ProphecyOfKings,
            Faction::VuilRaithCabal => Expansion::ProphecyOfKings,
            Faction::CouncilKeleres => Expansion::CodexIII,
            Faction::LastBastion => Expansion::ThundersEdge,
            Faction::RalNelConsortium => Expansion::ThundersEdge,
            Faction::DeepwroughtScolorate => Expansion::ThundersEdge,
            Faction::CrimsonRebellion => Expansion::ThundersEdge,
            Faction::FirmamentObsidian => Expansion::ThundersEdge,
        }
    }

    /// Returns the name of the faction in 'pretty' format.
    pub fn name(&self) -> String {
        String::from(match self {
            Faction::Arborec => "The Arborec",
            Faction::BaronyOfLetnev => "The Barony of Letnev",
            Faction::ClanOfSaar => "The Clan of Saar",
            Faction::EmbersOfMuaat => "The Embers of Muaat",
            Faction::EmiratesOfHacan => "The Emirates of Hacan",
            Faction::FederationOfSol => "The Federation of Sol",
            Faction::GhostsOfCreuss => "The Ghosts of Creuss",
            Faction::L1Z1XMindnet => "The L1Z1X Mindnet",
            Faction::MentakCoalition => "The Mentak Coalition",
            Faction::NaaluCollective => "The Naalu Collective",
            Faction::NekroVirus => "The Nekro Virus",
            Faction::SardakkNorr => "Sardakk N'orr",
            Faction::UniversitiesOfJolNar => "The Universities of Jol-Nar",
            Faction::Winnu => "The Winnu",
            Faction::XxchaKingdom => "The Xxcha Kingdom",
            Faction::YinBrotherhood => "The Yin Brotherhood",
            Faction::YssarilTribes => "The Yssaril Tribes",
            Faction::ArgentFlight => "The Argent Flight",
            Faction::Empyrean => "The Empyrean",
            Faction::MahactGeneSorcerers => "The Mahact Gene-Sorcerers",
            Faction::NaazRokhaAlliance => "The Naaz-Rokha Alliance",
            Faction::Nomad => "The Nomad",
            Faction::TitansOfUl => "The Titans of Ul",
            Faction::VuilRaithCabal => "The Vuil'Raith Cabal",
            Faction::CouncilKeleres => "The Council Keleres",
            Faction::LastBastion => "Last Bastion",
            Faction::RalNelConsortium => "The Ral Nel Consortium",
            Faction::DeepwroughtScolorate => "The Deepwrought Scolorate",
            Faction::CrimsonRebellion => "The Crimson Rebellion",
            Faction::FirmamentObsidian => "The Firmament / The Obsidian",
        })
    }

    /// Returns a set of the planets the faction starts with.
    pub fn get_starting_planets(&self) -> HashSet<Planet> {
        // TODO: Handle Council Keleres (they get to chose one from the Mentak/XXcha/Argent Flights starting systems)
        systems()
            .values()
            .filter(|s| match &s.system_type {
                SystemType::HomeSystem(f) => f.eq(self),
                _ => false,
            })
            .flat_map(|s| &s.planets)
            .cloned()
            .collect()
    }

    /// Returns a set of the technologies the faction starts with.
    pub fn get_starting_techs(&self, expansions: &Expansions) -> HashSet<Technology> {
        match self {
            Faction::Arborec => vec![
                Technology::MagenDefenceGrid,
                Technology::MagenDefenceGridOmega,
            ],
            Faction::BaronyOfLetnev => {
                vec![Technology::AntimassDeflectors, Technology::PlasmaScoring]
            }
            Faction::ClanOfSaar => vec![Technology::AntimassDeflectors],
            Faction::EmbersOfMuaat => vec![Technology::PlasmaScoring],
            Faction::EmiratesOfHacan => {
                vec![Technology::AntimassDeflectors, Technology::SarweenTools]
            }
            Faction::FederationOfSol => {
                vec![Technology::NeuralMotivator, Technology::AntimassDeflectors]
            }
            Faction::GhostsOfCreuss => vec![Technology::GravityDrive],
            Faction::L1Z1XMindnet => vec![Technology::NeuralMotivator, Technology::PlasmaScoring],
            Faction::MentakCoalition => vec![Technology::SarweenTools, Technology::PlasmaScoring],
            Faction::NaaluCollective => vec![Technology::NeuralMotivator, Technology::SarweenTools],
            Faction::NekroVirus => vec![Technology::DacxiveAnimators],
            Faction::SardakkNorr => vec![],
            Faction::UniversitiesOfJolNar => vec![
                Technology::NeuralMotivator,
                Technology::AntimassDeflectors,
                Technology::SarweenTools,
                Technology::PlasmaScoring,
            ],
            Faction::Winnu => vec![],
            Faction::XxchaKingdom => vec![Technology::GravitonLaserSystem],
            Faction::YinBrotherhood => vec![Technology::SarweenTools],
            Faction::YssarilTribes => vec![Technology::NeuralMotivator],
            Faction::ArgentFlight => vec![],
            Faction::Empyrean => vec![Technology::DarkEnergyTap],
            Faction::MahactGeneSorcerers => {
                vec![Technology::BioStims, Technology::PredictiveIntelligence]
            }
            Faction::NaazRokhaAlliance => vec![
                Technology::Psychoarchaeology,
                Technology::AiDevelopmentAlgorithm,
            ],
            Faction::Nomad => vec![Technology::SlingRelay],
            Faction::TitansOfUl => vec![
                Technology::AntimassDeflectors,
                Technology::ScanlinkDroneNetwork,
            ],
            Faction::VuilRaithCabal => vec![Technology::SelfAssemblyRoutines],
            Faction::CouncilKeleres => vec![/* Get's to pick starting techs */],
            Faction::LastBastion => vec![/* Get's to pick starting techs */],
        }
        .into_iter()
        .filter(|tech| tech.is_enabled_in(expansions))
        .collect()
    }

    /// Tries to parse the name of a faction into a faction (note: Must match w/e naming scheme milty draft is using!)
    pub fn parse(name: &str) -> eyre::Result<Self> {
        Ok(match name {
            "Sardakk N'orr" => Faction::SardakkNorr,
            "The Arborec" => Faction::Arborec,
            "The Barony of Letnev" => Faction::BaronyOfLetnev,
            "The Clan of Saar" => Faction::ClanOfSaar,
            "The Embers of Muaat" => Faction::EmbersOfMuaat,
            "The Emirates of Hacan" => Faction::EmiratesOfHacan,
            "The Federation of Sol" => Faction::FederationOfSol,
            "The Ghosts of Creuss" => Faction::GhostsOfCreuss,
            "The L1z1x Mindnet" => Faction::L1Z1XMindnet,
            "The Mentak Coalition" => Faction::MentakCoalition,
            "The Naalu Collective" => Faction::NaaluCollective,
            "The Nekro Virus" => Faction::NekroVirus,
            "The Universities of Jol-Nar" => Faction::UniversitiesOfJolNar,
            "The Winnu" => Faction::Winnu,
            "The Xxcha Kingdom" => Faction::XxchaKingdom,
            "The Yin Brotherhood" => Faction::YinBrotherhood,
            "The Yssaril Tribes" => Faction::YssarilTribes,
            "The Argent Flight" => Faction::ArgentFlight,
            "The Empyrean" => Faction::Empyrean,
            "The Mahact Gene-sorcerers" => Faction::MahactGeneSorcerers,
            "The Naaz-Rokha Alliance" => Faction::NaazRokhaAlliance,
            "The Nomad" => Faction::Nomad,
            "The Titans of Ul" => Faction::TitansOfUl,
            "The Vuil'raith Cabal" => Faction::VuilRaithCabal,
            "The Council Keleres" => Faction::CouncilKeleres,
            "Last Bastion" => Faction::LastBastion,
            "Augurs of Ilyxum"
            | "Celdauri Trade Confederation"
            | "Dih-Mohn Flotilla"
            | "Florzen Profiteers"
            | "Free Systems Compact"
            | "Ghemina Raiders"
            | "Glimmer of Mortheus"
            | "Kollecc Society"
            | "Kortali Tribunal"
            | "Li-Zho Dynasty"
            | "L'Tokk Khrask"
            | "Mirveda Protectorate"
            | "Myko-Mentori"
            | "Nivyn Star Kings"
            | "Olradin League"
            | "Roh'Dhna Mechatronics"
            | "Savages of Cymiae"
            | "Shipwrights of Axis"
            | "Tnelis Syndicate"
            | "Vaden Banking Clans"
            | "Vaylerian Scourge"
            | "Veldyr Sovereignty"
            | "Zealots of Rhodun"
            | "Zelian Purifier"
            | "Bentor Conglomerate"
            | "Berserkers of Kjalengard"
            | "Cheiran Hordes"
            | "Edyn Mandate"
            | "Ghoti Wayfarers"
            | "Gledge Union"
            | "Kyro Sodality"
            | "Lanefir Remnants"
            | "The Monks of Kolume"
            | "Nokar Sellships" => {
                eyre::bail!("Discordant stars factions are currently not supported!")
            }
            other => eyre::bail!("Unknown faction {other}"),
        })
    }
}
