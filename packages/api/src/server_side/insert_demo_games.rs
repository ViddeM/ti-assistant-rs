use std::{fs, str::FromStr};

use anyhow::Context;
use chrono::{DateTime, Utc};
use ti_helper_db::{db::DbPool, queries};
use ti_helper_game_data::{actions::event::Event, game_id::GameId};
use ti_helper_game_logic::gameplay::game::Game;

use crate::server_side::Opts;

pub async fn insert_demo_games(opt: &Opts, db_pool: &DbPool) -> anyhow::Result<()> {
    if !opt.demo_games_dir.exists() {
        anyhow::bail!("Demo games dir does not exist");
    }

    if !opt.demo_games_dir.is_dir() {
        anyhow::bail!("Demo games dir must be a directory!");
    }

    let games = fs::read_dir(opt.demo_games_dir.as_path())
        .context("Failed to read demo games dir")?
        .map(|f| f.expect("Failed to read dir entry"))
        .map(|f| f.path())
        .filter(|path| {
            path.is_file()
                && if let Some(ext) = path.extension() {
                    ext == "json"
                } else {
                    false
                }
        })
        .map(|path| {
            let file_name = path
                .file_name()
                .expect("Failed to retrieve filename for demo game")
                .to_str()
                .expect("Failed to convert filename to str");

            let (name, id) = file_name
                .split_once("__")
                .expect("Failed to split game name and id");

            let name = name.to_string();
            let id = id
                .strip_suffix(".json")
                .expect("Failed to strip .json from gameId");
            let id = GameId::from_str(id).unwrap_or_else(|err| {
                panic!("Failed to convert id {id} to gameId, err: {err}");
            });

            let json = fs::read_to_string(path).expect("Failed to read demo game");
            let events: Vec<(Event, DateTime<Utc>)> =
                serde_json::from_str(&json).unwrap_or_else(|err| {
                    panic!("Failed to deserialize demo game {name} (id: {id:?}), err: {err:?}")
                });

            // Verify that we can re-apply the events to a new game.
            let mut new_game = Game::default();
            for (event, timestamp) in events.into_iter() {
                new_game
                    .apply_or_err(event, timestamp)
                    .expect("Failed to apply event for demo game");
            }

            (name, id, new_game)
        })
        .collect::<Vec<(String, GameId, Game)>>();

    if !opt.demo_games_skip_db {
        // Insert the games into the database
        for (name, id, game) in games.into_iter() {
            let db_game = queries::try_get_game_by_id(db_pool, &id)
                .await
                .context("Failed to retrieve game from DB")?;

            if db_game.is_none() || opt.overwrite_db_games {
                if db_game.is_some() && opt.overwrite_db_games {
                    // Delete all events first.
                    queries::delete_all_events_for_game(db_pool, &id)
                        .await
                        .context("Failed to delete events for game")?;
                } else if db_game.is_none() {
                    // Create the game as well.
                    queries::create_game(db_pool, id, name)
                        .await
                        .context("Failed to create demo game")?;
                }

                for (event, timestamp) in game.history.into_iter() {
                    let event_json =
                        serde_json::to_value(event).context("Failed to serialize event?")?;
                    queries::insert_game_event(db_pool, id, event_json, timestamp)
                        .await
                        .context("Failed to insert game event for game")?;
                }
            }
        }
    }

    Ok(())
}
