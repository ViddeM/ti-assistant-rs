use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::strategic::StrategicPrimaryAction, common::faction::Faction,
    components::tech::Technology, state::game_state::StrategicPrimaryProgress,
};

use crate::{
    components::button::Button, data::game_context::GameContext, views::select_tech::SelectTechView,
};

#[component]
pub fn TechnologyPrimaryView(
    primary: Option<StrategicPrimaryProgress>,
    on_commit: EventHandler<StrategicPrimaryAction>,
) -> Element {
    let gc = use_context::<GameContext>();

    let current_player = use_memo(move || {
        gc.game_state()
            .current_player
            .clone()
            .expect("Current player to exist")
    });

    let player_faction = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .find(|&(id, _)| current_player().eq(id))
            .map(|(_, player)| player.faction)
            .expect("Current player to exist")
    });

    let mut first_tech = use_signal(|| None);

    if player_faction().eq(&Faction::NekroVirus) {
        return rsx! {
            div { class: "column",
                p { "--Unable to research technologies--" }
            }
        };
    }

    let filtered_techs = use_memo(move || first_tech().map(|t: Technology| vec![t]));

    if let Some(StrategicPrimaryProgress::Technology {
        first_tech,
        second_tech,
    }) = primary.clone()
    {
        // Action has already been taken, display the chosen techs.
        return rsx! {
            div { class: "column",
                if let Some(t) = first_tech {
                    p { "{t.info().name}" }
                }
                if let Some(e) = second_tech {
                    p { "{e.info().name}" }
                }
            }
        };
    }

    rsx! {
        div { class: "column primary-container",
            if let Some(first) = first_tech() {
                // One tech has been taken, allow them to pick another.
                p { "{first.info().name}" }
                fieldset { class: "primary-container",
                    legend {
                        h6 { "Take another?" }
                    }
                    p { class: "warning-text", "Remember: pay 6 resources" }
                    SelectTechView {
                        player_id: current_player(),
                        filtered_techs: filtered_techs(),
                        on_select: move |extra| {
                            on_commit(StrategicPrimaryAction::Technology {
                                // Expect here to avoid silent errors.
                                first_tech: Some(first_tech().expect("First tech to be selected")),
                                second_tech: Some(extra),
                            });
                        },
                    }
                }
            } else {
                fieldset { class: "primary-container",
                    legend {
                        h6 { "Pick a tech" }
                    }
                    SelectTechView {
                        player_id: current_player(),
                        on_select: move |tech| first_tech.set(Some(tech)),
                    }
                }
            }
            Button {
                onclick: move |_| {
                    on_commit(StrategicPrimaryAction::Technology {
                        first_tech: first_tech(),
                        second_tech: None,
                    })
                },
                if first_tech().is_some() {
                    "Done"
                } else {
                    "Skip"
                }
            }
        }
    }
}
