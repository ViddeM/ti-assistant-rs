use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::{
        event::Event,
        strategic::{StrategicSecondaryAction, StrategicSecondaryProgress},
    },
    common::{faction::Faction, player_id::PlayerId},
    components::tech::Technology,
    state::{game_state::StrategicProgress, player::Player},
};

use crate::{
    components::{button::Button, faction_icon::FactionIcon},
    data::{
        event_context::EventContext,
        game_context::GameContext,
        player_view::{PlayerView, PlayerViewContext},
    },
    views::select_tech::SelectTechView,
};

#[derive(Debug, Clone, PartialEq)]
enum Choice {
    NekroVirus,
    Skipped,
    OtherPlayer,
    Technoloy {
        tech: Technology,
    },
    TechnologyJolNar {
        first: Technology,
        second: Option<Technology>,
    },
    YetToChoose,
}

#[component]
pub fn TechSecondary(progress: ReadSignal<StrategicProgress>) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist in action phase")
    });

    let player_choices = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .iter()
            .filter(|&(id, _)| !id.eq(&current_player()))
            .map(|(id, player)| {
                if player.faction == Faction::NekroVirus {
                    return (id, player, Choice::NekroVirus);
                }

                if let Some(prog) = progress().other_players.get(id) {
                    return match prog.clone() {
                        StrategicSecondaryProgress::Skipped => (id, player, Choice::Skipped),
                        StrategicSecondaryProgress::Technology { tech } => {
                            (id, player, Choice::Technoloy { tech })
                        }
                        StrategicSecondaryProgress::TechnologyJolNar {
                            first_tech,
                            second_tech,
                        } => (
                            id,
                            player,
                            Choice::TechnologyJolNar {
                                first: first_tech,
                                second: second_tech,
                            },
                        ),
                        p => panic!("Unexpected progress {p:?} for tech secondary"),
                    };
                }

                let player_active = match view.get()() {
                    PlayerView::Global => true,
                    PlayerView::Player { player_id } => player_id.eq(id),
                };

                if !player_active {
                    return (id, player, Choice::OtherPlayer);
                }

                return (id, player, Choice::YetToChoose);
            })
            .map(|(id, player, choice)| (id.clone(), player.clone(), choice))
            .collect::<Vec<_>>();
        players.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));

        players
    });

    rsx! {
        div { class: "column generic-secondary-container",
            for (id , player , choice) in player_choices().into_iter() {
                fieldset {
                    legend { class: "aligned-legend",
                        h6 { class: "horizontal-padding", "{id}" }
                        FactionIcon { faction: player.faction.clone() }
                    }
                    RenderChoice { player_id: id, player, choice }
                }
            }
        }
    }
}

#[component]
fn RenderChoice(
    player_id: ReadSignal<PlayerId>,
    player: ReadSignal<Player>,
    choice: ReadSignal<Choice>,
) -> Element {
    let event = use_context::<EventContext>();

    match choice() {
        Choice::NekroVirus => rsx! {
            p { "--Nekro Virus cannot research technologies--" }
        },
        Choice::Skipped => rsx! {
            p { "--Skipped--" }
        },
        Choice::OtherPlayer => rsx! {
            p { "Yet to choose" }
        },
        Choice::Technoloy { tech } => rsx! {
            p { "Tech: {tech.info().name}" }
        },
        Choice::TechnologyJolNar { first, second } => rsx! {
            p { "First Tech: {first.info().name}" }
            if let Some(sec) = second {
                p { "Second Tech: {sec.info().name}" }
            } else {
                p { "Skipped second tech" }
            }
        },
        Choice::YetToChoose => {
            if player().faction == Faction::UniversitiesOfJolNar {
                rsx! {
                    JolnarTechSecondary { player_id, player }
                }
            } else {
                rsx! {
                    p { class: "warning-text", "Remember: pay 1 token and 4 resources" }
                    SelectTechView {
                        player_id: player_id(),
                        on_select: move |tech| {
                            event
                                .send_event(Event::StrategicActionSecondary {
                                    player: player_id(),
                                    action: StrategicSecondaryAction::Technology {
                                        tech,
                                    },
                                })
                        },
                    }
                    div { class: "skip-divider" }
                    div { class: "tech-skip-button",
                        Button {
                            onclick: move |_| {
                                event
                                    .send_event(Event::StrategicActionSecondary {
                                        player: player_id(),
                                        action: StrategicSecondaryAction::Skip,
                                    })
                            },
                            "Skip"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn JolnarTechSecondary(player_id: ReadSignal<PlayerId>, player: ReadSignal<Player>) -> Element {
    let mut first_tech: Signal<Option<Technology>> = use_signal(|| None);
    let event = use_context::<EventContext>();

    rsx! {
        p { class: "warning-text",
            "Remember Jol=Nar special, either:"
            br {}
            "Pay 1 token and 4 resources for 1 OR"
            br {}
            "Pay 1 token and 6 resources for 2 techs"
        }

        if let Some(tech) = first_tech() {
            p { "First tech: {tech}" }
            SelectTechView {
                player_id: player_id(),
                on_select: {
                    let f = tech.clone();
                    move |t| {
                        event
                            .send_event(Event::StrategicActionSecondary {
                                player: player_id(),
                                action: StrategicSecondaryAction::TechnologyJolNar {
                                    first_tech: f.clone(),
                                    second_tech: Some(t),
                                },
                            })
                    }
                },
            }
        } else {
            SelectTechView {
                player_id: player_id(),
                on_select: move |t| first_tech.set(Some(t)),
            }
        }
    }
}
