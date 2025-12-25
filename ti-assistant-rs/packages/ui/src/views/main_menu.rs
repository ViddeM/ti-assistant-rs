use dioxus::prelude::*;

const MAIN_MENU_SCSS: Asset = asset!("/assets/styling/views/main_menu.scss");

const VIDDE_LINK: &'static str = "https://www.github.com/viddem";
const TUX_LINK: &'static str = "https://www.github.com/hulthe";

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
                    nav.push(join_game(game_id())).expect("Internal navigation to succeed");
                },
                div { class: "source-code-pro-font game-id-container",
                    input {
                        placeholder: "Game ID",
                        value: "{game_id()}",
                        oninput: move |evt| {
                            let value = evt.value();
                            let prev = game_id();
                            let value = if value.chars().count() > 8 { prev } else { value };
                            game_id.set(value);
                        },
                    }
                }
            }

            {
                if game_id().len() == 8 {
                    rsx! {
                        Link { to: join_game(game_id()), class: "main-menu-link",
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
