use std::str::FromStr;

use dioxus::prelude::*;
use strum::IntoEnumIterator;
use ti_helper_game_data::{
    actions::{
        event::{ActionCardAction, Event},
        strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
    },
    common::{faction::Faction, player_id::PlayerId},
    components::{
        action_card::ActionCard,
        strategy_card::StrategyCard,
        tech::{TechOrigin, TechType, Technology},
    },
    state::game_state::{ActionPhaseProgress, PlanetLocation},
};

use crate::{
    components::{
        button::Button,
        dropdown::{Dropdown, PlanetDropdown, PlayerDropdown, TechDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::{
        phase_views::strategy_card::{
            STRATEGY_CARD_SCSS,
            primary_views::{
                imperial_primary_view::ImperialPrimaryView,
                politics_primary_view::PoliticsPrimaryView,
                technology_primary_view::TechnologyPrimaryView,
            },
            secondary_card::generic_strategy_card::{action_for_card, get_cost_warning},
        },
        select_tech::SelectTechView,
    },
};

const ACTION_CARD_SCSS: Asset = asset!("/assets/styling/views/phase_views/action_card.scss");

// TODO: Go over action cards as part of the roadmap.
#[component]
pub fn ActionCardView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::ActionCard(card)) => {
            rsx! {
                document::Stylesheet { href: ACTION_CARD_SCSS }

                div { class: "card column",
                    h2 { "{card.card.info().name}" }
                    {
                        if view.is_active() {
                            rsx! {
                                ActionCardProgressView { card: card.card.clone() }
                            }
                        } else {
                            rsx! {
                                p { "Not your turn, currently {current_player()} is playing" }
                            }
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

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist")
    });

    let send_commit_message = use_callback(move |action| {
        event.send_event(Event::ActionCardActionCommit {
            player: current_player_id(),
            data: action,
        })
    });

    match card {
        ActionCard::FocusedResearch => {
            rsx! {
                div {
                    SelectTechView {
                        player_id: current_player_id(),
                        on_select: move |tech| send_commit_message(
                            Some(ActionCardAction::FocusedResearch {
                                tech,
                            }),
                        ),
                    }
                }
            }
        }
        ActionCard::DivertFunding => {
            rsx! {
                DivertFundingView { send_commit_message }
            }
        }
        ActionCard::Plagiarize => {
            rsx! {
                PlagiarizeView { send_commit_message }
            }
        }
        ActionCard::Brilliance => {
            rsx! {
                BrillianceView { send_commit_message }
            }
        }
        ActionCard::MercenaryContract => {
            rsx! {
                MercenaryContractView { send_commit_message }
            }
        }
        ActionCard::Overrule => {
            rsx! {
                OverruleView { send_commit_message }
            }
        }
        ActionCard::Strategize => {
            rsx! {
                StrategizeView { send_commit_message }
            }
        }
        _ => {
            rsx! {
                div { class: "commit-button-row",
                    Button { onclick: move |_| send_commit_message(None), "Commit" }
                }
            }
        }
    }
}

#[component]
fn DivertFundingView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut remove_tech = use_signal(|| None);
    let mut gain_tech: Signal<Option<Technology>> = use_signal(|| None);

    let player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("There to be a current player in action phase")
    });

    let deletable_techs = use_memo(move || {
        let player = gc
            .game_state()
            .players
            .get(&player_id())
            .cloned()
            .expect("Current player to exist");

        player
            .technologies
            .iter()
            .filter(|t| {
                let i = t.info();
                if i.origin != TechOrigin::Base {
                    return false;
                }

                if i.tech_type == TechType::UnitUpgrade {
                    return false;
                }

                true
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "divert-funding-container",
            fieldset { class: "tech-change-container",
                legend { "Remove tech" }
                TechDropdown {
                    value: remove_tech,
                    on_select: move |tech| remove_tech.set(tech),
                    options: deletable_techs(),
                }
            }

            fieldset { class: "tech-change-container",
                legend { "Gain tech" }
                {
                    if let Some(t) = gain_tech.read().as_ref() {
                        rsx! {
                            p { "{t.info().name}" }
                        }
                    } else {
                        rsx! {
                            SelectTechView {
                                player_id: player_id(),
                                on_select: move |tech| {
                                    gain_tech.set(Some(tech));
                                },
                            }
                        }
                    }
                }
            }

            Button {
                disabled: remove_tech.read().is_none() || gain_tech.read().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::ActionCardActionCommit {
                            player: player_id(),
                            data: Some(ActionCardAction::DivertFunding {
                                remove_tech: remove_tech().expect("No remove tech selected"),
                                take_tech: gain_tech().expect("No gain tech selected"),
                            }),
                        })
                },
            }
        }
    }
}

#[component]
fn PlagiarizeView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();

    let selected_player = use_signal(|| None);
    let selected_tech = use_signal(|| None);

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("No current player")
    });
    let players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .keys()
            .filter(|&p| !p.eq(&current_player_id()))
            .cloned()
            .collect::<Vec<_>>();

        players.sort();

        players
    });

    rsx! {
        table { class: "plagiarize-table",
            thead {
                tr {
                    th { colspan: 2,
                        h6 { "Select neighbours tech" }
                    }
                }
                tr {
                    td { class: "align-center", "Player" }
                    td { class: "align-center", "Tech" }
                }
            }
            tbody {
                for player in players().iter() {
                    PlagiarizePlayerRow {
                        key: "{player}",
                        player_id: player.clone(),
                        selected_player,
                        selected_tech,
                    }
                }
            }
            tfoot {
                tr {
                    th { colspan: 2,
                        Button {
                            disabled: selected_player().is_none() || selected_tech().is_none(),
                            onclick: move |_| {
                                send_commit_message(
                                    Some(ActionCardAction::Plagiarize {
                                        tech: selected_tech().expect("Tech to be selected"),
                                    }),
                                )
                            },
                            "Plagiarize Tech"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BrillianceView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_player = use_signal(|| None);

    let players_without_breakthrough = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .filter(|(_, player)| player.breakthrough.is_none())
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "column",
            fieldset { class: "column",
                legend { "Choose a player to gain their breakthrough" }
                PlayerDropdown {
                    value: selected_player,
                    options: players_without_breakthrough(),
                    on_select: move |p| selected_player.set(p),
                    disabled: players_without_breakthrough().is_empty(),
                }
                Button {
                    class: "margin-top",
                    disabled: selected_player().is_none(),
                    onclick: move |_| {
                        send_commit_message(
                            Some(ActionCardAction::Brilliance {
                                player: selected_player(),
                            }),
                        )
                    },
                    "Gain breakthrough"
                }
            }
            Button {
                class: "margin-top",
                onclick: move |_| send_commit_message(
                    Some(ActionCardAction::Brilliance {
                        player: None,
                    }),
                ),
                "Ready a planet instead"
            }
        }
    }
}

#[component]
fn MercenaryContractView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();

    let mut selected_planet = use_signal(|| None);

    let other_players_planets = use_memo(move || {
        let state = gc.game_state();
        let current_player = state
            .current_player
            .clone()
            .expect("Current player to exist");
        let mut planets = state
            .players
            .iter()
            .filter(|&(id, _)| id != &current_player)
            .flat_map(|(_, player)| player.planets.keys())
            .filter(|planet| !planet.is_in_home_system())
            .filter(|planet| state.planet_location(planet) != PlanetLocation::NotOnBoard)
            .cloned()
            .collect::<Vec<_>>();
        planets.sort();
        planets
    });

    rsx! {
        div { class: "column",
            p { class: "warning-text", "Remember: spend 2 trade goods" }
            fieldset { class: "column",
                legend { "Planet owned by another player" }
                PlanetDropdown {
                    value: selected_planet,
                    options: other_players_planets(),
                    on_select: move |p| selected_planet.set(p),
                    disabled: other_players_planets().is_empty(),
                }
                Button {
                    class: "margin-top",
                    disabled: selected_planet().is_none(),
                    onclick: move |_| {
                        send_commit_message(
                            Some(ActionCardAction::MercenaryContract {
                                planet: selected_planet(),
                            }),
                        )
                    },
                    "Return planet card"
                }
            }
            Button {
                class: "margin-top",
                onclick: move |_| {
                    send_commit_message(
                        Some(ActionCardAction::MercenaryContract {
                            planet: None,
                        }),
                    )
                },
                "Planet not owned by another player"
            }
        }
    }
}

#[component]
fn OverruleView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();

    let card = use_signal(|| None);

    let is_nekro = use_memo(move || {
        let state = gc.game_state();
        let current_player = state
            .current_player
            .as_ref()
            .expect("Current player to exist");
        state.players[current_player].faction == Faction::NekroVirus
    });

    let commit = move |card, primary| {
        send_commit_message(Some(ActionCardAction::Overrule { card, primary }))
    };
    let on_commit =
        move |primary| commit(card().expect("Strategy card to be selected"), Some(primary));

    rsx! {
        document::Stylesheet { href: STRATEGY_CARD_SCSS }

        div { class: "column",
            ReadiedStrategyCardSelect { selected: card }
            match card() {
                None => rsx! {},
                Some(StrategyCard::Technology) if is_nekro() => rsx! {
                    p { "--Unable to research technologies--" }
                    Button {
                        onclick: move |_| {
                            commit(
                                StrategyCard::Technology,
                                Some(StrategicPrimaryAction::Technology {
                                    first_tech: None,
                                    second_tech: None,
                                }),
                            )
                        },
                        "Commit"
                    }
                },
                Some(StrategyCard::Technology) => rsx! {
                    TechnologyPrimaryView { on_commit }
                },
                Some(StrategyCard::Politics) => rsx! {
                    PoliticsPrimaryView { on_commit }
                },
                Some(StrategyCard::Imperial) => rsx! {
                    ImperialPrimaryView { on_commit }
                },
                Some(card) => rsx! {
                    p { "Primary not tracked" }
                    Button { onclick: move |_| commit(card, None), "Commit" }
                },
            }
        }
    }
}

#[component]
fn StrategizeView(send_commit_message: Callback<Option<ActionCardAction>>) -> Element {
    let gc = use_context::<GameContext>();

    let card = use_signal(|| None);

    let current_player_id = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist")
    });
    let is_nekro = use_memo(move || {
        gc.game_state().players[&current_player_id()].faction == Faction::NekroVirus
    });

    let commit = move |secondary| {
        send_commit_message(Some(ActionCardAction::Strategize {
            card: card().expect("Strategy card to be selected"),
            secondary,
        }))
    };

    rsx! {
        div { class: "column",
            ReadiedStrategyCardSelect { selected: card }
            match card() {
                None => rsx! {},
                Some(StrategyCard::Technology) if is_nekro() => rsx! {
                    p { "--Nekro Virus cannot research technologies--" }
                },
                Some(StrategyCard::Technology) => rsx! {
                    fieldset { class: "column",
                        legend { "Technology" }

                        p { class: "warning-text", "Remember: pay 1 token and 4 resources" }
                        SelectTechView {
                            player_id: current_player_id(),
                            on_select: move |tech| commit(StrategicSecondaryAction::Technology {
                                tech,
                            }),
                        }
                    }
                },
                Some(card) => rsx! {
                    fieldset { class: "column",
                        legend { "{card}" }

                        if let Some(cost) = get_cost_warning(&card, &gc.game_state().game_settings.expansions) {
                            p { class: "warning-text", "Remember {cost}" }
                        }
                        p { "Secondary not tracked" }
                        Button { class: "margin-top", onclick: move |_| commit(action_for_card(&card)), "Commit" }
                    }

                },
            }
        }
    }
}

/// Buttons to pick 1 of the strategy cards that are readied or haven't been chosen.
#[component]
fn ReadiedStrategyCardSelect(selected: Signal<Option<StrategyCard>>) -> Element {
    let gc = use_context::<GameContext>();

    let cards = use_memo(move || {
        StrategyCard::iter()
            .filter(|card| !gc.game_state().spent_strategy_cards.contains(card))
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "strategy-card-select",
            for card in cards() {
                Button {
                    key: "{card}",
                    disabled: selected() == Some(card),
                    onclick: move |_| selected.set(Some(card)),
                    "{card}"
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
struct PlagiarizePlayerRowProps {
    player_id: PlayerId,
    selected_player: Signal<Option<PlayerId>>,
    selected_tech: Signal<Option<Technology>>,
}

#[component]
fn PlagiarizePlayerRow(
    PlagiarizePlayerRowProps {
        player_id,
        mut selected_player,
        mut selected_tech,
    }: PlagiarizePlayerRowProps,
) -> Element {
    let gc = use_context::<GameContext>();
    let current_player_techs = use_memo(move || {
        let current_player = gc
            .game_state()
            .current_player
            .clone()
            .expect("Current player to exist");
        gc.game_state()
            .players
            .get(&current_player)
            .cloned()
            .expect("Current player to exist")
            .technologies
            .into_iter()
            .collect::<Vec<_>>()
    });
    let p1 = player_id.clone();
    let available_player_techs = use_memo(move || {
        let mut techs = gc
            .game_state()
            .players
            .get(&p1)
            .expect("Player to exist")
            .technologies
            .iter()
            .filter(|t| t.info().origin == TechOrigin::Base)
            .filter(|t| !current_player_techs().contains(t))
            .map(|t| (t.clone(), t.info()))
            .collect::<Vec<_>>();

        techs.sort_by(|(a, _), (b, _)| a.cmp(b));

        techs
    });

    let p2 = player_id.clone();
    let value = use_memo(move || {
        let tech = if selected_player().eq(&Some(p2.clone())) {
            selected_tech()
        } else {
            None
        };

        tech.map(|t| t.to_string()).unwrap_or_default()
    });

    rsx! {
        tr {
            td { class: "align-right", "{player_id}: " }
            td { class: "align-left",
                Dropdown {
                    disabled: available_player_techs().is_empty(),
                    value,
                    oninput: move |e: FormEvent| {
                        selected_player.set(Some(player_id.clone()));
                        let v = e.value();
                        if v.is_empty() {
                            selected_tech.set(None);
                        } else {
                            selected_tech.set(Some(Technology::from_str(&v).expect("Tech to be valid")))
                        }
                    },
                    option { value: "", "--Select a tech--" }
                    for (tech , tech_info) in available_player_techs().iter() {
                        option { key: "{tech}", value: "{tech.to_string()}", "{tech_info.name}" }
                    }

                }
            }
        }
    }
}
