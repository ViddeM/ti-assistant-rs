use dioxus::prelude::*;
use ti_helper_game_data::components::phase::Phase;

use crate::{
    data::game_context::GameContext,
    views::{
        frontier_card::FrontierCardView,
        phase_views::{
            action_card::ActionCardView, action_phase::ActionPhaseView,
            agenda_phase_view::AgendaPhaseView, creation_phase::CreationPhaseView,
            end_action_phase::EndActionPhaseView, entropic_scar_phase::EntropicScarPhaseView,
            fracture_roll::FractureRoll, minister_of_antiques::MinisterOfAntiques,
            rapid_mobilization::RapidMobilization, relic_card::RelicCardView,
            relics_phase::RelicsPhaseView, setup_phase::SetupPhaseView,
            status_phase_view::StatusPhaseView, strategy_card::StrategyCardView,
            strategy_phase::StrategyPhaseView, tactical_action::TacticalActionView,
        },
    },
};

#[component]
pub fn PhaseView() -> Element {
    let gc = use_context::<GameContext>();

    if let Some(player_id) = gc.game_state().fracture_roll.clone() {
        return rsx! {
            FractureRoll { player_id }
        };
    }

    if let Some(player_id) = gc.game_state().minister_of_antiques_relic.clone() {
        return rsx! {
            MinisterOfAntiques { player_id }
        };
    }

    if !gc.game_state().rapid_mobilization.is_empty() {
        return rsx! {
            RapidMobilization {}
        };
    }

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
        Phase::StrategicAction => rsx! {
            StrategyCardView {}
        },
        Phase::TacticalAction => rsx! {
            TacticalActionView {}
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
        Phase::EntropicScar => rsx! {
            EntropicScarPhaseView {}
        },
        Phase::Status => rsx! {
            StatusPhaseView {}
        },
        Phase::FrontierCardAction => rsx! {
            FrontierCardView {}
        },
        Phase::Agenda => rsx! {
            AgendaPhaseView {}
        },
        Phase::Relics => rsx! {
            RelicsPhaseView {}
        },
        Phase::LeaderAction => todo!(),
    }
}
