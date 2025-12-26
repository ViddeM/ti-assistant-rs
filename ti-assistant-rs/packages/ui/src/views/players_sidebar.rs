use std::{str::FromStr, sync::Arc};

use dioxus::{logger::tracing, prelude::*};
use ti_helper_game_data::common::player_id::PlayerId;

use crate::{
    components::dropdown::Dropdown,
    data::{
        game_context::GameContext,
        player_view::{PlayerView, PlayerViewContext},
    },
};

const PLAYERS_SIDEBAR_SCSS: Asset = asset!("/assets/styling/views/players_sidebar.scss");

#[component]
pub fn PlayersSidebar() -> Element {
    let gc = use_context::<GameContext>();
    let mut view = use_context::<PlayerViewContext>();

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

    rsx! {
        document::Stylesheet { href: PLAYERS_SIDEBAR_SCSS }

        div { class: "card",
            h2 { "Players" }
            label { r#for: "playing-as-dropdown", "Selected player view: " }
            Dropdown {
                id: "playing-as-dropdown",
                value: "{currently_viewing_as()}",
                oninput: handle_playing_as_change,
                option { value: "", "Global view" }
                {gc.game_state().players.keys().map(|p| rsx! {
                    option { value: p.to_string(), "{p}" }
                })}
            }
        }
    }
}
