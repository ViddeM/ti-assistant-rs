use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::strategic::StrategicPrimaryAction, state::game_state::StrategicPrimaryProgress,
};

use crate::{
    components::{button::Button, dropdown::PlayerDropdown},
    data::{game_context::GameContext, player_view::PlayerViewContext},
};

#[component]
pub fn PoliticsPrimaryView(
    primary: Option<StrategicPrimaryProgress>,
    on_commit: EventHandler<StrategicPrimaryAction>,
) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let mut new_speaker = use_signal(|| Some("".into()));

    let non_speaker_players = use_memo(move || {
        let speaker = gc.game_state().speaker.clone();
        let mut players = gc
            .game_state()
            .players
            .keys()
            .filter(|&p| !speaker.as_ref().eq(&Some(p)))
            .cloned()
            .collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "full-width",
            if let Some(StrategicPrimaryProgress::Politics { new_speaker }) = primary.clone() {
                p { "new speaker {new_speaker}" }
            } else if view.is_active() {
                fieldset {
                    legend { "Select new speaker" }
                    div { class: "select-primary-container",
                        PlayerDropdown {
                            value: new_speaker,
                            options: non_speaker_players(),
                            on_select: move |p| new_speaker.set(p),
                        }
                        Button {
                            class: "margin-top",
                            disabled: new_speaker().is_none(),
                            onclick: move |_| {
                                on_commit(StrategicPrimaryAction::Politics {
                                    new_speaker: new_speaker().expect("New speaker to be selected"),
                                })
                            },
                            "Commit"
                        }
                    }
                }
            } else {
                p { "Waiting for {current_player()} to pick" }
            }
        }
    }
}
