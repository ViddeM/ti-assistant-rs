use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    data::{
        common::{color::Color, faction::Faction},
        components::{
            public_objectives::PublicObjectives,
            secret_objectives::SecretObjectives,
            system::{systems, System},
        },
    },
    game::{Event, GameState},
    lobby::GameId,
    phases::Phase,
};

#[derive(Debug, Clone, Deserialize)]
pub enum WsMessageIn {
    JoinGame(GameId),
    NewGame,
    Event(Event),
}

#[derive(Debug, Clone, Serialize)]
pub enum WsMessageOut {
    GameOptions(GameOptions),
    GameState(GameState),
}

impl WsMessageOut {
    pub fn game_options() -> Self {
        Self::GameOptions(GameOptions {
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
            systems: systems().into_iter().map(|(_, system)| system).collect(),
            public_objectives: PublicObjectives::iter()
                .map(|o| {
                    let info = o.get_objective_info();
                    PublicObjectiveResponse {
                        id: o,
                        points: info.points,
                        name: info.name,
                        condition: info.condition,
                    }
                })
                .collect(),
            secret_objectives: SecretObjectives::iter()
                .map(|o| {
                    let info = o.get_objective_info();
                    SecretObjectiveResponse {
                        id: o,
                        phase: info.phase,
                        name: info.name,
                        condition: info.condition,
                    }
                })
                .collect(),
        })
    }

    pub fn game_state(state: GameState) -> Self {
        Self::GameState(state)
    }
}

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
    public_objectives: Vec<PublicObjectiveResponse>,
    secret_objectives: Vec<SecretObjectiveResponse>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionResponse {
    faction: Faction,
    name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicObjectiveResponse {
    id: PublicObjectives,
    points: u8,
    name: String,
    condition: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretObjectiveResponse {
    id: SecretObjectives,
    phase: Phase,
    name: String,
    condition: String,
}
