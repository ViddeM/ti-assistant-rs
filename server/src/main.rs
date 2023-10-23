#![forbid(unsafe_code)]
#[macro_use]
extern crate rocket;

pub mod data;
pub mod game;
pub mod phases;
pub mod player;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/api/core", core_routes())
}
