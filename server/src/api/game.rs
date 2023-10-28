use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

use crate::{
    data::components::system::{systems, System},
    game::{Game, GameState},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    game_state: GameState,
    systems: Vec<System>,
}

#[get("/game/example")]
pub fn get_example_game(game: &State<Game>) -> Json<GameResponse> {
    let game = game.inner().clone();
    Json(GameResponse {
        game_state: game.current,
        systems: systems().into_iter().map(|(_, system)| system).collect(),
    })
}
