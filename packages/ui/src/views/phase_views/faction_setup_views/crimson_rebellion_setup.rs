use crate::{
    components::{button::Button, dropdown::TechDropdown},
    data::{event_context::EventContext, game_context::GameContext},
};
use dioxus::prelude::*;
use std::sync::Arc;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    components::tech::{TechCategory, TechOrigin, TechType, Technology},
    state::player::Player,
};

#[component]
pub fn CrimsonRebellionSetup(player_id: PlayerId, player: Arc<Player>) -> Element {
    let event = use_context::<EventContext>();
    let gc = use_context::<GameContext>();

    let mut tech = use_signal(|| None::<Technology>);

    // "Choose one red or blue with no prerequisites."
    let available_techs = use_memo(move || {
        let mut techs = gc
            .game_options()
            .technologies
            .iter()
            .map(|t| (t, t.info()))
            .filter(|(_, info)| {
                matches!(
                    info.origin,
                    TechOrigin::Base | TechOrigin::Faction(Faction::CrimsonRebellion)
                )
            })
            .filter(|(_, info)| info.requirements.is_empty())
            .filter(|(_, info)| {
                matches!(
                    info.tech_type,
                    TechType::Category(TechCategory::Warfare)
                        | TechType::Category(TechCategory::Propulsion)
                )
            })
            .map(|(t, _)| t)
            .cloned()
            .collect::<Vec<_>>();
        techs.sort();
        techs
    });

    let taken_tech = player.technologies.iter().next();

    rsx! {
        div { class: "setup-column",
            if let Some(tech) = taken_tech {
                p { "{tech.info().name}" }
            } else {
                TechDropdown {
                    value: tech,
                    options: available_techs(),
                    on_select: move |t| tech.set(t),
                }
                Button {
                    disabled: tech().is_none(),
                    onclick: move |_| {
                        event
                            .send_event(Event::SetupPlayerTechs {
                                player: Arc::clone(&player_id),
                                technologies: vec![tech().unwrap()],
                            });
                    },
                    "Choose"
                }
            }

        }
    }
}
