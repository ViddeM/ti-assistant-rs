use dioxus::prelude::*;

use crate::views::phase_views::status_phase_view::{
    status_phase_actions_view::StatusPhaseActionsView,
    status_phase_instructions_view::StatusPhaseInstructionsView,
};

const STATUS_PHASE_VIEW_SCSS: Asset = asset!("/assets/styling/views/phase_views/status_phase.scss");

#[component]
pub fn StatusPhaseView() -> Element {
    rsx! {
        document::Stylesheet { href: STATUS_PHASE_VIEW_SCSS }

        div { class: "status-phase-container",
            StatusPhaseActionsView {}
            StatusPhaseInstructionsView {}
        }
    }
}
