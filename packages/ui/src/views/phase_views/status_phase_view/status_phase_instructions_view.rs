use dioxus::prelude::*;
use ti_helper_game_data::actions::event::Event;

use crate::{
    components::button::Button,
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn StatusPhaseInstructionsView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let status_phase_complete = use_memo(move || {
        gc.game_state()
            .status_phase_state
            .as_ref()
            .map(|s| s.revealed_objective.is_some())
            .expect("Status phase state to be set during status phase")
    });

    rsx! {
        div { class: "card status-view-card",
            h2 { "Status Phase" }
            ol {
                li { "Score Objectives" }
                li { "Reveal Public Objective" }
                li { "Draw Action Cards" }
                li { "Remove Command Tokens" }
                li { "Gain and Redistribute Tokens" }
                li { "Ready Cards" }
                li { "Repair Units" }
                li { "Return Strategy Cards" }
            }
            Button {
                class: "margin-top",
                disabled: !status_phase_complete(),
                onclick: move |_| event.send_event(Event::CompleteStatusPhase),
                "Next phase"
            }
        }
    }
}
