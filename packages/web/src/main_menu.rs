use dioxus::prelude::*;
use ui::views;

use crate::{Route, game::GameQuery};

#[component]
pub fn MainMenu() -> Element {
    rsx! {
        views::main_menu::MainMenu {
            new_game: || Route::NewGame,
            join_game: |game_id| Route::Game {
                id: game_id,
                query: GameQuery::default(),
            },
        }
    }
}
