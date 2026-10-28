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
pub fn BookOfLatviniaResearch() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let researcher = use_memo(move || gc.game_state().book_of_latvinia_research.clone());

    rsx! {
        if let Some(player_id) = researcher() {
            if view.display_for(&player_id) {
                BookOfLatviniaTechPicker { key: "{player_id}", player_id }
            }
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
        fieldset { class: "column screen-container margin-top",
            legend { "Book of Latvinia: {player_id}" }
            p { "Research up to 2 technologies that have no prerequisites." }
            for tech in selected() {
                p { key: "{tech}", class: "margin-top", "{tech.info().name}" }
            }
            if selected().len() < 2 {
                SelectTechView {
                    key: "{selected().len()}",
                    player_id: player_id.clone(),
                    filtered_techs: excluded_techs(),
                    on_select: move |tech| selected.write().push(tech),
                }
            }
            div { class: "center-row margin-top",
                Button {
                    class: "margin-right",
                    disabled: selected().is_empty(),
                    onclick: move |_| selected.set(Vec::new()),
                    "Clear"
                }
                Button {
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
