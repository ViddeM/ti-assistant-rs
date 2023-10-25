use serde::{Deserialize, Serialize};

use crate::data::components::{
    planet::Planet,
    system::{systems, SystemType},
    tech::Technology,
};

use super::expansions::Expansions;

pub const ALL_FACTIONS: [Faction; 25] = [
    Faction::Arborec,
    Faction::BaronyOfLetnev,
    Faction::ClanOfSaar,
    Faction::EmbersOfMuaat,
    Faction::EmiratesOfHacan,
    Faction::FederationOfSol,
    Faction::GhostsOfCreuss,
    Faction::L1Z1XMindnet,
    Faction::MentakCoalition,
    Faction::NaaluCollective,
    Faction::NekroVirus,
    Faction::SardakkNorr,
    Faction::UniversitiesOfJolNar,
    Faction::Winnu,
    Faction::XxchaKingdom,
    Faction::YinBrotherhood,
    Faction::YssarilTribes,
    Faction::ArgentFlight,
    Faction::Empyrean,
    Faction::MahactGeneSorcerers,
    Faction::NaazRokhaAlliance,
    Faction::Nomad,
    Faction::TitansOfUl,
    Faction::VuilRaithCabal,
    Faction::CouncilKeleres,
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    // PoK expansions
    ArgentFlight,
    Empyrean,
    MahactGeneSorcerers,
    NaazRokhaAlliance,
    Nomad,
    TitansOfUl,
    VuilRaithCabal,
    // Codex
    CouncilKeleres,
}

impl Faction {
    pub fn expansion(&self) -> Expansions {
        match self {
            Faction::Arborec => Expansions::Base,
            Faction::BaronyOfLetnev => Expansions::Base,
            Faction::ClanOfSaar => Expansions::Base,
            Faction::EmbersOfMuaat => Expansions::Base,
            Faction::EmiratesOfHacan => Expansions::Base,
            Faction::FederationOfSol => Expansions::Base,
            Faction::GhostsOfCreuss => Expansions::Base,
            Faction::L1Z1XMindnet => Expansions::Base,
            Faction::MentakCoalition => Expansions::Base,
            Faction::NaaluCollective => Expansions::Base,
            Faction::NekroVirus => Expansions::Base,
            Faction::SardakkNorr => Expansions::Base,
            Faction::UniversitiesOfJolNar => Expansions::Base,
            Faction::Winnu => Expansions::Base,
            Faction::XxchaKingdom => Expansions::Base,
            Faction::YinBrotherhood => Expansions::Base,
            Faction::YssarilTribes => Expansions::Base,
            Faction::ArgentFlight => Expansions::ProphecyOfKings,
            Faction::Empyrean => Expansions::ProphecyOfKings,
            Faction::MahactGeneSorcerers => Expansions::ProphecyOfKings,
            Faction::NaazRokhaAlliance => Expansions::ProphecyOfKings,
            Faction::Nomad => Expansions::ProphecyOfKings,
            Faction::TitansOfUl => Expansions::ProphecyOfKings,
            Faction::VuilRaithCabal => Expansions::ProphecyOfKings,
            Faction::CouncilKeleres => Expansions::Codex,
        }
    }

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
        })
    }

    pub fn get_starting_planets(&self) -> Vec<Planet> {
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

    pub fn get_starting_techs(&self) -> Vec<Technology> {
        match self {
            Faction::Arborec => vec![Technology::MagenDefenceGrid],
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
            Faction::Winnu => vec![], // Choose any 1 technology that has no prereqs
            Faction::XxchaKingdom => vec![Technology::GravitonLaserSystem],
            Faction::YinBrotherhood => vec![Technology::SarweenTools],
            Faction::YssarilTribes => vec![Technology::NeuralMotivator],
            Faction::ArgentFlight => vec![], // Choose TWO of the following: Neural Motivator, Sarween Tools, Plasma Scoring
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
            Faction::CouncilKeleres => vec![], // Choose 2 non-faction technologies owned by other players.
        }
    }
}
