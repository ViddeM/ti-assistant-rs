use dioxus::prelude::*;
use std::str::FromStr;
use ui::{data::view_mode::ViewMode, game_id::GameId, views::game::GameView};

use crate::Route;

#[component]
pub fn Game(id: String, view: Option<ViewMode>, playing_as: Option<String>) -> Element {
    let game_id = GameId::from_str(&id);
    let game_id = match game_id {
        Ok(g) => g,
        Err(err) => {
            return rsx! {
                div {
                    p { "Invalid Game ID: {id}" }
                    p { "Err: {err}" }
                    nav {
                        Link {
                            to: Route::MainMenu, "Back to main menu"
                        }
                    }
                    Outlet::<Route> {}
                }
            };
        }
    };

    rsx! {
        GameView {
            game_id,
            view,
            playing_as,
            main_menu: || Route::MainMenu,
            game_route: move |(view, playing_as)| Route::Game {
                id: id.clone(),
                view,
                playing_as,
            },
        }
    }
}
