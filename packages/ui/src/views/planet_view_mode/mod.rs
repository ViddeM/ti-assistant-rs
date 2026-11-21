use dioxus::prelude::*;
use ti_helper_game_data::common::expansions::Expansion;

use crate::{
    data::game_context::GameContext,
    views::{
        destroyed_planets::DestroyedPlanets,
        planet_view_mode::{
            add_planet_attachment::AddPlanetAttachment, coexistence_card::CoexistenceCard,
            player_planets_grid::PlayerPlanetsGrid, unclaimed_planets_table::UnclaimedPlanetsTable,
        },
    },
};

mod add_planet_attachment;
mod coexistence_card;
mod player_planets_grid;
mod unclaimed_planets_table;

const PLANET_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/planet_view_mode.scss");

#[component]
pub fn PlanetViewMode() -> Element {
    let gc = use_context::<GameContext>();

    rsx! {
        document::Stylesheet { href: PLANET_VIEW_MODE_SCSS }

        div { class: "planet-view-container",
            PlayerPlanetsGrid {}
            if gc.game_state().game_settings.expansions.is_enabled(&Expansion::ThundersEdge) {
                CoexistenceCard {}
            }
            if !gc.game_options().planet_attachments.is_empty() {
                AddPlanetAttachment {}
            }
            UnclaimedPlanetsTable {}
            DestroyedPlanets {}
        }
    }
}
