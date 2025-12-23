use dioxus::prelude::*;
use ui::components::button::Button;

use crate::Route;

#[component]
pub fn MainMenu() -> Element {
    let mut game_id = use_signal(|| "".to_string());

    rsx! {
        div {
            Link { to: Route::NewGame,
                Button { "New Game" }
            }

            input {
                placeholder: "Game ID",
                value: game_id(),
                oninput: move |evt| {
                    let value = evt.value();
                    let prev = game_id();
                    let value = if value.chars().count() > 8 { prev } else { value };
                    game_id.set(value);
                },
            }

            Link { to: Route::Game { id: game_id() },
                Button { disabled: game_id().len() != 8, "Join Game" }
            }
        }
    }
}
