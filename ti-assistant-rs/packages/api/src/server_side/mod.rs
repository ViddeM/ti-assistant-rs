pub mod gc;
pub mod lobby;
pub mod state;

use dioxus::prelude::*;

use crate::server_side::{lobby::Lobbies, state::State};

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

pub async fn setup() -> eyre::Result<State> {
    color_eyre::install()?;
    pretty_env_logger::init();
    let lobbies = Lobbies::default();
    let mut db_pool = None;

    Ok(State {})
}
