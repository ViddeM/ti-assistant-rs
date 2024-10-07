use std::collections::HashMap;

use serde::Deserialize;
use ti_helper_game_data::common::map::HexMap;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyDataResponse {
    pub draft: MiltyDraftResponse,
    pub success: bool,
    pub hex_map: HexMap,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyDraftResponse {
    pub done: bool,
    pub id: String,
    pub draft: MiltyDraftDataResponse,
    pub config: MiltyConfigDataResponse,
    pub name: String,
    pub slices: Vec<MiltySliceResponse>,
    pub factions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltySliceResponse {
    pub tiles: Vec<u32>,
    // We don't need these...
    // specialities, wormholes, has_legendary, legendaries, total_influence, total_reources, optimal_influcuence, optimal_resources
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyDraftDataResponse {
    pub players: HashMap<String, MiltyPlayerResponse>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyPlayerResponse {
    pub name: String,
    pub faction: String,
    pub position: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyConfigDataResponse {
    pub players: Vec<String>,
    pub name: String,
}
