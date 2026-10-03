use dioxus::prelude::*;

use crate::views::planet_view_mode::{
    add_planet_attachment::AddPlanetAttachment, player_planets_grid::PlayerPlanetsGrid,
    unclaimed_planets_table::UnclaimedPlanetsTable,
};

mod add_planet_attachment;
mod player_planets_grid;
mod unclaimed_planets_table;

const PLANET_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/planet_view_mode.scss");

#[component]
pub fn PlanetViewMode() -> Element {
    rsx! {
        document::Stylesheet { href: PLANET_VIEW_MODE_SCSS }

        div { class: "planet-view-container",
            PlayerPlanetsGrid {}
            AddPlanetAttachment {}
            UnclaimedPlanetsTable {}
        }
    }
}
