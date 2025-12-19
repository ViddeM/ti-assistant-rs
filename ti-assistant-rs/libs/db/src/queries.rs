use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, delete, insert_into};
use diesel_async::{AsyncConnection, RunQueryDsl};
use ti_helper_game_data::game_id::GameId;

use crate::{
    db::{self, DbPool},
    error::{DbError, DbResult},
    game_id::DBGameId,
};

/// Insert a new game with the given `id` and `name`.
pub async fn create_game(db_pool: &DbPool, id: GameId, name: String) -> DbResult<()> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;

    use crate::schema::game::dsl;
    insert_into(dsl::game)
        .values(&db::Game {
            id: id.into(),
            name,
        })
        .execute(&mut db)
        .await?;

    Ok(())
}

/// Get a list of ALL [GameId]s in the database. Use with care.
pub async fn get_all_game_ids(db_pool: &DbPool) -> DbResult<Vec<DBGameId>> {
    let mut db = db_pool.get().await?;

    use crate::schema::game::dsl;
    Ok(dsl::game.select(dsl::id).load(&mut db).await?)
}

/// Retrieves the game with the provided game id.
pub async fn get_game_by_id(db_pool: &DbPool, id: &GameId) -> DbResult<db::Game> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;

    use crate::schema::game::dsl;
    let game: db::Game = dsl::game.filter(dsl::id.eq(id)).get_result(&mut db).await?;
    Ok(game)
}

/// Try to get a game by its id, returns None if the game doesn't exist in the DB.
pub async fn try_get_game_by_id(db_pool: &DbPool, id: &GameId) -> DbResult<Option<db::Game>> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;

    use crate::schema::game::dsl;
    let game: Option<db::Game> = dsl::game
        .filter(dsl::id.eq(id))
        .first(&mut db)
        .await
        .optional()?;

    Ok(game)
}

/// Deletes all games associated with the game with the provided id.
pub async fn delete_all_events_for_game(db_pool: &DbPool, id: &GameId) -> DbResult<()> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;

    use crate::schema::game_event::dsl;
    delete(dsl::game_event)
        .filter(dsl::game_id.eq(id))
        .execute(&mut db)
        .await?;

    Ok(())
}

/// Returns a list of all the events for the game with provided id.
pub async fn get_events_for_game(db_pool: &DbPool, id: &GameId) -> DbResult<Vec<db::GameEvent>> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;

    use crate::schema::game_event::dsl;
    let events: Vec<db::GameEvent> = dsl::game_event
        .filter(dsl::game_id.eq(id))
        .order_by(dsl::seq)
        .load(&mut db)
        .await?;

    Ok(events)
}

/// Insert a game event for a game.
pub async fn insert_game_event(
    db_pool: &DbPool,
    id: GameId,
    event: serde_json::Value,
    timestamp: DateTime<Utc>,
) -> DbResult<()> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;

    use crate::schema::game_event::dsl::game_event;
    insert_into(game_event)
        .values(&db::NewGameEvent {
            game_id: id,
            event,
            timestamp,
        })
        .execute(&mut db)
        .await?;

    Ok(())
}

/// Deletes the latest event for the game with the provided [GameId].
pub async fn delete_latest_event_for_game(db_pool: &DbPool, id: &GameId) -> DbResult<()> {
    let id: DBGameId = id.into();
    let mut db = db_pool.get().await?;
    db.transaction(|db| {
        Box::pin(async move {
            use crate::schema::game_event::dsl;

            // query the last event for this game
            let last_event_id: Option<i32> = dsl::game_event
                .filter(dsl::game_id.eq(&id))
                .order_by(dsl::seq.desc())
                .select(dsl::id)
                .first(db)
                .await
                .optional()?;

            let Some(last_event_id) = last_event_id else {
                return Ok(());
            };

            delete(dsl::game_event)
                .filter(dsl::id.eq(last_event_id))
                .execute(db)
                .await
                .map_err(|err| DbError::DeleteGameError {
                    game_id: id.clone(),
                    error: err.to_string(),
                })
                // sanity check that we deleted exactly one event
                .map(|count| debug_assert_eq!(count, 1))
        })
    })
    .await?;

    Ok(())
}
