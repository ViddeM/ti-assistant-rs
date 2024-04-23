#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![allow(clippy::single_match)]

//! Gameplay for a game of TI4.

/// General game data.
pub mod data;
/// HashMap wrapper type for generating typescript bindings.
pub mod enum_map;
/// All the general game information.
pub mod game_options;
/// Game logic.
pub mod gameplay;
