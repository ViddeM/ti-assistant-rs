use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::tech::Technology,
};

use crate::{
    components::button::Button,
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::select_tech::SelectTechView,
};

#[component]
pub fn BookOfLatviniaResearch(player_id: PlayerId) -> Element {
    let view = use_context::<PlayerViewContext>();

    rsx! {
        if view.display_for(&player_id) {
            BookOfLatviniaTechPicker { key: "{player_id}", player_id }
        } else {
            p { "Waiting for {player_id} to research with the Book of Latvinia" }
        }
    }
}

#[component]
fn BookOfLatviniaTechPicker(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected: Signal<Vec<Technology>> = use_signal(Vec::new);

    let excluded_techs = use_memo(move || {
        gc.game_options()
            .technologies
            .iter()
            .filter(|t| !t.info().requirements.is_empty())
            .chain(selected.read().iter())
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "card column",
            h2 { "Book of Latvinia" }
            fieldset { class: "column margin-top",
                legend { "{player_id}" }
                p { "Research up to 2 technologies that have no prerequisites." }

                for tech in selected() {
                    p { key: "{tech}", class: "margin-top", "{tech.info().name}" }
                }

                if selected().len() < 2 {
                    div { class: "full-width",
                        SelectTechView {
                            key: "{selected().len()}",
                            player_id: player_id.clone(),
                            filtered_techs: excluded_techs(),
                            on_select: move |tech| selected.write().push(tech),
                        }
                    }
                }
                    Button {
                        class: "margin-top",
                        onclick: move |_| {
                            event
                                .send_event(Event::ResearchBookOfLatviniaTechs {
                                    player: player_id.clone(),
                                    technologies: selected(),
                                })
                        },
                        if selected().is_empty() {
                            "Skip"
                        } else {
                            "Research"
                        }
                }
            }
        }
    }
}
