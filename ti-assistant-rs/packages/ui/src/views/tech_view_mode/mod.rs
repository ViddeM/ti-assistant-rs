use dioxus::prelude::*;

use crate::views::tech_view_mode::{tech_ledger::TechLedger, tech_table::TechTable};

mod tech_ledger;
mod tech_table;

const TECH_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/tech_view_mode.scss");

#[component]
pub fn TechViewMode() -> Element {
    rsx! {
        document::Stylesheet { href: TECH_VIEW_MODE_SCSS }

        div { class: "tech-view-container",
            TechLedger {}
            TechTable {}
        }
    }
}
