#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

pub mod api;
pub mod data;
pub mod game;
pub mod phases;
pub mod player;

use api::game_options;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api/", routes![game_options::get_game_options])
}
