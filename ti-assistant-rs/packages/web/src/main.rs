use dioxus::prelude::*;

use game::Game;
use main_menu::MainMenu;
use new_game::NewGame;
use source_code_button::SourceCodeLinkButton;
use ui::Setup;

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
        #[route("/game/:id")]
        Game { id: String }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_SCSS: Asset = asset!("/assets/main.scss");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        use dioxus::{
            fullstack::extract::Request,
            server::axum::{self, middleware::Next, Extension},
        };
        use ui::server_side::setup;

        let state = setup().await.expect("failed to setup server");
        let router = dioxus::server::router(App).layer(Extension(state));

        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_SCSS }

        Setup {}

        Router::<Route> {}
    }
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        main { class: "main", Outlet::<Route> {} }
        SourceCodeLinkButton {}
    }
}
