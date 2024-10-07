#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Logic for importing game configuration from milty.

use std::collections::HashMap;

use eyre::Context;
use milty_response::{MiltyDataResponse, MiltyPlayerResponse};
use ti_helper_game_data::common::{faction::Faction, game_settings::Expansions, map::HexMap};

mod milty_response;

/// Data imported from miltydraft.
pub struct MiltyData {
    players: HashMap<String, MiltyPlayer>,
    expansions: Expansions,
    game_name: String,
}

/// Player imported from miltydraft.
pub struct MiltyPlayer {
    name: String,
    faction: Faction,
    order: u32,
}

impl TryFrom<MiltyPlayerResponse> for MiltyPlayer {
    type Error = eyre::Report;

    fn try_from(value: MiltyPlayerResponse) -> Result<Self, Self::Error> {}
}

/// Import game configuration from a finished milty-draft.
pub fn import_from_milty(milty_id: &str, tts_string: &str) -> eyre::Result<MiltyData> {
    let client = reqwest::blocking::Client::new();
    let get_milty_data_response: MiltyDataResponse = client
        .get(&format!(
            "https://milty.shenanigans.be/data.php?draft={milty_id}",
        ))
        .send()
        .wrap_err("Failed to retrieve game data from milty")?
        .json()
        .wrap_err("Failed to parse milty response")?;

    if !get_milty_data_response.success {
        eyre::bail!("Got error response from milty: {get_milty_data_response:?}");
    }

    if !get_milty_data_response.draft.done {
        eyre::bail!("Must finish milty draft before importing!");
    }

    Ok(MiltyData {
        players: get_milty_data_response
            .draft
            .draft
            .players
            .iter()
            .map(|(_, player)| MiltyPlayer::try_from(player).map(|p| (player.name, p)))
            .collect::<eyre::Result<HashMap<String, MiltyPlayer>>>()?,
        expansions: todo!(),
        game_name: todo!(),
    })
}
