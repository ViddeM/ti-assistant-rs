use dioxus::prelude::*;
use ti_helper_game_data::common::faction::Faction;

use crate::components::faction_icon::FactionIcon;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FactionButtonProps {
    faction: Faction,
    selected: bool,
    onclick: EventHandler<()>,
}

#[component]
pub fn FactionButton(
    FactionButtonProps {
        faction,
        selected,
        onclick,
    }: FactionButtonProps,
) -> Element {
    rsx! {
        button {
            class: "faction-button",
            class: if selected { "faction-button-selected" },
            onclick: move |_| onclick(()),
            FactionIcon { faction, width: 32, height: 32 }
        }
    }
}
