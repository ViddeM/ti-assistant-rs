//! This crate contains all shared UI for the workspace.

#[cfg(feature = "server")]
pub use api::server_side;

pub use api::endpoints;
pub use api::requests;
pub use ti_helper_game_data::*;

pub use api::messages::WsMessageOut;

pub mod data;
pub mod views;
