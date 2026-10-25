use std::collections::{BTreeSet, HashSet};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::{
    components::{
        breakthrough::Breakthrough,
        planet::Planet,
        system::{SystemType, systems},
        tech::{TechCategory, TechOrigin, TechType, Technology},
    },
    state::game_state::GameState,
};

use super::{expansions::Expansion, game_settings::Expansions};

// TODO: Avoid having duplicate strings for parsing / display.

/// A playable faction in the game.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, EnumString, Display,
)]
#[strum(serialize_all = "snake_case")]
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
    DeepwroughtScholarate,
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
            Faction::DeepwroughtScholarate => Expansion::ThundersEdge,
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
            Faction::DeepwroughtScholarate => "The Deepwrought Scholarate",
            Faction::CrimsonRebellion => "The Crimson Rebellion",
            Faction::FirmamentObsidian => "The Firmament / The Obsidian",
        })
    }

    /// Get the breakthrough associated with the faction.
    // TODO: How do we handle factions with flipped breakthroughs / obsdian firmament?
    pub fn breakthrough(&self) -> Breakthrough {
        match self {
            Faction::Arborec => Breakthrough::Psychospore,
            Faction::BaronyOfLetnev => Breakthrough::GravleashManeuvers,
            Faction::ClanOfSaar => Breakthrough::DeorbitBarrage,
            Faction::EmbersOfMuaat => Breakthrough::StellarGenesis,
            Faction::EmiratesOfHacan => Breakthrough::AutoFactories,
            Faction::FederationOfSol => Breakthrough::BellumGloriosum,
            Faction::GhostsOfCreuss => Breakthrough::ParticleSynthesis,
            Faction::L1Z1XMindnet => Breakthrough::FealtyUplink,
            Faction::MentakCoalition => Breakthrough::TheTablesGrace,
            Faction::NaaluCollective => Breakthrough::Mindsieve,
            Faction::NekroVirus => Breakthrough::ValefarAssimilatorZ,
            Faction::SardakkNorr => Breakthrough::NorrSupremacy,
            Faction::UniversitiesOfJolNar => Breakthrough::SpecializedCompounds,
            Faction::Winnu => Breakthrough::Imperator,
            Faction::XxchaKingdom => Breakthrough::ArchonsGift,
            Faction::YinBrotherhood => Breakthrough::YinAscendant,
            Faction::YssarilTribes => Breakthrough::DeepgloomExecutable,
            Faction::ArgentFlight => Breakthrough::WingTransfer,
            Faction::Empyrean => Breakthrough::VoidTether,
            Faction::MahactGeneSorcerers => Breakthrough::VaultsOfTheHeir,
            Faction::NaazRokhaAlliance => Breakthrough::AbsoluteSynergy,
            Faction::Nomad => Breakthrough::ThundersParadox,
            Faction::TitansOfUl => Breakthrough::SlumberstateComputing,
            Faction::VuilRaithCabal => Breakthrough::AlRaithIxIanovar,
            Faction::CouncilKeleres => Breakthrough::IIHQModernization,
            Faction::LastBastion => Breakthrough::TheIcon,
            Faction::RalNelConsortium => Breakthrough::DataSkimmer,
            Faction::DeepwroughtScholarate => Breakthrough::VisionariaSelect,
            Faction::CrimsonRebellion => Breakthrough::ResonanceGenerator,
            Faction::FirmamentObsidian => Breakthrough::TheSowing,
        }
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
            // The Firmament / The Obsidian has a two-sided home system tile and starts the game as
            // The Firmament (96A). 96B is the side it flips to when becoming The Obsidian.
            .filter(|s| s.id != "96B")
            .flat_map(|s| &s.planets)
            .cloned()
            .collect()
    }

    /// Returns a set of the technologies the faction starts with.
    pub fn get_starting_techs(&self, expansions: &Expansions) -> BTreeSet<Technology> {
        match self {
            Faction::Arborec => vec![
                Technology::MagenDefenceGrid,
                Technology::MagenDefenceGridOmega,
                Technology::MagenDefenceGridOmegaOmega,
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
            Faction::CouncilKeleres => vec![/* Gets to pick starting techs */],
            Faction::LastBastion => vec![/* Gets to pick starting techs */],
            Faction::RalNelConsortium => vec![/* Gets to pick starting techs */],
            Faction::DeepwroughtScholarate => {
                vec![/* Gets to research twice at the start of the game */]
            }
            Faction::CrimsonRebellion => vec![/* Gets to pick a starting tech */],
            Faction::FirmamentObsidian => vec![/* Gets to pick a starting tech */],
        }
        .into_iter()
        .filter(|tech| tech.is_enabled_in(expansions))
        .collect()
    }

    // TODO: Uggly circular dependency here, consider solutions.
    pub fn get_starting_tech_options(&self, game_state: &GameState) -> Option<Vec<Technology>> {
        let techs = Technology::iter()
            .filter(|t| t.is_enabled_in(&game_state.game_settings.expansions))
            .collect::<Vec<_>>();

        let techs: Vec<_> = match self {
            Faction::Winnu => techs
                .into_iter()
                .filter(|t| {
                    matches!(
                        t.info().origin,
                        TechOrigin::Base | TechOrigin::Faction(Faction::Winnu)
                    )
                })
                .filter(|t| t.info().requirements.is_empty())
                .collect(),
            Faction::ArgentFlight => vec![
                Technology::NeuralMotivator,
                Technology::SarweenTools,
                Technology::PlasmaScoring,
            ],
            Faction::CouncilKeleres => game_state
                .players
                .values()
                .filter(|p| p.faction != Faction::CouncilKeleres)
                .flat_map(|p| p.technologies.iter())
                .filter(|t| t.info().origin == TechOrigin::Base)
                .cloned()
                .collect(),
            Faction::CrimsonRebellion => techs
                .into_iter()
                .filter(|t| t.info().requirements.is_empty())
                .filter(|t| {
                    matches!(
                        t.info().tech_type,
                        TechType::Category(TechCategory::Warfare)
                            | TechType::Category(TechCategory::Propulsion)
                    )
                })
                .filter(|t| {
                    matches!(
                        t.info().origin,
                        TechOrigin::Base | TechOrigin::Faction(Faction::CrimsonRebellion)
                    )
                })
                .collect(),
            Faction::LastBastion => techs
                .into_iter()
                .filter(|t| t.info().requirements.is_empty())
                .filter(|t| {
                    matches!(
                        t.info().tech_type,
                        TechType::Category(TechCategory::Cybernetic)
                            | TechType::Category(TechCategory::Propulsion)
                    )
                })
                .filter(|t| {
                    matches!(
                        t.info().origin,
                        TechOrigin::Base | TechOrigin::Faction(Faction::LastBastion)
                    )
                })
                .collect(),
            Faction::FirmamentObsidian => techs
                .into_iter()
                .filter(|t| t.info().requirements.is_empty())
                .filter(|t| {
                    matches!(
                        t.info().tech_type,
                        TechType::Category(TechCategory::Biotic)
                            | TechType::Category(TechCategory::Cybernetic)
                    )
                })
                .filter(|t| {
                    matches!(
                        t.info().origin,
                        TechOrigin::Base | TechOrigin::Faction(Faction::FirmamentObsidian)
                    )
                })
                .collect(),
            Faction::DeepwroughtScholarate => techs
                .into_iter()
                .filter(|t| {
                    matches!(
                        t.info().origin,
                        TechOrigin::Base | TechOrigin::Faction(Faction::DeepwroughtScholarate)
                    )
                })
                .collect(),
            Faction::RalNelConsortium => techs
                .into_iter()
                .filter(|t| t.info().requirements.is_empty())
                .filter(|t| {
                    matches!(
                        t.info().tech_type,
                        TechType::Category(TechCategory::Warfare)
                            | TechType::Category(TechCategory::Biotic)
                    )
                })
                .filter(|t| {
                    matches!(
                        t.info().origin,
                        TechOrigin::Base | TechOrigin::Faction(Faction::RalNelConsortium)
                    )
                })
                .collect(),
            _ => return None,
        };

        // Filter out techs that aren't enabled.
        Some(techs)
    }

    pub fn get_starting_tech_choice_count(&self) -> usize {
        match self {
            Faction::Winnu => 1,
            Faction::ArgentFlight => 2,
            Faction::CouncilKeleres => 2,
            Faction::CrimsonRebellion => 1,
            Faction::LastBastion => 1,
            Faction::FirmamentObsidian => 1,
            Faction::DeepwroughtScholarate => 2,
            Faction::RalNelConsortium => 1,
            _ => 0,
        }
    }
}

impl PartialOrd for Faction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Faction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name().to_lowercase().cmp(&other.name().to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::common::faction::Faction;

    #[test]
    fn faction_breakthrough_matches() {
        for f in Faction::iter() {
            assert_eq!(
                f.breakthrough().info().faction,
                f,
                "Faction {:?} has a mismatched breakthrough {:?} which claims to belong to faction: {:?}",
                f,
                f.breakthrough(),
                f.breakthrough().info().faction,
            );
        }
    }
}
