use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::{event::Event, strategic::StrategicPrimaryAction},
    state::game_state::{ActionPhaseProgress, StrategicPrimaryProgress},
};

use crate::{
    components::{button::Button, dropdown::PlayerDropdown},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn PoliticsPrimaryView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let progress = use_memo(move || {
        gc.game_state()
            .action_progress
            .clone()
            .expect("Progress to exist")
    });

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let mut new_speaker = use_signal(|| "".into());

    let ActionPhaseProgress::Strategic(progress) = progress() else {
        return rsx! {};
    };

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
        div {
            if let Some(StrategicPrimaryProgress::Politics { new_speaker }) = progress.primary {
                p { "new speaker {new_speaker}" }
            } else if view.is_active() {
                fieldset {
                    legend { "Select new speaker" }
                    div { class: "select-primary-container",
                        PlayerDropdown {
                            value: new_speaker(),
                            options: non_speaker_players(),
                            on_select: move |p| new_speaker.set(p),
                        }
                        Button {
                            class: "margin-top",
                            disabled: new_speaker().is_empty(),
                            onclick: {
                                move |_| {
                                    event
                                        .send_event(Event::StrategicActionPrimary {
                                            player: current_player(),
                                            action: StrategicPrimaryAction::Politics {
                                                new_speaker: new_speaker(),
                                            },
                                        })
                                }
                            },
                            "Commit"
                        }
                    }
                }
            } else {
                p { "Waitine for {current_player()} to pick" }
            }
        }
    }
}
