use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::breakthrough::Breakthrough,
    state::game_state::{ActionPhaseProgress, FractureProgress},
};

use crate::{
    components::{button::Button, ti_icon::TiIcon},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn FractureView() -> Element {
    let gc = use_context::<GameContext>();

    let progress = use_memo(move || {
        gc.game_state()
            .action_progress
            .clone()
            .and_then(|p| match p {
                ActionPhaseProgress::Fracture(progress) => Some(progress),
                _ => None,
            })
            .expect("There to be fracture state")
    });

    match progress().progress {
        FractureProgress::Roll(roll_state) => {
            rsx! { FractureRoll { player_id: roll_state.current_player().clone() } }
        }
        FractureProgress::Spawned { player } => {
            rsx! { FractureEnteredPlay { player_id: player } }
        }
    }
}

#[component]
fn FractureRoll(player_id: PlayerId) -> Element {
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let roller = player_id.clone();
    let roll = use_callback(move |entered_play| {
        event.send_event(Event::RollForTheFracture {
            player: roller.clone(),
            entered_play,
        })
    });

    rsx! {
        div { class: "card column",
            h2 { "The Fracture" }
            p { "{player_id} gained their breakthrough. Roll 1 die: on a 1 or 10, the Fracture enters play." }
            if view.display_for(&player_id) {
                div { class: "center-row margin-top",
                    Button { class: "margin-right", onclick: move |_| roll(true), "It entered play" }
                    Button { onclick: move |_| roll(false), "It didn't" }
                }
            } else {
                p { "Waiting for {player_id} to roll." }
            }
        }
    }
}

#[component]
fn FractureEnteredPlay(player_id: PlayerId) -> Element {
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();
    let gc = use_context::<GameContext>();

    let spawner = player_id.clone();
    let spawn_fracture = use_callback(move |_| {
        event.send_event(Event::FractureSpawned {
            player: spawner.clone(),
        })
    });

    let p1 = player_id.clone();
    let breakthrough = use_memo(move || gc.game_state().players[&p1.clone()].breakthrough.clone());

    rsx! {
        div {
            class: "card column",
            h2 { "Fracture Setup" }
            ol {
                li {
                    "Place the fracture tiles along any edge of the game board."
                }
                li {
                    "Place neutral units in accordance with the backside of the fracture tiles."
                }

                if let Some(bt) = breakthrough() {
                    li { "Place ingress tokens:" }
                    IngressPlacement { breakthrough: bt }
                } else {
                    p { class: "warning-text", "Failed to read breakthrough, this may be a bug?"}
                    li { "Place ingress tokens" }
                }
            }

            if view.display_for(&player_id.clone()) {
                Button {
                    class: "margin-top",
                    onclick: move |_| spawn_fracture(()),
                    "Done"
                }
            } else {
                p { "Waiting for {player_id} to complete fracture setup." }
            }
        }
    }
}

#[component]
fn IngressPlacement(breakthrough: ReadSignal<Breakthrough>) -> Element {
    let gc = use_context::<GameContext>();

    let colors = use_memo(move || breakthrough().info().synergy);

    let expedition_is_finished = use_memo(move || {
        gc.game_state()
            .te_expedition
            .as_ref()
            .expect("Thunder's edge to be enabled")
            .completed
            .is_some()
    });
    let final_token = use_memo(move || {
        if expedition_is_finished() {
            rsx! { li {
                "Finally, also place one token in the Thunder's Edge system."
            }}
        } else {
            rsx! { li {
                "Finally, also place one token in the Mecatol Rex system."
            }}
        }
    });

    let Some((first_color, second_color)) = colors() else {
        return rsx! {
            ol {
                li {
                    "Choose four planets that:"
                    ol {
                        li { "Are in different systems." }
                        li { "Have different technology specialities." }
                    }
                }
                li {
                    "Place one ingress token in each of those systems, if there are not four different technology specialities in play, the remaining tokens are not placed."
                }

                {final_token}
            }
        };
    };

    rsx! {
        ol {
            li {
                span {
                    "Choose 3 systems that contain a "
                    TiIcon { icon: (&first_color).into() }
                    " planet, if able, and place an ingress token in each of them."
                }
            }

            li {
                span {
                    "Choose 3 systems that contain a "
                    TiIcon { icon: (&second_color).into() }
                    " planet, if able, and place an ingress token in each of them."
                }
            }

            p { class: "warning-text", "A system can only contain 1 ingress token and at most 6 ingress tokens can be placed this way."}

            {final_token}
        }
    }
}
