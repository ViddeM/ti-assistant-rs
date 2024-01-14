use std::collections::HashMap;

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::{
    data::{
        common::{color::Color, faction::Faction},
        components::{
            action_card::{ActionCard, ActionCardInfo},
            agenda::{Agenda, AgendaInfo},
            frontier_card::{FrontierCard, FrontierCardInfo},
            leaders::{Agent, Commander, Hero, Leader, LeaderInfo},
            objectives::{
                public::PublicObjective, secret::SecretObjective, Objective, ObjectiveInfo,
            },
            planet::{Planet, PlanetInfo},
            planet_attachment::{PlanetAttachment, PlanetAttachmentInfo},
            relic::{Relic, RelicInfo},
            system::{systems, System},
            tech::{TechInfo, Technology},
        },
    },
    gameplay::game_settings::Expansions,
};

const MIN_PLAYER_COUNT: usize = 3;

/// All information that is static for a game of TI4.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOptions {
    /// The minimum number of players allowed for the game.
    min_players: usize,
    /// The maximum number of players allowed for the game.
    max_players: usize,
    /// What colors players are allowed to have.
    colors: Vec<Color>,
    /// What factions exist within the game.
    factions: Vec<FactionResponse>,
    /// What systems exists in the game.
    systems: Vec<System>,
    /// What technologies exist in the game.
    technologies: HashMap<Technology, TechInfo>,
    /// What planets exist in the game.
    planet_infos: HashMap<Planet, PlanetInfo>,
    /// What planet attachments exist in the game.
    planet_attachments: HashMap<PlanetAttachment, PlanetAttachmentInfo>,
    /// What objectives exist in the game.
    objectives: HashMap<Objective, ObjectiveInfo>,
    /// What action cards exist in the game.
    action_cards: HashMap<ActionCard, ActionCardInfo>,
    /// What agendas exist in the game.
    agendas: HashMap<Agenda, AgendaInfo>,
    /// What leaders exist in the game.
    leaders: HashMap<Leader, LeaderInfo>,
    /// Map from all factions in the game to the leaders of that faction.
    leaders_by_faction: HashMap<Faction, Vec<Leader>>,
    /// What frontier cards exists in the game.
    frontier_cards: HashMap<FrontierCard, FrontierCardInfo>,
    /// What relics exists in the game.
    relics: HashMap<Relic, RelicInfo>,
}

impl GameOptions {
    /// Returns GameOptions for the specified expansions.
    pub fn new(expansions: &Expansions) -> Self {
        let leaders: HashMap<_, _> = Agent::iter()
            .map(Leader::from)
            .chain(Commander::iter().map(Leader::from))
            .chain(Hero::iter().map(Leader::from))
            .filter(|leader| leader.is_enabled_in(expansions))
            .map(|leader| (leader, leader.info()))
            .collect();

        Self {
            min_players: MIN_PLAYER_COUNT,
            max_players: expansions.max_number_of_players(),
            factions: Faction::iter()
                .filter(|f| expansions.is_enabled(&f.expansion()))
                .map(|f| FactionResponse {
                    faction: f.clone(),
                    name: f.name(),
                })
                .collect::<Vec<FactionResponse>>(),
            colors: Color::iter().collect(),
            systems: systems()
                .into_values()
                .filter(|s| expansions.is_enabled(&s.expansion))
                .collect(),
            planet_infos: Planet::iter()
                .map(|p| (p.clone(), p.info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .collect(),
            planet_attachments: PlanetAttachment::iter()
                .map(|a| (a.clone(), a.info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .collect(),
            objectives: PublicObjective::iter()
                .map(Objective::from)
                .chain(SecretObjective::iter().map(Objective::from))
                .map(|o| {
                    let info = o.info();
                    (o, info)
                })
                .filter(|(_, o)| expansions.is_enabled(&o.expansion))
                .collect(),
            technologies: Technology::iter()
                .map(|t| (t.clone(), t.info()))
                .filter(|(_, t)| expansions.is_enabled(&t.expansion))
                .collect(),
            action_cards: ActionCard::iter()
                .map(|card| (card.clone(), card.info()))
                .filter(|(_, card)| expansions.is_enabled(&card.expansion))
                .collect(),
            agendas: Agenda::iter()
                .map(|agenda| (agenda, agenda.info()))
                .filter(|(_, agenda)| expansions.is_enabled(&agenda.expansion))
                .filter(|(agenda, _)| !(expansions.prophecy_of_kings && agenda.disabled_in_pok()))
                .collect(),
            leaders_by_faction: leaders
                .iter()
                .map(|(leader, info)| (info.faction(), leader))
                .fold(HashMap::new(), |mut acc, (faction, leader)| {
                    acc.entry(faction).or_default().push(*leader);
                    acc
                }),
            leaders,
            frontier_cards: FrontierCard::iter()
                .map(|f| (f.clone(), f.info()))
                .filter(|(_, card)| expansions.is_enabled(&card.expansion))
                .collect(),
            relics: Relic::iter()
                .map(|relic| (relic.clone(), relic.info()))
                .filter(|(_, relic)| expansions.is_enabled(&relic.expansion))
                .collect(),
        }
    }
}

/// A faction in the game.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionResponse {
    /// The faction ID.
    faction: Faction,
    /// The name of the faction in 'pretty' format.
    name: String,
}
