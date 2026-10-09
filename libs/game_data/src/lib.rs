// Common types.
/// Actions that can be taken by a player.
pub mod actions;
pub mod common;
/// Game components
pub mod components;
/// Errors that can occur in the game data operations.
pub mod error;
/// The ID for a game.
pub mod game_id;
/// All game information
// TODO: Maybe we don't need this in a full Rust setup?
pub mod game_options;
/// A state of a game.
pub mod state;
