use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{BreakthroughAction, Event},
    common::player_id::PlayerId,
    components::{
        breakthrough::Breakthrough,
        relic::Relic,
        tech::{TechOrigin, TechType, Technology},
    },
    state::game_state::{ActionPhaseProgress, BreakthroughProgress},
};

use crate::{
    components::{
        button::Button,
        dropdown::{RelicDropdown, TechDropdown},
        info_button::InfoButton,
    },
    data::{
        event_context::EventContext, game_context::GameContext, info_context::Info,
        player_view::PlayerViewContext,
    },
    views::phase_views::action_phase::unowned_relics,
};

#[component]
pub fn BreakthroughActionView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player()
            .expect("Current player to exist in action phase")
    });

    let breakthrough = use_memo(move || {
        gc.game_state()
            .players
            .get(&current_player())
            .and_then(|p| p.breakthrough.clone())
            .expect("Current player to have a breakthrough")
    });

    rsx! {
        div { class: "card column",
            div { class: "center-row",
                h2 { "{breakthrough().info().name}" }
                InfoButton { info: Info::Breakthrough(breakthrough()) }
            }
            if breakthrough() == Breakthrough::VisionariaSelect {
                VisionariaSelectView { deepwrought: current_player() }
            } else if view.is_active() {
                BreakthroughActionForm { player: current_player(), breakthrough: breakthrough() }
            } else {
                p { "Not your turn, currently {current_player()} is playing" }
            }
        }
    }
}

#[component]
fn BreakthroughActionForm(player: PlayerId, breakthrough: Breakthrough) -> Element {
    let event = use_context::<EventContext>();

    let action = match breakthrough {
        Breakthrough::Psychospore => BreakthroughAction::Psychospore,
        Breakthrough::DeorbitBarrage => BreakthroughAction::DeorbitBarrage,
        Breakthrough::ResonanceGenerator => BreakthroughAction::ResonanceGenerator,
        Breakthrough::VaultsOfTheHeir => {
            return rsx! {
                VaultsOfTheHeirForm { player }
            };
        }
        _ => return rsx! {},
    };

    rsx! {
        Button {
            onclick: move |_| {
                event
                    .send_event(Event::BreakthroughActionCommit {
                        player: player.clone(),
                        action: action.clone(),
                    })
            },
            "Play"
        }
    }
}

#[component]
fn VaultsOfTheHeirForm(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut purged_tech: Signal<Option<Technology>> = use_signal(|| None);
    let mut relic: Signal<Option<Relic>> = use_signal(|| None);

    let player_techs = use_memo({
        let player = player.clone();
        move || {
            gc.game_state()
                .players
                .get(&player)
                .map(|p| p.technologies().iter().cloned().collect::<Vec<_>>())
                .unwrap_or_default()
        }
    });

    let available_relics = use_memo(move || unowned_relics(&gc));

    rsx! {
        div { class: "column",
            TechDropdown {
                value: purged_tech,
                options: player_techs(),
                on_select: move |t| purged_tech.set(t),
            }
            RelicDropdown {
                value: relic,
                options: available_relics(),
                on_select: move |r| relic.set(r),
            }
            Button {
                class: "margin-top",
                disabled: purged_tech().is_none() || relic().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::BreakthroughActionCommit {
                            player: player.clone(),
                            action: BreakthroughAction::VaultsOfTheHeir {
                                purged_tech: purged_tech().expect("Technology to be selected"),
                                relic: relic().expect("Relic to be selected"),
                            },
                        })
                },
                "Play"
            }
        }
    }
}

#[component]
fn VisionariaSelectView(deepwrought: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let decisions = use_memo(move || match gc.game_state().action_progress.as_ref() {
        Some(ActionPhaseProgress::Breakthrough(progress)) => progress.clone(),
        _ => BreakthroughProgress::default(),
    });

    let other_players = use_memo({
        let deepwrought = deepwrought.clone();
        move || {
            gc.game_state()
                .table_order
                .iter()
                .filter(|p| **p != deepwrought)
                .cloned()
                .collect::<Vec<_>>()
        }
    });

    let gains = use_memo({
        let deepwrought = deepwrought.clone();
        move || {
            let game_state = gc.game_state();
            game_state
                .players
                .get(&deepwrought)
                .map(|p| decisions().visionaria_select_gains(p))
                .unwrap_or_default()
        }
    });

    let all_decided = use_memo(move || {
        other_players()
            .iter()
            .all(|p| decisions().visionaria_select.contains_key(p))
    });

    rsx! {
        div { class: "column",
            for researcher in other_players() {
                VisionariaSelectDecisionView {
                    key: "{researcher}",
                    player: researcher.clone(),
                    deepwrought: deepwrought.clone(),
                    decision: decisions().visionaria_select.get(&researcher).cloned(),
                }
            }
            fieldset {
                legend { "{deepwrought} gains" }
                if gains().is_empty() {
                    p { "Nothing yet" }
                } else {
                    ul {
                        for tech in gains() {
                            li { key: "{tech}", "{tech.info().name}" }
                        }
                    }
                }
            }
            if view.display_for(&deepwrought) {
                Button {
                    class: "margin-top",
                    disabled: !all_decided(),
                    onclick: {
                        let deepwrought = deepwrought.clone();
                        move |_| {
                            event
                                .send_event(Event::BreakthroughActionCommit {
                                    player: deepwrought.clone(),
                                    action: BreakthroughAction::VisionariaSelect,
                                })
                        }
                    },
                    "Done"
                }
            } else if all_decided() {
                p { "Waiting for {deepwrought} to finish Visionaria Select" }
            }
        }
    }
}

#[component]
fn VisionariaSelectDecisionView(
    player: PlayerId,
    deepwrought: PlayerId,
    decision: Option<Option<Technology>>,
) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let mut selected: Signal<Option<Technology>> =
        use_signal(|| decision.clone().flatten());

    let options = use_memo({
        let player = player.clone();
        move || {
            let game_state = gc.game_state();
            let owner = game_state.players.get(&player);
            let mut techs = gc
                .game_options()
                .technologies
                .iter()
                .filter(|t| {
                    let info = t.info();
                    info.origin == TechOrigin::Base
                        && info.tech_type != TechType::UnitUpgrade
                        && !owner.is_some_and(|p| p.has_tech(t))
                })
                .cloned()
                .collect::<Vec<_>>();
            techs.sort();
            techs
        }
    });

    let decided = match &decision {
        Some(Some(tech)) => format!("Researches {}", tech.info().name),
        Some(None) => "Doesn't research".to_string(),
        None => format!("Waiting for {player} to decide"),
    };

    let send = move |player: PlayerId, tech: Option<Technology>| {
        event.send_event(Event::VisionariaSelectDecision { player, tech })
    };

    rsx! {
        fieldset {
            legend { "{player}" }
            if view.display_for(&player) {
                div { class: "column",
                    p { class: "warning-text",
                        "Pay 3 trade goods and give {deepwrought} 1 promissory note"
                    }
                    TechDropdown {
                        value: selected,
                        options: options(),
                        on_select: move |tech| selected.set(tech),
                    }
                    div { class: "center-row margin-top",
                        Button {
                            class: "margin-right",
                            disabled: selected().is_none(),
                            onclick: {
                                let player = player.clone();
                                move |_| send(player.clone(), selected())
                            },
                            "Research"
                        }
                        Button {
                            onclick: {
                                let player = player.clone();
                                move |_| send(player.clone(), None)
                            },
                            "Don't research"
                        }
                    }
                    if decision.is_some() {
                        p { class: "margin-top", "{decided}" }
                    }
                }
            } else {
                p { "{decided}" }
            }
        }
    }
}
