use rocket::serde::json::Json;
use serde::Serialize;

use crate::data::common::faction::{Faction, ALL_FACTIONS};

const MIN_PLAYER_COUNT: u32 = 3;
const MAX_PLAYER_COUNT: u32 = 8;
const MIN_SCORE: u32 = 4;
const MAX_SCORE: u32 = 20;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameOptionsResponse {
    player_counts: Vec<u32>,
    min_score: u32,
    max_score: u32,
    factions: Vec<FactionResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FactionResponse {
    faction: Faction,
    name: String,
}

#[get("/game_options")]
pub async fn get_game_options() -> Json<GameOptionsResponse> {
    Json(GameOptionsResponse {
        player_counts: (MIN_PLAYER_COUNT..=MAX_PLAYER_COUNT).collect::<Vec<u32>>(),
        min_score: MIN_SCORE,
        max_score: MAX_SCORE,
        factions: ALL_FACTIONS
            .into_iter()
            .map(|f| FactionResponse {
                faction: f.clone(),
                name: f.name(),
            })
            .collect::<Vec<FactionResponse>>(),
    })
}
