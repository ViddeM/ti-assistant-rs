#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Logic for importing game configuration from milty.

use std::collections::HashMap;

use eyre::Context;
use serde::Deserialize;

/// Data imported from miltydraft.
pub struct MiltyData {}

// Example miltydata response:

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MiltyDataResponse {
    draft: MiltyDraftResponse,
    success: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MiltyDraftResponse {
    done: bool,
    id: String,
    draft: MiltyDraftDataResponse,
    config: MiltyConfigDataResponse,
    name: String,
    // Dont think we need these...
    // slices
    // factions
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MiltyDraftDataResponse {
    players: HashMap<String, MiltyPlayerResponse>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MiltyPlayerResponse {
    name: String,
    faction: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
struct MiltyConfigDataResponse {
    players: Vec<String>,
    name: String,
}

/// Import game configuration from a finished milty-draft.
pub fn import_from_milty(milty_id: &str) -> eyre::Result<MiltyData> {
    let client = reqwest::blocking::Client::new();
    let get_milty_data_response = client
        .get(&format!(
            "https://milty.shenanigans.be/data.php?draft={milty_id}",
        ))
        .send()
        .wrap_err("Failed to retrieve game data from milty")?;

    todo!("not implemented");
}
