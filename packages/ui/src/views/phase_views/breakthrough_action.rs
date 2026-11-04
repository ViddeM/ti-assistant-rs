use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{BreakthroughAction, Event},
    common::player_id::PlayerId,
    components::breakthrough::Breakthrough,
};

use crate::{components::button::Button, data::event_context::EventContext};

#[component]
pub fn BreakthroughActionView(player: PlayerId, breakthrough: Breakthrough) -> Element {
    let name = breakthrough.info().name;

    rsx! {
        div {
            fieldset { class: "play-action-card-container",
                legend { "{name}" }
                BreakthroughActionForm { player, breakthrough }
            }
        }
    }
}

#[component]
fn BreakthroughActionForm(player: PlayerId, breakthrough: Breakthrough) -> Element {
    let event = use_context::<EventContext>();

    let action = match breakthrough {
        Breakthrough::Psychospore => BreakthroughAction::Psychospore,
        Breakthrough::DeorbitBarrage => BreakthroughAction::DeorbitBarrage,
        Breakthrough::ResonanceGenerator => BreakthroughAction::ResonanceGenerator,
        _ => return rsx! {},
    };

    rsx! {
        Button {
            onclick: move |_| {
                event
                    .send_event(Event::PlayBreakthroughAction {
                        player: player.clone(),
                        action: action.clone(),
                    })
            },
            "Play"
        }
    }
}
