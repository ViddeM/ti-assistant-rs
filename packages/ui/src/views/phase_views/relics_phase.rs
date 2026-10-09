use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    components::{planet_attachment::PlanetAttachment, relic::Relic},
};

use crate::{
    components::button::Button,
    data::{event_context::EventContext, game_context::GameContext},
    views::select_tech::SelectTechView,
};

#[component]
pub fn RelicsPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let all_players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .keys()
            .map(|p| {
                (
                    p.clone(),
                    gc.game_state()
                        .players
                        .get(p)
                        .expect("Player to exist")
                        .clone(),
                )
            })
            .collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| a.cmp(b));
        players
    });

    let maw_of_worlds_holder = use_memo(move || {
        all_players()
            .iter()
            .find(|(_, player)| player.relics.contains(&Relic::TheCrownOfEmphidia))
            .map(|(id, _)| id.clone())
    });

    let crown_of_emphidia_holder = use_memo(move || {
        all_players()
            .iter()
            .filter(|(_, player)| player.relics.contains(&Relic::TheCrownOfEmphidia))
            .find(|(_, player)| {
                player
                    .planets
                    .values()
                    .flatten()
                    .find(|&attachments| attachments == &PlanetAttachment::TombOfEmphidia)
                    .is_some()
            })
            .map(|(id, _)| id.clone())
    });

    rsx! {
        div { class: "column card",
            if let Some(holder) = crown_of_emphidia_holder() {
                h2 { "Play the Crown of Emphidia?" }
                fieldset {
                    legend { "{holder}" }
                    Button {
                        onclick: {
                            let holder = holder.clone();
                            move |_| {
                                event
                                    .send_event(Event::PlayCrownOfEmphidia {
                                        player: holder.clone(),
                                    })
                            }
                        },
                        "Play"
                    }
                }
            }

            if let Some(holder) = maw_of_worlds_holder() {
                h2 { "Play of maw of Worlds?" }
                fieldset { class: "screen-container",
                    legend { "{holder}" }
                    SelectTechView {
                        player_id: holder.clone(),
                        on_select: {
                            let holder = holder.clone();
                            move |tech| {
                                event
                                    .send_event(Event::PlayMawOfWorlds {
                                        player: holder.clone(),
                                        tech,
                                    });
                            }
                        },
                    }
                }
            }

            if maw_of_worlds_holder().is_none() && crown_of_emphidia_holder().is_none() {
                h2 { "Continue to status phase" }
            }

            Button {
                class: "margin-top",
                onclick: move |_| event.send_event(Event::CompleteRelicsPhase),
                "Continue"
            }
        }
    }
}
