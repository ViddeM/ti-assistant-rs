use std::{collections::BTreeMap, fmt::Display, time::Duration};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{common::player_id::PlayerId, components::phase::Phase};

/// The phases of a round, with the sub-phases of each folded into it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PhaseGroup {
    /// Game creation and setup.
    Setup,
    /// The strategy phase.
    Strategy,
    /// The action phase, including every kind of action.
    Action,
    /// The status phase.
    Status,
    /// The agenda phase.
    Agenda,
}

impl PhaseGroup {
    /// Every phase group, in the order they occur.
    pub const ALL: [PhaseGroup; 5] = [
        PhaseGroup::Setup,
        PhaseGroup::Strategy,
        PhaseGroup::Action,
        PhaseGroup::Status,
        PhaseGroup::Agenda,
    ];
}

impl From<Phase> for PhaseGroup {
    fn from(phase: Phase) -> Self {
        match phase {
            Phase::Creation | Phase::Setup | Phase::PostSetup => PhaseGroup::Setup,
            Phase::Strategy => PhaseGroup::Strategy,
            Phase::Action
            | Phase::StrategicAction
            | Phase::TacticalAction
            | Phase::ActionCardAction
            | Phase::LeaderAction
            | Phase::FrontierCardAction
            | Phase::BreakthroughAction
            | Phase::RelicAction
            | Phase::Fracture
            | Phase::EndActionTurn => PhaseGroup::Action,
            Phase::EntropicScar | Phase::Status | Phase::Relics => PhaseGroup::Status,
            Phase::Agenda | Phase::AgendaEffect => PhaseGroup::Agenda,
        }
    }
}

impl Display for PhaseGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            PhaseGroup::Setup => "Setup",
            PhaseGroup::Strategy => "Strategy",
            PhaseGroup::Action => "Action",
            PhaseGroup::Status => "Status",
            PhaseGroup::Agenda => "Agenda",
        };
        write!(f, "{name}")
    }
}

/// A stretch of time during which nothing relevant to time-tracking changed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeSegment {
    /// When the segment started.
    pub start: DateTime<Utc>,
    /// When the segment ended.
    pub end: DateTime<Utc>,
    /// The round of the game.
    pub round: u32,
    /// The phase of the game.
    pub phase: Phase,
    /// The action turn counter of the game.
    pub turn_number: u32,
    /// The current player, if any.
    pub player: Option<PlayerId>,
    /// Whether the time counts toward the current player's play time.
    pub tracked: bool,
    /// Whether time-tracking was paused.
    pub paused: bool,
}

impl TimeSegment {
    /// The length of the segment.
    pub fn duration(&self) -> Duration {
        (self.end - self.start).to_std().unwrap_or_default()
    }

    fn tracked_player(&self) -> Option<&PlayerId> {
        self.player.as_ref().filter(|_| self.tracked)
    }
}

/// A single turn of a player, possibly interrupted by pauses.
#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    /// The player taking the turn.
    pub player: PlayerId,
    /// The round of the turn.
    pub round: u32,
    /// The phase the turn started in.
    pub phase: PhaseGroup,
    /// When the turn started.
    pub start: DateTime<Utc>,
    /// The tracked time of the turn.
    pub duration: Duration,
}

/// Summary of the turns of a single player.
#[derive(Clone, Debug, PartialEq)]
pub struct TurnSummary {
    /// The number of turns.
    pub count: usize,
    /// The average turn time.
    pub average: Duration,
    /// The median turn time.
    pub median: Duration,
    /// The longest turn time.
    pub longest: Duration,
}

/// How time has been spent over a game.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimeStats {
    /// Every stretch of time from the first event of the game, in order.
    pub segments: Vec<TimeSegment>,
}

impl TimeStats {
    /// The total tracked time of each player.
    pub fn player_totals(&self) -> BTreeMap<PlayerId, Duration> {
        let mut totals = BTreeMap::<PlayerId, Duration>::new();
        for segment in &self.segments {
            if let Some(player) = segment.tracked_player() {
                *totals.entry(player.clone()).or_default() += segment.duration();
            }
        }
        totals
    }

    /// The tracked time of each player, per round.
    pub fn per_round(&self) -> BTreeMap<u32, BTreeMap<PlayerId, Duration>> {
        let mut rounds = BTreeMap::<u32, BTreeMap<PlayerId, Duration>>::new();
        for segment in &self.segments {
            if let Some(player) = segment.tracked_player() {
                *rounds
                    .entry(segment.round)
                    .or_default()
                    .entry(player.clone())
                    .or_default() += segment.duration();
            }
        }
        rounds
    }

    /// Every turn of the game, in order.
    pub fn turns(&self) -> Vec<Turn> {
        let mut turns: Vec<(Turn, u32)> = Vec::new();
        for segment in &self.segments {
            let Some(player) = segment.tracked_player() else {
                continue;
            };
            match turns.last_mut() {
                Some((turn, turn_number))
                    if turn.player == *player
                        && turn.round == segment.round
                        && *turn_number == segment.turn_number =>
                {
                    turn.duration += segment.duration();
                }
                _ => turns.push((
                    Turn {
                        player: player.clone(),
                        round: segment.round,
                        phase: segment.phase.into(),
                        start: segment.start,
                        duration: segment.duration(),
                    },
                    segment.turn_number,
                )),
            }
        }
        turns.into_iter().map(|(turn, _)| turn).collect()
    }

    /// The longest turns of the game, longest first.
    pub fn longest_turns(&self, count: usize) -> Vec<Turn> {
        let mut turns = self.turns();
        turns.sort_by_key(|turn| std::cmp::Reverse(turn.duration));
        turns.truncate(count);
        turns
    }

    /// A summary of the turns of each player.
    pub fn turn_summaries(&self) -> BTreeMap<PlayerId, TurnSummary> {
        let mut durations = BTreeMap::<PlayerId, Vec<Duration>>::new();
        for turn in self.turns() {
            durations
                .entry(turn.player)
                .or_default()
                .push(turn.duration);
        }
        durations
            .into_iter()
            .map(|(player, mut durations)| {
                durations.sort();
                let count = durations.len();
                let total: Duration = durations.iter().sum();
                let median = if count % 2 == 0 {
                    (durations[count / 2 - 1] + durations[count / 2]) / 2
                } else {
                    durations[count / 2]
                };
                let summary = TurnSummary {
                    count,
                    average: total / count as u32,
                    median,
                    longest: durations[count - 1],
                };
                (player, summary)
            })
            .collect()
    }

    /// The wall-clock time spent in each phase, per round.
    pub fn phase_durations(&self) -> BTreeMap<u32, BTreeMap<PhaseGroup, Duration>> {
        let mut rounds = BTreeMap::<u32, BTreeMap<PhaseGroup, Duration>>::new();
        for segment in &self.segments {
            *rounds
                .entry(segment.round)
                .or_default()
                .entry(segment.phase.into())
                .or_default() += segment.duration();
        }
        rounds
    }

    /// The total time that time-tracking was paused.
    pub fn paused_total(&self) -> Duration {
        self.segments
            .iter()
            .filter(|segment| segment.paused)
            .map(TimeSegment::duration)
            .sum()
    }
}
