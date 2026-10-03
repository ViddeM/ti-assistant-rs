pub mod gc;
pub mod insert_demo_games;
pub mod lobby;
pub mod opts;
pub mod state;

use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use ti_helper_db::{
    db::{self, DbPool},
    queries,
};

use crate::server_side::{lobby::Lobbies, opts::Opts, state::State};

pub async fn setup() -> anyhow::Result<Arc<State>> {
    dotenvy::dotenv().ok();
    let opt = Opts::parse();

    pretty_env_logger::init();

    let lobbies = Lobbies::default();
    let mut db_pool = None;

    if let Some(db_url) = &opt.database_url {
        if opt.migrate {
            db::run_migrations(db_url).context("failed to run migrations")?;
        }

        db_pool = Some(
            db::setup_pool(db_url)
                .await
                .context("failed to set up database pool")?,
        )
    }

    if !opt.demo_games_skip_db {
        let Some(db_pool) = &db_pool else {
            anyhow::bail!("DEMO_GAMES_SKIP_DB is not set but no DB has been configured")
        };

        insert_demo_games::insert_demo_games(&opt, db_pool).await?;
    }

    if cfg!(debug_assertions) {
        if let Some(db_pool) = &db_pool {
            print_all_games(db_pool)
                .await
                .context("failed to print games")?;
        }
    }

    let shared = Arc::new(State {
        opt,
        lobbies,
        db_pool,
    });

    gc::setup_game_gc(&shared)?;

    Ok(shared)
}

async fn print_all_games(pool: &DbPool) -> anyhow::Result<()> {
    let game_ids = match queries::get_all_game_ids(pool)
        .await
        .context("Failed to query list of GameIds")
    {
        Ok(game_ids) => game_ids,
        Err(e) => {
            log::error!("{e:#}");
            return Err(e);
        }
    };

    if game_ids.is_empty() {
        log::info!("No games in database");
    } else {
        log::info!("Games in database:");
        for game_id in game_ids {
            log::info!("{game_id}");
        }
    }

    Ok(())
}
