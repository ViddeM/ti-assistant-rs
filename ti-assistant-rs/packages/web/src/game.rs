use std::str::FromStr;

use dioxus::{
    fullstack::{use_websocket, WebSocketOptions},
    logger::tracing,
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

    let mut game_state = use_signal(|| None);
    let mut game_options = use_signal(|| None);

    use_future(move || async move {
        while let Ok(message) = socket.recv().await {
            match message {
                ui::WsMessageOut::GameOptions(go) => {
                    tracing::info!("Game options message: {go:?}");
                    game_options.set(Some(go));
                }
                ui::WsMessageOut::GameState(gs) => {
                    tracing::info!("Game state message: {gs:?}");
                    game_state.set(Some(gs));
                }
                ui::WsMessageOut::HandleEventError(err) => {
                    ws_error.set(Some(err));
                }
                ui::WsMessageOut::JoinedGame(game_id) => tracing::info!("Joined game {game_id}"),
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

    if let Some(gs) = game_state.read().as_ref() {
        if let Some(go) = game_options.read().as_ref() {
            return rsx! {
                p { "Finished loading!" }
            };
        }
    }

    rsx! {
        div {
            p { "Loading Game {id}" }
        }
    }
}
