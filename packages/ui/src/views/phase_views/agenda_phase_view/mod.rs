use dioxus::prelude::*;
use ti_helper_game_data::state::agenda::MinisterOfAntiquesProgress;

use crate::{
    data::game_context::GameContext,
    views::{
        book_of_latvinia_research::BookOfLatviniaResearch,
        phase_views::{
            agenda_phase_view::{
                agenda_actions_view::AgendaActionsView, agenda_info_view::AgendaInfoView,
            },
            minister_of_antiques::MinisterOfAntiques,
        },
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
            match agenda_state.minister_of_antiques.clone() {
                Some(MinisterOfAntiquesProgress::GainRelic { player }) => rsx! {
                    MinisterOfAntiques { player_id: player }
                },
                Some(MinisterOfAntiquesProgress::BookOfLatviniaResearch { player }) => {
                    rsx! {
                        BookOfLatviniaResearch { player_id: player }
                    }
                }
                None => rsx! {
                    div { class: "agenda-phase-container",
                        AgendaInfoView { state: agenda_state.clone() }
                        AgendaActionsView { state: agenda_state.clone() }
                    }
                },
            }
        } else {
            p { "Agenda state not set during agenda phase?" }
        }
    }
}
