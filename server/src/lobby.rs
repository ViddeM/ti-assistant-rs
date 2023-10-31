use rand::random;
use serde::{de::Error, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::sync::{broadcast, RwLock};

use crate::game::{Game, GameState};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameId(pub u32);

#[derive(Default)]
pub struct Lobbies {
    pub list: RwLock<HashMap<GameId, Arc<Lobby>>>,
}

pub struct Lobby {
    pub game: RwLock<Game>,
    pub state_updates: broadcast::Sender<GameState>,
}

impl GameId {
    pub fn random() -> Self {
        GameId(random())
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
