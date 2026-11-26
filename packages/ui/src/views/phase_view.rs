use dioxus::prelude::*;
use ti_helper_game_data::components::phase::Phase;

use crate::{
    data::{game_context::GameContext, player_view::PlayerViewContext},
    views::{
        frontier_card::FrontierCardView,
        phase_views::{
            action_card::ActionCardView, action_phase::ActionPhaseView,
            agenda_phase_view::{AgendaEffectView, AgendaPhaseView}, breakthrough_action::BreakthroughActionView,
            creation_phase::CreationPhaseView,
            end_action_phase::EndActionPhaseView, entropic_scar_phase::EntropicScarPhaseView,
            fracture_view::FractureView, post_setup_phase::PostSetupPhaseView,
            relic_card::RelicCardView, relics_phase::RelicsPhaseView, setup_phase::SetupPhaseView,
            status_phase_view::StatusPhaseView, strategy_card::StrategyCardView,
            strategy_phase::StrategyPhaseView, tactical_action::TacticalActionView,
        },
    },
};

#[component]
pub fn PhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let phase = gc.game_state().phase;

    if view.is_spectator() && matches!(phase, Phase::Creation | Phase::Setup | Phase::PostSetup) {
        return rsx! {
            div { class: "card",
                p { "Waiting for the game to be set up" }
            }
        };
    }

    match phase {
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
        Phase::BreakthroughAction => rsx! {
            BreakthroughActionView {}
        },
        Phase::Agenda => rsx! {
            AgendaPhaseView {}
        },
        Phase::AgendaEffect => rsx! {
            AgendaEffectView {}
        },
        Phase::Relics => rsx! {
            RelicsPhaseView {}
        },
        Phase::Fracture => rsx! {
            FractureView {}
        },
        Phase::LeaderAction => todo!(),
    }
}
