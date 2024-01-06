use serde::{Deserialize, Serialize};
use strum_macros::{EnumDiscriminants, EnumIter};

use crate::{data::common::expansions::Expansion, gameplay::player::PlayerId};

use super::{objectives::secret::SecretObjective, planet::Planet, strategy_card::StrategyCard};

/// All information concerning an agenda.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct AgendaInfo {
    /// The name of the agenda card.
    pub name: &'static str,
    /// A description of what the agenda is about.
    pub description: &'static str,
    /// What type of agenda this is.
    pub kind: AgendaKind,
    /// What is to be elected for this agenda.
    pub elect: AgendaElectKind,
    /// What expansion this agenda belongs to.
    pub expansion: Expansion,
}

/// The type of agenda this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AgendaKind {
    /// A law that will either be enacted or denied.
    Law,
    /// A directive for an action to be taken.
    Directive,
}

/// A vote type where players can either vote For or Against.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ForOrAgainst {
    For,
    Against,
}

/// What is to be elected for an agenda.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumDiscriminants,
)]
#[strum_discriminants(name(AgendaElectKind))]
#[strum_discriminants(derive(PartialOrd, Ord, Hash, Serialize, Deserialize))]
#[serde(tag = "electKind", content = "value")]
pub enum AgendaElect {
    /// Select either a for or against alternative.
    ForOrAgainst(ForOrAgainst),

    /// Elect a player.
    Player(PlayerId),

    /// Elect a strategy card.
    StrategyCard(StrategyCard),

    /// Elect a Law that is currently in play.
    Law(Agenda),

    /// Elect a scored secret objective.
    SecretObjective(SecretObjective),

    /// Elect a planet.
    Planet(Planet),

    /// Elect a non-home planet except Mecatol Rex.
    PlanetWithTrait(Planet),

    /// Elect a cultural planet.
    CulturalPlanet(Planet),

    /// Elect a hazardous planet.
    HazardousPlanet(Planet),

    /// Elect an industrial planet.
    IndustrialPlanet(Planet),
}

/// An agenda in the game.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[allow(missing_docs)]
pub enum Agenda {
    // Base-game laws
    AntiIntellectualRevolution,
    ClassifiedDocumentLeaks,
    CommitteeFormation,
    ConventionsOfWar,
    CoreMining,
    DemilitarizedZone,
    EnforcedTravelBan,
    ExecutiveSanctions,
    FleetRegulations,
    HolyPlanetOfIxth,
    HomelandDefenseAct,
    ImperialArbiter,
    MinisterOfCommerce,
    MinisterOfExploration,
    MinisterOfIndustry,
    MinisterOfPeace,
    MinisterOfPolicy,
    MinisterOfSciences,
    MinisterOfWar,
    ProphecyOfIxth,
    PublicizeWeaponSchematics,
    RegulatedConscription,
    RepresentativeGovernmentTI4, // patched in PoK
    ResearchTeamBiotic,
    ResearchTeamCybernetic,
    ResearchTeamPropulsion,
    ResearchTeamWarfare,
    SenateSanctuary,
    ShardOfTheThrone,
    SharedResearch,
    TerraformingInitiative,
    TheCrownOfEmphidia,
    TheCrownOfThalnos,
    WormholeReconstruction,

    // Base-game directives
    ArchivedSecret,
    ArmsReduction,
    ColonialRedistribution,
    CompensatedDisarmament,
    EconomicEquality,
    IncentiveProgram,
    IxthianArtifact,
    JudicialAbolishment,
    MiscountDisclosed,
    Mutiny,
    NewConstitution,
    PublicExecution,
    SeedOfAnEmpire,
    SwordsToPlowshares,
    UnconventionalMeasures,
    WormholeResearch,

    // PoK laws
    ArticlesOfWar,
    ChecksAndBalances,
    NexusSovereignty,
    PoliticalCensure,
    RepresentativeGovernmentPOK, // overrides RepresentativeGovernmentTI4
    SearchWarrant,

    // PoK directives
    ArmedForcesStandardization,
    ClandestineOperations,
    CovertLegislation,
    GalacticCrisisPact,
    MinisterOfAntiques,
    RearmamentAgreement,
    ResearchGrantReallocation,
}

impl Agenda {
    /// Get the agenda info for this agenda.
    pub fn info(&self) -> AgendaInfo {
        macro_rules! info {
            ($ident:ident, $name:literal, $kind:ident, $elect:ident, $expansion:ident) => {
                AgendaInfo {
                    name: $name,
                    description: include_str!(concat!("./description/", stringify!($ident))),
                    kind: AgendaKind::$kind,
                    elect: AgendaElectKind::$elect,
                    expansion: Expansion::$expansion,
                }
            };
        }

        macro_rules! base_law {
            (ident: $ident:ident, name: $name:literal, elect: $elect:ident,) => {
                info! { $ident, $name, Law, $elect, Base }
            };
        }

        macro_rules! pok_law {
            (ident: $ident:ident, name: $name:literal, elect: $elect:ident,) => {
                info! { $ident, $name, Law, $elect, ProphecyOfKings }
            };
        }

        macro_rules! base_directive {
            (ident: $ident:ident, name: $name:literal, elect: $elect:ident,) => {
                info! { $ident, $name, Directive, $elect, Base }
            };
        }

        macro_rules! pok_directive {
            (ident: $ident:ident, name: $name:literal, elect: $elect:ident,) => {
                info! { $ident, $name, Directive, $elect, ProphecyOfKings }
            };
        }

        match self {
            Agenda::AntiIntellectualRevolution => base_law! {
                ident: AntiIntellectualRevolution,
                name: "Anti-Intellectual Revolution",
                elect: ForOrAgainst,
            },
            Agenda::ClassifiedDocumentLeaks => base_law! {
                ident: ClassifiedDocumentLeaks,
                name: "Classified Document Leaks",
                elect: SecretObjective,
            },
            Agenda::CommitteeFormation => base_law! {
                ident: CommitteeFormation,
                name: "Committee Formation",
                elect: Player,
            },
            Agenda::ConventionsOfWar => base_law! {
                ident: ConventionsOfWar,
                name: "Conventions of War",
                elect: ForOrAgainst,
            },
            Agenda::CoreMining => base_law! {
                ident: CoreMining,
                name: "Core Mining",
                elect: HazardousPlanet,
            },
            Agenda::DemilitarizedZone => base_law! {
                ident: DemilitarizedZone,
                name: "Demilitarized Zone",
                elect: CulturalPlanet,
            },
            Agenda::EnforcedTravelBan => base_law! {
                ident: EnforcedTravelBan,
                name: "Enforced Travel Ban",
                elect: ForOrAgainst,
            },
            Agenda::ExecutiveSanctions => base_law! {
                ident: ExecutiveSanctions,
                name: "Executive Sanctions",
                elect: ForOrAgainst,
            },
            Agenda::FleetRegulations => base_law! {
                ident: FleetRegulations,
                name: "Fleet Regulations",
                elect: ForOrAgainst,
            },
            Agenda::HolyPlanetOfIxth => base_law! {
                ident: HolyPlanetOfIxth,
                name: "Holy Planet of Ixth",
                elect: CulturalPlanet,
            },
            Agenda::HomelandDefenseAct => base_law! {
                ident: HomelandDefenseAct,
                name: "Homeland Defence Act",
                elect: ForOrAgainst,
            },
            Agenda::ImperialArbiter => base_law! {
                ident: ImperialArbiter,
                name: "Imperial Arbiter",
                elect: Player,
            },
            Agenda::MinisterOfCommerce => base_law! {
                ident: MinisterOfCommerce,
                name: "Minister of Commerce",
                elect: Player,
            },
            Agenda::MinisterOfExploration => base_law! {
                ident: MinisterOfExploration,
                name: "Minister of Exploration",
                elect: Player,
            },
            Agenda::MinisterOfIndustry => base_law! {
                ident: MinisterOfIndustry,
                name: "Minister of Industry",
                elect: Player,
            },
            Agenda::MinisterOfPeace => base_law! {
                ident: MinisterOfPeace,
                name: "Minister of Peace",
                elect: Player,
            },
            Agenda::MinisterOfPolicy => base_law! {
                ident: MinisterOfPolicy,
                name: "Minister of Policy",
                elect: Player,
            },
            Agenda::MinisterOfSciences => base_law! {
                ident: MinisterOfSciences,
                name: "Minister of Sciences",
                elect: Player,
            },
            Agenda::MinisterOfWar => base_law! {
                ident: MinisterOfWar,
                name: "Minister of War",
                elect: Player,
            },
            Agenda::ProphecyOfIxth => base_law! {
                ident: ProphecyOfIxth,
                name: "Prophecy of Ixth",
                elect: Player,
            },
            Agenda::PublicizeWeaponSchematics => base_law! {
                ident: PublicizeWeaponSchematics,
                name: "Publicize Weapon Schematics",
                elect: ForOrAgainst,
            },
            Agenda::RegulatedConscription => base_law! {
                ident: RegulatedConscription,
                name: "Regulated Conscription",
                elect: ForOrAgainst,
            },
            Agenda::RepresentativeGovernmentTI4 => base_law! {
                ident: RepresentativeGovernmentTI4,
                name: "Representative Government",
                elect: ForOrAgainst,
            },
            Agenda::ResearchTeamBiotic => base_law! {
                ident: ResearchTeamBiotic,
                name: "Research Team: Biotic",
                elect: IndustrialPlanet,
            },
            Agenda::ResearchTeamCybernetic => base_law! {
                ident: ResearchTeamCybernetic,
                name: "Research Team: Cybernetic",
                elect: IndustrialPlanet,
            },
            Agenda::ResearchTeamPropulsion => base_law! {
                ident: ResearchTeamPropulsion,
                name: "Research Team: Propulsion",
                elect: IndustrialPlanet,
            },
            Agenda::ResearchTeamWarfare => base_law! {
                ident: ResearchTeamWarfare,
                name: "Research Team: Warfare",
                elect: HazardousPlanet,
            },
            Agenda::SenateSanctuary => base_law! {
                ident: SenateSanctuary,
                name: "Senate Sanctuary",
                elect: CulturalPlanet,
            },
            Agenda::ShardOfTheThrone => base_law! {
                ident: ShardOfTheThrone,
                name: "Shard of the Throne",
                elect: Player,
            },
            Agenda::SharedResearch => base_law! {
                ident: SharedResearch,
                name: "Shared Research",
                elect: ForOrAgainst,
            },
            Agenda::TerraformingInitiative => base_law! {
                ident: TerraformingInitiative,
                name: "Terraforming Initiative",
                elect: HazardousPlanet,
            },
            Agenda::TheCrownOfEmphidia => base_law! {
                ident: TheCrownOfEmphidia,
                name: "The Crown of Emphidia",
                elect: Player,
            },
            Agenda::TheCrownOfThalnos => base_law! {
                ident: TheCrownOfThalnos,
                name: "The Crown of Thalnos",
                elect: Player,
            },
            Agenda::WormholeReconstruction => base_law! {
                ident: WormholeReconstruction,
                name: "Wormhole Reconstruction",
                elect: ForOrAgainst,
            },

            // Directives
            Agenda::ArchivedSecret => base_directive! {
                ident: ArchivedSecret,
                name: "Archived Secret",
                elect: Player,
            },
            Agenda::ArmsReduction => base_directive! {
                ident: ArmsReduction,
                name: "Arms Reduction",
                elect: ForOrAgainst,
            },
            Agenda::ColonialRedistribution => base_directive! {
                ident: ColonialRedistribution,
                name: "Colonial Redistribution",
                elect: PlanetWithTrait,
            },
            Agenda::CompensatedDisarmament => base_directive! {
                ident: CompensatedDisarmament,
                name: "Compensated Disarmament",
                elect: Planet,
            },
            Agenda::EconomicEquality => base_directive! {
                ident: EconomicEquality,
                name: "Economic Equality",
                elect: ForOrAgainst,
            },
            Agenda::IncentiveProgram => base_directive! {
                ident: IncentiveProgram,
                name: "Incentive Program",
                elect: ForOrAgainst,
            },
            Agenda::IxthianArtifact => base_directive! {
                ident: IxthianArtifact,
                name: "Ixthian Artifact",
                elect: ForOrAgainst,
            },
            Agenda::JudicialAbolishment => base_directive! {
                ident: JudicialAbolishment,
                name: "Judicial Abolishment",
                elect: Law,
            },
            Agenda::MiscountDisclosed => base_directive! {
                ident: MiscountDisclosed,
                name: "Mount Disclosed",
                elect: Law,
            },
            Agenda::Mutiny => base_directive! {
                ident: Mutiny,
                name: "Mutiny",
                elect: ForOrAgainst,
            },
            Agenda::NewConstitution => base_directive! {
                ident: NewConstitution,
                name: "New Constitution",
                elect: ForOrAgainst,
            },
            Agenda::PublicExecution => base_directive! {
                ident: PublicExecution,
                name: "Public Execution",
                elect: Player,
            },
            Agenda::SeedOfAnEmpire => base_directive! {
                ident: SeedOfAnEmpire,
                name: "Seed of an Empire",
                elect: ForOrAgainst,
            },
            Agenda::SwordsToPlowshares => base_directive! {
                ident: SwordsToPlowshares,
                name: "Swords to Plowshares",
                elect: ForOrAgainst,
            },
            Agenda::UnconventionalMeasures => base_directive! {
                ident: UnconventionalMeasures,
                name: "Unconventional Measures",
                elect: ForOrAgainst,
            },
            Agenda::WormholeResearch => base_directive! {
                ident: WormholeResearch,
                name: "Wormhole Research",
                elect: ForOrAgainst,
            },

            // PoK laws
            Agenda::ArticlesOfWar => pok_law! {
                ident: ArticlesOfWar,
                name: "Articles of War",
                elect: ForOrAgainst,
            },
            Agenda::ChecksAndBalances => pok_law! {
                ident: ChecksAndBalances,
                name: "Checks and Balances",
                elect: ForOrAgainst,
            },
            Agenda::NexusSovereignty => pok_law! {
                ident: NexusSovereignty,
                name: "Nexus Sovereignty",
                elect: ForOrAgainst,
            },
            Agenda::PoliticalCensure => pok_law! {
                ident: PoliticalCensure,
                name: "Political Censure",
                elect: Player,
            },
            Agenda::RepresentativeGovernmentPOK => pok_law! {
                ident: RepresentativeGovernmentPOK,
                name: "Representative Government",
                elect: ForOrAgainst,
            },
            Agenda::SearchWarrant => pok_law! {
                ident: SearchWarrant,
                name: "Search Warrant",
                elect: Player,
            },

            // PoK directives
            Agenda::ArmedForcesStandardization => pok_directive! {
                ident: ArmedForcesStandardization,
                name: "Armed Forces Standardization",
                elect: Player,
            },
            Agenda::ClandestineOperations => pok_directive! {
                ident: ClandestineOperations,
                name: "Clandestine Operations",
                elect: ForOrAgainst,
            },
            Agenda::CovertLegislation => pok_directive! {
                ident: CovertLegislation,
                name: "Covert Legislation",
                elect: ForOrAgainst,
            },
            Agenda::GalacticCrisisPact => pok_directive! {
                ident: GalacticCrisisPact,
                name: "Galactic Crisis Pact",
                elect: StrategyCard,
            },
            Agenda::MinisterOfAntiques => pok_directive! {
                ident: MinisterOfAntiques,
                name: "Minister of Antiques",
                elect: Player,
            },
            Agenda::RearmamentAgreement => pok_directive! {
                ident: RearmamentAgreement,
                name: "Rearmament Agreement",
                elect: ForOrAgainst,
            },
            Agenda::ResearchGrantReallocation => pok_directive! {
                ident: ResearchGrantReallocation,
                name: "Research Grant Reallocation",
                elect: Player,
            },
        }
    }
}
