use dioxus::prelude::*;
use std::{fmt::Display, str::FromStr};
use ui::{data::view_mode::ViewMode, game_id::GameId, views::game::GameView};

use crate::Route;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GameQuery {
    pub view: Option<ViewMode>,
    pub playing_as: Option<String>,
}

impl From<&str> for GameQuery {
    fn from(query: &str) -> Self {
        let mut game_query = Self::default();
        for (key, value) in query.split('&').filter_map(|s| s.split_once('=')) {
            match key {
                "view" => game_query.view = value.parse().ok(),
                "playing_as" => game_query.playing_as = Some(value.to_string()),
                _ => {}
            }
        }
        game_query
    }
}

impl Display for GameQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = [
            self.view.map(|view| format!("view={view}")),
            self.playing_as
                .as_ref()
                .map(|playing_as| format!("playing_as={playing_as}")),
        ]
        .into_iter()
        .flatten()
        .collect();
        write!(f, "{}", params.join("&"))
    }
}

#[component]
pub fn Game(id: String, query: GameQuery) -> Element {
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
            view: query.view,
            playing_as: query.playing_as,
            main_menu: || Route::MainMenu,
            game_route: move |(view, playing_as)| Route::Game {
                id: id.clone(),
                query: GameQuery { view, playing_as },
            },
        }
    }
}
