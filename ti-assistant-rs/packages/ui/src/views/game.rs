use std::sync::Arc;

use api::{
    endpoints::join_game,
    messages::{WsMessageIn, WsMessageOut},
};
use dioxus::{
    fullstack::{use_websocket, WebSocketOptions},
    logger::tracing,
    prelude::*,
};
use ti_helper_game_data::{
    components::phase::Phase, game_id::GameId, game_options::GameOptions,
    state::game_state::GameState,
};

use crate::{
    components::{button::Button, info_modal::InfoModal, spinner::Spinner},
    data::{
        event_context::EventContext, game_context::GameContext, info_context::InfoContext,
        player_view::PlayerViewContext, view_mode::ViewMode,
    },
    views::{
        info_box::InfoBox, phase_view::PhaseView, players_sidebar::PlayersSidebar,
        score_view_mode::ScoreViewMode, tech_view_mode::TechViewMode,
    },
};

const GAME_SCSS: Asset = asset!("/assets/styling/views/game.scss");

#[component]
pub fn GameView(game_id: GameId) -> Element {
    use_context_provider(|| game_id);
    let mut socket = use_websocket(move || {
        join_game(game_id, WebSocketOptions::new().with_automatic_reconnect())
    });
    let mut ws_error = use_signal(|| None);

    let send_event = use_callback(move |msg: WsMessageIn| {
        spawn(async move {
            if let Err(err) = socket.send(msg).await {
                ws_error.set(Some(format!("Failed to send event {err}")));
            };
        });
    });

    use_context_provider(|| EventContext::new(send_event));

    let mut game_state = use_signal(|| None);
    let mut game_options = use_signal(|| None);

    use_future(move || async move {
        while let Ok(message) = socket.recv().await {
            match message {
                WsMessageOut::GameOptions(go) => {
                    tracing::info!("Game options message: {go:?}");
                    game_options.set(Some(go));
                }
                WsMessageOut::GameState(gs) => {
                    tracing::info!("Game state message: {gs:?}");
                    game_state.set(Some(gs));
                }
                WsMessageOut::HandleEventError(err) => {
                    ws_error.set(Some(err));
                }
                WsMessageOut::JoinedGame(game_id) => tracing::info!("Joined game {game_id}"),
                WsMessageOut::NotFound(game_id) => ws_error.set(Some(game_id.to_string())),
            }
        }
    });

    if let Some(err) = ws_error() {
        return rsx! {
            div { class: "card column",
                h2 { "Invalid event" }
                p { "Error: {err}" }
                Button {
                    onclick: move |_| async move {
                        let new_socket = join_game(
                                game_id,
                                WebSocketOptions::new().with_automatic_reconnect(),
                            )
                            .await;
                        socket.set(new_socket);
                    },
                    "Reload Game"
                }
            }
        };
    }

    if let Some(gs) = game_state.read().as_ref() {
        if let Some(go) = game_options.read().as_ref() {
            return rsx! {
                MainGameView { game_options: Arc::clone(go), game_state: Arc::clone(gs) }
            };
        }
    }

    rsx! {
        "Game loading..."
        Spinner {}
    }
}

#[component]
fn MainGameView(
    game_options: ReadSignal<Arc<GameOptions>>,
    game_state: ReadSignal<Arc<GameState>>,
) -> Element {
    use_context_provider(|| GameContext::new(game_state, game_options));
    use_context_provider(|| PlayerViewContext::new(game_state));

    let info = use_signal(|| None);
    use_context_provider(|| InfoContext::new(info));

    let view_mode = use_signal(|| ViewMode::Game);

    rsx! {
        document::Stylesheet { href: GAME_SCSS }
        InfoModal {}
        InfoBox { view_mode }
        DisplayViewMode { view_mode }
    }
}

#[component]
fn DisplayViewMode(view_mode: ReadSignal<ViewMode>) -> Element {
    let mode = *view_mode.read();
    let gc = use_context::<GameContext>();
    let show_sidebar = use_memo(move || match gc.game_state().phase {
        Phase::Creation | Phase::Setup => false,
        _ => true,
    });

    let sidebar = if show_sidebar() {
        rsx! {
            PlayersSidebar {}
        }
    } else {
        rsx! {}
    };

    match mode {
        ViewMode::Game => rsx! {
            div { class: "game-page-container",
                {sidebar}
                div { class: "phase-container", PhaseView {} } // TODO: Sidebar should also be in here.
            }
        },
        ViewMode::Score => rsx! {
            ScoreViewMode {}
        },
        ViewMode::Techs => rsx! {
            TechViewMode {}
        },
        ViewMode::Planets => rsx! { "Planets" },
        ViewMode::Laws => rsx! { "Laws" },
        ViewMode::Map => rsx! { "Map" },
    }
}
