// @generated automatically by Diesel CLI.

diesel::table! {
    game (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
    }
}

diesel::table! {
    game_event (id) {
        id -> Int4,
        game_id -> Int4,
        event -> Jsonb,
    }
}

diesel::joinable!(game_event -> game (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    game,
    game_event,
);
