#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Logic for importing game configuration from milty.

use std::collections::{HashMap, HashSet};

use eyre::Context;
use milty_response::{MiltyDataResponse, MiltyPlayerResponse};
use ti_helper_game_data::common::{
    faction::Faction,
    game_settings::Expansions,
    map::HexMap,
    milty_data::{MiltyData, MiltyPlayer},
};

mod milty_response;

impl TryFrom<&MiltyPlayerResponse> for MiltyPlayer {
    type Error = eyre::Report;

    fn try_from(value: &MiltyPlayerResponse) -> Result<Self, Self::Error> {
        Ok(MiltyPlayer {
            name: html_escape::decode_html_entities(&value.name).to_string(),
            faction: Faction::parse(&value.faction).wrap_err_with(|| {
                format!(
                    "Failed to parse faction for player {} ({})",
                    value.name, value.faction
                )
            })?,
            order: value
                .position
                .parse::<u32>()
                .wrap_err("Failed to parse order into u32")?,
        })
    }
}

/// Trait for importing data from milty draft.
pub trait MiltyImport {
    /// Try to Import data from milty draft.
    fn import_from_milty(
        milty_id: &str,
        tts_string: &str,
    ) -> impl std::future::Future<Output = eyre::Result<MiltyData>> + Send;
}

impl MiltyImport for MiltyData {
    /// Import game configuration from a finished milty-draft.
    async fn import_from_milty(milty_id: &str, tts_string: &str) -> eyre::Result<MiltyData> {
        let client = reqwest::Client::new();

        let get_milty_data_response: MiltyDataResponse = client
            .get(format!(
                "https://milty.shenanigans.be/api/data?draft={milty_id}",
            ))
            .send()
            .await
            .wrap_err("Failed to retrieve game data from milty")?
            .json()
            .await
            .wrap_err("Failed to parse milty response")?;

        log::debug!("Get milty data response {get_milty_data_response:?}");

        if !get_milty_data_response.success {
            eyre::bail!("Got error response from milty: {get_milty_data_response:?}");
        }

        if !get_milty_data_response.draft.done {
            eyre::bail!("Must finish milty draft before importing!");
        }

        let milty_conf = get_milty_data_response.draft.config;
        if milty_conf.any_ds_enabled() {
            eyre::bail!("Discordant stars is currently not supported");
        }

        let players = get_milty_data_response
            .draft
            .draft
            .players
            .values()
            .map(|player| MiltyPlayer::try_from(player).map(|p| (player.name.clone(), p)))
            .collect::<eyre::Result<HashMap<String, MiltyPlayer>>>()?;

        let expansions = Expansions {
            prophecy_of_kings: milty_conf.include_pok,
            codex_1: true, // TODO: not entirely sure about these.
            codex_2: true,
            codex_3: true,
            thunders_edge: milty_conf.include_te_factions,
        };

        for player in players.values() {
            eyre::ensure!(
                expansions.is_enabled(&player.faction.expansion()),
                "Milty import included a faction from an expansion that is not enabled!"
            );
        }

        eyre::ensure!(
            players.keys().collect::<HashSet<&String>>().len() == players.len(),
            "Two or more players had the same name!"
        );

        eyre::ensure!(
            players
                .values()
                .map(|p| &p.faction)
                .collect::<HashSet<&Faction>>()
                .len()
                == players.len(),
            "Two or more players had the same faction!"
        );

        Ok(MiltyData {
            players,
            expansions,
            game_name: html_escape::decode_html_entities(&get_milty_data_response.draft.name)
                .to_string(),
            hex_map: HexMap::from_milty_string(tts_string)
                .wrap_err("Failed to parse milty tts string")?,
        })
    }
}
