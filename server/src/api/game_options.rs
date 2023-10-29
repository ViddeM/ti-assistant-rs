use serde::Serialize;

use crate::{
    data::{
        common::faction::Faction,
        components::{public_objectives::PublicObjectives, secret_objectives::SecretObjectives},
    },
    phases::Phase,
};

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
    public_objectives: Vec<PublicObjectiveResponse>,
    secret_objectives: Vec<SecretObjectiveResponse>,
}

#[derive(Serialize)]
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
