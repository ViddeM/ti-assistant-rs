//! Tracking how long each player takes on their turns.

use std::time::Duration;

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{SIMPLE_PICKS, three_players};

scenarios![time_tracking_and_pause];

fn minutes(minutes: i64) -> TimeDelta {
    TimeDelta::minutes(minutes)
}

/// Turn time is tracked in the strategy and action phases, and not while paused.
pub fn time_tracking_and_pause() -> TestGame {
    // The clock only moves when told to.
    let mut g = three_players().clock_step(TimeDelta::zero()).start();

    // A minute per pick, so two minutes each.
    for &(player, card) in SIMPLE_PICKS {
        g.advance(minutes(1)).pick_strategy_card(player, card);
    }
    g.apply(Event::CompleteStrategyPhase);

    // Alice thinks for 5 minutes, pauses for a 30 minute break, then takes 2 more minutes.
    g.advance(minutes(5))
        .apply(Event::TrackTime { paused: true })
        .advance(minutes(30))
        .apply(Event::TrackTime { paused: false })
        .advance(minutes(2));
    g.turn("Alice").tactical().commit().end_turn();
    g.advance(minutes(3));
    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.advance(minutes(4));
    g.turn("Carol").play_card(StrategyCard::Politics);
    g.play_out_action_phase();

    // Time in the status phase isn't tracked.
    g.advance(minutes(10))
        .status_phase()
        .reveal(PublicObjective::ExpandBorders)
        .complete();

    let play_time = |player: &str| g.state().players_play_time[&p(player)];
    assert_eq!(play_time("Alice"), Duration::from_secs(9 * 60));
    assert_eq!(play_time("Bob"), Duration::from_secs(5 * 60));
    assert_eq!(play_time("Carol"), Duration::from_secs(6 * 60));
    g
}
