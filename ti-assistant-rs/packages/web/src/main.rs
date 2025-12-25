use dioxus::prelude::*;

mod main_menu;
use main_menu::MainMenu;
mod new_game;
use new_game::NewGame;
mod game;
use game::Game;
use ui::Setup;

use crate::source_code_button::SourceCodeLinkButton;

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
        use dioxus::server::axum::Extension;
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
