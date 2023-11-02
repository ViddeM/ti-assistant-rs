use diesel::{
    backend::Backend,
    deserialize::FromSql,
    prelude::{Insertable, Queryable},
    serialize::{self, Output, ToSql},
    sql_types::Integer,
    Selectable,
};

use crate::lobby::GameId;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: GameId,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::game_event)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameEvent {
    pub id: i32,
    pub game_id: GameId,
    pub event: serde_json::Value,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::game_event)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGameEvent {
    pub game_id: GameId,
    pub event: serde_json::Value,
}

impl<DB: Backend> FromSql<Integer, DB> for GameId
where
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        <i32 as FromSql<Integer, DB>>::from_sql(bytes).map(|n| GameId(n as u32))
    }
}

impl<DB> ToSql<Integer, DB> for GameId
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        let GameId(id) = self;
        <i32 as ToSql<Integer, DB>>::to_sql(bytemuck::cast_ref(id), out)
    }
}
