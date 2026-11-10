use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{component::Component, expansions::Expansion, faction::Faction, player_id::PlayerId},
    components::{
        galactic_event::GalacticEvent,
        objectives::{Objective, ObjectiveKind, PUBLIC_OBJECTIVES_PER_STAGE},
        system::SystemId,
    },
    state::player::Player,
};

use crate::views::{
    phase_views::{
        faction_setup_views::{
            argent_flight_setup::ArgentFlightSetup, council_keleres_setup::CouncilKeleresSetup,
            deepwrought_scholarate_setup::DeepwroughtScholarateSetup,
            pick_one_tech_setup::PickOneTechSetup,
        },
        speaker_step::SpeakerStep,
    },
    player_view_dropdown::PlayerViewDropdown,
};
use crate::{
    components::{
        button::Button,
        dropdown::{FactionDropdown, ObjectiveDropdown, SystemDropdown},
        faction_icon::FactionIcon,
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

const SETUP_PHASE_SCSS: Asset = asset!("/assets/styling/views/phase_views/setup_phase.scss");

#[component]
pub fn SetupPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let speaker_chosen = use_memo(move || gc.game_state().speaker.is_some());
    let revealed_objectives = use_memo(move || gc.game_state().score.revealed_objectives.len());
    let unfinished_players = use_memo(move || {
        gc.game_state()
            .players
            .keys()
            .filter(|p| {
                !gc.game_state()
                    .player_initialization_finished(p)
                    .expect("Unexpected error")
            })
            .count()
    });

    rsx! {
        document::Stylesheet { href: SETUP_PHASE_SCSS }

        div { class: "card setup-container",
            h2 { "Setup" }

            PlayerViewDropdown {}

            PlayersSetup {}

            ObjectivesSetup {}

            SpeakerStep {
                ready: speaker_chosen() && revealed_objectives() > 0 && unfinished_players() == 0,
                waiting_for: "start the game",
                onclick: move |_| event.send_event(Event::StartGame),
                "Start Game"
            }
        }
    }
}

#[component]
fn PlayersSetup() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();
    let players = use_memo(move || {
        let game_state = Arc::clone(&gc.game_state());

        let mut players = game_state.players.clone().into_iter().collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| {
            let a_pos = gc
                .game_state()
                .table_order
                .iter()
                .position(|p| p.eq(a))
                .expect("Player to be at the table!");
            let b_pos = gc
                .game_state()
                .table_order
                .iter()
                .position(|p| p.eq(b))
                .expect("Player to be at the table!");
            a_pos.cmp(&b_pos)
        });
        players
    });
    let cultural_exchange = use_memo(move || {
        gc.game_state()
            .galactic_events
            .contains(&GalacticEvent::CulturalExchangeProgram)
    });
    let minor_factions = use_memo(move || {
        gc.game_state()
            .galactic_events
            .contains(&GalacticEvent::MinorFactions)
    });

    rsx! {
        div { class: "player-specific-setup-container",
            h3 { "Player specific setup" }
            div { class: "players-setup-container",
                {
                    players()
                        .into_iter()
                        .map(|(name, player)| {
                            let player = Arc::new(player);
                            let n = Arc::clone(&name);
                            rsx! {
                                fieldset {
                                    key: "{n}",
                                    class: format!("player-color-border-{} setup-player-fieldset", player.color.name())
                                        .as_str(),
                                    legend { class: format!("player-color-border-{} setup-player-legend", player.color.name()).as_str(),
                                        "{&name}"
                                    }

                                    div { class: "setup-row",
                                        if gc.game_state().is_speaker(&name) {
                                            p { "Speaker" }
                                        } else if view.is_global_or_speaker() {
                                            Button {
                                                disabled: gc.game_state().is_speaker(&name),
                                                onclick: move |_| {
                                                    event
                                                        .send_event(Event::SetupSpeaker {
                                                            player: n.clone(),
                                                        })
                                                },
                                                "Set Speaker"
                                            }
                                        }
                                        FactionIcon {
                                            faction: player.faction,
                                            faction_state: player.faction_state.clone(),
                                        }
                                    }

                                    {player.faction_name()}
                                    FactionSpecificSetup { player_id: Arc::clone(&name), player: Arc::clone(&player) }
                                    if cultural_exchange() {
                                        CulturalExchangeSetup { player_id: Arc::clone(&name), player: Arc::clone(&player) }
                                    }
                                    if minor_factions() {
                                        MinorFactionSetup { player_id: Arc::clone(&name) }
                                    }
                                }
                            }
                        })
                }
            }
        }
    }
}

#[component]
fn FactionSpecificSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    match player.faction {
        Faction::Winnu => {
            rsx! {
                PickOneTechSetup { player_id, player }
            }
        }
        Faction::ArgentFlight => {
            rsx! {
                ArgentFlightSetup { player_id, player }
            }
        }
        Faction::CouncilKeleres => {
            rsx! {
                CouncilKeleresSetup { player_id, player }
            }
        }
        Faction::CrimsonRebellion => {
            rsx! {
                PickOneTechSetup { player_id, player }
            }
        }
        Faction::FirmamentObsidian => {
            rsx! {
                PickOneTechSetup { player_id, player }
            }
        }
        Faction::DeepwroughtScholarate => {
            rsx! {
                DeepwroughtScholarateSetup { player_id }
            }
        }
        Faction::LastBastion => {
            rsx! {
                PickOneTechSetup { player_id, player }
            }
        }
        Faction::RalNelConsortium => {
            rsx! {
                PickOneTechSetup { player_id, player }
            }
        }
        f if f.expansion() == Expansion::ThundersEdge => {
            rsx! {
                div { "Thunder's edge factions not yet implemented" }
            }
        }
        _ => rsx! {
            div { "No faction specific setup" }
        },
    }
}

#[component]
fn CulturalExchangeSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let p1 = Arc::clone(&player_id);
    let leaders_from = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .and_then(|player| player.leaders_from)
    });

    let p2 = Arc::clone(&player_id);
    let options = use_memo(move || {
        let game_state = gc.game_state();
        let mut factions = (game_state.players.values())
            .map(|player| player.faction)
            .filter(|faction| *faction != Faction::FirmamentObsidian)
            .filter(|faction| {
                !(game_state.players.iter())
                    .any(|(id, player)| *id != p2 && player.leaders_from == Some(*faction))
            })
            .collect::<Vec<_>>();
        factions.sort();
        factions
    });

    if player.faction == Faction::FirmamentObsidian {
        return rsx! {
            p { "Starts with The Obsidian" }
        };
    }

    if !view.display_for(&player_id) {
        return rsx! {
            div { class: "setup-column",
                label { "Cultural Exchange" }
                if let Some(faction) = leaders_from() {
                    p { "{faction.name()}" }
                } else {
                    p { "Waiting for {player_id}" }
                }
            }
        };
    }

    rsx! {
        div { class: "setup-column",
            label { "Cultural Exchange" }
            FactionDropdown {
                value: leaders_from,
                options: options(),
                on_select: move |faction| {
                    event
                        .send_event(Event::SetupCulturalExchange {
                            player: Arc::clone(&player_id),
                            faction,
                        })
                },
            }
        }
    }
}

#[component]
fn MinorFactionSetup(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let p1 = Arc::clone(&player_id);
    let minor_faction = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .and_then(|player| player.minor_faction)
    });

    let p2 = Arc::clone(&player_id);
    let options = use_memo(move || {
        let game_state = gc.game_state();
        let mut factions = (gc.game_options().factions.iter())
            .map(|f| f.faction)
            .filter(|faction| game_state.is_minor_faction_available(&p2, *faction))
            .collect::<Vec<_>>();
        factions.sort();
        factions
    });

    let has_milty_map = use_memo(move || gc.game_state().map_data.milty_information.is_some());
    let replaceable_systems = use_memo(move || {
        (gc.game_state().map_data.milty_information.as_ref())
            .map(|milty| milty.hex_map.minor_faction_slots())
            .unwrap_or_default()
    });
    let mut selected_faction: Signal<Option<Faction>> = use_signal(|| None);
    let mut selected_system: Signal<Option<SystemId>> = use_signal(|| None);
    let can_place = use_memo(move || {
        selected_faction().is_some_and(|faction| options().contains(&faction))
            && selected_system().is_some_and(|system| replaceable_systems().contains(&system))
    });

    if !view.display_for(&player_id) {
        return rsx! {
            div { class: "setup-column",
                label { "Minor faction" }
                if let Some(faction) = minor_faction() {
                    p { "{faction.name()}" }
                } else {
                    p { "Waiting for {player_id}" }
                }
            }
        };
    }

    if !has_milty_map() {
        return rsx! {
            div { class: "setup-column",
                label { "Minor faction" }
                FactionDropdown {
                    value: minor_faction,
                    options: options(),
                    on_select: move |faction| {
                        event
                            .send_event(Event::SetupMinorFaction {
                                player: Arc::clone(&player_id),
                                faction,
                                replaced_system: None,
                            })
                    },
                }
            }
        };
    }

    if let Some(faction) = minor_faction() {
        return rsx! {
            div { class: "setup-column",
                label { "Minor faction" }
                p { "{faction.name()}" }
            }
        };
    }

    rsx! {
        div { class: "setup-column",
            label { "Minor faction" }
            FactionDropdown {
                value: selected_faction,
                options: options(),
                on_select: move |faction| selected_faction.set(faction),
            }
            label { "Replaces" }
            SystemDropdown {
                value: selected_system,
                options: replaceable_systems,
                on_select: move |system| selected_system.set(system),
            }
            Button {
                disabled: !can_place(),
                onclick: move |_| {
                    event
                        .send_event(Event::SetupMinorFaction {
                            player: Arc::clone(&player_id),
                            faction: selected_faction(),
                            replaced_system: selected_system(),
                        })
                },
                "Place"
            }
        }
    }
}

#[component]
fn ObjectivesSetup() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

    let mut first_objective: Signal<Option<Objective>> = use_signal(|| None);
    let mut second_objective: Signal<Option<Objective>> = use_signal(|| None);

    let available_objectives = use_memo(move || {
        let mut objectives = gc
            .game_options()
            .objectives
            .iter()
            .filter(|o| o.info().kind == ObjectiveKind::StageI)
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    let first_available_objectives = use_memo(move || {
        available_objectives()
            .iter()
            .filter(|o| !second_objective().as_ref().eq(&Some(o)))
            .cloned()
            .collect::<Vec<_>>()
    });

    let second_available_objectives = use_memo(move || {
        available_objectives()
            .iter()
            .filter(|o| !first_objective().as_ref().eq(&Some(o)))
            .cloned()
            .collect::<Vec<_>>()
    });

    let revealed_objectives = use_memo(move || {
        let mut objectives = gc
            .game_state()
            .score
            .revealed_objectives
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    let civilized_society = use_memo(move || {
        gc.game_state()
            .galactic_events
            .contains(&GalacticEvent::CivilizedSociety)
    });

    rsx! {
        div { class: "reveal-objectives-container",
            if civilized_society() {
                h3 { "Select all public objectives" }
            } else {
                h3 { "Select initial objectives" }
            }
            {
                if revealed_objectives.is_empty() && !view.is_global_or_speaker() {
                    rsx! {
                        p { "Waiting for the speaker to reveal objectives" }
                    }
                } else if revealed_objectives.is_empty() && civilized_society() {
                    rsx! {
                        AllObjectivesSetup {}
                    }
                } else if revealed_objectives.is_empty() {
                    rsx! {
                        ObjectiveDropdown {
                            value: first_objective,
                            options: first_available_objectives(),
                            on_select: move |obj| first_objective.set(obj),
                        }
                        ObjectiveDropdown {
                            value: second_objective,
                            options: second_available_objectives(),
                            on_select: move |obj| second_objective.set(obj),
                        }
                        Button {
                            disabled: first_objective().is_none() || second_objective().is_none(),
                            onclick: move |_| {
                                event
                                    .send_event(Event::RevealInitialObjectives {
                                        first_objective: first_objective()
                                            .expect("First objective to be selected"),
                                        second_objective: second_objective()
                                            .expect("Second objective to be selected"),
                                    })
                            },
                            "Select Objectives"
                        }
                    }
                } else {
                    rsx! {
                        {
                            revealed_objectives
                                .iter()
                                .map(|obj| {
                                    rsx! {
                                        p { key: "{obj}", "{obj.info().name}" }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AllObjectivesSetup() -> Element {
    let event = use_context::<EventContext>();

    let stage_one: Signal<Vec<Option<Objective>>> =
        use_signal(|| vec![None; PUBLIC_OBJECTIVES_PER_STAGE]);
    let stage_two: Signal<Vec<Option<Objective>>> =
        use_signal(|| vec![None; PUBLIC_OBJECTIVES_PER_STAGE]);

    let all_picked = use_memo(move || {
        stage_one
            .read()
            .iter()
            .chain(stage_two.read().iter())
            .all(Option::is_some)
    });

    rsx! {
        StageObjectivesSetup { kind: ObjectiveKind::StageI, picks: stage_one }
        StageObjectivesSetup { kind: ObjectiveKind::StageII, picks: stage_two }
        Button {
            disabled: !all_picked(),
            onclick: move |_| {
                event
                    .send_event(Event::RevealAllPublicObjectives {
                        stage_one: stage_one.read().iter().flatten().copied().collect(),
                        stage_two: stage_two.read().iter().flatten().copied().collect(),
                    })
            },
            "Reveal Objectives"
        }
    }
}

#[component]
fn StageObjectivesSetup(kind: ObjectiveKind, picks: Signal<Vec<Option<Objective>>>) -> Element {
    let gc = use_context::<GameContext>();

    let legend = if kind == ObjectiveKind::StageI {
        "Stage I"
    } else {
        "Stage II"
    };

    let options = use_memo(move || {
        let mut objectives = gc
            .game_options()
            .objectives
            .iter()
            .filter(|o| o.info().kind == kind)
            .cloned()
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    rsx! {
        fieldset { class: "setup-column",
            legend { "{legend}" }
            for index in 0..PUBLIC_OBJECTIVES_PER_STAGE {
                ObjectiveSlot {
                    key: "{index}",
                    index,
                    options,
                    picks,
                }
            }
        }
    }
}

#[component]
fn ObjectiveSlot(
    index: usize,
    options: ReadSignal<Vec<Objective>>,
    mut picks: Signal<Vec<Option<Objective>>>,
) -> Element {
    let selected = use_memo(move || picks.read()[index]);

    let available = use_memo(move || {
        let picks = picks.read();
        options()
            .into_iter()
            .filter(|o| {
                !(picks.iter().enumerate()).any(|(i, pick)| i != index && pick.as_ref() == Some(o))
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        ObjectiveDropdown {
            value: selected,
            options: available(),
            on_select: move |obj| picks.write()[index] = obj,
        }
    }
}
