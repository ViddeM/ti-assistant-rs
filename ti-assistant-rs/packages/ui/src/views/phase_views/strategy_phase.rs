use std::collections::HashMap;

use dioxus::prelude::*;
use strum::IntoEnumIterator;
use ti_helper_game_data::{
    actions::event::Event, common::faction::Faction, components::strategy_card::StrategyCard,
};

use crate::{
    components::{
        button::{Button, ButtonBase},
        dropdown::Dropdown,
        faction_icon::FactionIcon,
        info_button::InfoButton,
        ti_icon::{TiIcon, TiIconType},
    },
    data::{
        event_context::EventContext,
        game_context::GameContext,
        info_context::{Info, InfoDescription},
    },
};

const STRATEGY_PHASE_SCSS: Asset = asset!("/assets/styling/views/phase_views/strategy_phase.scss");

#[component]
pub fn StrategyPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("There to be a current player?")
    });

    let selected_cards = use_memo(move || {
        gc.game_state()
            .strategy_card_holders
            .iter()
            .map(|(strategy_card, holder)| (strategy_card, holder))
            .map(|(card, player)| {
                let faction = gc
                    .game_state()
                    .players
                    .get(player)
                    .expect("Player to exist")
                    .faction;

                (card.clone(), faction)
            })
            .collect::<HashMap<_, _>>()
    });

    let finished_selecting_cards = use_memo(move || {
        let no_players = gc.game_state().players.len();

        if no_players > 4 {
            no_players == selected_cards().len()
        } else {
            no_players * 2 == selected_cards().len()
        }
    });

    let select_card = move |card: StrategyCard| {
        event.send_event(Event::TakeStrategyCard {
            player: current_player(),
            card,
        });
    };

    rsx! {
        document::Stylesheet { href: STRATEGY_PHASE_SCSS }

        div { class: "card select-strategy-card-card",
            h2 { "Select a strategy card" }
            div { class: "strategy-cards-container",
                {StrategyCard::iter().map(|s| rsx! {
                    div { key: "{s}", class: "select-card-row",
                        InfoButton { info: Info::Strategy(s) }
                        StrategyCardButton {
                            strategy_card: s,
                            selected_by_faction: selected_cards().get(&s).cloned(),
                            set_selected: move |_| select_card(s),
                            finished_selecting_cards: finished_selecting_cards(),
                        }
                    }
                })}

                NaaluTelepathy {}

                Button {
                    disabled: !finished_selecting_cards(),
                    onclick: move |_| event.send_event(Event::CompleteStrategyPhase),
                    "Start Action Phase"
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct StrategyCardButtonProps {
    strategy_card: StrategyCard,
    selected_by_faction: Option<Faction>,
    set_selected: EventHandler<()>,
    finished_selecting_cards: bool,
}

#[component]
fn StrategyCardButton(
    StrategyCardButtonProps {
        strategy_card,
        selected_by_faction,
        set_selected,
        finished_selecting_cards,
    }: StrategyCardButtonProps,
) -> Element {
    let faction = use_memo(move || {
        if let Some(f) = selected_by_faction.as_ref() {
            rsx! {
                FactionIcon { faction: f.clone() }
            }
        } else {
            rsx! {}
        }
    });

    rsx! {
        ButtonBase {
            onclick: move |_| set_selected(()),
            disabled: selected_by_faction.is_some() || finished_selecting_cards,
            class: format!("strategy-card-button style-{}", strategy_card.name()),
            "{strategy_card.card_number()}."
            p { "{strategy_card}" }
            {faction()}
        }
    }
}

#[component]
fn NaaluTelepathy() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let naalu = use_memo(move || gc.game_state().naalu_telepathy.clone());
    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    if naalu().is_none() {
        return rsx! {};
    }

    rsx! {
        div { class: "naalu-telepathy",
            InfoButton {
                info: Info::Custom {
                    title: "Telepathic".to_string(),
                    subtitle: "Faction Ability / Promisary Note".to_string(),
                    description: InfoDescription::Custom(rsx! {
                        div {
                            br {}
                            h3 { "TELEPATHIC:" }
                            div {
                                "At the end of the strategy phase, place the Naalu 0 token on your strategy card; you are first in initiative order."
                            }
                            br {}
                            br {}
                            h3 { "GIFT OF PRESCIENCE:" }
                            div { "At the end of the strategy phase:" }
                            br {}
                            div {
                                "Place this card face-up in your play area and place the
                                                                                                                                                                                                                                                                                                                        Naalu 0 token on your strategy card; you are first in the
                                                                                                                                                                                                                                                                                                                        initiative order. The Naalu player cannot use their
                                                                                                                                                                                                                                                                                                                        TELEPATHIC faction ability during this game round."
                            }
                            br {}
                            div { "Return this card to the Naalu player at the end of the status phase." }
                        }
                    }),
                },
            }
            div { class: "naalu-telepathy-body",
                div { class: "naalu-telepathy-header",
                    TiIcon {
                        icon: TiIconType::Naalu0Token,
                        width: 32,
                        height: 32,
                    }
                    "Telepathic / Gift of Prescience"
                }
                Dropdown {
                    required: true,
                    id: "naalu_telepathy_dropdown",
                    value: "{naalu().unwrap()}",
                    oninput: move |e: FormEvent| {
                        event
                            .send_event(Event::PlayGiftOfPrescience {
                                player: e.value().into(),
                            })
                    },
                    {
                        players
                            .iter()
                            .map(|p| {
                                rsx! {
                                    option { key: "{p}", value: "{p}", "{p}" }
                                }
                            })
                    }
                }
            }
        }
    }
}
