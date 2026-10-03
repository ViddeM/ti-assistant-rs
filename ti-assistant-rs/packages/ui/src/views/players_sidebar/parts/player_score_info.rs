use dioxus::prelude::*;
use ti_helper_game_data::common::player_id::PlayerId;

use crate::{
    components::ti_icon::{TiIcon, TiIconType},
    data::game_context::GameContext,
};

const PLAYER_SCORE_INFO_SCSS: Asset =
    asset!("/assets/styling/views/players_sidebar/parts/player_score_info.scss");

#[component]
pub fn PlayerScoreInfo(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    let p1 = player_id.clone();
    let current_score = use_memo(move || {
        gc.game_state()
            .score
            .player_points
            .get(&p1)
            .expect("Player to exist")
            .clone()
    });

    let is_custodian = use_memo(move || {
        gc.game_state()
            .score
            .custodians
            .as_ref()
            .eq(&Some(&player_id))
    });

    let custodian = use_memo(move || {
        if is_custodian() {
            rsx! {
                div {
                    TiIcon { icon: TiIconType::Custodians }
                }
            }
        } else {
            rsx! {}
        }
    });

    rsx! {
        document::Stylesheet { href: PLAYER_SCORE_INFO_SCSS }

        div { class: "score-container",
            h3 { class: "score-text", "{current_score()}" }
            {custodian}
        }
    }
}
