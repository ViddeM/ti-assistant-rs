use dioxus::prelude::*;
use ti_helper_game_data::{
    components::objectives::Objective,
    state::game_state::{StrategicPrimaryProgress, StrategicProgress},
};

use crate::{components::dropdown::ObjectiveDropdown, data::game_context::GameContext};

#[component]
pub fn ImperialPrimaryView(progress: ReadSignal<StrategicProgress>) -> Element {
    let primary = use_memo(move || progress().primary);

    rsx! {
        if let Some(StrategicPrimaryProgress::Imperial { objective }) = primary() {
            ImperialPrimaryProgressView { objective }
        } else {

        }
    }
}

#[component]
fn ImperialPrimaryProgressView(objective: ReadSignal<Option<Objective>>) -> Element {
    let gc = use_context::<GameContext>();

    // let mut selected_objective = use_signal(|| None);

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
            .filter(|&o| current_player_objectives().contains(o))
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    rsx! {
        div { class: "primary-container",
            if let Some(objective) = objective() {
                div { class: "primary-choice-container",
                    p { "{objective.info().name}" }
                }
            } else {
                ObjectiveDropdown {
                    value: objective(),
                    options: objectives(),
                    on_select: move |s| {},
                }
            }
        }
    }
}
