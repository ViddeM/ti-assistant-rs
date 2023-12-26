use std::collections::HashMap;

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::data::{
    common::{color::Color, faction::Faction},
    components::{
        action_card::{ActionCard, ActionCardInfo},
        agenda::{Agenda, AgendaInfo},
        objectives::{public::PublicObjective, secret::SecretObjective, Objective, ObjectiveInfo},
        planet::{Planet, PlanetInfo},
        system::{systems, System},
        tech::{TechInfo, Technology},
    },
};

const MIN_PLAYER_COUNT: u32 = 3;
const MAX_PLAYER_COUNT: u32 = 8;
const MIN_SCORE: u32 = 4;
const MAX_SCORE: u32 = 20;

/// All information that is static for a game of TI4.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOptions {
    /// What numbers of players are allowed for the game.
    player_counts: Vec<u32>,
    /// The minimum score required to win a game.
    min_score: u32,
    /// The maximum score required to win a game.
    max_score: u32,
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

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            player_counts: (MIN_PLAYER_COUNT..=MAX_PLAYER_COUNT).collect::<Vec<u32>>(),
            min_score: MIN_SCORE,
            max_score: MAX_SCORE,
            factions: Faction::iter()
                .map(|f| FactionResponse {
                    faction: f.clone(),
                    name: f.name(),
                })
                .collect::<Vec<FactionResponse>>(),
            colors: Color::iter().collect(),
            systems: systems().into_values().collect(),
            planet_infos: Planet::iter()
                .map(|p| (p.clone(), p.planet_info()))
                .collect(),
            objectives: PublicObjective::iter()
                .map(Objective::from)
                .chain(SecretObjective::iter().map(Objective::from))
                .map(|o| {
                    let info = o.get_objective_info();
                    (o, info)
                })
                .collect(),
            technologies: Technology::iter().map(|t| (t.clone(), t.info())).collect(),
            action_cards: ActionCard::iter()
                .map(|card| (card.clone(), card.info()))
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
