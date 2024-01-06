use std::collections::HashMap;

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::{
    data::{
        common::{color::Color, faction::Faction},
        components::{
            action_card::{ActionCard, ActionCardInfo},
            agenda::{Agenda, AgendaInfo},
            objectives::{
                public::PublicObjective, secret::SecretObjective, Objective, ObjectiveInfo,
            },
            planet::{Planet, PlanetInfo},
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
    /// What objectives exist in the game.
    objectives: HashMap<Objective, ObjectiveInfo>,
    /// What action cards exist in the game.
    action_cards: HashMap<ActionCard, ActionCardInfo>,
    /// What agendas exist in the game.
    agendas: HashMap<Agenda, AgendaInfo>,
}

impl GameOptions {
    /// Returns GameOptions for the specified expansions.
    pub fn new(expansions: &Expansions) -> Self {
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
                .map(|p| (p.clone(), p.planet_info()))
                .filter(|(_, info)| expansions.is_enabled(&info.expansion))
                .collect(),
            objectives: PublicObjective::iter()
                .map(Objective::from)
                .chain(SecretObjective::iter().map(Objective::from))
                .map(|o| {
                    let info = o.get_objective_info();
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
