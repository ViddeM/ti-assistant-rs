use dioxus::prelude::*;
use ti_helper_game_data::components::phase::Phase;

use crate::{
    data::game_context::GameContext,
    views::phase_views::{
        action_card::ActionCardView, action_phase::ActionPhaseView,
        creation_phase::CreationPhaseView, end_action_phase::EndActionPhaseView,
        relic_card::RelicCardView, setup_phase::SetupPhaseView, strategy_phase::StrategyPhaseView,
    },
};

#[component]
pub fn PhaseView() -> Element {
    let gc = use_context::<GameContext>();

    match gc.game_state().phase {
        Phase::Creation => rsx! {
            CreationPhaseView {}
        },
        Phase::Setup => rsx! {
            SetupPhaseView {}
        },
        Phase::Strategy => rsx! {
            StrategyPhaseView {}
        },
        Phase::Action => rsx! {
            ActionPhaseView {}
        },
        Phase::ActionCardAction => rsx! {
            ActionCardView {}
        },
        Phase::EndActionTurn => rsx! {
            EndActionPhaseView {}
        },
        Phase::RelicAction => rsx! {
            RelicCardView {}
        },
        phase => rsx! {
            p { "Phase {phase} is not yet implemented" }
        },
    }
}
