use dioxus::prelude::*;

use crate::{
    data::game_context::GameContext,
    views::phase_views::agenda_phase_view::{
        agenda_actions_view::AgendaActionsView, agenda_info_view::AgendaInfoView,
    },
};

mod agenda_actions_view;
mod agenda_info_view;

const AGENDA_PHASE_VIEW_SCSS: Asset =
    asset!("/assets/styling/views/phase_views/agenda_phase_view.scss");

#[component]
pub fn AgendaPhaseView() -> Element {
    let gc = use_context::<GameContext>();

    let state = use_memo(move || gc.game_state().agenda.clone());

    rsx! {
        document::Stylesheet { href: AGENDA_PHASE_VIEW_SCSS }

        if let Some(agenda_state) = state() {
            div { class: "agenda-phase-container",
                AgendaInfoView { state: agenda_state.clone() }
                AgendaActionsView { state: agenda_state.clone() }
            }
        } else {
            p { "Agenda state not set during agenda phase?" }
        }
    }
}
