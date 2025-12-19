use ti_helper_db::db::DbPool;

use crate::server_side::{lobby::Lobbies, opts::Opts};

/// Global shared state
#[derive(Clone)]
pub struct State {
    /// Program arguments
    pub opt: Opts,

    /// Index of running and loaded games
    pub lobbies: Lobbies,

    /// Database pool (if any)
    pub db_pool: Option<DbPool>,
}
