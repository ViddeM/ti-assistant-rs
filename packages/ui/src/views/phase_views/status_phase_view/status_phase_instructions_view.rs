use dioxus::prelude::*;
use ti_helper_game_data::actions::event::Event;

use crate::{
    components::button::Button,
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn StatusPhaseInstructionsView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let reveals_objective = use_memo(move || gc.game_state().status_phase_reveals_objective());

    let status_phase_complete = use_memo(move || {
        let game_state = gc.game_state();
        let state = game_state
            .status_phase_state
            .as_ref()
            .expect("Status phase state to be set during status phase");
        if reveals_objective() {
            state.revealed_objective.is_some()
        } else {
            state.can_reveal_objective(game_state.players.len())
        }
    });

    rsx! {
        div { class: "card status-view-card",
            h2 { "Status Phase" }
            ol {
                li { "Score Objectives" }
                if reveals_objective() {
                    li { "Reveal Public Objective" }
                }
                li { "Draw Action Cards" }
                li { "Remove Command Tokens" }
                li { "Gain and Redistribute Tokens" }
                li { "Ready Cards" }
                li { "Repair Units" }
                li { "Return Strategy Cards" }
            }
            Button {
                class: "margin-top",
                disabled: !status_phase_complete() || !view.is_global_or_speaker(),
                onclick: move |_| event.send_event(Event::CompleteStatusPhase),
                "Next phase"
            }
        }
    }
}
