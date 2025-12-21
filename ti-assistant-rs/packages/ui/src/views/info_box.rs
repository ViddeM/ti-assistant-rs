use dioxus::{logger::tracing, prelude::*};
use ti_helper_game_data::game_id::GameId;

use crate::data::{
    event_context::EventContext, game_context::GameContext, view_mode::ViewModeContext,
};

#[component]
pub fn InfoBox() -> Element {
    let game_id = use_context::<GameId>();
    let game_context = use_context::<GameContext>();
    let view = use_context::<ViewModeContext>();
    let ec = use_context::<EventContext>();

    let game_state = game_context.game_state();

    tracing::info!(
        "Infobox was refreshed time tracking is now {}",
        game_state.time_tracking_paused
    );

    rsx! {
        div {
            b { "Game: {game_id}" }
            div {
                button { "Game" }
                button { "Score" }
                button { "Techs" }
                button { "Planets" }
                button { "Law" }
                button { "Map" }
            }
            div {
                b { "Round: {game_state.round}" }
            }
            div {
                b { "Currently viewing: {view.display()}" }
            }
            div {
                if game_state.time_tracking_paused {
                    button { onclick: move |_| ec.pause(), "Pause" }
                } else {
                    button { onclick: move |_| ec.play(), "Play" }
                }
                button { onclick: move |_| ec.undo(), "Undo" }
            }
        }
    }
}
