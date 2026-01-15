use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use {
    anyhow::Context,
    ti_helper_game_data::actions::event::Event,
    ti_helper_game_data::common::{
        game_settings::{Expansions, GameSettings},
        milty_data::MiltyData,
    },
    ti_helper_milty::MiltyImport,
};

#[derive(Serialize, Deserialize)]
pub struct NewGame {
    pub points: u32,
    pub game_config: GameConfig,
}

/// The game configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameConfig {
    /// Config specified by the user.
    #[serde(rename_all = "camelCase")]
    CustomGameConfig {
        /// If Prophecy of kings is to be used.
        pok: bool,
        /// If Codex I should be used.
        cod1: bool,
        /// If Codex II should be used.
        cod2: bool,
        /// If Codex III should be used.
        cod3: bool,
        /// If Thunder's Edge is to be used.
        te: bool,
    },
    /// Game config imported from milty draft.
    #[serde(rename_all = "camelCase")]
    ImportFromMilty {
        /// The game id from milty (from the URL).
        milty_game_id: String,
        /// The milty tts map string.
        milty_tts_string: String,
    },
}

impl NewGame {
    #[cfg(feature = "server")]
    /// Creates the appropriate new game event for this [NewGame].
    pub async fn to_new_game_event(&self) -> anyhow::Result<Event> {
        log::info!("Trying to create new game event");
        Ok(match &self.game_config {
            GameConfig::CustomGameConfig {
                pok,
                cod1,
                cod2,
                cod3,
                te,
            } => {
                log::info!("Creating new game event from settings");
                Event::SetSettings {
                    settings: GameSettings {
                        max_points: self.points,
                        expansions: Expansions {
                            prophecy_of_kings: *pok,
                            codex_1: *cod1,
                            codex_2: *cod2,
                            codex_3: *cod3,
                            thunders_edge: *te,
                        },
                    },
                }
            }
            GameConfig::ImportFromMilty {
                milty_game_id,
                milty_tts_string,
            } => {
                log::info!("Importing game from milty");
                let milty_data = MiltyData::import_from_milty(milty_game_id, milty_tts_string)
                    .await
                    .context("Failed to import game from milty")?;
                log::debug!("Milty data import {milty_data:?}");
                Event::ImportFromMilty {
                    max_points: self.points,
                    milty_data: Box::new(milty_data),
                }
            }
        })
    }
}
