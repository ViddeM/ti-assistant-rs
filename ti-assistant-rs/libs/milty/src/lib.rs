#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Logic for importing game configuration from milty.

use std::collections::{HashMap, HashSet};

use milty_response::MiltyPlayerResponse;
use ti_helper_game_data::common::{
    faction::Faction,
    game_settings::Expansions,
    map::HexMap,
    milty_data::{MiltyData, MiltyPlayer},
};

use crate::{
    error::{MiltyError, MiltyResult},
    faction_parser::parse_faction,
    milty_response::MiltyDraftResponse,
};

/// Errors that can occurr when importing from milty draft.
pub mod error;

mod faction_parser;
mod milty_response;

impl TryFrom<&MiltyPlayerResponse> for MiltyPlayer {
    type Error = MiltyError;

    fn try_from(value: &MiltyPlayerResponse) -> Result<Self, Self::Error> {
        Ok(MiltyPlayer {
            name: html_escape::decode_html_entities(&value.name).to_string(),
            faction: parse_faction(&value.faction).map_err(|err| {
                MiltyError::FactionParseError {
                    player: value.name.clone(),
                    faction: value.faction.clone(),
                    error: err.to_string(),
                }
            })?,
            order: value
                .position
                .parse::<u32>()
                .map_err(|err| MiltyError::OrderParseError {
                    position: value.position.clone(),
                    error: err.to_string(),
                })?,
        })
    }
}

/// Trait for importing data from milty draft.
pub trait MiltyImport {
    /// Try to Import data from milty draft.
    fn import_from_milty(
        milty_id: &str,
        tts_string: &str,
    ) -> impl std::future::Future<Output = MiltyResult<MiltyData>> + Send;
}

impl MiltyImport for MiltyData {
    /// Import game configuration from a finished milty-draft.
    async fn import_from_milty(milty_id: &str, tts_string: &str) -> MiltyResult<MiltyData> {
        let client = reqwest::Client::new();

        let get_milty_data_response: MiltyDraftResponse = client
            .get(format!("https://milty.shenanigans.be/api/draft/{milty_id}",))
            .send()
            .await?
            .json()
            .await
            .map_err(|err| MiltyError::ParseResponseError(err))?;

        log::debug!("Get milty data response {get_milty_data_response:?}");

        if !get_milty_data_response.done {
            return Err(MiltyError::DraftNotComplete);
        }

        let milty_conf = get_milty_data_response.config;
        if milty_conf.any_ds_enabled() {
            return Err(MiltyError::DiscordantStarsNotSupported(
                "Discordant stars options were enabled in milty config".to_string(),
            ));
        }

        let players = get_milty_data_response
            .draft
            .players
            .values()
            .map(|player| MiltyPlayer::try_from(player).map(|p| (player.name.clone(), p)))
            .collect::<MiltyResult<HashMap<String, MiltyPlayer>>>()?;

        let expansions = Expansions {
            prophecy_of_kings: milty_conf.include_pok_factions || milty_conf.include_pok,
            codex_1: true, // TODO: not entirely sure about these.
            codex_2: true,
            codex_3: true,
            thunders_edge: milty_conf.include_te_factions || milty_conf.include_te_tiles,
        };

        for player in players.values() {
            if !expansions.is_enabled(&player.faction.expansion()) {
                return Err(MiltyError::FactionsExpansionNotEnabled {
                    faction: player.faction,
                    expansion: player.faction.expansion(),
                });
            }
        }

        if players.keys().collect::<HashSet<&String>>().len() != players.len() {
            return Err(MiltyError::DuplicatePlayerNames);
        }

        if players
            .values()
            .map(|p| &p.faction)
            .collect::<HashSet<&Faction>>()
            .len()
            != players.len()
        {
            return Err(MiltyError::DuplicatePlayerFactions);
        }

        let game_name = milty_conf.name.clone().unwrap_or_default();

        Ok(MiltyData {
            players,
            expansions,
            game_name: html_escape::decode_html_entities(&game_name).to_string(),
            hex_map: HexMap::from_milty_string(tts_string, milty_conf.include_te_tiles)?,
        })
    }
}

#[cfg(test)]
mod tests {

    use ti_helper_game_data::common::milty_data::MiltyData;

    use crate::MiltyImport;

    #[tokio::test]
    async fn test_import_game() {
        let resp = MiltyData::import_from_milty(
            "693b2c674a032",
            "65 74 41 85A0 113 100 63 39 102 28 36 87A2 78 87A0 59 34 32 68 96a 47 49 95 50 64 94 24 83A0 85A0 83A1 35 93 80 67 0 107 110",
        ).await;

        println!("RESP: {resp:?}");
        assert_eq!(resp.is_ok(), true);

        let data = resp.unwrap();

        assert_eq!(data.players.len(), 5);

        assert_eq!(data.expansions.codex_1, true);
        assert_eq!(data.expansions.codex_2, true);
        assert_eq!(data.expansions.codex_3, true);
        assert_eq!(data.expansions.thunders_edge, true);
        assert_eq!(data.expansions.prophecy_of_kings, true);
    }
}
