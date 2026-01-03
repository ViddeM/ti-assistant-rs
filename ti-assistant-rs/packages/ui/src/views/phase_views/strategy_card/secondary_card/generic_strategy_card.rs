use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_regular_icons::{FaCircleCheck, FaCircleXmark},
    Icon,
};
use ti_helper_game_data::{
    actions::{
        event::Event,
        strategic::{StrategicSecondaryAction, StrategicSecondaryProgress},
    },
    components::strategy_card::StrategyCard,
    state::game_state::StrategicProgress,
};

use crate::{
    components::{button::Button, faction_icon::FactionIcon},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn GenericStrategyCard(progress: ReadSignal<StrategicProgress>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let secondary_players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .iter()
            .filter(|&(id, _)| !gc.game_state().current_player.as_ref().eq(&Some(id)))
            .map(|(id, player)| (id.clone(), player.clone()))
            .collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| a.cmp(b));
        players
    });

    let other_players = use_memo(move || progress().other_players);

    rsx! {
        div { class: "generic-secondary-container",
            for (id , player) in secondary_players() {
                fieldset { key: "{id}", class: "generic-player-container",
                    legend { class: "aligned-legend",
                        h6 { class: "horizontal-padding", "{id}" }
                        " "
                        FactionIcon { faction: player.faction }
                    }
                    if let Some(state) = other_players().get(&id) {
                        RenderPerformedAction { skipped: state.eq(&StrategicSecondaryProgress::Skipped) }
                    } else if view.display_for(id.clone()) {
                        RenderSecondaryAction {
                            card: progress().card,
                            send_secondary_message: {
                                let p = id.clone();
                                move |action| {
                                    event
                                        .send_event(Event::StrategicActionSecondary {
                                            player: p.clone(),
                                            action,
                                        })
                                }
                            },
                        }
                    } else {
                        p { "Has yet to choose" }
                    }
                }
            }
        }
    }
}

#[component]
fn RenderPerformedAction(skipped: bool) -> Element {
    rsx! {
        div { class: "action-container",
            if skipped {
                Icon { icon: FaCircleXmark, class: "skipped" }
            } else {
                Icon { icon: FaCircleCheck, class: "performed" }
            }
        }
    }
}

#[component]
fn RenderSecondaryAction(
    card: StrategyCard,
    send_secondary_message: EventHandler<StrategicSecondaryAction>,
) -> Element {
    let cost_warning = get_cost_warning(&card);

    rsx! {
        if let Some(c) = cost_warning {
            p { class: "warning-text", "Remember {c}" }
        }

        div { class: "buttons-container",
            Button { onclick: move |_| send_secondary_message(StrategicSecondaryAction::Skip),
                "Skip"
            }
            Button { onclick: move |_| send_secondary_message(action_for_card(&card)),
                "Play"
            }
        }
    }
}

fn action_for_card(card: &StrategyCard) -> StrategicSecondaryAction {
    match card {
        StrategyCard::Leadership => StrategicSecondaryAction::Leadership,
        StrategyCard::Diplomacy => StrategicSecondaryAction::Diplomacy,
        StrategyCard::Politics => StrategicSecondaryAction::Politics,
        StrategyCard::Construction => StrategicSecondaryAction::Construction,
        StrategyCard::Trade => StrategicSecondaryAction::Trade,
        StrategyCard::Warfare => StrategicSecondaryAction::Warfare,
        StrategyCard::Imperial => StrategicSecondaryAction::Imperial,
        _ => panic!("Unexpected strategy card for generic strategy card secondary {card}"),
    }
}

fn get_cost_warning(card: &StrategyCard) -> Option<String> {
    Some(
        match card {
            StrategyCard::Leadership => "pay 3 influence per token",
            StrategyCard::Diplomacy => "pay 1 token",
            StrategyCard::Politics => "pay 1 token",
            StrategyCard::Construction => "place token in system",
            StrategyCard::Trade => "pay 1 token",
            StrategyCard::Warfare => "pay 1 token",
            StrategyCard::Imperial => "pay 1 token",
            _ => return None,
        }
        .to_string(),
    )
}
