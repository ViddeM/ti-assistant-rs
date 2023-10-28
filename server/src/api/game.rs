use rocket::{serde::json::Json, State};

use crate::game::Game;

#[get("/game/example")]
pub fn get_example_game(game: &State<Game>) -> Json<Game> {
    let game = game.inner().clone();
    Json(game)
}
