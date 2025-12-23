use dioxus::prelude::*;
use std::str::FromStr;
use ui::{game_id::GameId, views::game::GameView};

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

    rsx! {
        div {
            GameView { game_id }
        }
    }
}
