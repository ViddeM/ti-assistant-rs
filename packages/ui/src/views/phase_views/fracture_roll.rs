use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::button::Button,
    data::{event_context::EventContext, player_view::PlayerViewContext},
};

#[component]
pub fn FractureRoll(player_id: PlayerId) -> Element {
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let roller = player_id.clone();
    let roll = use_callback(move |entered_play| {
        event.send_event(Event::RollForTheFracture {
            player: roller.clone(),
            entered_play,
        })
    });

    rsx! {
        div { class: "card column",
            h2 { "The Fracture" }
            p { "{player_id} gained their breakthrough. Roll 1 die: on a 1 or 10, the Fracture enters play." }
            if view.display_for(&player_id) {
                div { class: "center-row margin-top",
                    Button { class: "margin-right", onclick: move |_| roll(true), "It entered play" }
                    Button { onclick: move |_| roll(false), "It didn't" }
                }
            } else {
                p { "Waiting for {player_id} to roll." }
            }
        }
    }
}
