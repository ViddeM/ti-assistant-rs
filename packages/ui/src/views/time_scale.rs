use std::time::Duration;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::phase::Phase,
    state::time_stats::{PhaseGroup, TimeSegment},
};

use crate::data::game_context::GameContext;

const TICK_INTERVALS: [i64; 9] = [
    60,
    2 * 60,
    5 * 60,
    10 * 60,
    15 * 60,
    30 * 60,
    3600,
    2 * 3600,
    4 * 3600,
];

pub(crate) fn color_class(gc: &GameContext, player: &PlayerId) -> String {
    gc.game_state()
        .players
        .get(player)
        .map(|p| format!("player-color-background-{}", p.color.name()))
        .unwrap_or_default()
}

pub(crate) fn phase_class(phase: PhaseGroup) -> String {
    format!("time-phase-{}", phase.to_string().to_lowercase())
}

pub(crate) const BREAK_WIDTH_PX: f64 = 6.0;

fn local_time(time: DateTime<Utc>) -> String {
    time.with_timezone(&chrono::Local)
        .format("%H:%M")
        .to_string()
}

pub(crate) fn local_day_and_time(time: DateTime<Utc>) -> String {
    time.with_timezone(&chrono::Local)
        .format("%a %H:%M")
        .to_string()
}

fn local_date(time: DateTime<Utc>) -> chrono::NaiveDate {
    time.with_timezone(&chrono::Local).date_naive()
}

fn is_break(segment: &TimeSegment) -> bool {
    segment.paused || segment.phase == Phase::Creation
}

/// Maps times onto the timeline, where every break (a pause, or the time before the game was set
/// up) is collapsed into a fixed-width marker and the rest is proportional to the time played.
#[derive(Clone, PartialEq)]
pub(crate) struct TimeScale {
    start: DateTime<Utc>,
    pub(crate) end: DateTime<Utc>,
    pub(crate) breaks: Vec<(DateTime<Utc>, DateTime<Utc>)>,
    pub(crate) active_seconds: i64,
}

impl TimeScale {
    pub(crate) fn new(segments: &[&TimeSegment]) -> Self {
        let start = segments.first().map(|s| s.start).unwrap_or_default();
        let end = segments.last().map(|s| s.end).unwrap_or_default();

        let mut runs: Vec<(DateTime<Utc>, DateTime<Utc>)> = Vec::new();
        let mut previous_was_break = false;
        for segment in segments {
            let breaking = is_break(segment);
            if breaking {
                match runs.last_mut() {
                    Some(run) if previous_was_break && run.1 == segment.start => {
                        run.1 = segment.end;
                    }
                    _ => runs.push((segment.start, segment.end)),
                }
            }
            previous_was_break = breaking;
        }
        let breaks = runs;

        let break_seconds: i64 = breaks
            .iter()
            .map(|(from, to)| (*to - *from).num_seconds())
            .sum();
        let active_seconds = ((end - start).num_seconds() - break_seconds).max(1);
        Self {
            start,
            end,
            breaks,
            active_seconds,
        }
    }

    pub(crate) fn breaks_width_px(&self) -> f64 {
        self.breaks.len() as f64 * BREAK_WIDTH_PX
    }

    /// The played seconds before `time`, and the number of break widths before it.
    fn offset(&self, time: DateTime<Utc>) -> (f64, f64) {
        let seconds =
            |from: DateTime<Utc>, to: DateTime<Utc>| (to - from).num_milliseconds() as f64 / 1000.0;
        let time = time.clamp(self.start, self.end);
        let mut active = seconds(self.start, time);
        let mut breaks = 0.0;
        for (from, to) in &self.breaks {
            if time <= *from {
                break;
            }
            if time >= *to {
                active -= seconds(*from, *to);
                breaks += 1.0;
            } else {
                active -= seconds(*from, time);
                breaks += seconds(*from, time) / seconds(*from, *to);
            }
        }
        (active, breaks)
    }

    fn css(&self, (active, breaks): (f64, f64)) -> String {
        format!(
            "calc((100% - {}px) * {} + {}px)",
            self.breaks_width_px(),
            active / self.active_seconds as f64,
            breaks * BREAK_WIDTH_PX
        )
    }

    pub(crate) fn position(&self, time: DateTime<Utc>) -> String {
        self.css(self.offset(time))
    }

    pub(crate) fn width(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> String {
        let (from, to) = (self.offset(from), self.offset(to));
        self.css((to.0 - from.0, to.1 - from.1))
    }

    /// The axis ticks for a window of `visible` seconds, labelled with the day at the start of
    /// every stretch and day.
    pub(crate) fn ticks(&self, visible: i64) -> Vec<(DateTime<Utc>, String)> {
        let tick = TICK_INTERVALS
            .into_iter()
            .find(|interval| visible / interval <= 10)
            .unwrap_or(TICK_INTERVALS[TICK_INTERVALS.len() - 1]);
        let mut ticks: Vec<(DateTime<Utc>, String)> = Vec::new();
        for (from, to) in self.stretches() {
            let mut new_stretch = true;
            let to_offset = self.offset(to).0;
            let first_tick = (from.timestamp() / tick + 1) * tick;
            for time in (first_tick..to.timestamp())
                .step_by(tick as usize)
                .filter_map(|t| DateTime::from_timestamp(t, 0))
                .filter(|time| {
                    let offset = self.offset(*time).0;
                    ((to_offset - offset) as i64) * 14 > visible
                })
            {
                let new_day = ticks
                    .last()
                    .is_none_or(|(previous, _)| local_date(*previous) != local_date(time));
                let label = if new_stretch || new_day {
                    local_day_and_time(time)
                } else {
                    local_time(time)
                };
                new_stretch = false;
                ticks.push((time, label));
            }
        }
        ticks
    }

    /// The start of every round in `segments`, with its name when there is room for it in a
    /// window of `visible` seconds.
    pub(crate) fn round_markers(
        &self,
        segments: &[&TimeSegment],
        visible: i64,
    ) -> Vec<(u32, DateTime<Utc>, Option<String>)> {
        let mut round_starts: Vec<(u32, DateTime<Utc>)> = Vec::new();
        for segment in segments {
            if round_starts
                .last()
                .is_none_or(|(round, _)| *round != segment.round)
            {
                round_starts.push((segment.round, segment.start));
            }
        }
        round_starts
            .iter()
            .enumerate()
            .map(|(index, (round, time))| {
                let next = round_starts
                    .get(index + 1)
                    .map_or(self.end, |(_, next)| *next);
                let (from, to) = (self.offset(*time), self.offset(next));
                let width = (to.0 - from.0) + (to.1 - from.1) * visible as f64 / 20.0;
                let fits = (width as i64) * 8 > visible;
                (*round, *time, fits.then(|| round_name(*round)))
            })
            .collect()
    }

    /// The stretches of play between the breaks.
    fn stretches(&self) -> Vec<(DateTime<Utc>, DateTime<Utc>)> {
        let mut stretches = Vec::new();
        let mut from = self.start;
        for (break_start, break_end) in &self.breaks {
            if *break_start > from {
                stretches.push((from, *break_start));
            }
            from = *break_end;
        }
        if self.end > from {
            stretches.push((from, self.end));
        }
        stretches
    }
}

pub(crate) fn short_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

pub(crate) fn round_name(round: u32) -> String {
    if round == 0 {
        "Setup".to_string()
    } else {
        format!("Round {round}")
    }
}

#[component]
pub(crate) fn PlayerName(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    rsx! {
        span { class: "time-player-name",
            span { class: "time-swatch {color_class(&gc, &player)}" }
            "{player}"
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct Bar {
    pub(crate) start: DateTime<Utc>,
    pub(crate) end: DateTime<Utc>,
    pub(crate) class: String,
    pub(crate) title: String,
}

pub(crate) fn merge_bars<'a>(
    segments: impl Iterator<Item = &'a TimeSegment>,
    key: impl Fn(&TimeSegment) -> Option<(String, String)>,
) -> Vec<Bar> {
    let mut bars: Vec<(Bar, String)> = Vec::new();
    for segment in segments {
        let Some((class, title)) = key(segment) else {
            continue;
        };
        match bars.last_mut() {
            Some((bar, last_title)) if bar.end == segment.start && *last_title == title => {
                bar.end = segment.end;
            }
            _ => bars.push((
                Bar {
                    start: segment.start,
                    end: segment.end,
                    class,
                    title: String::new(),
                },
                title,
            )),
        }
    }
    bars.into_iter()
        .map(|(bar, title)| {
            let duration = (bar.end - bar.start).to_std().unwrap_or_default();
            Bar {
                title: format!("{title}: {}", short_duration(duration)),
                ..bar
            }
        })
        .collect()
}
