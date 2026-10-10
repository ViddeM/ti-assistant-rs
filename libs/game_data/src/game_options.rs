use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    common::{color::Color, faction::Faction, game_settings::Expansions},
    components::{
        action_card::ActionCard,
        agenda::Agenda,
        frontier_card::FrontierCard,
        leaders::Leader,
        objectives::{Objective, public::PublicObjective, secret::SecretObjective},
        planet::{Planet, PlanetInfo},
        planet_attachment::{PlanetAttachment, PlanetAttachmentInfo},
        relic::Relic,
        system::{System, SystemId, systems},
        tech::Technology,
    },
};

const MIN_PLAYER_COUNT: usize = 3;

/// All information that is static for a game of TI4.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameOptions {
    /// The minimum number of players allowed for the game.
    pub min_players: usize,
    /// The maximum number of players allowed for the game.
    pub max_players: usize,
    /// What colors players are allowed to have.
    pub colors: Vec<Color>,
    /// What factions exist within the game.
    pub factions: Vec<FactionResponse>,
    /// What systems exists in the game.
    pub systems: HashMap<SystemId, System>,
    /// What technologies exist in the game.
    pub technologies: Vec<Technology>,
    /// What planets exist in the game.
    pub planet_infos: HashMap<Planet, PlanetInfo>,
    /// What planet attachments exist in the game.
    pub planet_attachments: HashMap<PlanetAttachment, PlanetAttachmentInfo>,
    /// What objectives exist in the game.
    pub objectives: Vec<Objective>,
    /// What action cards exist in the game.
    pub action_cards: Vec<ActionCard>,
    /// What agendas exist in the game.
    pub agendas: Vec<Agenda>,
    /// What leaders exist in the game.
    pub leaders: Vec<Leader>,
    /// Map from all factions in the game to the leaders of that faction.
    pub leaders_by_faction: HashMap<Faction, Vec<Leader>>,
    /// What frontier cards exists in the game.
    pub frontier_cards: Vec<FrontierCard>,
    /// What relics exists in the game.
    pub relics: Vec<Relic>,
}

impl std::ops::Deref for GameOptions {
    type Target = HashMap<Faction, Vec<Leader>>;

    fn deref(&self) -> &Self::Target {
        &self.leaders_by_faction
    }
}

impl GameOptions {
    /// Returns GameOptions for the specified expansions.
    pub fn new(expansions: &Expansions) -> Self {
        let leaders: Vec<Leader> = Leader::iter()
            .filter(|leader| leader.is_enabled_in(expansions))
            .collect();

        Self {
            min_players: MIN_PLAYER_COUNT,
            max_players: expansions.max_number_of_players(),
            factions: Faction::iter()
                .filter(|f| expansions.is_enabled(&f.expansion()))
                .map(|faction| FactionResponse {
                    faction,
                    name: faction.name(),
                })
                .collect::<Vec<FactionResponse>>(),
            colors: Color::iter().collect(),
            systems: systems()
                .into_iter()
                .filter(|(_, s)| expansions.is_enabled(&s.expansion))
                .collect(),
            planet_infos: Planet::iter()
                .map(|p| (p.clone(), p.info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .filter(|(planet, _)| {
                    if expansions.thunders_edge {
                        planet.ne(&Planet::MecatolRex)
                    } else {
                        planet.ne(&Planet::MecatolRexOmega)
                    }
                })
                .collect(),
            planet_attachments: PlanetAttachment::iter()
                .map(|a| (a.clone(), a.info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .collect(),
            objectives: PublicObjective::iter()
                .map(Objective::from)
                .chain(SecretObjective::iter().map(Objective::from))
                .filter(|o| expansions.is_enabled(&o.info().expansion))
                .collect(),
            technologies: Technology::iter()
                .filter(|tech| tech.is_enabled_in(expansions))
                .collect(),
            action_cards: ActionCard::iter()
                .filter(|card| expansions.is_enabled(&card.info().expansion))
                .collect(),
            agendas: Agenda::iter()
                .filter(|agenda| expansions.is_enabled(&agenda.info().expansion))
                .filter(|agenda| !(expansions.prophecy_of_kings && agenda.disabled_in_pok()))
                .collect(),
            leaders_by_faction: leaders
                .iter()
                .map(|leader| (leader.info().faction(), leader))
                .fold(HashMap::new(), |mut acc, (faction, leader)| {
                    acc.entry(faction).or_default().push(*leader);
                    acc
                }),
            leaders,
            frontier_cards: FrontierCard::iter()
                .filter(|card| expansions.is_enabled(&card.info().expansion))
                .collect(),
            relics: Relic::iter()
                .filter(|relic| expansions.is_enabled(&relic.info().expansion))
                .collect(),
        }
    }
}

/// A faction in the game.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FactionResponse {
    /// The faction ID.
    pub faction: Faction,
    /// The name of the faction in 'pretty' format.
    pub name: String,
}
