use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::{event::Event, strategic::StrategicPrimaryAction},
    state::game_state::{StrategicPrimaryProgress, StrategicProgress},
};

use crate::{
    components::{button::Button, dropdown::ObjectiveDropdown},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn ImperialPrimaryView(progress: ReadSignal<StrategicProgress>) -> Element {
    let view = use_context::<PlayerViewContext>();
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let primary = use_memo(move || progress().primary);

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });
    let current_player_objectives = use_memo(move || {
        gc.game_state()
            .score
            .revealed_objectives
            .iter()
            .filter(|(_, ps)| ps.contains(&current_player()))
            .map(|(o, _)| o.clone())
            .collect::<Vec<_>>()
    });

    let objectives = use_memo(move || {
        let mut objectives = gc
            .game_state()
            .score
            .revealed_objectives
            .keys()
            .filter(|&o| !current_player_objectives().contains(o))
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    let mut selected_objective = use_signal(|| None);

    rsx! {
        div { class: "primary-container",
            if let Some(StrategicPrimaryProgress::Imperial { objective }) = primary() {
                div { class: "primary-choice-container",
                    p {
                        if let Some(objective) = objective {
                            "{objective.info().name}"
                        } else {
                            "No objective taken"
                        }
                    }
                }
            } else {
                fieldset {
                    legend { "Score objective" }
                    if view.is_active() {
                        div { class: "select-primary-container",
                            ObjectiveDropdown {
                                value: selected_objective(),
                                options: objectives(),
                                on_select: move |s| selected_objective.set(s),
                            }
                            div { class: "action-buttons-container",
                                Button {
                                    onclick: move |_| {
                                        event
                                            .send_event(Event::StrategicActionPrimary {
                                                player: current_player(),
                                                action: StrategicPrimaryAction::Imperial {
                                                    score_objective: None,
                                                },
                                            });
                                        selected_objective.set(None);
                                    },
                                    "Skip"
                                }
                                Button {
                                    onclick: move |_| {
                                        event
                                            .send_event(Event::StrategicActionPrimary {
                                                player: current_player(),
                                                action: StrategicPrimaryAction::Imperial {
                                                    score_objective: Some(
                                                        selected_objective().expect("Selected objective to be set"),
                                                    ),
                                                },
                                            });
                                        selected_objective.set(None);
                                    },
                                    disabled: selected_objective().is_none(),
                                    "Score"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
