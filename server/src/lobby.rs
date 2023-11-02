use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use rand::random;
use serde::{de::Error, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::sync::{broadcast, RwLock};

use crate::game::{Game, GameState};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub struct GameId(pub u32);

#[derive(Default)]
pub struct Lobbies {
    pub list: RwLock<HashMap<GameId, Arc<RwLock<Lobby>>>>,
}

pub struct Lobby {
    /// The Game State
    pub game: Game,

    /// Broadcaster to send GameState updates to all websocket clients.
    pub state_updates: broadcast::Sender<GameState>,
}

impl GameId {
    pub fn random() -> Self {
        GameId(random())
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
        format!("{:08x}", self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        let id = u32::from_str_radix(s, 16)
            .map_err(|_| D::Error::custom("failed to parse game id, expected hex string"))?;
        Ok(Self(id))
    }
}

impl Debug for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x}", self.0)
    }
}

pub fn generate_game_name(_id: GameId) -> String {
    // TODO
    "Funny Game".into()
}
