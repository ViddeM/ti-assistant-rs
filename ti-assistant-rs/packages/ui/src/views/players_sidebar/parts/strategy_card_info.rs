use dioxus::prelude::*;
use ti_helper_game_data::components::strategy_card::StrategyCard;

use crate::{components::info_button::InfoButton, data::info_context::Info};

const STRATEGY_CARD_INFO_SCSS: Asset =
    asset!("/assets/styling/views/players_sidebar/parts/strategy_card_info.scss");

#[component]
pub fn StrategyCardInfo(cards: Vec<StrategyCard>) -> Element {
    rsx! {
        document::Stylesheet { href: STRATEGY_CARD_INFO_SCSS }

        div { class: "strategy-cards-container",
            {cards.iter().map(|c| rsx! {
                div { key: "{c}", class: format!("card-container style-{}", c.name()),
                    p { "{c}" }
                    InfoButton { info: Info::Strategy(c.clone()) }
                }
            })}
        }
    }
}
