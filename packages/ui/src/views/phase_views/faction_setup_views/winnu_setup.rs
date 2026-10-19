use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::tech::Technology,
    state::player::Player,
};

use crate::{
    components::{button::Button, dropdown::TechDropdown},
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn WinnuSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_tech = use_signal::<Option<Technology>>(|| None);

    let available_techs = use_memo(move || {
        let mut techs = Faction::Winnu
            .get_starting_tech_options(&gc.game_state())
            .expect("Winnu to have starting tech options");
        techs.sort();
        techs
    });

    let tech = player.technologies.iter().next().map(|t| t.info().name);

    rsx! {
        div {
            {
                if let Some(tech) = tech {
                    rsx! {
                        p { {tech} }
                    }
                } else {
                    rsx! {
                        div { class: "setup-column",
                            TechDropdown {
                                value: selected_tech,
                                options: available_techs(),
                                on_select: move |t| selected_tech.set(t),
                            }
                            Button {
                                disabled: selected_tech().is_none(),
                                onclick: move |_| {
                                    event
                                        .send_event(Event::SetupPlayerTechs {
                                            player: Arc::clone(&player_id),
                                            technologies: vec![selected_tech().expect("Tech to be selected")],
                                        })
                                },
                                "Select"
                            }
                        }
                    }
                }
            }
        }
    }
}
