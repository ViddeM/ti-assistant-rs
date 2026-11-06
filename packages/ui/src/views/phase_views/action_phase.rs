use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{Event, GalacticEventAction},
    common::{
        map::{Coordinate, HexPosition},
        player_id::PlayerId,
    },
    components::{
        action_card::{ActionCard, ActionCardPlay},
        breakthrough::{Breakthrough, BreakthroughKind},
        frontier_card::FrontierCardType,
        galactic_event::GalacticEvent,
        leaders::LeaderAbilityKind,
        relic::{Relic, RelicPlay},
        strategy_card::StrategyCard,
        tech::Technology,
    },
    state::{faction_state::FirmamentSide, game_state::GameState},
};

use crate::{
    components::{
        button::Button,
        dropdown::{
            ActionCardDropdown, Dropdown, FrontierCardDropdown, GalacticEventDropdown,
            PlanetDropdown, RelicDropdown, SystemDropdown,
        },
        info_button::InfoButton,
    },
    data::{
        event_context::EventContext, game_context::GameContext, info_context::Info,
        player_view::PlayerViewContext,
    },
    views::{
        destroyed_planets::planets_on_board,
        phase_views::breakthrough_action::BreakthroughActionView,
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
    use_effect(move || {
        let _ = current_player_id();
        is_component.set(false);
    });

    let playable_strategy_cards =
        use_memo(move || get_playable_strategy_cards(gc.game_state(), current_player_id()));

    let has_passed = use_memo(move || {
        gc.game_state()
            .passed_players
            .contains(&current_player_id())
    });

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
            if let Some(_) = gc.game_state().interrupted_turn.as_ref() {
                p { class: "info-text",
                    "Puppets on a String: perform 1 action"
                }
            }

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
                            disabled: !playable_strategy_cards().is_empty() || has_passed(),
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
                            onclick: move |_| {
                                event
                                    .send_event(Event::TacticalActionBegin {
                                        player: current_player_id(),
                                    })
                            },
                            "Tactical"
                        }
                        Button {
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
    GalacticEvent,
    Breakthrough {
        player: PlayerId,
        breakthrough: Breakthrough,
    },
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

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist during action phase")
    });

    let available_leaders = use_memo(move || {
        gc.game_state()
            .available_leaders
            .get(&current_player())
            .expect("Current player to be in available_leaders map")
            .clone()
    });

    let can_play_leaders = use_memo(move || {
        available_leaders()
            .iter()
            .filter(|l| l.info().ability_kind() == LeaderAbilityKind::Action)
            .count()
            > 0
    });

    let event = use_context::<EventContext>();

    let can_become_obsidian = use_memo(move || {
        gc.game_state()
            .players
            .get(&current_player())
            .and_then(|p| p.firmament_side())
            == Some(FirmamentSide::Firmament)
    });

    let has_galactic_event_action = use_memo(move || {
        gc.game_state()
            .galactic_events
            .iter()
            .any(|e| e.has_action())
    });

    let action_breakthrough = use_memo(move || {
        gc.game_state()
            .players
            .get(&current_player())
            .and_then(|p| p.breakthrough.clone())
            .filter(|b| b.info().kind == BreakthroughKind::Action)
    });

    rsx! {
        div { class: "component-select-container",
            Button {
                disabled: component_mode.read().eq(&ComponentMode::ActionCard),
                onclick: move |_| component_mode.set(ComponentMode::ActionCard),
                "Action Card"
            }
            if has_galactic_event_action() {
                Button {
                    disabled: component_mode.read().eq(&ComponentMode::GalacticEvent),
                    onclick: move |_| component_mode.set(ComponentMode::GalacticEvent),
                    "Galactic Event"
                }
            }
            if let Some(breakthrough) = action_breakthrough() {
                Button {
                    disabled: matches!(*component_mode.read(), ComponentMode::Breakthrough { .. }),
                    onclick: move |_| {
                        component_mode
                            .set(ComponentMode::Breakthrough {
                                player: current_player(),
                                breakthrough: breakthrough.clone(),
                            })
                    },
                    "Breakthrough"
                }
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
            if can_become_obsidian() {
                Button {
                    onclick: move |_| {
                        event
                            .send_event(Event::PuppetsOfTheBlade {
                                player: current_player(),
                            })
                    },
                    "Puppets of the Blade"
                }
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
        ComponentMode::GalacticEvent => rsx! {
            GalacticEventActionView {}
        },
        ComponentMode::Breakthrough {
            player,
            breakthrough,
        } => rsx! {
            BreakthroughActionView { player: player.clone(), breakthrough: breakthrough.clone() }
        },
        ComponentMode::PlayRelic => rsx! {
            PlayRelicView {}
        },
        ComponentMode::GainRelic => rsx! {
            GainRelicView {}
        },
        ComponentMode::FrontierCard => rsx! {
            FrontierCardView {}
        },
        ComponentMode::PlayLeader => rsx! {
            PlayLeaderView {}
        },
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
        let mut acs = gc
            .game_options()
            .action_cards
            .iter()
            .filter(|ac| ac.info().play == ActionCardPlay::Action)
            .cloned()
            .collect::<Vec<_>>();
        acs.sort();
        acs
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
                    value: selected_relic,
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

    let available_relics = use_memo(move || unowned_relics(&gc));

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
                    value: selected_relic,
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

#[component]
fn FrontierCardView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected = use_signal(|| None);

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let available_cards = use_memo(move || {
        let mut cards = gc
            .game_options()
            .frontier_cards
            .iter()
            .filter(|fc| fc.info().frontier_type == FrontierCardType::Action)
            .cloned()
            .collect::<Vec<_>>();
        cards.sort();
        cards
    });

    rsx! {
        div {
            fieldset { class: "play-action-card-container",
                legend { "Play Frontier Card" }
                FrontierCardDropdown {
                    value: selected,
                    options: available_cards,
                    on_select: move |card| selected.set(card),
                }
                Button {
                    disabled: selected().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::FrontierCardActionBegin {
                                player: current_player(),
                                card: selected().expect("Selected card to be set"),
                            })
                    },
                    "Play"
                }
            }
        }
    }
}

#[component]
fn GalacticEventActionView() -> Element {
    let gc = use_context::<GameContext>();

    let mut selected = use_signal(|| None);

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let events_with_action = use_memo(move || {
        gc.game_state()
            .galactic_events
            .iter()
            .filter(|e| e.has_action())
            .copied()
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            fieldset { class: "play-action-card-container",
                legend { "Play Galactic Event" }
                GalacticEventDropdown {
                    value: selected,
                    options: events_with_action(),
                    on_select: move |e| selected.set(e),
                }
                if let Some(galactic_event) = selected() {
                    GalacticEventActionForm { player: current_player(), galactic_event }
                }
            }
        }
    }
}

#[component]
fn GalacticEventActionForm(player: PlayerId, galactic_event: GalacticEvent) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let action = match galactic_event {
        GalacticEvent::TotalWar => GalacticEventAction::TotalWar,
        GalacticEvent::MercenariesForHire => GalacticEventAction::MercenariesForHire,
        GalacticEvent::StellarAtomics if gc.game_state().stellar_atomics_used.contains(&player) => {
            return rsx! {
                p { "Stellar Atomics token already used" }
            };
        }
        GalacticEvent::StellarAtomics => GalacticEventAction::StellarAtomics,
        GalacticEvent::ConventionsOfWarAbandoned => {
            return rsx! {
                ConventionsOfWarAbandonedForm { player }
            };
        }
        GalacticEvent::AgeOfExploration => {
            return rsx! {
                AgeOfExplorationForm { player }
            };
        }
        _ => return rsx! {},
    };

    rsx! {
        Button {
            onclick: move |_| {
                event
                    .send_event(Event::PlayGalacticEventAction {
                        player: player.clone(),
                        action: action.clone(),
                    })
            },
            "Play"
        }
    }
}

#[component]
fn ConventionsOfWarAbandonedForm(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_planet = use_signal(|| None);

    let has_x89 = use_memo({
        let player = player.clone();
        move || {
            gc.game_state()
                .players
                .get(&player)
                .is_some_and(|p| p.has_x89_bacterial_weapon())
        }
    });

    let planets_on_board = use_memo(move || planets_on_board(&gc));

    if !has_x89() {
        return rsx! {
            p { "Requires X-89 Bacterial Weapon" }
        };
    }

    rsx! {
        div { class: "column",
            PlanetDropdown {
                value: selected_planet,
                on_select: move |p| selected_planet.set(p),
                options: planets_on_board(),
            }
            Button {
                class: "margin-top",
                disabled: selected_planet().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::PlayGalacticEventAction {
                            player: player.clone(),
                            action: GalacticEventAction::ConventionsOfWarAbandoned {
                                planet: selected_planet().expect("Planet to be selected"),
                            },
                        })
                },
                "Play"
            }
        }
    }
}

#[component]
fn AgeOfExplorationForm(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_system = use_signal(|| None);
    let mut selected_ring = use_signal(|| None::<u32>);
    let mut selected_position = use_signal(|| None::<u32>);

    let has_dark_energy_tap = use_memo({
        let player = player.clone();
        move || {
            gc.game_state()
                .players
                .get(&player)
                .is_some_and(|p| p.has_tech(&Technology::DarkEnergyTap))
        }
    });

    let hex_map = use_memo(move || {
        gc.game_state()
            .map_data
            .milty_information
            .as_ref()
            .map(|milty| milty.hex_map.clone())
    });

    let drawable_tiles = use_memo(move || {
        let Some(hex_map) = hex_map() else {
            return Vec::new();
        };
        let mut tiles = gc
            .game_options()
            .systems
            .values()
            .filter(|system| hex_map.is_unused_red_or_blue_tile(system))
            .map(|system| system.id.clone())
            .collect::<Vec<_>>();
        tiles.sort_by_key(|id| id.parse::<u32>().unwrap_or(u32::MAX));
        tiles
    });

    let spots = use_memo(move || {
        let minor_factions = gc.game_state().minor_factions();
        hex_map()
            .map(|hex_map| hex_map.exploration_positions(&minor_factions))
            .unwrap_or_default()
    });
    let rings = use_memo(move || {
        let mut rings = spots().iter().map(|coord| coord.ring).collect::<Vec<_>>();
        rings.dedup();
        rings
    });
    let ring_positions = use_memo(move || {
        spots()
            .iter()
            .filter(|coord| Some(coord.ring) == selected_ring())
            .map(|coord| coord.position)
            .collect::<Vec<_>>()
    });
    let ring_value = use_memo(move || selected_ring().map(|r| r.to_string()).unwrap_or_default());
    let position_value = use_memo(move || {
        selected_position()
            .map(|p| p.to_string())
            .unwrap_or_default()
    });

    let next_to = use_memo(move || {
        let (Some(hex_map), Some(ring), Some(position)) =
            (hex_map(), selected_ring(), selected_position())
        else {
            return Vec::new();
        };
        Coordinate {
            ring,
            position,
            rotation: 0,
        }
        .neighbors()
        .iter()
        .filter_map(|neighbor| {
            hex_map.tiles.iter().find_map(|tile| match &tile.position {
                HexPosition::Pos(coord)
                    if coord.ring == neighbor.ring && coord.position == neighbor.position =>
                {
                    Some(format!("System {}", tile.system))
                }
                _ => None,
            })
        })
        .collect::<Vec<_>>()
    });

    if !has_dark_energy_tap() {
        return rsx! {
            p { "Requires Dark Energy Tap" }
        };
    }

    if hex_map().is_none() {
        return rsx! {
            div { class: "column",
                p { "Place the drawn tile at the table" }
                Button {
                    onclick: move |_| {
                        event
                            .send_event(Event::PlayGalacticEventAction {
                                player: player.clone(),
                                action: GalacticEventAction::AgeOfExploration {
                                    tile: None,
                                },
                            })
                    },
                    "Play"
                }
            }
        };
    }

    rsx! {
        div { class: "column",
            SystemDropdown {
                value: selected_system,
                options: drawable_tiles,
                on_select: move |s| selected_system.set(s),
            }
            p {
                "Ring 0 is Mecatol Rex. Ring N has 6 × N positions, numbered from 0 directly above Mecatol Rex, going clockwise."
            }
            Dropdown {
                value: ring_value,
                oninput: move |e: FormEvent| {
                    selected_ring.set(e.value().parse().ok());
                    selected_position.set(None);
                },
                option { value: "", "--Select ring--" }
                for ring in rings().iter() {
                    option { key: "{ring}", value: "{ring}", "Ring {ring}" }
                }
            }
            Dropdown {
                value: position_value,
                disabled: selected_ring().is_none(),
                oninput: move |e: FormEvent| selected_position.set(e.value().parse().ok()),
                option { value: "", "--Select position--" }
                for position in ring_positions().iter() {
                    option { key: "{position}", value: "{position}", "Position {position}" }
                }
            }
            if !next_to().is_empty() {
                p { "Next to: {next_to().join(\", \")}" }
            }
            Button {
                class: "margin-top",
                disabled: selected_system().is_none() || selected_ring().is_none()
                    || selected_position().is_none(),
                onclick: move |_| {
                    let (Some(system), Some(ring), Some(position)) = (
                        selected_system(),
                        selected_ring(),
                        selected_position(),
                    ) else {
                        return;
                    };
                    event
                        .send_event(Event::PlayGalacticEventAction {
                            player: player.clone(),
                            action: GalacticEventAction::AgeOfExploration {
                                tile: Some((
                                    system,
                                    Coordinate {
                                        ring,
                                        position,
                                        rotation: 0,
                                    },
                                )),
                            },
                        })
                },
                "Play"
            }
        }
    }
}

#[component]
fn PlayLeaderView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });
    let available_leaders = use_memo(move || {
        gc.game_state()
            .available_leaders
            .get(&current_player())
            .map(|leaders| leaders.to_vec())
            .unwrap_or_default()
    });
    let action_leaders = use_memo(move || {
        available_leaders()
            .iter()
            .filter(|l| l.info().ability_kind() == LeaderAbilityKind::Action)
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div {
            fieldset { class: "play-action-card-container",
                legend { "Play Leader" }
                table {
                    for leader in action_leaders().iter() {
                        tr { key: "{leader}",
                            Button {
                                onclick: {
                                    let leader = *leader;
                                    move |_| {
                                        event
                                            .send_event(Event::LeaderActionBegin {
                                                player: current_player(),
                                                leader,
                                            });
                                    }
                                },
                                "{leader}"
                            }
                            InfoButton { info: Info::Leader(*leader) }
                        }
                    }
                }
            }
        }
    }
}

/// The relics in the game that nobody owns, sorted.
pub fn unowned_relics(gc: &GameContext) -> Vec<Relic> {
    let game_state = gc.game_state();
    let mut relics = gc
        .game_options()
        .relics
        .iter()
        .filter(|r| !game_state.players.values().any(|p| p.relics.contains(r)))
        .cloned()
        .collect::<Vec<_>>();
    relics.sort();
    relics
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
