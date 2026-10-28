use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, components::relic::Relic};

use crate::{
    components::{dropdown::PlayerDropdown, info_button::InfoButton},
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
};

#[component]
pub fn RelicOwnersView() -> Element {
    let gc = use_context::<GameContext>();

    let relics = use_memo(move || {
        let mut relics = gc.game_options().relics.clone();
        relics.sort();
        relics
    });

    rsx! {
        if !relics().is_empty() {
            div { class: "card margin-bottom",
                table { class: "relic-owners-table",
                    thead {
                        tr {
                            th { colspan: 3,
                                h2 { "Relic Owners" }
                            }
                        }
                    }
                    tbody {
                        for relic in relics() {
                            RelicOwnerView { key: "{relic}", relic }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RelicOwnerView(relic: Relic) -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let r = relic.clone();
    let owner = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .find(|(_, player)| player.relics.contains(&r))
            .map(|(id, _)| id.clone())
    });

    let players = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    let selected_relic = relic.clone();

    rsx! {
        tr {
            td {
                InfoButton { info: Info::Relic(relic.clone()) }
            }
            td { {relic.info().name} }
            td {
                PlayerDropdown {
                    value: owner,
                    options: players(),
                    on_select: move |player| {
                        event
                            .send_event(Event::SetRelicOwner {
                                relic: selected_relic.clone(),
                                player,
                            })
                    },
                }
            }
        }
    }
}
