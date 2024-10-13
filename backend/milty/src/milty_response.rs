use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyDataResponse {
    pub draft: MiltyDraftResponse,
    pub success: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltyDraftResponse {
    pub done: bool,
    pub _id: String,
    pub draft: MiltyDraftDataResponse,
    pub config: MiltyConfigDataResponse,
    pub name: String,
    pub _slices: Vec<MiltySliceResponse>,
    pub _factions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MiltySliceResponse {
    pub _tiles: Vec<u32>,
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
    pub _players: Vec<String>,
    pub _name: Option<String>,
    pub include_pok: bool,
    pub _include_keleres: bool,
    pub include_ds_tiles: bool,
    pub include_discordant: bool,
    pub include_discordantexp: bool,
}

impl MiltyConfigDataResponse {
    pub fn any_ds_enabled(&self) -> bool {
        self.include_ds_tiles || self.include_discordant || self.include_discordantexp
    }
}
