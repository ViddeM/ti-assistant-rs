#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::net::TcpListener;

use data::{common::faction::Faction, components::system::System};
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::{
    data::components::system::systems,
    game::{Event, Game},
    websocket_client::WsClient,
    ws_message::{GameOptions, WsMessage},
};

#[macro_use]
extern crate rocket;

pub mod api;
pub mod data;
pub mod example_game;
pub mod game;
pub mod phases;
pub mod player;
pub mod websocket_client;
pub mod ws_message;

#[tokio::main]
pub async fn main() {
    let host = format!("0.0.0.0:5555");
    println!("Setting up websocket on host {host}");

    let server = TcpListener::bind(host).expect("Failed to setup TCP listener");
    server
        .set_nonblocking(false)
        .expect("Failed to set TCP listener non-blocking");

    let (stream, _) = server.accept().expect("Failed to accept client");
    let mut ws_client = WsClient::accept(stream);
    let mut game = Game::new(vec![]);

    ws_client.send_message(&WsMessage::game_options());
    ws_client.send_message(&WsMessage::game_state(game.current.clone()));

    loop {
        if let Some(event) = ws_client.receive_message::<Event>() {
            game.apply(event);
            ws_client.send_message(&WsMessage::game_state(game.current.clone()));
        }
    }
}
