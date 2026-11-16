use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, components::relic::Relic};

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
            .filter(|_| gc.game_state().is_agenda_enabled())
            .map(|(id, _)| id.clone())
    });

    let crown_of_emphidia_holder = use_memo(move || {
        all_players()
            .iter()
            .find(|(_, player)| {
                player.relics.contains(&Relic::TheCrownOfEmphidia)
                    && player.controls_tomb_of_emphidia()
            })
            .map(|(id, _)| id.clone())
    });

    let next_phase = use_memo(move || {
        if gc.game_state().is_agenda_enabled() {
            "agenda"
        } else {
            "strategy"
        }
    });

    rsx! {
        div { class: "column card",
            if let Some(holder) = crown_of_emphidia_holder() {
                h2 { "Play the Crown of Emphidia?" }
                fieldset {
                    legend { "{holder}" }
                    if view.display_for(&holder) {
                        p { class: "warning-text", "Remember to purge this card" }
                        p { class: "info-text", "Gain 1 VP" }
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
                        p { class: "warning-text",
                            "Remember to exhaust all of your planets and purge this card"
                        }
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
                h2 { "Continue to the {next_phase} phase" }
            }

            SpeakerStep {
                class: "margin-top",
                ready: true,
                waiting_for: "start the {next_phase} phase",
                onclick: move |_| event.send_event(Event::CompleteRelicsPhase),
                "Continue"
            }
        }
    }
}
