use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    components::objectives::{Objective, ObjectiveKind},
};

use crate::{
    components::{button::Button, dropdown::ObjectiveDropdown},
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn RevealObjectiveForm() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_stage_one = use_signal(|| None);
    let mut selected_stage_two = use_signal(|| None);

    let revealed_objectives = use_memo(move || {
        let mut objectives = gc
            .game_state()
            .score
            .revealed_objectives
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });
    let stage_one_objectives = use_memo(move || {
        let mut objectives = gc
            .game_options()
            .objectives
            .keys()
            .filter(|o| o.info().kind == ObjectiveKind::StageI)
            .filter(|o| !revealed_objectives().contains(o))
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });
    let stage_two_objectives = use_memo(move || {
        let mut objectives = gc
            .game_options()
            .objectives
            .keys()
            .filter(|o| o.info().kind == ObjectiveKind::StageII)
            .filter(|o| !revealed_objectives().contains(o))
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    rsx! {
        form {
            class: "card reveal-objective-form",
            onsubmit: move |e| e.prevent_default(),
            h2 { "Reveal Objectives" }
            div { class: "reveal-objective-row",
                ObjectiveDropdown {
                    value: selected_stage_one(),
                    options: stage_one_objectives(),
                    on_select: move |o| { selected_stage_one.set(o) },
                    default_text: "--Select Stage I objective",
                }
                Button {
                    disabled: selected_stage_one().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::RevealExtraPublicObjective {
                                objective: match selected_stage_one().expect("Objective to be set") {
                                    Objective::Public(po) => po,
                                    Objective::Secret(_) => panic!("Expected public objective!"),
                                },
                            });
                        selected_stage_one.set(None);
                    },
                    "Reveal"
                }
            }
            div { class: "reveal-objective-row",
                ObjectiveDropdown {
                    value: selected_stage_two(),
                    options: stage_two_objectives(),
                    on_select: move |o| selected_stage_two.set(o),
                    default_text: "--Select Stage II objective",
                }
                Button {
                    disabled: selected_stage_two().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::RevealExtraPublicObjective {
                                objective: match selected_stage_two().expect("Objective to be set") {
                                    Objective::Public(po) => po,
                                    Objective::Secret(_) => panic!("Expected public objective!"),
                                },
                            });
                        selected_stage_two.set(None);
                    },
                    "Reveal"
                }
            }
        }
    }
}
