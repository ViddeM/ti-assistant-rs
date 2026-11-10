use dioxus::prelude::*;
use ti_helper_game_data::common::player_id::PlayerId;

use crate::{
    components::dropdown::Dropdown,
    data::{
        game_context::GameContext,
        player_view::{PlayerView, PlayerViewContext},
    },
};

#[component]
pub fn PlayerViewDropdown() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let handle_playing_as_change = move |event: FormEvent| {
        let v = event.value();

        if v.is_empty() {
            view.set_global();
        } else {
            let player_id = PlayerId::from(v);
            view.set_player(player_id);
        }
    };

    let currently_viewing_as = use_memo(move || match view.get()() {
        PlayerView::Global => "".to_string(),
        PlayerView::Player { player_id } => player_id.to_string(),
    });

    let players = use_memo(move || gc.game_state().players.keys().cloned().collect::<Vec<_>>());

    rsx! {
        div {
            label { r#for: "playing-as-dropdown", "Selected player view: " }
            Dropdown {
                id: "playing-as-dropdown",
                value: "{currently_viewing_as()}",
                oninput: handle_playing_as_change,
                option { value: "", "Global view" }
                for player in players().iter() {
                    option { value: player.to_string(), "{player}" }
                }
            }
        }
    }
}
