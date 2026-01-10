use dioxus::prelude::*;
use ti_helper_game_data::state::game_state::ActionPhaseProgress;

use crate::data::{game_context::GameContext, player_view::PlayerViewContext};

const TACTICAL_ACTION_VIEW: Asset =
    asset!("/assets/styling/views/phase_views/tactical_action.scss");

#[component]
pub fn TacticalActionView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let tactical = use_memo(move || gc.game_state().action_progress.clone());

    rsx! {
        document::Stylesheet { href: TACTICAL_ACTION_VIEW }

        if let Some(progress) = tactical() {
            div { class: "card tactical-container",
                h2 { "Tactical" }
                if view.is_active() {

                } else {
                    p { "Not your turn, waiting for {current_player()}" }
                }
            }
        }
    }
}

#[component]
fn TacticalActionProgressView(progress: ReadSignal<ActionPhaseProgress>) -> Element {
    rsx! {}
}
