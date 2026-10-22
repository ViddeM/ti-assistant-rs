use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, state::player::Player,
};

use crate::{components::dropdown::BreakthroughDropdown, data::event_context::EventContext};

#[component]
pub fn PlayerBreakthrough(player_id: PlayerId, player: Player) -> Element {
    let event = use_context::<EventContext>();

    rsx! {
        fieldset {
            class: format!(
                "player-color-border-{} action-player-container",
                player.color.name(),
            ),
            legend {
                class: format!(
                    "player-color-border-{} action-player-container",
                    player.color.name(),
                ),
                h4 { "{player_id}" }
            }
            div {
                BreakthroughDropdown {
                    class: "margin-top",
                    value: player.breakthrough.clone(),
                    faction: player.faction,
                    on_select: move |b| event.send_event(Event::SetBreakthrough {
                        player: player_id.clone(),
                        breakthrough: b,
                    }),
                }
            }
        }
    }
}
