#![forbid(unsafe_code)]
#![allow(dead_code)]

use example_game::play_example_game;

#[macro_use]
extern crate rocket;

pub mod api;
pub mod data;
pub mod example_game;
pub mod game;
pub mod phases;
pub mod player;

#[launch]
async fn rocket() -> _ {
    let game = play_example_game();

    rocket::build()
        .mount(
            "/api/",
            routes![
                api::game_options::get_game_options,
                api::game::get_example_game
            ],
        )
        .manage(game)
}
