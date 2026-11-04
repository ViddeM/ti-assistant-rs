use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::galactic_event::GalacticEvent,
};

use crate::{
    components::{
        button::Button, dropdown::PlayerDropdown, inc_dec::IncDecView, info_button::InfoButton,
    },
    data::{event_context::EventContext, game_context::GameContext, info_context::Info},
    views::destroyed_planets::DestroyedPlanets,
};

const GALACTIC_EVENTS_VIEW_MODE_SCSS: Asset =
    asset!("/assets/styling/views/galactic_events_view_mode.scss");

/// Whether the galactic event has a section on the Galactic Events page.
pub fn has_admin_section(event: &GalacticEvent) -> bool {
    matches!(
        event,
        GalacticEvent::StellarAtomics
            | GalacticEvent::ZealousOrthodoxy
            | GalacticEvent::TotalWar
            | GalacticEvent::ConventionsOfWarAbandoned
    )
}

/// The players in the game, sorted.
fn sorted_players(gc: &GameContext) -> Vec<PlayerId> {
    let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
    players.sort();
    players
}

#[component]
pub fn GalacticEventsViewMode() -> Element {
    let gc = use_context::<GameContext>();

    let in_play = use_memo(move || gc.game_state().galactic_events.clone());

    rsx! {
        document::Stylesheet { href: GALACTIC_EVENTS_VIEW_MODE_SCSS }

        div { class: "galactic-events-view-container",
            if in_play().contains(&GalacticEvent::StellarAtomics) {
                StellarAtomics {}
            }
            if in_play().contains(&GalacticEvent::ZealousOrthodoxy) {
                ZealousOrthodoxy {}
            }
            if in_play().contains(&GalacticEvent::TotalWar) {
                TotalWar {}
            }
            if in_play().contains(&GalacticEvent::ConventionsOfWarAbandoned) {
                DestroyedPlanets {}
            }
        }
    }
}

#[component]
fn SectionHeading(galactic_event: GalacticEvent) -> Element {
    rsx! {
        div { class: "galactic-events-heading",
            h3 { "{galactic_event.info().name}" }
            InfoButton { info: Info::GalacticEvent(galactic_event) }
        }
    }
}

#[component]
fn StellarAtomics() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let players = use_memo(move || sorted_players(&gc));
    let used = use_memo(move || gc.game_state().stellar_atomics_used.clone());

    rsx! {
        div { class: "card column margin-top",
            SectionHeading { galactic_event: GalacticEvent::StellarAtomics }
            table { class: "galactic-events-table",
                tbody {
                    for player in players() {
                        tr { key: "{player}",
                            td { "{player}" }
                            td {
                                if used().contains(&player) {
                                    "Token used"
                                } else {
                                    "Token on the card"
                                }
                            }
                            td { class: "align-right",
                                Button {
                                    onclick: {
                                        let player = player.clone();
                                        move |_| {
                                            event
                                                .send_event(Event::SetStellarAtomicsUsed {
                                                    player: player.clone(),
                                                    used: !used().contains(&player),
                                                })
                                        }
                                    },
                                    if used().contains(&player) {
                                        "Return token"
                                    } else {
                                        "Use token"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ZealousOrthodoxy() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let players = use_memo(move || sorted_players(&gc));
    let holder = use_memo(move || {
        gc.game_state()
            .score
            .galactic_events
            .zealous_orthodoxy
            .clone()
    });

    rsx! {
        div { class: "card column margin-top",
            SectionHeading { galactic_event: GalacticEvent::ZealousOrthodoxy }
            PlayerDropdown {
                class: "margin-top full-width",
                value: holder,
                options: players(),
                on_select: move |player| {
                    event
                        .send_event(Event::SetZealousOrthodoxyOwner {
                            player,
                        })
                },
            }
        }
    }
}

#[component]
fn TotalWar() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let players = use_memo(move || sorted_players(&gc));
    let total_war = use_memo(move || gc.game_state().score.galactic_events.total_war.clone());

    rsx! {
        div { class: "card column margin-top",
            SectionHeading { galactic_event: GalacticEvent::TotalWar }
            table { class: "galactic-events-table",
                thead {
                    tr {
                        for player in players() {
                            th { key: "{player}", class: "align-center", "{player}" }
                        }
                    }
                }
                tbody {
                    tr {
                        for player in players() {
                            td { key: "{player}", class: "align-center",
                                IncDecView {
                                    points: total_war().get(&player).copied().unwrap_or(0),
                                    change_points: {
                                        let player = player.clone();
                                        move |points| {
                                            event
                                                .send_event(Event::SetTotalWarPoints {
                                                    player: player.clone(),
                                                    points,
                                                })
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
