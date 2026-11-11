use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    components::{planet_attachment::PlanetAttachment, relic::Relic},
};

use crate::{
    components::button::Button,
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
    views::{phase_views::speaker_step::SpeakerStep, select_tech::SelectTechView},
};

#[component]
pub fn RelicsPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let view = use_context::<PlayerViewContext>();

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
            .find(|(_, player)| player.relics.contains(&Relic::MawOfWorlds))
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
                    if view.display_for(&holder) {
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
                    } else {
                        p { "Waiting for {holder}" }
                    }
                }
            }

            if let Some(holder) = maw_of_worlds_holder() {
                h2 { "Play the Maw of Worlds?" }
                fieldset { class: "screen-container",
                    legend { "{holder}" }
                    if view.display_for(&holder) {
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
                    } else {
                        p { "Waiting for {holder}" }
                    }
                }
            }

            if maw_of_worlds_holder().is_none() && crown_of_emphidia_holder().is_none() {
                h2 { "Continue to agenda phase" }
            }

            SpeakerStep {
                class: "margin-top",
                ready: true,
                waiting_for: "start the agenda phase",
                onclick: move |_| event.send_event(Event::CompleteRelicsPhase),
                "Continue"
            }
        }
    }
}
