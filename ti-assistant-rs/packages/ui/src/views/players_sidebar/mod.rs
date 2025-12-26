use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{common::player_id::PlayerId, components::strategy_card::StrategyCard};

use crate::{
    components::{dropdown::Dropdown, faction_icon::get_faction_icon, info_button::InfoButton},
    data::{
        game_context::GameContext,
        player_view::{PlayerView, PlayerViewContext},
    },
    views::players_sidebar::parts::strategy_card_info::StrategyCardInfo,
};

pub mod parts;

const PLAYERS_SIDEBAR_SCSS: Asset =
    asset!("/assets/styling/views/players_sidebar/players_sidebar.scss");

#[component]
pub fn PlayersSidebar() -> Element {
    let gc = use_context::<GameContext>();
    let mut view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || gc.game_state().current_player.clone());
    let next_player = use_memo(move || {
        if let Some(current) = current_player() {
            if let Some(next) = gc
                .game_state()
                .next_player_after(&current)
                .expect("Failed to retrieve next player")
            {
                return next.to_string();
            }
        }
        "None".to_string()
    });

    let handle_playing_as_change = move |event: FormEvent| {
        let v = event.value();

        if v.is_empty() {
            view.set_global();
        } else {
            let player_id = PlayerId::from(v);
            view.set_player(player_id);
        }
    };

    let currently_viewing_as = use_memo(move || match view.get()() {
        PlayerView::Global => "".to_string(),
        PlayerView::Player { player_id } => player_id.to_string(),
    });

    rsx! {
        document::Stylesheet { href: PLAYERS_SIDEBAR_SCSS }

        div { class: "card",
            h2 { "Players" }
            label { r#for: "playing-as-dropdown", "Selected player view: " }
            Dropdown {
                id: "playing-as-dropdown",
                value: "{currently_viewing_as()}",
                oninput: handle_playing_as_change,
                option { value: "", "Global view" }
                {gc.game_state().players.keys().map(|p| rsx! {
                    option { value: p.to_string(), "{p}" }
                })}
            }
            div { class: "player-side-bar-card",
                {
                    if let Some(current) = current_player() {
                        rsx! {
                            fieldset { class: "player-box-container",
                                legend { "Current player" }
                                "{current}"
                            }
                            fieldset { class: "player-box-container",
                                legend { "Next up" }
                                "{next_player()}"
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
                {gc.game_state().turn_order.iter().map(|p| rsx! {
                    PlayerBox { player_id: Arc::clone(&p) }
                })}
            }
        }
    }
}

#[component]
fn PlayerBox(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let p = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p)
            .expect("Player to exist")
            .clone()
    });

    let p2 = player_id.clone();
    let is_active = use_memo(move || gc.game_state().current_player.as_ref().eq(&Some(&p2)));
    let p3 = player_id.clone();
    let has_passed = use_memo(move || gc.game_state().passed_players.contains(&p3));
    let p4 = player_id.clone();
    let is_speaker = use_memo(move || gc.game_state().speaker.as_ref().eq(&Some(&p4)));
    let state_style = use_memo(move || {
        if is_active() {
            "active-player"
        } else if has_passed() {
            "passed-player"
        } else {
            ""
        }
    });

    let player_name_display = use_memo(move || {
        format!(
            "{}{}",
            player().name,
            if is_speaker() { " - Speaker" } else { "" }
        )
    });

    let p5 = player_id.clone();
    let player_strategy_cards = use_memo(move || {
        let mut strategy_cards = gc
            .game_state()
            .strategy_card_holders
            .iter()
            .filter(|&(_, p)| p.eq(&p5))
            .map(|(card, _)| card)
            .cloned()
            .collect::<Vec<_>>();
        strategy_cards.sort_by(|a, b| b.card_number().cmp(&a.card_number()));
        strategy_cards
    });

    rsx! {
        fieldset {
            background_image: format!("url(\"{}\")", get_faction_icon(&player().faction)),
            background_size: "contain",
            background_repeat: "no-repeat",
            background_position: "center",
            class: format!(
                "player-color-border-{0} player-box-container player-color-{0} {1}",
                player().color.name(),
                state_style(),
            ),
            legend { class: format!("player-box-legend player-color-border-{}", player().color.name()),
                h6 { class: "player-name", "{player_name_display()}" }
            }
            div { class: "content-row",
                StrategyCardInfo { cards: player_strategy_cards() }
                div { class: "score-time-container" }
            }
                // TOOD: Player resources
        }
    }
}
