use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::objectives::{secret::SecretObjective, Objective, ObjectiveKind},
};

use crate::{
    components::{button::Button, dropdown::ObjectiveDropdown, info_button::InfoButton},
    data::{
        event_context::EventContext, game_context::GameContext, info_context::Info,
        player_view::PlayerViewContext,
    },
};

#[component]
pub fn StatusPhaseActionsView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let mut selected_objective: Signal<Option<Objective>> = use_signal(|| None);

    let players = use_memo(move || {
        gc.game_state()
            .turn_order
            .iter()
            .filter(|&p| view.display_for(p))
            .cloned()
            .collect::<Vec<_>>()
    });

    let speaker = use_memo(move || {
        gc.game_state()
            .speaker
            .clone()
            .expect("There to be a speaker during status phase")
    });

    let state = use_memo(move || {
        gc.game_state()
            .status_phase_state
            .clone()
            .expect("Status phase state to be set during status phase")
    });
    let num_publics = use_memo(move || state().scored_public_objectives.len());
    let num_secrets = use_memo(move || state().scored_secret_objectives.len());
    let num_players = use_memo(move || gc.game_state().players.len());

    let reveal_unlocked =
        use_memo(move || num_players() == num_publics() && num_players() == num_secrets());

    let revealed_objectives = use_memo(move || {
        gc.game_state()
            .score
            .revealed_objectives
            .keys()
            .cloned()
            .collect::<Vec<_>>()
    });

    let unrevealed_objectives = use_memo(move || {
        gc.game_options()
            .objectives
            .keys()
            .filter(|&o| !revealed_objectives().contains(o))
            .cloned()
            .collect::<Vec<_>>()
    });

    let reveal_stage_two = use_memo(move || {
        revealed_objectives.len() - (state().revealed_objective.map(|_| 0).unwrap_or(1))
            >= state().expected_objectives_before_stage_two
    });
    let stage = use_memo(move || if reveal_stage_two() { "II" } else { "I" });

    let selectable_objectives = use_memo(move || {
        unrevealed_objectives()
            .iter()
            .filter(|&o| match (o.info().kind, reveal_stage_two()) {
                (ObjectiveKind::StageI, false) => true,
                (ObjectiveKind::StageII, true) => true,
                _ => false,
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    let revealed_objective = use_memo(move || state().revealed_objective);

    rsx! {
        div { class: "card",
            h2 { "Score Objectives" }
            for player in players().iter() {
                PlayerObjectives { key: "{player}", player: player.clone() }
            }
            fieldset { class: "reveal-objective-container",
                legend {
                    h3 { "Reveal Stage {stage()} Objective" }
                }
                if let Some(revealed) = revealed_objective() {
                    div { class: "row",
                        p { "{revealed.info().name}" }
                        InfoButton { info: Info::Objective(revealed) }
                    }
                } else if view.is_global_or_speaker() {
                    ObjectiveDropdown {
                        disabled: !reveal_unlocked(),
                        value: selected_objective,
                        options: selectable_objectives(),
                        on_select: move |obj| selected_objective.set(obj),
                    }
                    Button {
                        disabled: selected_objective().is_none(),
                        onclick: move |_| {
                            let Objective::Public(p) = selected_objective()
                                .expect("Revealed objective to be set") else {
                                panic!("Objective should be public")
                            };
                            event
                                .send_event(Event::RevealPublicObjective {
                                    objective: p,
                                })
                        },
                        "Reveal"
                    }
                } else {
                    p { "Waiting for {speaker()} to reveal a stage {stage()} objective" }
                }
            }
        }
    }
}

#[component]
fn PlayerObjectives(player: ReadSignal<PlayerId>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let sp_state = use_memo(move || {
        gc.game_state()
            .status_phase_state
            .clone()
            .expect("Status phase state to be set in status phase")
    });

    let public_objective =
        use_memo(move || sp_state().scored_public_objectives.get(&player()).cloned());
    let secret_objective =
        use_memo(move || sp_state().scored_secret_objectives.get(&player()).cloned());

    let mut selected_public_objective = use_signal(|| None);
    let mut selected_secret_objective = use_signal(|| None);

    let score_public = use_callback(move |o: Option<Objective>| {
        event.send_event(Event::ScorePublicObjective {
            player: player(),
            objective: o,
        })
    });
    let score_secret = use_callback(move |o: Option<SecretObjective>| {
        event.send_event(Event::ScoreSecretObjective {
            player: player(),
            objective: o,
        })
    });

    let available_pubs = use_memo(move || {
        let mut objs = gc
            .game_state()
            .score
            .revealed_objectives
            .iter()
            .filter(|(_, ps)| !ps.contains(&player()))
            .map(|(o, _)| o.clone())
            .collect::<Vec<_>>();
        objs.sort();
        objs
    });

    let player_scored_secrets = use_memo(move || {
        gc.game_state()
            .score
            .secret_objectives
            .get(&player())
            .map(|objs| objs.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    });

    let available_secs = use_memo(move || {
        let mut objs = gc
            .game_options()
            .objectives
            .keys()
            .filter_map(|o| match o {
                Objective::Public(_) => None,
                Objective::Secret(secret) => Some((o.clone(), secret.clone())),
            })
            .filter(|(_, s)| !player_scored_secrets().contains(s))
            .map(|(obj, _)| obj)
            .collect::<Vec<_>>();
        objs.sort();
        objs
    });

    rsx! {
        fieldset {
            legend {
                h4 { "{player()}" }
            }
            if let Some(choice) = public_objective() {
                p { "Public " }
                if let Some(p) = choice {
                    "{p.info().name}"
                    InfoButton { info: Info::Objective(p) }
                } else {
                    "Skipped"
                }
            } else {
                div { class: "score-objectives-container",
                    ObjectiveDropdown {
                        value: selected_public_objective,
                        options: available_pubs(),
                        on_select: move |obj| selected_public_objective.set(obj),
                    }
                    div { class: "score-objective-buttons-container",
                        Button { onclick: move |_| score_public(None), "Skip" }
                        Button {
                            disabled: selected_public_objective().is_none(),
                            onclick: move |_| score_public(
                                Some(selected_public_objective().expect("Selected public objective to be set")),
                            ),
                            "Score"
                        }
                    }
                }
            }
            if let Some(choice) = secret_objective() {
                p {
                    "Secret "
                    if let Some(s) = choice {
                        "{s.info().name}"
                        InfoButton { info: Info::Objective(Objective::Secret(s)) }
                    } else {
                        "Skipped"
                    }
                }
            } else {
                div { class: "score-objectives-container",
                    ObjectiveDropdown {
                        value: selected_secret_objective,
                        options: available_secs(),
                        on_select: move |obj| selected_secret_objective.set(obj),
                    }
                    div { class: "score-objective-buttons-container",
                        Button { onclick: move |_| score_secret(None), "Skip" }
                        Button {
                            disabled: selected_secret_objective().is_none(),
                            onclick: move |_| {
                                let Objective::Secret(s) = selected_secret_objective()
                                    .expect("Selected secret objective to be set") else {
                                    panic!("Objective should be secret objective?")
                                };
                                score_secret(Some(s))
                            },
                            "Score"
                        }
                    }
                }
            }
        }
    }
}
