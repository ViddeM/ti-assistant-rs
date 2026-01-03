use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::faction::Faction,
    components::strategy_card::StrategyCard,
    state::game_state::{ActionPhaseProgress, StrategicProgress},
};

use crate::{
    components::{button::Button, info_button::InfoButton},
    data::{
        event_context::EventContext, game_context::GameContext, info_context::Info,
        player_view::PlayerViewContext,
    },
    views::phase_views::strategy_card::{
        primary_views::politics_primary_view::PoliticsPrimaryView,
        secondary_card::generic_strategy_card::GenericStrategyCard,
    },
};

pub mod primary_views;
pub mod secondary_card;

const STRATEGY_CARD_SCSS: Asset = asset!("/assets/styling/views/strategy_card/strategy_card.scss");

#[component]
pub fn StrategyCardView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let card = use_memo(move || {
        gc.game_state()
            .action_progress
            .clone()
            .map(|p| match p {
                ActionPhaseProgress::Strategic(strategic_progress) => Some(strategic_progress),
                _ => None,
            })
            .flatten()
    });

    // TODO: Solve reactivity here.
    let Some(progress) = card() else {
        return rsx! {};
    };

    let progress = Arc::new(progress);

    let p1 = progress.clone();
    let card = use_memo(move || p1.card);

    let expected_secondaries = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .filter(|&(id, _)| gc.game_state().current_player.as_ref().eq(&Some(id)))
            .filter(|&(_, p)| {
                p.faction == Faction::NekroVirus && card() == StrategyCard::Technology
            })
            .count()
    });

    let p2 = progress.clone();
    let primary_done = use_memo(move || p2.is_done());
    let p3 = progress.clone();
    let secondary_done = use_memo(move || p3.other_players.len() == expected_secondaries());

    rsx! {
        document::Stylesheet { href: STRATEGY_CARD_SCSS }

        div { class: "card strategy-card-view",
            h2 { "{card()}" }

            InfoButton { info: Info::Strategy(card()) }

            div { class: "part-divider" }
            h6 { "Primary" }
            StrategyCardPrimary { progress: progress.clone() }

            div { class: "part-divider" }
            h6 { "Secondary" }

            if view.is_active() {
                "{primary_done()} :: {secondary_done()} :: {view.is_active()}"
                Button {
                    class: "margin-top",
                    disabled: !primary_done() || !secondary_done() || !view.is_active(),
                    onclick: move |_| { event.send_event(Event::StrategicActionCommit) },
                    "Submit"
                }
            }
        }
    }
}

#[component]
fn StrategyCardPrimary(progress: Arc<StrategicProgress>) -> Element {
    match progress.card {
        StrategyCard::Politics => rsx! {
            PoliticsPrimaryView {}
        },
        StrategyCard::Technology => todo!(),
        StrategyCard::Imperial => todo!(),
        _ => {
            rsx! {
                if progress.card == StrategyCard::Leadership {
                    p { class: "warning-text", "Remember: pay 3 influence per extra token" }
                }
                p { "Primary not tracked" }
            }
        }
    }
}

#[component]
fn StrategyCardSecondary(progress: Arc<StrategicProgress>) -> Element {
    match progress.card {
        StrategyCard::Technology => todo!(),
        _ => rsx! {
            GenericStrategyCard { progress: progress.clone() }
        },
    }
}
