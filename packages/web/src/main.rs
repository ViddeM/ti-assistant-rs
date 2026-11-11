use dioxus::prelude::*;

use game::Game;
use main_menu::MainMenu;
use new_game::NewGame;
use source_code_button::SourceCodeLinkButton;
use ui::{Setup, components::toast::ToastProvider, data::view_mode::ViewMode};

mod game;
mod main_menu;
mod new_game;
mod source_code_button;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        MainMenu,
        #[route("/game/new")]
        NewGame,
        #[route("/game/:id?:view&:playing_as")]
        Game { id: String, view: Option<ViewMode>, playing_as: Option<String> }
}

const FAVICON: Asset = asset!("/assets/favicon.svg");
const MAIN_SCSS: Asset = asset!("/assets/main.scss");
const WASM_PROGRESS_JS: &str = include_str!("wasm_progress.js");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use dioxus::server::axum::Extension;
        use ui::server_side::setup;

        let state = setup().await.expect("failed to setup server");
        let router = dioxus::server::router(App).layer(Extension(state));

        Ok(router)
    })
}

#[component]
fn App() -> Element {
    let mut started = use_signal(|| false);
    use_effect(move || started.set(true));

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_SCSS }
        document::Title { "TI Helper" }
        WasmProgressScript {}

        Setup {}

        ToastProvider { Router::<Route> {} }

        if !started() {
            div { class: "app-loading",
                div { class: "app-loading-bar",
                    div { class: "app-loading-fill" }
                }
                div { class: "app-loading-label" }
            }
        }
    }
}

#[component]
fn WasmProgressScript() -> Element {
    rsx! {
        document::Script { "{WASM_PROGRESS_JS}" }
    }
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        main { class: "main", Outlet::<Route> {} }
        SourceCodeLinkButton {}
    }
}
