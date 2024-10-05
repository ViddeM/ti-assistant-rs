use std::{fs, path::Path};

use chrono::{DateTime, Utc};
use ti_helper_game_logic::gameplay::{event::Event, game::Game};

fn main() {
    dotenvy::dotenv().ok();
    let verify = read_env_var_bool("VERIFY_DEMO_GAMES");
    if !verify {
        return;
    }

    let demo_games_dir = read_env_var_string("DEMO_GAMES_DIR");

    let dir = Path::new(&demo_games_dir);
    if !dir.exists() {
        panic!("Demo games dir does not exist!");
    }

    if !dir.is_dir() {
        panic!("Demo games dir must be a directory!");
    }

    fs::read_dir(dir)
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
        .for_each(|path| {
            let file_name = path
                .file_name()
                .expect("Failed to retrieve filename for demo game")
                .to_str()
                .expect("Failed to convert filename to str");

            let (name, id) = file_name
                .split_once("__")
                .expect("Failed to split game name and id");
            let id = id
                .strip_suffix(".json")
                .expect("Failed to strip .json from filename");

            let json = fs::read_to_string(path.as_path()).expect("Failed to read demo game");
            let events: Vec<(Event, DateTime<Utc>)> =
                serde_json::from_str(&json).unwrap_or_else(|err| {
                    panic!("Failed to deserialize demo game {name} (id: {id:?}), err: {err:?}")
                });

            // Verify that we can re-apply the events to a new game.
            let mut new_game = Game::default();
            for (event, timestamp) in events.into_iter() {
                if let Err(e) = new_game.apply_or_err(event.clone(), timestamp) {
                    panic!("Failed to apply event {event:?} for game {name} due to err {e}");
                }
            }
        });
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
