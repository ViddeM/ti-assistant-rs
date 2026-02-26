use chrono::Duration;
use dioxus::prelude::*;
use ti_helper_game_data::common::player_id::PlayerId;

use crate::data::game_context::GameContext;

#[component]
pub fn PlayerTimeInfo(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    let p1 = player_id.clone();
    let play_time = use_memo(move || {
        let dur = gc
            .game_state()
            .players_play_time
            .get(&p1)
            .cloned()
            .unwrap_or_default();
        format_duration(&Duration::from_std(dur).expect("Duration to be in range"))
    });

    // TODO: Make the time 'tick' whilst the player is active.

    rsx! {
        p { "{play_time()}" }
    }
}

fn format_duration(duration: &Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes(),
        duration.num_seconds()
    )
}
