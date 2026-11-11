use dioxus::prelude::*;
use ui::views;

use crate::{Route, game::GameQuery};

#[component]
pub fn NewGame() -> Element {
    rsx! {
        views::new_game::NewGame {
            join_game: |game_id| Route::Game {
                id: game_id,
                query: GameQuery::default(),
            },
        }
    }
}
