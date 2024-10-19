use std::ops::Deref;
use std::thread;

use chrono::{DateTime, Utc};
use diesel::{
    backend::Backend,
    deserialize::FromSql,
    prelude::{Insertable, Queryable},
    serialize::{self, Output, ToSql},
    sql_types::Text,
    Connection, Selectable,
};
use diesel_async::AsyncPgConnection;
use diesel_async::{
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use eyre::{eyre, Context};

use crate::game_id::GameId;

/// A database pool.
pub type DbPool = Pool<AsyncPgConnection>;

/// The root table for a game.
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::game)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    /// A unique ID for the game.
    pub id: GameId,
    /// The name of the game.
    pub name: String,
}

/// An event that occured in a game.
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::game_event)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GameEvent {
    /// A unique ID for the event.
    pub id: i32,
    /// The game that this event occurred in.
    pub game_id: GameId,
    /// When this event occurred for that game (must be unique for a particular game).
    pub seq: i32,
    /// At what timestamp the event was taken.
    pub timestamp: DateTime<Utc>,
    /// The event information in json format.
    pub event: serde_json::Value,
}

/// The values required to insert a new [GameEvent].
#[derive(Insertable)]
#[diesel(table_name = crate::schema::game_event)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGameEvent {
    /// The ID this event belongs to.
    pub game_id: GameId,
    /// The event information in json format.
    pub event: serde_json::Value,
    /// The timestamp at which the event occurred.
    pub timestamp: DateTime<Utc>,
}

impl<DB: Backend> FromSql<Text, DB> for GameId
where
    String: FromSql<Text, DB>,
    //*const str: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        // having to allocate a string here brings me great sadness
        let s = <String as FromSql<Text, DB>>::from_sql(bytes)?;

        // This is the alternative, diesel-endored way of getting around allocations:
        //let p = <*const str as FromSql<Text, DB>>::from_sql(bytes)?;
        //let s = unsafe { p.as_ref() }.expect("string can't be null");

        Ok(s.parse()?)
    }
}

impl<DB> ToSql<Text, DB> for GameId
where
    DB: Backend,
    str: ToSql<Text, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        <str as ToSql<Text, DB>>::to_sql(self.deref(), out)
    }
}

/// Configure and setup a database pool
pub async fn setup_pool(db_url: &str) -> eyre::Result<DbPool> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(db_url);
    Pool::builder(config)
        .build()
        .wrap_err("create connection pool")
}

/// Run database migrations
pub fn run_migrations(db_url: &str) -> eyre::Result<()> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    // diesel_async doesn't work well with diesel_migrations, so we have to use this
    // AsyncConnectionWrapper weirdness that makes the connection blocking,
    // and we can't do this from within an existing async runtime, so we have to spawn a thread.
    // ...
    // kill me.
    thread::scope(|s| {
        s.spawn(|| {
            let mut db = AsyncConnectionWrapper::<AsyncPgConnection>::establish(db_url)?;
            db.run_pending_migrations(MIGRATIONS)
                .map_err(|e| eyre!("{e}"))?;

            Ok(())
        })
        .join()
        .expect("joining a thread should be fine, right?")
    })
}
