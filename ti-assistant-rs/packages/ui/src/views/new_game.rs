use api::{
    endpoints,
    requests::new_game::{self, GameConfig},
};
use dioxus::prelude::*;
use ti_helper_game_data::game_id::GameId;

use crate::components::button::Button;

const NEW_GAME_SCSS: Asset = asset!("/assets/styling/views/new_game.scss");

#[derive(PartialEq)]
enum CreateGameMode {
    New,
    MiltyImport,
}

#[component]
pub fn NewGame<R: PartialEq + Clone + 'static + Routable>(
    join_game: Callback<String, R>,
) -> Element {
    let mut winning_score = use_signal(|| 10);
    let mut mode = use_signal(|| CreateGameMode::New);
    let nav = navigator();

    let new_game_result: Signal<Option<Result<GameId, ServerFnError>>> = use_signal(|| None);

    let view_mode = use_memo(move || match *mode.read() {
        CreateGameMode::New => rsx! {
            CleanNewGameView { new_game_result, winning_score }
        },
        CreateGameMode::MiltyImport => rsx! {
            ImportMiltyGame { new_game_result, winning_score }
        },
    });

    match &new_game_result() {
        Some(Err(ServerFnError::ServerError {
            message,
            code: _,
            details,
        })) => rsx! {
            p { "Failed to create game: {message}" }
            if let Some(d) = details {
                br {}
                p { "Details {d}" }
            }
        },
        Some(Err(_)) => rsx! {
            p { "An unknown error occurred" }
        },
        Some(Ok(game_id)) => match nav.push(join_game(game_id.to_string())) {
            Some(s) => rsx! {
                p { "Failed to navigate... {s:?}" }
            },
            None => {
                rsx! {
                    p { "Navigating..." }
                }
            }
        },
        None => {
            rsx! {
                document::Stylesheet { href: NEW_GAME_SCSS }

                div { class: "card create-game-container",
                    h2 { "Create Game" }
                    label { r#for: "winning_score", "Winning Score" }
                    div { class: "create-game-row center-row",
                        input {
                            r#type: "range",
                            name: "winning_score",
                            min: 4,
                            max: 16,
                            value: winning_score,
                            oninput: move |event| *winning_score.write() = event.value().parse().unwrap(),
                        }
                        p { "{winning_score}" }
                    }

                    div { class: "view-mode-button-group margin-bottom",
                        Button {
                            disabled: mode.read().eq(&CreateGameMode::New),
                            onclick: move |_| *mode.write() = CreateGameMode::New,
                            "New Game"
                        }
                        Button {
                            disabled: mode.read().eq(&CreateGameMode::MiltyImport),
                            onclick: move |_| *mode.write() = CreateGameMode::MiltyImport,
                            "Import from Milty"
                        }
                    }

                    {view_mode}
                }
            }
        }
    }
}

#[component]
fn CleanNewGameView(
    new_game_result: WriteSignal<Option<Result<GameId, ServerFnError>>>,
    winning_score: ReadSignal<u32>,
) -> Element {
    let mut pok = use_signal(|| false);
    let mut te = use_signal(|| false);
    let mut codexI = use_signal(|| false);
    let mut codexII = use_signal(|| false);
    let mut codexIII = use_signal(|| false);

    rsx! {
        div { class: "create-game-row",
            input {
                r#type: "checkbox",
                name: "pok_cb",
                onchange: move |_| pok.toggle(),
            }
            label { r#for: "pok_cb", "Prophecy of Kings" }
        }

        div { class: "create-game-row",
            input {
                r#type: "checkbox",
                name: "pok_codI_cb",
                onchange: move |_| codexI.toggle(),
            }
            label { r#for: "pok_codI_cb", "Codex I" }
        }

        div { class: "create-game-row",
            input {
                r#type: "checkbox",
                name: "pok_codII_cb",
                onchange: move |_| codexII.toggle(),
            }
            label { r#for: "pok_codII_cb", "Codex II" }
        }

        div { class: "create-game-row",
            input {
                r#type: "checkbox",
                name: "pok_codIII_cb",
                onchange: move |_| codexIII.toggle(),
            }
            label { r#for: "pok_codIII_cb", "Codex III" }
        }

        div { class: "create-game-row",
            input {
                r#type: "checkbox",
                name: "te_cb",
                onchange: move |_| te.toggle(),
            }
            label { r#for: "te_cb", "Thunder's Edge" }
        }

        Button {
            onclick: move |_| async move {
                let ngr = endpoints::new_game(new_game::NewGame {
                        points: winning_score(),
                        game_config: GameConfig::CustomGameConfig {
                            pok: pok(),
                            cod1: codexI(),
                            cod2: codexII(),
                            cod3: codexIII(),
                            te: te(),
                        },
                    })
                    .await;
                *new_game_result.write() = Some(ngr);
            },
            "Create Game"
        }

    }
}

#[component]
fn ImportMiltyGame(
    new_game_result: WriteSignal<Option<Result<GameId, ServerFnError>>>,
    winning_score: ReadSignal<u32>,
) -> Element {
    let mut milty_game_id = use_signal(|| String::new());
    let mut milty_tts_string = use_signal(|| String::new());

    rsx! {
        div { class: "right-aligned-row",
            label { r#for: "milty-string-input", "Milty draft ID:" }
            input {
                r#type: "text",
                required: true,
                id: "milty-string-input",
                value: milty_game_id(),
                onchange: move |e: FormEvent| {
                    milty_game_id.set(e.value());
                },
            }
        }

        div { class: "right-aligned-row",
            label { r#for: "milty-tts-input", "Milty TTS Map string:" }
            input {
                r#type: "text",
                required: true,
                id: "milty-tts-input",
                value: milty_tts_string(),
                onchange: move |e: FormEvent| {
                    milty_tts_string.set(e.value());
                },
            }
        }

        Button {
            onclick: move |_| async move {
                let ngr = endpoints::new_game(new_game::NewGame {
                        points: winning_score(),
                        game_config: GameConfig::ImportFromMilty {
                            milty_game_id: milty_game_id(),
                            milty_tts_string: milty_tts_string(),
                        },
                    })
                    .await;
                *new_game_result.write() = Some(ngr);
            },
            "Create Game"
        }
    }
}
