use dioxus::prelude::*;

use game::Game;
use main_menu::MainMenu;
use new_game::NewGame;
use source_code_button::SourceCodeLinkButton;
use ui::{
    Setup,
    components::{spinner::Spinner, toast::ToastProvider},
};

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

const FAVICON: Asset = asset!("/assets/favicon.svg");
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
    let mut started = use_signal(|| false);
    use_effect(move || started.set(true));

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_SCSS }
        document::Title { "TI Helper" }

        Setup {}

        ToastProvider { Router::<Route> {} }

        if !started() {
            div { class: "app-loading", Spinner {} }
        }
    }
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        main { class: "main", Outlet::<Route> {} }
        SourceCodeLinkButton {}
    }
}
