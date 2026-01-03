use std::{fmt::Display, ops::Deref};

use diesel::{deserialize::FromSqlRow, expression::AsExpression};
use ti_helper_game_data::game_id::GameId;

/// Database compatible wrapper for the GameId.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct DBGameId(GameId);

impl From<&GameId> for DBGameId {
    fn from(value: &GameId) -> Self {
        Self(*value)
    }
}

impl From<GameId> for DBGameId {
    fn from(value: GameId) -> Self {
        Self(value)
    }
}

impl From<&DBGameId> for GameId {
    fn from(value: &DBGameId) -> Self {
        value.0
    }
}

impl From<DBGameId> for GameId {
    fn from(value: DBGameId) -> Self {
        value.0
    }
}

impl Deref for DBGameId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Display for DBGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
