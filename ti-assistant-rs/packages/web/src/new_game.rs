use dioxus::prelude::*;
use ui::{
    components::button::Button,
    endpoints,
    game_id::GameId,
    requests::new_game::{self, GameConfig},
};

use crate::Route;

#[derive(PartialEq)]
enum CreateGameMode {
    New,
    MiltyImport,
}

#[component]
pub fn NewGame() -> Element {
    let mut winning_score = use_signal(|| 10);
    let mut mode = use_signal(|| CreateGameMode::New);
    let mut pok = use_signal(|| false);
    let mut te = use_signal(|| false);
    let mut codexI = use_signal(|| false);
    let mut codexII = use_signal(|| false);
    let mut codexIII = use_signal(|| false);

    let nav = navigator();

    let mut new_game_result: Signal<Option<Result<GameId, ServerFnError>>> = use_signal(|| None);

    match &new_game_result() {
        Some(Err(err)) => rsx! {
            p { "Failed to create game: {err:?}" }
        },
        Some(Ok(game_id)) => {
            match nav.push(Route::Game {
                id: game_id.to_string(),
            }) {
                Some(s) => rsx! {
                    p { "Failed to navigate... {s:?}" }
                },
                None => {
                    rsx! {
                        p { "Navigating..." }
                    }
                }
            }
        }
        None => {
            rsx! {
                div {
                    label { r#for: "winning_score", "Winning Score" }
                    input {
                        r#type: "range",
                        name: "winning_score",
                        min: 4,
                        max: 16,
                        value: winning_score,
                        oninput: move |event| *winning_score.write() = event.value().parse().unwrap(),
                    }
                    p { "{winning_score}" }

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

                    label { r#for: "pok_cb", "Prophecy of Kings" }
                    input {
                        r#type: "checkbox",
                        name: "pok_cb",
                        onchange: move |_| pok.toggle(),
                    }

                    label { r#for: "pok_codI_cb", "Codex I" }
                    input {
                        r#type: "checkbox",
                        name: "pok_codI_cb",
                        onchange: move |_| codexI.toggle(),
                    }

                    label { r#for: "pok_codII_cb", "Codex II" }
                    input {
                        r#type: "checkbox",
                        name: "pok_codII_cb",
                        onchange: move |_| codexII.toggle(),
                    }

                    label { r#for: "pok_codIII_cb", "Codex III" }
                    input {
                        r#type: "checkbox",
                        name: "pok_codIII_cb",
                        onchange: move |_| codexIII.toggle(),
                    }

                    label { r#for: "te_cb", "Thunder's Edge" }
                    input {
                        r#type: "checkbox",
                        name: "te_cb",
                        onchange: move |_| te.toggle(),
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
        }
    }
}
