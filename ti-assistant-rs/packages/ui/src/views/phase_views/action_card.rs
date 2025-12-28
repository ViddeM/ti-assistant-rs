use dioxus::prelude::*;
use ti_helper_game_data::{
    components::action_card::ActionCard, state::game_state::ActionPhaseProgress,
};

use crate::data::{
    event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
};

const ACTION_CARD_SCSS: Asset = asset!("/assets/styling/views/phase_views/action_card.scss");

#[component]
pub fn ActionCardView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::ActionCard(card)) => {
            rsx! {
                document::Stylesheet { href: ACTION_CARD_SCSS }

                div { class: "card",
                    h2 { "{card.card.info().name}" }
                    if view.is_active() {
                        rsx! {
                            ActionCardProgressView { card_progress: card.card }
                        }
                    } else {
                        rsx! {
                            p { "Not your turn, currently {gc.game_state().current_player.unwrap()} is playing" }
                        }
                    }
                }
            }
        }
        _ => rsx! {},
    }
}

#[component]
fn ActionCardProgressView(card: ActionCard) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    match card {
        ActionCard::FocusedResearch => {
            rsx! {
                div {

                }
            }
        }
    }
}
