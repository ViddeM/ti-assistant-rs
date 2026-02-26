use dioxus::prelude::*;
use ti_helper_game_data::{
    components::strategy_card::StrategyCard, state::game_state::ActionPhaseProgress,
};

use crate::{
    components::info_button::InfoButton,
    data::{game_context::GameContext, info_context::Info},
};

const STRATEGY_CARD_INFO_SCSS: Asset =
    asset!("/assets/styling/views/players_sidebar/parts/strategy_card_info.scss");

#[component]
pub fn StrategyCardInfo(cards: ReadSignal<Vec<StrategyCard>>) -> Element {
    let gc = use_context::<GameContext>();

    let spent_cards = use_memo(move || gc.game_state().spent_strategy_cards.clone());
    let active_card = use_memo(move || {
        gc.game_state()
            .action_progress
            .clone()
            .map(|prog| match prog {
                ActionPhaseProgress::Strategic(strategic_progress) => Some(strategic_progress.card),
                _ => None,
            })
            .flatten()
    });

    rsx! {
        document::Stylesheet { href: STRATEGY_CARD_INFO_SCSS }

        div { class: "strategy-cards-container",
            for card in cards().iter() {
                div {
                    key: "{card}",
                    class: if active_card().as_ref().eq(&Some(card)) { "card-active" } else if spent_cards().contains(card) { "card-played" },
                    class: "card-container style-{card.name()}",
                    p { "{card}" }
                    InfoButton { info: Info::Strategy(card.clone()) }
                }
            }
        }
    }
}
