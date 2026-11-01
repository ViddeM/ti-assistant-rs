use dioxus::prelude::*;

const MAIN_MENU_SCSS: Asset = asset!("/assets/styling/views/main_menu.scss");

const VIDDE_LINK: &str = "https://www.github.com/viddem";
const TUX_LINK: &str = "https://www.github.com/hulthe";

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MainMenuProps<R>
where
    R: PartialEq + Clone + 'static + Routable,
{
    new_game: Callback<(), R>,
    join_game: Callback<String, R>,
}

#[component]
pub fn MainMenu<R: PartialEq + Clone + 'static + Routable>(
    MainMenuProps {
        new_game,
        join_game,
    }: MainMenuProps<R>,
) -> Element {
    let mut game_id = use_signal(|| "".to_string());
    let nav = navigator();

    rsx! {
        document::Stylesheet { href: MAIN_MENU_SCSS }
        div { class: "main-menu-card",
            div { class: "title-text",
                h1 { "TI Helper" }
                p {
                    "Made with ❤️ by "
                    Link {
                        to: NavigationTarget::<R>::External(VIDDE_LINK.to_string()),
                        new_tab: true,
                        "Vidde"
                    }
                    " & "
                    Link {
                        to: NavigationTarget::<R>::External(TUX_LINK.to_string()),
                        new_tab: true,
                        "Tux"
                    }
                }
            }


            Link { to: new_game(()), class: "main-menu-link",
                h2 { "New Game" }
            }

            form {
                class: "game-id-input-content",
                onsubmit: move |e: FormEvent| {
                    e.prevent_default();
                    if let Some(code) = full_game_code(&game_id()) {
                        nav.push(join_game(code));
                    }
                },
                div { class: "source-code-pro-font game-id-container",
                    input {
                        id: "game-id-input",
                        value: "{game_id()}",
                        maxlength: 8,
                        autocapitalize: "none",
                        autocorrect: "off",
                        autocomplete: "off",
                        spellcheck: false,
                        onmounted: move |_| async move {
                            let typed = document::eval("return document.getElementById('game-id-input').value");
                            if let Ok(typed) = typed.join::<String>().await {
                                game_id.set(typed);
                            }
                        },
                        oninput: move |evt| game_id.set(evt.value()),
                    }
                }
            }

            {
                if let Some(code) = full_game_code(&game_id()) {
                    rsx! {
                        Link { to: join_game(code), class: "main-menu-link",
                            h2 { "JOIN GAME" }
                        }
                    }
                } else {
                    rsx! {
                        h2 { class: "main-menu-link-disabled", "JOIN GAME" }
                    }
                }
            }
        }
    }
}

fn full_game_code(input: &str) -> Option<String> {
    let code = input.trim().to_ascii_lowercase();
    (code.len() == 8 && code.chars().all(|c| c.is_ascii_hexdigit())).then_some(code)
}
