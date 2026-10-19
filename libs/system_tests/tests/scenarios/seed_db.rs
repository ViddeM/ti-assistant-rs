//! Writes every scenario into a database, for manually testing them in the UI.
//!
//! ```bash
//! docker compose up -d
//! cargo test -p ti_helper_system_tests --features seed-db --test scenarios seed_database -- --ignored --nocapture
//! ```

use chrono::Utc;
use ti_helper_db::{db, queries};
use ti_helper_game_data::game_id::GameId;

use crate::ALL_SCENARIOS;

#[tokio::test(flavor = "multi_thread")]
#[ignore = "writes all scenarios into DATABASE_URL, run it manually"]
async fn seed_database() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    db::run_migrations(&url)?;
    let pool = db::setup_pool(&url).await?;

    for scenario in ALL_SCENARIOS.iter().flat_map(|scenarios| scenarios.iter()) {
        let game = (scenario.build)();
        // Each scenario always gets the same game id, so re-seeding replaces it.
        let id = GameId::from(fnv1a(scenario.name));
        if queries::try_get_game_by_id(&pool, &id).await?.is_some() {
            queries::delete_all_events_for_game(&pool, &id).await?;
        } else {
            queries::create_game(&pool, id, scenario.name.to_string()).await?;
        }

        // Move the game to the present, so that continuing it doesn't count the time since
        // the scenario's fake clock as turn time.
        let offset = Utc::now() - game.now();
        for (event, timestamp) in game.history() {
            let event = serde_json::to_value(event)?;
            queries::insert_game_event(&pool, id, event, *timestamp + offset).await?;
        }
        println!("{:<45} {}", scenario.name, &*id);
    }
    Ok(())
}

fn fnv1a(name: &str) -> u32 {
    name.bytes().fold(0x811c9dc5, |hash, byte| {
        (hash ^ u32::from(byte)).wrapping_mul(0x01000193)
    })
}
