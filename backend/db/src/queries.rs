use chrono::{DateTime, Utc};
use diesel::{delete, insert_into, ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::{AsyncConnection, RunQueryDsl};
use eyre::Context;

use crate::{
    db::{self, DbPool},
    game_id::GameId,
};

/// Insert a new game with the given `id` and `name`.
pub async fn create_game(db_pool: &DbPool, id: GameId, name: String) -> eyre::Result<()> {
    let mut db = db_pool.get().await?;

    use crate::schema::game::dsl;
    insert_into(dsl::game)
        .values(&db::Game { id, name })
        .execute(&mut db)
        .await?;

    Ok(())
}

/// Retrieves the game with the provided game id.
pub async fn get_game_by_id(db_pool: &DbPool, id: &GameId) -> eyre::Result<db::Game> {
    let mut db = db_pool.get().await?;

    use crate::schema::game::dsl;
    let game: db::Game = dsl::game.filter(dsl::id.eq(id)).get_result(&mut db).await?;
    Ok(game)
}

/// Try to get a game by its id, returns None if the game doesn't exist in the DB.
pub async fn try_get_game_by_id(db_pool: &DbPool, id: &GameId) -> eyre::Result<Option<db::Game>> {
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
pub async fn delete_all_events_for_game(db_pool: &DbPool, id: &GameId) -> eyre::Result<()> {
    let mut db = db_pool.get().await?;

    use crate::schema::game_event::dsl;
    delete(dsl::game_event)
        .filter(dsl::game_id.eq(id))
        .execute(&mut db)
        .await?;

    Ok(())
}

/// Returns a list of all the events for the game with provided id.
pub async fn get_events_for_game(
    db_pool: &DbPool,
    id: &GameId,
) -> eyre::Result<Vec<db::GameEvent>> {
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
) -> eyre::Result<()> {
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
pub async fn delete_latest_event_for_game(db_pool: &DbPool, id: &GameId) -> eyre::Result<()> {
    let mut db = db_pool.get().await?;
    db.transaction(|db| {
        Box::pin(async move {
            use crate::schema::game_event::dsl;

            // query the last event for this game
            let last_event_id: Option<i32> = dsl::game_event
                .filter(dsl::game_id.eq(id))
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
                .wrap_err_with(|| format!("error querying game events ({id:?})"))
                // sanity check that we deleted exactly one event
                .map(|count| debug_assert_eq!(count, 1))
        })
    })
    .await?;

    Ok(())
}
