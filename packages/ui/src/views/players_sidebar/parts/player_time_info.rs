use std::time::Duration as StdDuration;

use chrono::{Duration, Utc};
use dioxus::prelude::*;
use dioxus_sdk::time::use_interval;
use ti_helper_game_data::common::player_id::PlayerId;

use crate::data::game_context::GameContext;

#[component]
pub fn PlayerTimeInfo(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    let p1 = player_id.clone();
    let logged = use_memo(move || {
        gc.game_state()
            .players_play_time
            .get(&p1)
            .cloned()
            .map(|d| Duration::from_std(d).expect("Duration to be in range"))
            .unwrap_or_default()
    });

    let p2 = player_id.clone();
    let running_since = use_memo(move || {
        if gc.game_state().current_player.as_ref() == Some(&p2) {
            gc.game_state().current_turn_start_time
        } else {
            None
        }
    });

    let mut now = use_signal(Utc::now);
    use_interval(StdDuration::from_secs(1), move |()| {
        // Check if we're the active player and if so ensure that the timer is updated.
        if running_since.peek().is_some() {
            now.set(Utc::now());
        }
    });

    let total = {
        let extra = match running_since() {
            Some(start) => (now() - start).max(Duration::zero()),
            None => Duration::zero(),
        };

        logged() + extra
    };

    rsx! {
        p { "{format_duration(&total)}" }
    }
}

fn format_duration(duration: &Duration) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}
