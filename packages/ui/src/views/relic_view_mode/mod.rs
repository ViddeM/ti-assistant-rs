use dioxus::prelude::*;

use crate::views::{
    book_of_latvinia_research::BookOfLatviniaResearch,
    relic_view_mode::{give_relic::GiveRelic, player_relics_grid::PlayerRelicsGrid},
};

mod give_relic;
mod player_relics_grid;

const RELIC_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/relic_view_mode.scss");

#[component]
pub fn RelicViewMode() -> Element {
    rsx! {
        document::Stylesheet { href: RELIC_VIEW_MODE_SCSS }

        div { class: "relic-view-container",
            PlayerRelicsGrid {}
            GiveRelic {}
            BookOfLatviniaResearch {}
        }
    }
}
