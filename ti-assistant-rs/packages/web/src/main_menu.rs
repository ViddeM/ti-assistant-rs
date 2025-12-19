use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn MainMenu() -> Element {
    rsx! {
        div {
            Link { to: Route::NewGame,
                button { "New Game" }
            }
            input { placeholder: "Game ID" }
            button { "Join Game" }
        }
    }
}
