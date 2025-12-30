use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, common::player_id::PlayerId};

use crate::{
    components::{button::Button, dropdown::Dropdown, faction_icon::FactionIcon},
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

    let p1 = player_id.clone();
    let currently_selected_player = use_memo(move || {
        gc.game_state()
            .score
            .support_for_the_throne
            .get(&p1)
            .cloned()
            .unwrap_or_default()
    });

    let mut selected_player = use_signal(|| currently_selected_player());
    // Ensure this is initialized correctly
    use_effect(move || selected_player.set(currently_selected_player()));

    let p2 = player_id.clone();
    let player = use_memo(move || {
        gc.game_state()
            .players
            .get(&p2)
            .cloned()
            .expect("Player to exist")
    });

    let p3 = player_id.clone();
    let players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .keys()
            .filter(|&p| !p.eq(&p3))
            .cloned()
            .collect::<Vec<_>>();
        players.sort();
        players
    });

    rsx! {
        tr { key: "{player_id}",
            td {
                FactionIcon { faction: player().faction }
            }
            td { "{player_id}" }
            td { "->" }
            td {
                Dropdown {
                    value: "{selected_player()}",
                    oninput: move |e: FormEvent| selected_player.set(e.value().into()),
                    option { value: "", "None" }
                    {
                        players()
                            .iter()
                            .map(|p| {
                                rsx! {
                                    option { key: "{p}", value: "{p}", "{p}" }
                                }
                            })
                    }
                }
            }
            td {
                Button {
                    disabled: currently_selected_player().eq(&selected_player()),
                    onclick: move |_| {
                        event
                            .send_event(Event::GiveSupportForTheThrone {
                                giver: player_id.clone(),
                                receiver: selected_player(),
                            })
                    },
                    "Give"
                }
            }
        }
    }
}
