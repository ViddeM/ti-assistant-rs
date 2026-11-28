//! Tracking how long each player takes on their turns.

use std::time::Duration;

use ti_helper_game_data::state::time_stats::PhaseGroup;
use ti_helper_game_logic::gameplay::time_stats::time_stats;
use ti_helper_system_tests::prelude::*;

use crate::fixtures::{SIMPLE_PICKS, agenda_phase, three_players};

scenarios![time_tracking_and_pause, time_agenda_phase_not_tracked];

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
    g.advance(minutes(4))
        .status_phase()
        .secret("Bob", SecretObjective::BecomeTheGatekeeper);
    g.advance(minutes(6))
        .status_phase()
        .reveal(PublicObjective::ExpandBorders)
        .complete();

    let play_time = |player: &str| g.state().players_play_time[&p(player)];
    assert_eq!(play_time("Alice"), Duration::from_secs(9 * 60));
    assert_eq!(play_time("Bob"), Duration::from_secs(5 * 60));
    assert_eq!(play_time("Carol"), Duration::from_secs(6 * 60));

    let now = g.history().last().expect("Events to exist").1;
    let stats = time_stats(g.history(), now).expect("History to replay");
    assert_eq!(stats.paused_total(), Duration::from_secs(30 * 60));

    let alice_turn = stats
        .turns()
        .into_iter()
        .find(|turn| turn.player == p("Alice") && turn.phase == PhaseGroup::Action)
        .expect("Alice to have taken a turn");
    assert_eq!(alice_turn.duration, Duration::from_secs(7 * 60));

    let round_1 = &stats.phase_durations()[&1];
    assert_eq!(round_1[&PhaseGroup::Status], Duration::from_secs(10 * 60));

    let bob_scored = stats
        .segments
        .iter()
        .find(|segment| segment.scores.get(&p("Bob")) == Some(&1))
        .expect("Bob's point to be in a segment");
    assert_eq!(bob_scored.phase, Phase::Status);
    assert_eq!(bob_scored.duration(), Duration::from_secs(6 * 60));
    g
}

/// Time in the agenda phase isn't anyone's turn.
pub fn time_agenda_phase_not_tracked() -> TestGame {
    let mut g = agenda_phase(three_players().pok().clock_step(TimeDelta::zero()));
    let before = g.state().players_play_time.clone();

    g.advance(minutes(10))
        .agenda(Agenda::ConventionsOfWar)
        .vote_all(FOR, 2)
        .resolve_expected();
    g.advance(minutes(5))
        .agenda(Agenda::PoliticalCensure)
        .vote_all(elect("Carol"), 1)
        .resolve_expected();
    g.advance(minutes(5)).complete_agenda_phase();
    assert_eq!(g.phase(), Phase::Strategy);
    assert_eq!(g.state().players_play_time, before);

    let now = g.history().last().expect("Events to exist").1;
    let stats = time_stats(g.history(), now).expect("History to replay");
    assert!(
        stats
            .turns()
            .iter()
            .all(|turn| turn.phase != PhaseGroup::Agenda),
        "No turn in the agenda phase"
    );
    g
}
