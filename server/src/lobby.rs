use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use rand::random;
use serde::{de::Error, Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::Deref,
    str::{self, FromStr},
    sync::Arc,
};
use tokio::sync::{broadcast, RwLock};

use crate::gameplay::{game::Game, game_state::GameState};

/// A game ID, which is always an 8 character hexadecimal string.
///
/// You can create a [GameId] by calling `.parse` on a string.
///
/// # Invariant
///
/// The inner `[u8; 8]` must always be a valid 8-character hexadecimal string.
/// This is enforced through the FromStr impl, which is the only valid way to create a GameId.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct GameId([u8; 8]);

#[derive(Default)]
pub struct Lobbies {
    pub list: RwLock<HashMap<GameId, Arc<RwLock<Lobby>>>>,
}

pub struct Lobby {
    /// The Game State
    pub game: Game,

    /// Broadcaster to send GameState updates to all websocket clients.
    pub state_updates: broadcast::Sender<Arc<GameState>>,
}

impl GameId {
    pub fn random() -> Self {
        GameId(random())
    }
}

impl Deref for GameId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.0).expect("GameId must always be a hexadecimal str")
    }
}

impl FromStr for GameId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: &[u8; 8] = s
            .as_bytes()
            .try_into()
            .map_err(|_| "GameId must be exactly 8 bytes long")?;

        if s.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err("GameId must be a hexadecimal string");
        }

        let _parsed = u32::from_str_radix(s, 16)
            .map_err(|_| "failed to parse game id, expected hex string of length 8")?;

        Ok(GameId(*id))
    }
}

impl Lobby {
    pub fn new(game: Game) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            game,
            state_updates: broadcast::channel(100).0,
        }))
    }
}

impl Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

impl Debug for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

pub fn generate_game_name(_id: GameId) -> String {
    // TODO
    "Funny Game".into()
}
