use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Clone)]
pub struct Opts {
    #[clap(long, env = "BIND_HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[clap(long, env = "BIND_PORT", default_value = "5555")]
    pub port: u16,

    /// Postgres URI
    #[clap(long = "db", env = "DATABASE_URL")]
    pub database_url: Option<String>,

    /// Weather or not the demo games should overwrite any existing games with the same ID in the DB.
    #[clap(long, env = "OVERWRITE_DB_DEMO_GAMES")]
    pub overwrite_db_games: bool,

    /// Weather or not to insert demo games into the DB at startup.
    #[clap(long, env = "DEMO_GAMES_SKIP_DB")]
    pub demo_games_skip_db: bool,

    /// The directory of the demo games.
    #[clap(long, env = "DEMO_GAMES_DIR")]
    pub demo_games_dir: PathBuf,

    /// Automatically run database migrations, if needed.
    #[clap(short, long, env = "MIGRATE_DB", requires("database_url"))]
    pub migrate: bool,

    /// Cron string that defines when to unload inactive games from memory.
    ///
    /// Format: "sec min hour day_of_month month day_of_week year"
    #[clap(long, env = "MEM_GC_CRON")]
    pub mem_gc_cron: Option<String>,
}
