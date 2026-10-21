use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    state::expedition::{TECompletion, TEExpeditionProgress},
};

use crate::{
    components::{
        button::Button,
        dropdown::PlayerDropdown,
        faction_icon::FactionIcon,
        ti_icon::{TiIcon, TiIconType},
    },
    data::game_context::GameContext,
};

#[component]
pub fn ThundersEdgeViewMode() -> Element {
    let gc = use_context::<GameContext>();

    let thunders_edge = use_memo(move || gc.game_state().te_expedition.clone());

    rsx! {
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
fn Expedition(progress: TEExpeditionProgress) -> Element {
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

            ExpeditionOption {
                text: "Spend 5 resources",
                picked_by: progress.resources.clone(),
                selected_player,
            }

            ExpeditionOption {
                text: "Discard 2 Action Cards",
                picked_by: progress.action_cards.clone(),
                selected_player,
            }

            ExpeditionOption {
                text: "Spend 5 influence",
                picked_by: progress.influence.clone(),
                selected_player,
            }

            ExpeditionOption {
                text: "Discard 1 unscored Secret Objective",
                picked_by: progress.secret_objective.clone(),
                selected_player,
            }

            ExpeditionOption {
                text: "Exhaust 1 Technology Speciality Planet",
                picked_by: progress.tech_speciality_planet.clone(),
                selected_player,
            }

            ExpeditionOption {
                text: "Spend 3 Trade Goods",
                picked_by: progress.trade_goods.clone(),
                selected_player,
            }
        }
    }
}

#[component]
fn ExpeditionOption(
    text: &'static str,
    picked_by: Option<PlayerId>,
    selected_player: ReadSignal<Option<PlayerId>>,
) -> Element {
    let gc = use_context::<GameContext>();

    let p1 = picked_by.clone();
    let can_change = use_memo(move || selected_player().is_some() && !p1.eq(&selected_player()));

    let p2 = picked_by.clone();
    let player_faction = use_memo(move || {
        if let Some(p) = &p2 {
            let f = gc.game_state().players[p].faction;
            rsx! {
                FactionIcon { faction: f }
            }
        } else {
            rsx! {}
        }
    });

    rsx! {
        p { class: "margn-top", "{text}" }
        if let Some(p) = picked_by.as_ref() {
            p { "{p}" }
            {player_faction()}
        }
        Button {
            disabled: !can_change(),
            onclick: move |_| {
                println!("TODO: NOT IMPLEMENTED");
            },
            "Claim"
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
