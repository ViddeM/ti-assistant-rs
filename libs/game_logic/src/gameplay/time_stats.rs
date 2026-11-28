use chrono::{DateTime, Utc};
use ti_helper_game_data::{
    actions::event::Event,
    state::{
        game_state::GameState,
        time_stats::{TimeSegment, TimeStats},
    },
};

use super::{error::GameError, game_event_handler::update_game_state};

/// Replays the `history` of a game to find how time has been spent, with an ongoing tracked
/// turn lasting until `now`.
pub fn time_stats(
    history: &[(Event, DateTime<Utc>)],
    now: DateTime<Utc>,
) -> Result<TimeStats, GameError> {
    let mut state = GameState::default();
    let mut segments = Vec::new();
    let mut open: Option<TimeSegment> = None;

    for (event, timestamp) in history {
        update_game_state(&mut state, event.clone(), *timestamp)?;
        let next = TimeSegment {
            start: *timestamp,
            end: *timestamp,
            round: state.round,
            phase: state.phase,
            turn_number: state.turn_number,
            player: state.current_player.clone(),
            tracked: state.current_turn_start_time.is_some(),
            paused: state.time_tracking_paused,
            scores: (state.score.player_points.iter())
                .map(|(player, points)| (player.clone(), *points))
                .collect(),
        };

        match open.take() {
            Some(segment) if same_period(&segment, &next) => open = Some(segment),
            Some(mut segment) => {
                segment.end = *timestamp;
                if segment.end > segment.start {
                    segments.push(segment);
                }
                open = Some(next);
            }
            None => open = Some(next),
        }
    }

    if let Some(mut segment) = open {
        if segment.tracked {
            segment.end = now.max(segment.start);
        } else if let Some((_, last)) = history.last() {
            segment.end = *last;
        }
        if segment.end > segment.start {
            segments.push(segment);
        }
    }

    Ok(TimeStats { segments })
}

fn same_period(a: &TimeSegment, b: &TimeSegment) -> bool {
    a.round == b.round
        && a.phase == b.phase
        && a.turn_number == b.turn_number
        && a.player == b.player
        && a.tracked == b.tracked
        && a.paused == b.paused
        && a.scores == b.scores
}
