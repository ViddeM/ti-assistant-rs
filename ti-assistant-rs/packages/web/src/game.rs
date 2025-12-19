use dioxus::prelude::*;

#[component]
pub fn Game(id: String) -> Element {
    rsx! {
        div {
            p { "Game {id}" }
        }
    }
}
