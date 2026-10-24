use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{
        objectives::{Objective, ObjectiveKind, secret::SecretObjective},
        relic::Relic,
    },
};

use crate::{
    components::{
        button::Button,
        dropdown::{ObjectiveDropdown, RelicDropdown},
        info_button::InfoButton,
    },
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
            .iter()
            .filter(|&o| !revealed_objectives().contains(o))
            .cloned()
            .collect::<Vec<_>>()
    });

    let revealed_objective = use_memo(move || state().revealed_objective);

    let neuraloop_holder = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .find(|(_, p)| p.relics.contains(&Relic::Neuraloop))
            .map(|(id, _)| id.clone())
    });

    let reveal_stage_two = use_memo(move || {
        revealed_objectives().len() >= state().expected_objectives_before_stage_two
    });
    let stage = use_memo(move || if reveal_stage_two() { "II" } else { "I" });

    let selectable_objectives = use_memo(move || {
        unrevealed_objectives()
            .iter()
            .filter(|&o| {
                matches!(
                    (o.info().kind, reveal_stage_two()),
                    (ObjectiveKind::StageI, false) | (ObjectiveKind::StageII, true)
                )
            })
            .cloned()
            .collect::<Vec<_>>()
    });

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
                    if let Some(holder) = neuraloop_holder() {
                        if view.display_for(&holder) {
                            NeuraloopView { player: holder }
                        }
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
            .map(|(o, _)| *o)
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
            .iter()
            .filter_map(|o| match o {
                Objective::Public(_) => None,
                Objective::Secret(secret) => Some((*o, *secret)),
            })
            .filter(|(_, s)| !player_scored_secrets().contains(s))
            .filter(|(obj, _)| !gc.game_state().score.revealed_objectives.contains_key(obj))
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
                p {
                    "Public "
                    if let Some(p) = choice {
                        "{p.info().name}"
                        InfoButton { info: Info::Objective(p) }
                    } else {
                        "Skipped"
                    }
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

#[component]
fn NeuraloopView(player: ReadSignal<PlayerId>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_relic = use_signal(|| None);
    let mut selected_replacement = use_signal(|| None);

    let player_relics = use_memo(move || {
        let mut relics = gc
            .game_state()
            .players
            .get(&player())
            .map(|p| p.relics.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default();
        relics.sort();
        relics
    });

    let replacements = use_memo(move || {
        let state = gc.game_state();
        let mut objs = gc
            .game_options()
            .objectives
            .iter()
            .filter(|&o| !state.score.revealed_objectives.contains_key(o))
            .filter(|o| match o {
                Objective::Public(_) => true,
                Objective::Secret(secret) => !state
                    .score
                    .secret_objectives
                    .values()
                    .any(|scored| scored.contains(secret)),
            })
            .cloned()
            .collect::<Vec<_>>();
        objs.sort();
        objs
    });

    rsx! {
        fieldset {
            legend {
                h4 { "Neuraloop ({player()})" }
            }
            div { class: "column",
                p { "Purge a relic to replace the revealed objective." }
                RelicDropdown {
                    value: selected_relic,
                    options: player_relics(),
                    on_select: move |r| selected_relic.set(r),
                }
                ObjectiveDropdown {
                    value: selected_replacement,
                    options: replacements(),
                    on_select: move |o| selected_replacement.set(o),
                    default_text: "--Select Replacement--",
                }
                Button {
                    class: "margin-top",
                    disabled: selected_relic().is_none() || selected_replacement().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::PlayNeuraloop {
                                player: player(),
                                purged_relic: selected_relic().expect("Relic to be selected"),
                                replacement: selected_replacement()
                                    .expect("Replacement objective to be selected"),
                            });
                        selected_relic.set(None);
                        selected_replacement.set(None);
                    },
                    "Replace Objective"
                }
            }
        }
    }
}
