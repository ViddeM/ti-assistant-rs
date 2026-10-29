use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::button::Button,
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn FractureRolls() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let rolls = use_memo(move || gc.game_state().fracture_rolls.clone());

    rsx! {
        for player_id in rolls() {
            if view.display_for(&player_id) {
                FractureRoll { key: "{player_id}", player_id }
            }
        }
    }
}

#[component]
fn FractureRoll(player_id: PlayerId) -> Element {
    let event = use_context::<EventContext>();

    let roller = player_id.clone();
    let roll = use_callback(move |entered_play| {
        event.send_event(Event::RollForTheFracture {
            player: roller.clone(),
            entered_play,
        })
    });

    rsx! {
        fieldset { class: "column screen-container margin-top",
            legend { "The Fracture: {player_id}" }
            p { "{player_id} gained their breakthrough. Roll 1 die: on a 1 or 10, the Fracture enters play." }
            div { class: "center-row margin-top",
                Button { class: "margin-right", onclick: move |_| roll(true), "It entered play" }
                Button { onclick: move |_| roll(false), "It didn't" }
            }
        }
    }
}
