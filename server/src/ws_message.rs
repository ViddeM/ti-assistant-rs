use serde::Serialize;
use strum::IntoEnumIterator;

use crate::{
    data::{
        common::faction::Faction,
        components::system::{systems, System},
    },
    game::GameState,
};

#[derive(Debug, Clone, Serialize)]
pub enum WsMessage {
    GameOptions(GameOptions),
    GameState(GameState),
}

impl WsMessage {
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
            systems: systems().into_iter().map(|(_, system)| system).collect(),
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
    factions: Vec<FactionResponse>,
    systems: Vec<System>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionResponse {
    faction: Faction,
    name: String,
}
