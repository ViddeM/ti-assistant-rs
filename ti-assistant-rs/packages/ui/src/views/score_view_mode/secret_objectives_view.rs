use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaTrash, Icon};
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::objectives::Objective,
};

use crate::{
    components::{
        button::Button, dropdown::ObjectiveDropdown, faction_icon::FactionIcon,
        info_button::InfoButton,
    },
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
};

#[component]
pub fn SecretObjectivesView() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "card",
            h2 { "Secret objectives" }

            {players().into_iter().map(|p| rsx! {
                PlayerSecretView { key: "{p}", player_id: p }
            })}
        }
    }
}

#[component]
fn PlayerSecretView(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_secret_objective = use_signal(|| None);

    let p1 = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .cloned()
            .expect("Player to exist")
    });

    let p2 = player_id.clone();
    let player_secrets = use_memo(move || {
        gc.game_state()
            .score
            .secret_objectives
            .get(&p2)
            .map(|objs| objs.iter().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    });

    let taken_secret_objectives = use_memo(move || {
        gc.game_state()
            .score
            .secret_objectives
            .values()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
    });
    let unrevealed_secret_objectives = use_memo(move || {
        let mut objectives = gc
            .game_options()
            .objectives
            .iter()
            .filter_map(|(o, _)| match o {
                Objective::Public(_) => None,
                Objective::Secret(secret_objective) => Some(secret_objective),
            })
            .filter(|&o| !taken_secret_objectives().contains(o))
            .map(|&o| Objective::Secret(o))
            .collect::<Vec<_>>();
        objectives.sort();
        objectives
    });

    rsx! {
        div { class: "player-secret-title-row",
            FactionIcon { faction: player().faction }
            h6 { "{player_id.clone()}" }
            div {}
        }
        {
            player_secrets()
                .into_iter()
                .map(|secret| {
                    let p3 = player_id.clone();
                    rsx! {
                        div { key: "{secret}", class: "secret-objective-row",
                            "{secret.clone().info().name}"
                            InfoButton { info: Info::Objective(Objective::Secret(secret.clone())) }
                            Button {
                                class: "delete-secret-objective-button",
                                onclick: move |_| {
                                    event
                                        .send_event(Event::UnscoreSecretObjective {
                                            player: p3.clone(),
                                            objective: secret.clone(),
                                        })
                                },
                                Icon {
                                    icon: FaTrash,
                                    width: None,
                                    height: None,
                                    class: "inline-icon",
                                }
                            }
                        }
                    }
                })
        }
        ObjectiveDropdown {
            value: selected_secret_objective,
            on_select: move |obj| selected_secret_objective.set(obj),
            options: unrevealed_secret_objectives(),
        }
        Button {
            disabled: selected_secret_objective().is_none(),
            onclick: move |_| {
                event
                    .send_event(Event::ScoreExtraSecretObjective {
                        player: player_id.clone(),
                        objective: match selected_secret_objective()
                            .expect("Secret objective to be set")
                        {
                            Objective::Public(_) => panic!("Objective to be a secret objective"),
                            Objective::Secret(secret_objective) => secret_objective,
                        },
                    })
            },
            "Score"
        }
    }
}
