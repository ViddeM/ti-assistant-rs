use std::collections::BTreeMap;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::{BreakthroughAction, Event},
    common::player_id::PlayerId,
    components::{
        breakthrough::Breakthrough,
        relic::Relic,
        tech::{TechOrigin, TechType, Technology},
    },
};

use crate::{
    components::{
        button::Button,
        dropdown::{RelicDropdown, TechDropdown},
    },
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
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
            h2 { "{breakthrough().info().name}" }
            if view.is_active() {
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
        Breakthrough::VisionariaSelect => {
            return rsx! {
                VisionariaSelectForm { player }
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
fn VisionariaSelectForm(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let researched: Signal<BTreeMap<PlayerId, Technology>> = use_signal(BTreeMap::new);

    let other_players = use_memo({
        let player = player.clone();
        move || {
            gc.game_state()
                .table_order
                .iter()
                .filter(|p| **p != player)
                .cloned()
                .collect::<Vec<_>>()
        }
    });

    rsx! {
        div { class: "column",
            for researcher in other_players() {
                VisionariaSelectPick {
                    key: "{researcher}",
                    player: researcher.clone(),
                    researched,
                }
            }
            Button {
                class: "margin-top",
                onclick: move |_| {
                    event
                        .send_event(Event::BreakthroughActionCommit {
                            player: player.clone(),
                            action: BreakthroughAction::VisionariaSelect {
                                researched: researched(),
                            },
                        })
                },
                "Play"
            }
        }
    }
}

#[component]
fn VisionariaSelectPick(
    player: PlayerId,
    mut researched: Signal<BTreeMap<PlayerId, Technology>>,
) -> Element {
    let gc = use_context::<GameContext>();

    let selected = use_memo({
        let player = player.clone();
        move || researched.read().get(&player).cloned()
    });

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

    rsx! {
        fieldset {
            legend { "{player}" }
            TechDropdown {
                value: selected,
                options: options(),
                on_select: move |tech| {
                    let mut researched = researched.write();
                    match tech {
                        Some(tech) => researched.insert(player.clone(), tech),
                        None => researched.remove(&player),
                    };
                },
            }
        }
    }
}
