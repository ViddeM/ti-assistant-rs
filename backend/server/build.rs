use std::{fs, path::Path, str::FromStr};

use ti_helper_db::{db, game_id::GameId, queries};
use ti_helper_game::gameplay::game::Game;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let verify = read_env_var_bool("VERIFY_DEMO_GAMES");
    if !verify {
        return;
    }

    let overwrite_db_games = read_env_var_bool("OVERWRITE_DB_DEMO_GAMES");
    let database_url = read_env_var_string("DATABASE_URL");
    let demo_games_dir = read_env_var_string("DEMO_GAMES_DIR");
    let demo_games_skip_db = read_env_var_bool("DEMO_GAMES_SKIP_DB");

    let dir = Path::new(&demo_games_dir);
    if !dir.exists() {
        panic!("Demo games dir does not exist!");
    }

    if !dir.is_dir() {
        panic!("Demo games dir must be a directory!");
    }

    let games = fs::read_dir(dir)
        .expect("Failed to read demo games dir")
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
            let game: Game = serde_json::from_str(&json).unwrap_or_else(|err| {
                panic!("Failed to deserialize demo game {name} (id: {id:?}), err: {err:?}")
            });

            // Verify that we can re-apply the events to a new game.
            let mut new_game = Game::default();
            for (event, timestamp) in game.history.into_iter() {
                new_game.apply_or_fail(event, timestamp);
            }

            (name, id, new_game)
        })
        .collect::<Vec<(String, GameId, Game)>>();

    if !demo_games_skip_db {
        let db_pool = db::setup_pool(&database_url)
            .await
            .expect("failed to set up database pool");
        // Insert the games into the database
        for (_, id, game) in games.into_iter() {
            let db_game = queries::try_get_game_by_id(&db_pool, &id)
                .await
                .expect("Failed to retrieve game from DB");

            if db_game.is_none() || overwrite_db_games {
                if db_game.is_some() && overwrite_db_games {
                    // Delete all events first.
                    queries::delete_all_events_for_game(&db_pool, &id)
                        .await
                        .expect("Failed to delete events for game");
                }

                for (event, timestamp) in game.history.into_iter() {
                    let event_json =
                        serde_json::to_value(event).expect("Failed to serialize event?");
                    queries::insert_game_event(&db_pool, id, event_json, timestamp)
                        .await
                        .expect("Failed to insert game event for game");
                }
            }
        }
    }
}

fn read_env_var_string(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Missing environment variable '{key}'"))
}

fn read_env_var_bool(key: &str) -> bool {
    let s = read_env_var_string(key);
    match s.to_lowercase().as_str() {
        "true" => true,
        "false" => false,
        o => panic!("Invalid boolean '{o}' for env var '{key}'"),
    }
}
