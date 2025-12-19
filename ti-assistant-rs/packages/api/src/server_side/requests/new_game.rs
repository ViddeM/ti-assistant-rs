use serde::{Deserialize, Serialize};
use ti_helper_game_data::common::{
    game_settings::{Expansions, GameSettings},
    milty_data::MiltyData,
};
use ti_helper_game_logic::gameplay::event::Event;
use ti_helper_milty::MiltyImport;
use ti_helper_websocket::ws_message::GameConfig;

#[derive(Serialize, Deserialize)]
pub struct NewGame {
    points: u32,
    game_config: GameConfig,
}

impl NewGame {
    /// Creates the appropriate new game event for this [NewGame].
    pub async fn to_new_game_event(&self) -> anyhow::Result<Event> {
        Ok(match &self.game_config {
            GameConfig::CustomGameConfig {
                pok,
                cod1,
                cod2,
                cod3,
                te,
            } => Event::SetSettings {
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
            },
            GameConfig::ImportFromMilty {
                milty_game_id,
                milty_tts_string,
            } => {
                let milty_data =
                    MiltyData::import_from_milty(milty_game_id, milty_tts_string).await?;
                log::debug!("Milty data import {milty_data:?}");
                Event::ImportFromMilty {
                    max_points: self.points,
                    milty_data: Box::new(milty_data),
                }
            }
        })
    }
}
