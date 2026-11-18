use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::tech::Technology,
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
            div { class: "card column",
                h2 { "Book of Latvinia" }
                fieldset { class: "column margin-top",
                    legend { "{player_id}" }
                    BookOfLatviniaTechPicker { key: "{player_id}", player_id }
                }
            }
        } else {
            p { "Waiting for {player_id} to research with the Book of Latvinia" }
        }
    }
}

#[component]
pub fn BookOfLatviniaTechPicker(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected: Signal<Vec<Technology>> = use_signal(Vec::new);

    let is_nekro = {
        let player_id = player_id.clone();
        use_memo(move || {
            gc.game_state()
                .players
                .get(&player_id)
                .is_some_and(|p| p.faction == Faction::NekroVirus)
        })
    };

    let excluded_techs = use_memo(move || {
        gc.game_options()
            .technologies
            .iter()
            .filter(|t| !t.info().requirements.is_empty())
            .chain(selected.read().iter())
            .cloned()
            .collect::<Vec<_>>()
    });

    let research = {
        let player_id = player_id.clone();
        move |technologies| {
            event.send_event(Event::ResearchBookOfLatviniaTechs {
                player: player_id.clone(),
                technologies,
            })
        }
    };

    if is_nekro() {
        return rsx! {
            p { "Nekro Virus cannot research technologies" }
            p { class: "info-text", "Gain 6 command tokens instead" }
            Button { class: "margin-top", onclick: move |_| research(vec![]), "Continue" }
        };
    }

    rsx! {
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
        Button { class: "margin-top", onclick: move |_| research(selected()),
            if selected().is_empty() {
                "Skip"
            } else {
                "Research"
            }
        }
    }
}
