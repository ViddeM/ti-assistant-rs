use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaTrash};
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::{button::Button, faction_icon::FactionIcon, info_button::InfoButton},
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
};

#[component]
pub fn PlayerRelicsGrid() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "player-relic-cards-container",
            for player_id in players().iter() {
                PlayerRelicsCard { key: "{player_id}", player_id: player_id.clone() }
            }
        }
    }
}

#[component]
fn PlayerRelicsCard(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&player_id)
            .cloned()
            .expect("Player to exist")
    });

    rsx! {
        div { class: "card player-relics-table-container",
            table { class: "player-relics-table",
                thead {
                    tr {
                        th { colspan: 3,
                            div { class: "player-name-row",
                                FactionIcon {
                                    faction: player().faction,
                                    faction_state: player().faction_state.clone(),
                                }
                                h2 { "{player().name}" }
                                FactionIcon {
                                    faction: player().faction,
                                    faction_state: player().faction_state.clone(),
                                }
                            }
                        }
                    }
                }
                tbody {
                    for relic in player().relics.iter().cloned() {
                        tr { key: "{relic}", class: "relic-row",
                            td {
                                InfoButton { info: Info::Relic(relic.clone()) }
                            }
                            td { class: "align-left", "{relic.info().name}" }
                            td { class: "align-right",
                                Button {
                                    class: "remove-relic-button",
                                    onclick: {
                                        let r = relic.clone();
                                        move |_| {
                                            event
                                                .send_event(Event::SetRelicOwner {
                                                    relic: r.clone(),
                                                    player: None,
                                                })
                                        }
                                    },
                                    Icon { class: "inline-icon", icon: FaTrash }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
