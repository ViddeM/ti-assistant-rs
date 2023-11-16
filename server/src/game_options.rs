use std::collections::HashMap;

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::data::{
    common::{color::Color, faction::Faction},
    components::{
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOptions {
    player_counts: Vec<u32>,
    min_score: u32,
    max_score: u32,
    colors: Vec<Color>,
    factions: Vec<FactionResponse>,
    systems: Vec<System>,
    technologies: HashMap<Technology, TechInfo>,
    planet_infos: HashMap<Planet, PlanetInfo>,
    objectives: HashMap<Objective, ObjectiveInfo>,
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
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionResponse {
    faction: Faction,
    name: String,
}
