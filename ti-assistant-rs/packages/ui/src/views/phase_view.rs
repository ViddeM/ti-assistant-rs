use dioxus::prelude::*;
use ti_helper_game_data::components::phase::Phase;

use crate::{
    data::game_context::GameContext, views::phase_views::creation_phase::CreationPhaseView,
};

#[component]
pub fn PhaseView() -> Element {
    let gc = use_context::<GameContext>();

    match gc.game_state().phase {
        Phase::Creation => rsx! {
            CreationPhaseView {}
        },
        p => rsx! {
            p { "Phase {gc.game_state().phase} is not yet implemented" }
        },
    }
}
