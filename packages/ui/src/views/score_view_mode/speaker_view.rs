use std::sync::Arc;

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{faction::Faction, player_id::PlayerId},
    state::faction_state::FactionState,
};

use crate::{
    components::faction_button::FactionButton,
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn SpeakerView() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || {
        let mut players = (gc.game_state().players.iter())
            .map(|(id, p)| (id.clone(), p.clone()))
            .collect::<Vec<_>>();
        players.sort_by_key(|(p, _)| p.to_lowercase());
        players
    });
    let speaker = use_memo(move || gc.game_state().speaker.clone());

    rsx! {
        div { class: "card margin-bottom speaker-view",
            h2 { "Speaker" }
            div { class: "speaker-view-players",
                for (id , player) in players().into_iter() {
                    SpeakerItem {
                        key: "{id}",
                        player_id: id.clone(),
                        faction: player.faction,
                        faction_state: player.faction_state.clone(),
                        is_speaker: speaker().as_ref() == Some(&id),
                    }
                }
            }
        }
    }
}

#[component]
fn SpeakerItem(
    player_id: PlayerId,
    faction: Faction,
    faction_state: Arc<FactionState>,
    is_speaker: bool,
) -> Element {
    let event = use_context::<EventContext>();

    let p = player_id.clone();
    rsx! {
        div { class: "speaker-view-player",
            FactionButton {
                faction,
                faction_state,
                selected: is_speaker,
                onclick: move |_| {
                    if !is_speaker {
                        event
                            .send_event(Event::SetSpeaker {
                                player: p.clone(),
                            })
                    }
                },
            }
            span { "{player_id}" }
        }
    }
}
