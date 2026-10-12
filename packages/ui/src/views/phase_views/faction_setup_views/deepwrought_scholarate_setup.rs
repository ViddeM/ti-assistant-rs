use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::tech::Technology,
};

use crate::{
    components::button::Button,
    data::{event_context::EventContext, game_context::GameContext},
    views::select_tech::SelectTechView,
};

#[component]
pub fn DeepwroughtScholarateSetup(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut first_tech: Signal<Option<Technology>> = use_signal(|| None);
    let mut second_tech: Signal<Option<Technology>> = use_signal(|| None);

    let pi = player_id.clone();
    let player = use_memo(move || gc.game_state().players[&pi].clone());

    use_effect(move || {
        if player().technologies.is_empty() && (first_tech().is_some() || second_tech().is_some()) {
            first_tech.set(None);
            second_tech.set(None);
        }
    });

    let selected_technologies = use_memo(move || {
        if player().technologies.is_empty() {
            return None;
        }

        let mut techs = player().technologies.iter().cloned().collect::<Vec<_>>();
        techs.sort();
        if techs.len() < 2 {
            return None;
        }

        Some((techs[0].clone(), techs[1].clone()))
    });

    if let Some((first, second)) = selected_technologies() {
        return rsx! {
            div { class: "setup-column",
                p { "{first.info().name}" }
                p { "{second.info().name}" }
            }
        };
    }

    rsx! {
        div { class: "setup-column",
            if let Some(tech) = first_tech() {
                p { "First technology: {tech.info().name}" }
                Button {
                    onclick: move |_| {
                        first_tech.set(None);
                    },
                    "Change"
                }
            } else {
                SelectTechView {
                    player_id: player_id.clone(),
                    on_select: move |t| first_tech.set(Some(t)),
                }
            }

            if let Some(tech) = second_tech() {
                p { "Second technology: {tech.info().name}" }
                Button {
                    onclick: move |_| {
                        second_tech.set(None);
                    },
                    "Change"
                }
            } else if first_tech().is_some() {
                SelectTechView {
                    player_id: player_id.clone(),
                    on_select: move |t| second_tech.set(Some(t)),
                }
            } else {
                "Select the first technology before choosing the second."
            }

            Button {
                disabled: first_tech().is_none() || second_tech().is_none(),
                onclick: move |_| {
                    let first_tech = first_tech().expect("First technology must be selected");
                    let second_tech = second_tech().expect("Second technology must be selected");
                    event
                        .send_event(Event::SetupPlayerTechs {
                            player: player_id.clone(),
                            technologies: vec![first_tech, second_tech],
                        });
                },
                {"Confirm"}
            }
        }
    }
}
