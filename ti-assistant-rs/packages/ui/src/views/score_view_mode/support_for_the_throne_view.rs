use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::{button::Button, dropdown::PlayerDropdown, faction_icon::FactionIcon},
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn SupportForTheThroneView() -> Element {
    let gc = use_context::<GameContext>();

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        div { class: "card margin-bottom",
            table { class: "support-for-the-throne-table",
                thead {
                    tr {
                        th { colspan: 5,
                            h2 { "Support for the Throne" }
                        }
                    }
                }
                tbody {
                    {players().into_iter().map(|p| rsx! {
                        PlayerSupportForTheThroneView { player_id: p }
                    })}
                }
            }
        }
    }
}

#[component]
fn PlayerSupportForTheThroneView(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut selected_player = use_signal(|| None);

    let p1 = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p1)
            .cloned()
            .expect("Player to exist")
    });

    let p2 = player_id.clone();
    let players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .keys()
            .filter(|&p| !p.eq(&p2))
            .cloned()
            .collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        tr {
            td {
                FactionIcon { faction: player().faction }
            }
            td { "{player().name}" }
            td { "->" }
            td {
                PlayerDropdown {
                    value: selected_player,
                    options: players(),
                    on_select: move |p| selected_player.set(p),
                }
            }
            td {
                Button {
                    disabled: selected_player().eq(&Some(player_id.clone())),
                    onclick: move |_| {
                        event
                            .send_event(Event::GiveSupportForTheThrone {
                                giver: player_id.clone(),
                                receiver: selected_player().expect("Player to be selected"),
                            })
                    },
                    "Give"
                }
            }
        }
    }
}
