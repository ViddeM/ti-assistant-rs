use std::{fs, path::PathBuf};

use anyhow::Context;
use chrono::{DateTime, Utc};
use clap::Parser;
use ti_helper_db::{db, queries};
use ti_helper_game_data::{actions::event::Event, game_id::GameId};

#[derive(Parser)]
pub struct Opt {
    /// The GameId of the game to create a demo game from.
    game_id: GameId,

    /// The name of the new demo game.
    demo_game_name: String,

    #[clap(long, env = "DEMO_GAMES_DIR")]
    demo_games_dir: PathBuf,

    /// Postgres URI
    #[clap(long = "db", env = "DATABASE_URL")]
    database_url: String,
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let opt = Opt::parse();

    if !opt.demo_games_dir.exists() {
        anyhow::bail!("DEMO_GAMES_DIR does not exist");
    }

    if !opt.demo_games_dir.is_dir() {
        anyhow::bail!("DEMO_GAMES_DIR is not a directory");
    }

    let name = format!("{}__{}", opt.demo_game_name, opt.game_id);
    let new_demo_game_path = opt.demo_games_dir.join(&name).with_extension("json");

    if new_demo_game_path.exists() {
        anyhow::bail!("There is already a demo game with the name {name}");
    }

    let db_pool = db::setup_pool(&opt.database_url)
        .await
        .context("failed to set up database pool")?;

    // Check that the game exists.
    queries::get_game_by_id(&db_pool, &opt.game_id)
        .await
        .with_context(|| format!("Failed to retrieve game with id {}", opt.game_id))?;

    let events = queries::get_events_for_game(&db_pool, &opt.game_id)
        .await
        .with_context(|| format!("Failed to get events for game with id {}", opt.game_id))?
        .into_iter()
        .map(|game_event| {
            let event: Event =
                serde_json::from_value(game_event.event).context("Failed to read json value")?;

            Ok((event, game_event.timestamp))
        })
        .collect::<anyhow::Result<Vec<(Event, DateTime<Utc>)>>>()?;

    let json = serde_json::to_string_pretty(&events).context("Failed to serialize events")?;
    fs::write(&new_demo_game_path, json).with_context(|| {
        format!("Failed to write demo game file at {new_demo_game_path:?}")
    })?;

    println!("Created demo game {new_demo_game_path:?}");

    Ok(())
}
