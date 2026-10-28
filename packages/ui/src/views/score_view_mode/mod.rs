use dioxus::prelude::*;

use crate::views::score_view_mode::{
    relic_owners_view::RelicOwnersView, reveal_objective_form::RevealObjectiveForm,
    score_table_view::ScoreTableView, secret_objectives_view::SecretObjectivesView,
    support_for_the_throne_view::SupportForTheThroneView,
};

pub mod relic_owners_view;
pub mod reveal_objective_form;
pub mod score_table_view;
pub mod secret_objectives_view;
pub mod support_for_the_throne_view;

const SCORE_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/score_view_mode.scss");

#[component]
pub fn ScoreViewMode() -> Element {
    rsx! {
        document::Stylesheet { href: SCORE_VIEW_MODE_SCSS }

        div { class: "score-view-container",
            ScoreTableView {}
            RevealObjectiveForm {}
            SecretObjectivesView {}
            SupportForTheThroneView {}
            RelicOwnersView {}
        }
    }
}
