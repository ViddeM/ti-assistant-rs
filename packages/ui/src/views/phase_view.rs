use dioxus::prelude::*;
use ti_helper_game_data::{
    components::phase::Phase, state::game_state::MinisterOfAntiquesProgress,
};

use crate::{
    data::game_context::GameContext,
    views::{
        book_of_latvinia_research::BookOfLatviniaResearch,
        frontier_card::FrontierCardView,
        phase_views::{
            action_card::ActionCardView, action_phase::ActionPhaseView,
            agenda_phase_view::AgendaPhaseView, creation_phase::CreationPhaseView,
            end_action_phase::EndActionPhaseView, entropic_scar_phase::EntropicScarPhaseView,
            fracture_roll::FractureRoll, minister_of_antiques::MinisterOfAntiques,
            post_setup_phase::PostSetupPhaseView, relic_card::RelicCardView,
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

    match gc.game_state().minister_of_antiques.clone() {
        Some(MinisterOfAntiquesProgress::GainRelic { player }) => {
            return rsx! {
                MinisterOfAntiques { player_id: player }
            };
        }
        Some(MinisterOfAntiquesProgress::BookOfLatviniaResearch { player }) => {
            return rsx! {
                BookOfLatviniaResearch { player_id: player }
            };
        }
        None => {}
    }

    match gc.game_state().phase {
        Phase::Creation => rsx! {
            CreationPhaseView {}
        },
        Phase::Setup => rsx! {
            SetupPhaseView {}
        },
        Phase::PostSetup => rsx! {
            PostSetupPhaseView {}
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
