use dioxus::prelude::*;
use ti_helper_game_data::actions::event::Event;

use crate::{
    components::button::Button,
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

const END_ACTION_PHASE_SCSS: Asset =
    asset!("/assets/styling/views/phase_views/end_action_phase.scss");

#[component]
pub fn EndActionPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist")
    });

    rsx! {
        document::Stylesheet { href: END_ACTION_PHASE_SCSS }

        div { class: "card end-action-phase-view-container",
            h2 { "End player turn?" }
            {
                if view.is_active() {
                    rsx! {
                        Button {
                            onclick: move |_| {
                                event
                                    .send_event(Event::TakeAnotherTurn {
                                        player: current_player_id(),
                                    })
                            },
                            "Take another turn"
                        }
                        Button {
                            onclick: move |_| {
                                event
                                    .send_event(Event::EndTurn {
                                        player: current_player_id(),
                                    })
                            },
                            "End Turn"
                        }
                    }
                } else {
                    rsx! {
                        p { "Not your turn, currently {current_player_id()} is playing" }
                    }
                }
            }
        }
    }
}
