use std::sync::Arc;

// TODO: maybe make this be not a string...
/// A (per-game) unique ID for a player.
pub type PlayerId = Arc<str>;
