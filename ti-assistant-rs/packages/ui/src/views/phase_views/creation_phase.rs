use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::{color::Color, faction::Faction, player_id::PlayerId},
    game_options::FactionResponse,
    state::player::{NewPlayer, Player},
};

use crate::{
    components::{
        button::Button,
        dropdown::{Dropdown, FactionDropdown},
        faction_icon::FactionIcon,
    },
    data::{event_context::EventContext, game_context::GameContext},
};

const CREATION_PHASE_SCSS: Asset = asset!("/assets/styling/views/phase_views/creation_phase.scss");

#[component]
pub fn CreationPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();
    let players = &gc.game_state().players;
    let player_count = players.len();

    let allowed_number_of_players = player_count >= gc.game_options().min_players
        && player_count <= gc.game_options().max_players;

    rsx! {
        document::Stylesheet { href: CREATION_PHASE_SCSS }

        div { class: "card screen-container setup-card",
            h2 { "Add Players" }
            {
                players
                    .iter()
                    .map(|(_, p)| {
                        rsx! {
                            DisplayPlayer { player: p.clone() }
                        }
                    })
            }
            if players.len() < gc.game_options().max_players {
                AddPlayer {}
            }
            Button {
                class: "start-game-button",
                disabled: !allowed_number_of_players,
                onclick: move |_| event.send_event(Event::CreationDone),
                "Start Game"
            }
        }
    }
}

#[component]
fn DisplayPlayer(player: Player) -> Element {
    let faction = player.faction;
    rsx! {
        div { key: "{player.name}", class: "display-player-container",
            h3 { {player.name} }
            div { class: "faction-row",
                FactionIcon { faction }
                p { class: "faction-name", "{faction.name()}:" }
                p { class: "margin-left",
                    span { class: format!("player-color-{}", player.color.name()), "{player.color}" }
                }
            }
        }
    }
}

const NO_FACTION_SELECTED: &'static str = "not_selected";

#[component]
fn AddPlayer() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let available_factions = use_memo(move || {
        get_available_factions(&gc.game_options().factions, &gc.game_state().players)
    });
    let colors = use_memo(move || gc.game_options().colors.iter().cloned().collect::<Vec<_>>());
    let taken_colors = use_memo(move || {
        gc.game_state()
            .players
            .iter()
            .map(|(_, p)| p.color)
            .collect::<HashSet<_>>()
    });

    let mut new_player_name = use_signal(|| String::new());
    let mut new_player_faction: Signal<Option<Faction>> = use_signal(|| None);
    let mut color: Signal<Color> =
        use_signal(|| *colors().first().expect("there should be colors"));

    let set_faction = move |e: FormEvent| match e.value().as_str() {
        NO_FACTION_SELECTED => {
            new_player_faction.set(None);
        }
        f => {
            let f = f.parse().expect("Failed to parse faction in dropdown?");
            new_player_faction.set(Some(f));
        }
    };

    let add_player = move || {
        event.send_event(Event::AddPlayer {
            player: NewPlayer {
                name: new_player_name(),
                faction: new_player_faction().unwrap(),
                color: color(),
            },
        });
    };

    let mut reset_form = move || {
        new_player_name.set(String::new());
        new_player_faction.set(None);
        color.set(
            colors()
                .iter()
                .find(|c| !taken_colors.contains(c))
                .expect("There to be an available color")
                .clone(),
        )
    };

    rsx! {
        form {
            class: "add-player-form",
            onsubmit: move |e| {
                e.prevent_default();
                reset_form();
            },
            div {
                label { r#for: "player_name_input", "Name: " }
                input {
                    id: "player_name_input",
                    required: true,
                    value: "{new_player_name.read()}",
                    onchange: move |e: FormEvent| new_player_name.set(e.value()),
                }
            }
            div { class: "margin-top",
                label { r#for: "player_faction_dropdown", "Faction: " }
                FactionDropdown {
                    id: "player_faction_dropdown",
                    required: true,
                    value: new_player_faction(),
                    options: available_factions(),
                    on_select: move |f| new_player_faction.set(f),
                }
            }
            div { class: "colors-container",
                {
                    colors()
                        .iter()
                        .cloned()
                        .map(|c| {
                            rsx! {
                                div { key: "{c.name()}", class: "color-container",
                                    label { r#for: "id-{c.name()}",
                                        div {
                                            class: format!(
                                                "color-button {}",
                                                if taken_colors.contains(&c) {
                                                    "disabled-color-button".to_string()
                                                } else {
                                                    format!("player-color-background-{}", c.name())
                                                },
                                            ),
                                        }
                                    }
                                    input {
                                        name: "color",
                                        id: "id-{c.name()}",
                                        r#type: "radio",
                                        value: "{c.name()}",
                                        checked: c == color(),
                                        disabled: taken_colors.contains(&c),
                                        onchange: move |_| color.set(c),
                                    }
                                }
                            }
                        })
                }
            }
            Button {
                class: "margin-top",
                r#type: "submit",
                disabled: new_player_name().is_empty() || new_player_faction().is_none(),
                onclick: move |_| add_player(),
                "Add Player"
            }
        }
    }
}

fn get_available_factions(
    all_factions: &Vec<FactionResponse>,
    players: &HashMap<PlayerId, Player>,
) -> Vec<Faction> {
    let taken_factions = players.values().map(|p| p.faction).collect::<HashSet<_>>();

    let mut factions = all_factions
        .iter()
        .map(|f| f.faction)
        .filter(|f| taken_factions.contains(f) == false)
        .collect::<Vec<_>>();

    factions.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));

    factions
}
