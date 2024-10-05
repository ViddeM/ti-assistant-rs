use std::{fs, io::Write, path::PathBuf};

use chrono::{DateTime, Utc};
use clap::Parser;
use eyre::Context;
use ti_helper_db::{db, game_id::GameId, queries};
use ti_helper_game_logic::gameplay::event::Event;

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
pub async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    let opt = Opt::parse();

    color_eyre::install()?;

    if !opt.demo_games_dir.exists() {
        eyre::bail!("DEMO_GAMES_DIR does not exist");
    }

    if !opt.demo_games_dir.is_dir() {
        eyre::bail!("DEMO_GAMES_DIR is not a directory")
    }

    let name = format!("{}__{}", opt.demo_game_name, opt.game_id);
    let new_demo_game_path = opt.demo_games_dir.join(name.clone()).with_extension("json");

    if new_demo_game_path.exists() {
        eyre::bail!("There is already a demo game with the name {}", name);
    }

    let db_pool = db::setup_pool(&opt.database_url)
        .await
        .wrap_err("failed to set up database pool")?;

    // Check that the game exists.
    let _game = queries::get_game_by_id(&db_pool, &opt.game_id)
        .await
        .wrap_err_with(|| format!("Failed to retrieve game with id {}", opt.game_id))?;

    let events = queries::get_events_for_game(&db_pool, &opt.game_id)
        .await
        .wrap_err_with(|| format!("Failed to get events for game with id {}", opt.game_id))?
        .into_iter()
        .map(|game_event| {
            let event: Event =
                serde_json::from_value(game_event.event).wrap_err("Failed to read json value")?;

            Ok((event, game_event.timestamp))
        })
        .collect::<eyre::Result<Vec<(Event, DateTime<Utc>)>>>()?;

    let mut file = fs::File::create(new_demo_game_path.clone()).wrap_err_with(|| {
        format!("Failed to create new demo game file at {new_demo_game_path:?}")
    })?;

    let json = serde_json::to_string_pretty(&events).wrap_err("Failed to serialize events")?;
    file.write_all(json.as_bytes())
        .wrap_err("Failed to write to demo game file")?;

    Ok(())
}
