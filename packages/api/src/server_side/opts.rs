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

    /// Automatically run database migrations, if needed.
    #[clap(short, long, env = "MIGRATE_DB", requires("database_url"))]
    pub migrate: bool,

    /// Cron string that defines when to unload inactive games from memory.
    ///
    /// Format: "sec min hour day_of_month month day_of_week year"
    #[clap(long, env = "MEM_GC_CRON")]
    pub mem_gc_cron: Option<String>,
}
