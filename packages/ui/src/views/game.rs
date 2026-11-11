use std::{sync::Arc, time::Duration};

use api::{
    endpoints::join_game,
    messages::{WsMessageIn, WsMessageOut},
};
use dioxus::{
    fullstack::{WebSocketOptions, use_websocket},
    logger::tracing,
    prelude::*,
};
use dioxus_primitives::toast::{ToastOptions, use_toast};
use dioxus_sdk::time::sleep;
use ti_helper_game_data::{
    components::phase::Phase, game_id::GameId, game_options::GameOptions,
    state::game_state::GameState,
};

use crate::{
    components::{button::Button, info_modal::InfoModal, spinner::Spinner},
    data::{
        event_context::EventContext,
        game_context::GameContext,
        info_context::InfoContext,
        player_view::{PlayerView, PlayerViewContext},
        view_mode::{ViewMode, ViewModeContext},
    },
    views::{
        galactic_events_view_mode::GalacticEventsViewMode, history_view_mode::HistoryViewMode,
        info_box::InfoBox, laws_view_mode::LawsViewMode, map_view_mode::MapViewMode,
        phase_view::PhaseView, planet_view_mode::PlanetViewMode, players_sidebar::PlayersSidebar,
        relic_view_mode::RelicViewMode, score_view_mode::ScoreViewMode,
        tech_view_mode::TechViewMode, thunders_edge_view_mode::ThundersEdgeViewMode,
    },
};

const GAME_SCSS: Asset = asset!("/assets/styling/views/game.scss");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GameViewProps<R>
where
    R: PartialEq + Clone + 'static + Routable,
{
    game_id: GameId,
    view: ReadSignal<Option<ViewMode>>,
    playing_as: ReadSignal<Option<String>>,
    main_menu: Callback<(), R>,
    game_route: Callback<(Option<ViewMode>, Option<String>), R>,
}

#[derive(Debug, Clone, PartialEq, Props)]
struct BackToMainMenuButtonProps<R>
where
    R: PartialEq + Clone + 'static + Routable,
{
    main_menu: Callback<(), R>,
}

#[component]
fn BackToMainMenuButton<R: PartialEq + Clone + 'static + Routable>(
    BackToMainMenuButtonProps { main_menu }: BackToMainMenuButtonProps<R>,
) -> Element {
    let nav = navigator();

    rsx! {
        Button {
            onclick: move |_| {
                nav.push(main_menu(()));
            },
            "Back to main menu"
        }
    }
}

#[component]
pub fn GameView<R: PartialEq + Clone + 'static + Routable>(
    GameViewProps {
        game_id,
        view,
        playing_as,
        main_menu,
        game_route,
    }: GameViewProps<R>,
) -> Element {
    use_context_provider(|| game_id);
    let nav = navigator();
    let set_view = use_callback(move |playing_as: Option<String>| {
        nav.replace(game_route((*view.peek(), playing_as)));
    });
    let set_view_mode = use_callback(move |mode: ViewMode| {
        nav.replace(game_route((
            (mode != ViewMode::Game).then_some(mode),
            playing_as.peek().clone(),
        )));
    });
    let mut socket = use_websocket(move || {
        join_game(game_id, WebSocketOptions::new().with_automatic_reconnect())
    });
    let mut ws_error = use_signal(|| None);
    let mut not_found = use_signal(|| false);
    let toasts = use_toast();
    let mut awaiting_reply = use_signal(|| false);

    let send_event = use_callback(move |msg: WsMessageIn| {
        awaiting_reply.set(true);
        spawn(async move {
            if let Err(err) = socket.send(msg).await {
                ws_error.set(Some(format!("Failed to send event {err}")));
            };
        });
    });

    use_context_provider(|| EventContext::new(send_event));

    let mut game_state = use_signal(|| None);
    let mut game_options = use_signal(|| None);
    let loaded = use_memo(move || game_state.read().is_some() && game_options.read().is_some());

    use_future(move || async move {
        loop {
            while let Ok(message) = socket.recv().await {
                match message {
                    WsMessageOut::GameOptions(go) => {
                        // tracing::info!("Game options message: {go:?}");
                        game_options.set(Some(go));
                    }
                    WsMessageOut::GameState(gs) => {
                        // tracing::info!("Game state message: {gs:?}");
                        awaiting_reply.set(false);
                        game_state.set(Some(gs));
                    }
                    WsMessageOut::HandleEventError(err) => {
                        awaiting_reply.set(false);
                        ws_error.set(Some(err));
                    }
                    WsMessageOut::JoinedGame(game_id) => {
                        tracing::info!("Joined game {game_id}");
                        if *awaiting_reply.peek() {
                            awaiting_reply.set(false);
                            toasts
                                .error("Disconnected, try again".to_string(), ToastOptions::new());
                        }
                    }
                    WsMessageOut::NotFound(game_id) => {
                        ws_error.set(Some(format!("Game {game_id} not found")));
                        not_found.set(true);
                    }
                    WsMessageOut::UndoRejected => {
                        awaiting_reply.set(false);
                        toasts.error(
                            "The last event changed, nothing was undone".to_string(),
                            ToastOptions::new(),
                        );
                    }
                }
            }

            if *not_found.peek() {
                return;
            }

            sleep(Duration::from_secs(1)).await;
            socket.set(join_game(game_id, WebSocketOptions::new()).await);
        }
    });

    if let Some(err) = ws_error() {
        return rsx! {
            div { class: "card column",
                h2 { class: "margin-top", "Something went wrong" }
                p { class: "margin-top", "Error: {err}" }
                if !not_found() {
                    Button {
                        class: "margin-top",
                        onclick: move |_| async move {
                            ws_error.set(None);
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
                BackToMainMenuButton { main_menu }
            }
        };
    }

    if loaded() {
        return rsx! {
            MainGameView {
                game_options,
                game_state,
                view,
                playing_as,
                set_view,
                set_view_mode,
            }
        };
    }

    rsx! {
        "Game loading..."
        Spinner {}
    }
}

#[component]
fn MainGameView(
    game_options: ReadSignal<Option<Arc<GameOptions>>>,
    game_state: ReadSignal<Option<Arc<GameState>>>,
    view: ReadSignal<Option<ViewMode>>,
    playing_as: ReadSignal<Option<String>>,
    set_view: Callback<Option<String>>,
    set_view_mode: Callback<ViewMode>,
) -> Element {
    let game_state = use_memo(move || game_state().expect("Game state to be loaded"));
    let game_options = use_memo(move || game_options().expect("Game options to be loaded"));
    let player_view = use_memo(move || {
        playing_as()
            .and_then(|name| {
                game_state()
                    .players
                    .get_key_value(name.as_str())
                    .map(|(player_id, _)| PlayerView::Player {
                        player_id: player_id.clone(),
                    })
            })
            .unwrap_or(PlayerView::Global)
    });
    use_context_provider(|| GameContext::new(game_state.into(), game_options.into()));
    use_context_provider(|| {
        PlayerViewContext::new(game_state.into(), player_view.into(), set_view)
    });

    let info = use_signal(|| None);
    use_context_provider(|| InfoContext::new(info));

    let view_mode = use_memo(move || view().unwrap_or_default());
    use_context_provider(|| ViewModeContext::new(view_mode.into(), set_view_mode));

    rsx! {
        document::Stylesheet { href: GAME_SCSS }
        Fragment {
            InfoModal {}
            InfoBox {}
            DisplayViewMode {}
        }
    }
}

#[component]
fn DisplayViewMode() -> Element {
    let mode = use_context::<ViewModeContext>().get();
    let gc = use_context::<GameContext>();
    let show_sidebar = use_memo(move || {
        !matches!(
            gc.game_state().phase,
            Phase::Creation | Phase::Setup | Phase::PostSetup
        )
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
                div { class: "phase-container", PhaseView {} }
            }
        },
        ViewMode::Score => rsx! {
            ScoreViewMode {}
        },
        ViewMode::Techs => rsx! {
            TechViewMode {}
        },
        ViewMode::Planets => rsx! {
            PlanetViewMode {}
        },
        ViewMode::Relics => rsx! {
            RelicViewMode {}
        },
        ViewMode::Laws => rsx! {
            LawsViewMode {}
        },
        ViewMode::Map => rsx! {
            MapViewMode {}
        },
        ViewMode::ThundersEdge => rsx! {
            ThundersEdgeViewMode {}
        },
        ViewMode::GalacticEvents => rsx! {
            GalacticEventsViewMode {}
        },
        ViewMode::History => rsx! {
            HistoryViewMode {}
        },
    }
}
