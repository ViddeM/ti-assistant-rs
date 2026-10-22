use std::collections::BTreeMap;

use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    state::expedition::{TECompletion, TEExpeditionSlot},
};

use crate::{
    components::dropdown::PlayerDropdown, data::game_context::GameContext,
    views::thunders_edge_view_mode::expedition_token::ExpeditionToken,
};

mod expedition_token;

const THUNDERS_EDGE_VIEW_MODE_SCSS: Asset =
    asset!("/assets/styling/views/thunders_edge_view_mode.scss");

#[component]
pub fn ThundersEdgeViewMode() -> Element {
    let gc = use_context::<GameContext>();

    let thunders_edge = use_memo(move || gc.game_state().te_expedition.clone());

    rsx! {
        document::Stylesheet { href: THUNDERS_EDGE_VIEW_MODE_SCSS }

        if let Some(thunders_edge) = thunders_edge() {
            Expedition { progress: thunders_edge.progress.clone() }
            Completed { completed: thunders_edge.completed.clone() }
        } else {
            div { class: "card te-error-card",
                p { "Thunder's edge not enabled but you are in Thunder's Edge view mode?" }
                p { "This is a bug, please report it!" }
            }
        }
    }
}

#[component]
fn Expedition(progress: BTreeMap<TEExpeditionSlot, PlayerId>) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_player: Signal<Option<PlayerId>> = use_signal(|| None);

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "card column",
            h3 { "Thunder's Edge Expedition" }

            p { "Select a player to claim" }
            PlayerDropdown {
                value: selected_player,
                options: players(),
                on_select: move |p| selected_player.set(p),
            }

            ExpeditionToken { progress, selected_player }
        }
    }
}

#[component]
fn Completed(completed: Option<TECompletion>) -> Element {
    let gc = use_context::<GameContext>();

    rsx! {
        div { class: "card margin-top",
            h3 { "Completed" }
        }
    }
}
