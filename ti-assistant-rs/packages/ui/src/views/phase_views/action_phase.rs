use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{
        action_card::{ActionCard, ActionCardPlay},
        leaders::LeaderAbilityKind,
        relic::RelicPlay,
        strategy_card::StrategyCard,
    },
    state::game_state::GameState,
};

use crate::{
    components::{
        button::Button,
        dropdown::{ActionCardDropdown, RelicDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

const ACTION_PHASE_SCSS: Asset = asset!("/assets/styling/views/phase_views/action_phase.scss");

#[component]
pub fn ActionPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to be set in action phase")
    });

    let current_player = use_memo(move || {
        gc.game_state()
            .players
            .get(&current_player_id())
            .cloned()
            .expect("Current player to exist")
    });

    let mut is_component = use_signal(|| false);

    let playable_strategy_cards =
        use_memo(move || get_playable_strategy_cards(gc.game_state(), current_player_id()));

    let player_name = use_memo(move || {
        format!(
            "{} {}",
            current_player_id(),
            player_emoji(&current_player_id())
        )
    });

    rsx! {
        document::Stylesheet { href: ACTION_PHASE_SCSS }

        div { class: "card action-phase-view-container",
            h2 { "ACTION PHASE" }
            if view.is_active() {
                fieldset {
                    class: format!(
                        "player-color-border-{} action-player-container",
                        current_player().color.name(),
                    ),
                    legend {
                        class: format!(
                            "player-color-border-{} action-player-container",
                            current_player().color.name(),
                        ),
                        h4 { "{player_name}" }
                    }
                    div { class: "actions-container",
                        Button {
                            class: "action-button",
                            disabled: !playable_strategy_cards().is_empty(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::PassAction {
                                        player: current_player_id(),
                                    })
                            },
                            "Pass"
                        }
                        {playable_strategy_cards().into_iter().map(|card| rsx! {
                            Button {
                                key: "{card}",
                                class: "action-button",
                                onclick: move |_| {
                                    event
                                        .send_event(Event::StrategicActionBegin {
                                            player: current_player_id(),
                                            card,
                                        })
                                },
                                "{card}"
                            }
                        })}
                        Button {
                            class: "action-button",
                            onclick: move |_| {
                                event
                                    .send_event(Event::TacticalActionBegin {
                                        player: current_player_id(),
                                    })
                            },
                            "Tactical"
                        }
                        Button {
                            class: "action-button",
                            disabled: is_component(),
                            onclick: move |_| { is_component.set(true) },
                            "Component"
                        }
                    }
                    if is_component() {
                        ComponentSelectRow {}
                    }
                }
            } else {
                p { "Not your turn, current {current_player_id()} is playing" }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ComponentMode {
    ActionCard,
    PlayRelic,
    GainRelic,
    FrontierCard,
    PlayLeader,
    None,
}

#[component]
fn ComponentSelectRow() -> Element {
    let gc = use_context::<GameContext>();

    let mut component_mode = use_signal(|| ComponentMode::None);

    let can_play_leaders = use_memo(move || {
        let current_player = gc.game_state().current_player.clone();
        let available_leaders = current_player
            .map(|p| {
                gc.game_state()
                    .available_leaders
                    .get(&p)
                    .expect("Players leaders to be in map")
                    .clone()
            })
            .unwrap_or_default();
        available_leaders
            .into_iter()
            .filter(|l| l.info().ability_kind() == LeaderAbilityKind::Action)
            .count()
            > 0
    });

    rsx! {
        div { class: "component-select-container",
            Button {
                disabled: component_mode.read().eq(&ComponentMode::ActionCard),
                onclick: move |_| component_mode.set(ComponentMode::ActionCard),
                "Action Card"
            }
            Button {
                disabled: component_mode.read().eq(&ComponentMode::GainRelic)
                    || gc.game_options().relics.is_empty(),
                onclick: move |_| component_mode.set(ComponentMode::GainRelic),
                "Gain Relic"
            }
            Button {
                disabled: component_mode.read().eq(&ComponentMode::PlayRelic)
                    || gc.game_options().relics.is_empty(),
                onclick: move |_| component_mode.set(ComponentMode::PlayRelic),
                "Play Relic"
            }
            Button {
                disabled: component_mode.read().eq(&ComponentMode::FrontierCard)
                    || gc.game_options().frontier_cards.is_empty(),
                onclick: move |_| component_mode.set(ComponentMode::FrontierCard),
                "Frontier Card"
            }
            Button {
                disabled: component_mode.read().eq(&ComponentMode::PlayLeader) || !can_play_leaders(),
                onclick: move |_| component_mode.set(ComponentMode::PlayLeader),
                "Play Leader"
            }
        }
        if !component_mode.read().eq(&ComponentMode::None) {
            DisplayComponentMode { mode: component_mode() }
        }
    }
}

#[component]
fn DisplayComponentMode(mode: ReadSignal<ComponentMode>) -> Element {
    match &*mode.read() {
        ComponentMode::ActionCard => rsx! {
            ActionCardSelectView {}
        },
        ComponentMode::PlayRelic => rsx! {
            PlayRelicView {}
        },
        ComponentMode::GainRelic => rsx! {
            GainRelicView {}
        },
        ComponentMode::FrontierCard => todo!(),
        ComponentMode::PlayLeader => todo!(),
        ComponentMode::None => rsx! {
            p { "Invalid display mode None" }
        },
    }
}

#[component]
fn ActionCardSelectView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let action_cards = use_memo(move || {
        gc.game_options()
            .action_cards
            .iter()
            .filter(|(_, info)| info.play == ActionCardPlay::Action)
            .map(|(c, _)| c)
            .cloned()
            .collect::<Vec<_>>()
    });

    let mut card: Signal<Option<ActionCard>> = use_signal(|| None);

    rsx! {
        fieldset { class: "play-action-card-container",
            legend { "Play Action Card" }
            ActionCardDropdown {
                value: card,
                options: action_cards(),
                on_select: move |c| card.set(c),
            }
            Button {
                disabled: card().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::ActionCardActionBegin {
                            player: gc
                                .game_state()
                                .current_player()
                                .expect("No current player when playing action card"),
                            card: card().expect("No action card selected"),
                        })
                },
                "Play"
            }
        }
    }
}

#[component]
fn PlayRelicView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_relic = use_signal(|| None);

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let available_relics = use_memo(move || {
        let mut relics = gc
            .game_state()
            .players
            .get(&current_player_id())
            .expect("Current player to exist")
            .relics
            .iter()
            .filter(|r| r.info().play == RelicPlay::Action)
            .cloned()
            .collect::<Vec<_>>();

        relics.sort();

        relics
    });

    rsx! {
        div {
            fieldset { class: "play-action-card-container",
                legend { "Play Relic" }
                RelicDropdown {
                    value: selected_relic(),
                    disabled: available_relics().is_empty(),
                    on_select: move |r| selected_relic.set(r),
                    options: available_relics(),
                }
                Button {
                    disabled: selected_relic().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::RelicActionBegin {
                                player: current_player_id(),
                                relic: selected_relic().expect("Relic to be set"),
                            })
                    },
                    "Play"
                }
            }
        }
    }
}

#[component]
fn GainRelicView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_relic = use_signal(|| None);

    let taken_relics = use_memo(move || {
        gc.game_state()
            .players
            .values()
            .map(|p| p.relics.iter())
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    });

    let available_relics = use_memo(move || {
        let mut relics = gc
            .game_options()
            .relics
            .keys()
            .filter(|r| !taken_relics().contains(r))
            .cloned()
            .collect::<Vec<_>>();

        relics.sort();

        relics
    });

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to be set in action phase")
    });

    rsx! {
        div {
            fieldset { class: "play-action-card-container",
                legend { "Gain Relic" }
                RelicDropdown {
                    value: selected_relic(),
                    options: available_relics(),
                    on_select: move |r| selected_relic.set(r),
                }
                Button {
                    disabled: selected_relic().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::GainRelicAction {
                                player: current_player_id(),
                                relic: selected_relic().expect("Relic to be set"),
                            })
                    },
                    "Gain"
                }
            }
        }
    }
}

fn get_playable_strategy_cards(
    game_state: Arc<GameState>,
    current_player: PlayerId,
) -> Vec<StrategyCard> {
    let mut cards = game_state
        .strategy_card_holders
        .iter()
        .filter(|&(s, _)| !game_state.spent_strategy_cards.contains(s))
        .filter(|&(_, p)| p.eq(&current_player))
        .map(|(card, _)| card)
        .cloned()
        .collect::<Vec<_>>();

    cards.sort();

    cards
}

fn player_emoji(name: &PlayerId) -> &str {
    match name.as_ref() {
        "portals" => "👑",
        "potholes" => "❤️",
        "tux" => "🐢",
        "sponken" => "👺",
        "vidde" => "🛸",
        "swexbe" => "💥",
        "håll" => "🧬",
        "hoidi" => "🐦",
        "gurr" => "⛴️️",
        _ => "",
    }
}
