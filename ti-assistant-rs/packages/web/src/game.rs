use std::str::FromStr;

use dioxus::{
    fullstack::{use_websocket, WebSocketOptions},
    prelude::*,
};
use ui::{endpoints::join_game, game_id::GameId};

#[component]
pub fn Game(id: String) -> Element {
    let game_id = GameId::from_str(&id);
    let game_id = match game_id {
        Ok(g) => g,
        Err(err) => {
            return rsx! {
                div {
                    p { "Invalid Game ID: {id}" }
                    p { "Err: {err}" }
                }
            }
        }
    };

    let mut socket = use_websocket(move || join_game(game_id, WebSocketOptions::new()));

    let mut ws_error = use_signal(|| None);

    use_future(move || async move {
        while let Ok(message) = socket.recv().await {
            match message {
                ui::WsMessageOut::GameOptions(game_options) => todo!(),
                ui::WsMessageOut::GameState(game_state) => todo!(),
                ui::WsMessageOut::HandleEventError(_) => todo!(),
                ui::WsMessageOut::JoinedGame(game_id) => todo!(),
                ui::WsMessageOut::NotFound(game_id) => ws_error.set(Some(game_id.to_string())),
            }
        }
    });

    if let Some(err) = ws_error() {
        return rsx! {
            div {
                p { "Game Err: {err}" }
            }
        };
    }

    rsx! {
        div {
            p { "Game {id}" }
        }
    }
}
