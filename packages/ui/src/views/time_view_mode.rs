use std::{collections::BTreeSet, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::phase::Phase,
    state::time_stats::{PhaseGroup, TimeSegment, TimeStats},
};

use crate::{
    components::{button::Button, dropdown::Dropdown, spinner::Spinner},
    data::{game_context::GameContext, server_data::ServerDataContext},
};

const TIME_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/time_view_mode.scss");

const ZOOM_LEVELS: [(&str, i64); 5] = [
    ("2 h", 2 * 3600),
    ("1 h", 3600),
    ("30 min", 30 * 60),
    ("10 min", 10 * 60),
    ("2 min", 2 * 60),
];

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

#[component]
pub fn TimeViewMode() -> Element {
    let gc = use_context::<GameContext>();
    let server_data = use_context::<ServerDataContext>();

    use_effect(move || {
        let _ = gc.game_state();
        server_data.request_time_stats();
    });

    rsx! {
        document::Stylesheet { href: TIME_VIEW_MODE_SCSS }

        div { class: "time-view-container",
            match server_data.time_stats() {
                None => rsx! {
                    Spinner {}
                },
                Some(stats) if stats.segments.is_empty() => rsx! {
                    div { class: "card",
                        p { "No time has passed yet" }
                    }
                },
                Some(stats) => rsx! {
                    Timeline { stats: stats.clone() }
                    TimePerRound { stats: stats.clone() }
                    Leaderboard { stats: stats.clone() }
                    TurnStats { stats: stats.clone() }
                    LongestTurns { stats: stats.clone() }
                    PhaseDurations { stats }
                },
            }
        }
    }
}

fn color_class(gc: &GameContext, player: &PlayerId) -> String {
    gc.game_state()
        .players
        .get(player)
        .map(|p| format!("player-color-background-{}", p.color.name()))
        .unwrap_or_default()
}

fn phase_class(phase: PhaseGroup) -> String {
    format!("time-phase-{}", phase.to_string().to_lowercase())
}

const BREAK_WIDTH_PX: f64 = 6.0;

fn local_time(time: DateTime<Utc>) -> String {
    time.with_timezone(&chrono::Local)
        .format("%H:%M")
        .to_string()
}

fn local_day_and_time(time: DateTime<Utc>) -> String {
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
struct TimeScale {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    breaks: Vec<(DateTime<Utc>, DateTime<Utc>)>,
    active_seconds: i64,
}

impl TimeScale {
    fn new(segments: &[&TimeSegment]) -> Self {
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

    fn breaks_width_px(&self) -> f64 {
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

    fn position(&self, time: DateTime<Utc>) -> String {
        self.css(self.offset(time))
    }

    fn width(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> String {
        let (from, to) = (self.offset(from), self.offset(to));
        self.css((to.0 - from.0, to.1 - from.1))
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

fn short_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let (hours, minutes, seconds) = (seconds / 3600, seconds / 60 % 60, seconds % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

fn round_name(round: u32) -> String {
    if round == 0 {
        "Setup".to_string()
    } else {
        format!("Round {round}")
    }
}

fn percent(part: Duration, whole: Duration) -> f64 {
    if whole.is_zero() {
        0.0
    } else {
        part.as_secs_f64() / whole.as_secs_f64() * 100.0
    }
}

#[component]
fn PlayerName(player: PlayerId) -> Element {
    let gc = use_context::<GameContext>();

    rsx! {
        span { class: "time-player-name",
            span { class: "time-swatch {color_class(&gc, &player)}" }
            "{player}"
        }
    }
}

#[derive(Clone, PartialEq)]
struct Bar {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    class: String,
    title: String,
}

fn merge_bars<'a>(
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

#[component]
fn Timeline(stats: Arc<TimeStats>) -> Element {
    let gc = use_context::<GameContext>();
    let players = gc.game_state().table_order.clone();
    let mut zoom: Signal<Option<i64>> = use_signal(|| None);
    let mut round: Signal<Option<u32>> = use_signal(|| None);

    let rounds = stats
        .segments
        .iter()
        .map(|s| s.round)
        .collect::<BTreeSet<_>>();
    let segments = stats
        .segments
        .iter()
        .filter(|segment| round().is_none_or(|r| segment.round == r))
        .collect::<Vec<_>>();

    let scale = TimeScale::new(&segments);
    let total = scale.active_seconds;
    let bar_style = |bar: &Bar| {
        format!(
            "left: {}; width: {}",
            scale.position(bar.start),
            scale.width(bar.start, bar.end)
        )
    };

    let visible = zoom().filter(|window| *window < total).unwrap_or(total);
    let timeline_width = if visible == total {
        "100%".to_string()
    } else {
        format!(
            "calc({}% + {}px)",
            total as f64 / visible as f64 * 100.0,
            scale.breaks_width_px()
        )
    };
    let tick = TICK_INTERVALS
        .into_iter()
        .find(|interval| visible / interval <= 10)
        .unwrap_or(TICK_INTERVALS[TICK_INTERVALS.len() - 1]);
    let mut ticks: Vec<(DateTime<Utc>, String)> = Vec::new();
    for (from, to) in scale.stretches() {
        let mut new_stretch = true;
        let to_offset = scale.offset(to).0;
        let first_tick = (from.timestamp() / tick + 1) * tick;
        for time in (first_tick..to.timestamp())
            .step_by(tick as usize)
            .filter_map(|t| DateTime::from_timestamp(t, 0))
            .filter(|time| {
                let offset = scale.offset(*time).0;
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
    let mut round_starts: Vec<(u32, DateTime<Utc>)> = Vec::new();
    for segment in &segments {
        if round_starts
            .last()
            .is_none_or(|(round, _)| *round != segment.round)
        {
            round_starts.push((segment.round, segment.start));
        }
    }
    let round_markers = round_starts
        .iter()
        .enumerate()
        .map(|(index, (round, time))| {
            let next = round_starts
                .get(index + 1)
                .map_or(scale.end, |(_, next)| *next);
            let (from, to) = (scale.offset(*time), scale.offset(next));
            let width = (to.0 - from.0) + (to.1 - from.1) * visible as f64 / 20.0;
            let fits = (width as i64) * 8 > visible;
            (*round, *time, fits.then(|| round_name(*round)))
        })
        .collect::<Vec<_>>();

    let phase_bars = merge_bars(segments.iter().copied(), |segment| {
        let phase = PhaseGroup::from(segment.phase);
        Some(if segment.paused {
            ("time-paused".to_string(), "Paused".to_string())
        } else {
            (
                phase_class(phase),
                format!("{}, {phase}", round_name(segment.round)),
            )
        })
    });
    let player_bars = players
        .iter()
        .map(|player| {
            let class = format!("time-player-bar {}", color_class(&gc, player));
            let bars = merge_bars(segments.iter().copied(), |segment| {
                (segment.tracked && segment.player.as_ref() == Some(player)).then(|| {
                    (
                        class.clone(),
                        format!(
                            "{player}, {}, {} (turn {})",
                            round_name(segment.round),
                            PhaseGroup::from(segment.phase),
                            segment.turn_number
                        ),
                    )
                })
            });
            (player.clone(), bars)
        })
        .collect::<Vec<_>>();

    rsx! {
        div { class: "card time-card time-wide",
            h2 { "Timeline" }
            div { class: "time-controls",
                div { class: "time-zoom-buttons",
                    Button {
                        disabled: zoom().is_none(),
                        onclick: move |_| zoom.set(None),
                        "Fit"
                    }
                    for (label, window) in ZOOM_LEVELS.into_iter().filter(|(_, window)| *window < total) {
                        Button {
                            key: "{label}",
                            disabled: zoom() == Some(window),
                            onclick: move |_| zoom.set(Some(window)),
                            "{label}"
                        }
                    }
                }
                Dropdown {
                    value: round().map(|r| r.to_string()).unwrap_or_default(),
                    oninput: move |e: FormEvent| round.set(e.value().parse().ok()),
                    option { value: "", "Whole game" }
                    for r in rounds.iter() {
                        option { key: "{r}", value: "{r}", {round_name(*r)} }
                    }
                }
            }
            div { class: "time-legend",
                for phase in PhaseGroup::ALL {
                    span { key: "{phase}",
                        span { class: "time-swatch {phase_class(phase)}" }
                        "{phase}"
                    }
                }
                span {
                    span { class: "time-swatch time-paused" }
                    "Paused"
                }
            }
            div { class: "time-timeline-body",
                div { class: "time-labels",
                    div { class: "time-row time-row-rounds" }
                    div { class: "time-row", "Phase" }
                    for player in players.iter() {
                        div { key: "{player}", class: "time-row",
                            PlayerName { player: player.clone() }
                        }
                    }
                }
                div { class: "time-timeline-scroll",
                    div {
                        class: "time-timeline",
                        style: "width: {timeline_width}",
                        for (from , _) in scale.breaks.iter() {
                            div {
                                key: "break-{from}",
                                class: "time-break time-paused",
                                style: "left: {scale.position(*from)}; width: {BREAK_WIDTH_PX}px",
                            }
                        }
                        div { class: "time-row time-row-rounds time-track",
                            for (round, time, label) in round_markers.iter() {
                                span {
                                    key: "{round}",
                                    class: "time-marker",
                                    style: "left: {scale.position(*time)}",
                                    title: "{round_name(*round)}",
                                    {label.clone().unwrap_or_default()}
                                }
                            }
                        }
                        div { class: "time-row time-track",
                            for (index, bar) in phase_bars.iter().enumerate() {
                                div {
                                    key: "{index}",
                                    class: "time-bar {bar.class}",
                                    style: bar_style(bar),
                                    title: "{bar.title}",
                                }
                            }
                        }
                        for (player, bars) in player_bars.iter() {
                            div { key: "{player}", class: "time-row time-track",
                                for (index, bar) in bars.iter().enumerate() {
                                    div {
                                        key: "{index}",
                                        class: "time-bar {bar.class}",
                                        style: bar_style(bar),
                                        title: "{bar.title}",
                                    }
                                }
                            }
                        }
                        div { class: "time-row time-axis time-track",
                            for (time , label) in ticks.iter() {
                                span {
                                    key: "{time}",
                                    class: "time-marker",
                                    style: "left: {scale.position(*time)}",
                                    "{label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Leaderboard(stats: Arc<TimeStats>) -> Element {
    let gc = use_context::<GameContext>();
    let mut totals = stats.player_totals().into_iter().collect::<Vec<_>>();
    totals.sort_by_key(|(_, total)| std::cmp::Reverse(*total));
    let sum: Duration = totals.iter().map(|(_, total)| *total).sum();
    let longest = totals.first().map(|(_, total)| *total).unwrap_or_default();

    rsx! {
        div { class: "card time-card",
            h2 { "Leaderboard" }
            table { class: "time-table",
                tbody {
                    for (rank, (player, total)) in totals.iter().enumerate() {
                        tr { key: "{player}",
                            td { "{rank + 1}." }
                            td {
                                PlayerName { player: player.clone() }
                            }
                            td { class: "time-bar-cell",
                                div {
                                    class: "time-hbar {color_class(&gc, player)}",
                                    style: "width: {percent(*total, longest)}%",
                                }
                            }
                            td { class: "time-number", "{short_duration(*total)}" }
                            td { class: "time-number", {format!("{:.0}%", percent(*total, sum))} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TimePerRound(stats: Arc<TimeStats>) -> Element {
    let gc = use_context::<GameContext>();
    let rounds = stats.per_round();
    let longest = rounds
        .values()
        .map(|players| players.values().sum::<Duration>())
        .max()
        .unwrap_or_default();
    let players = gc.game_state().table_order.clone();

    rsx! {
        div { class: "card time-card time-wide",
            h2 { "Time per round" }
            table { class: "time-table",
                tbody {
                    for (round, times) in rounds.iter() {
                        tr { key: "{round}",
                            td { {round_name(*round)} }
                            td { class: "time-bar-cell",
                                div { class: "time-stacked-bar",
                                    for player in players.iter().filter(|p| times.contains_key(*p)) {
                                        div {
                                            key: "{player}",
                                            class: "time-hbar {color_class(&gc, player)}",
                                            style: "width: {percent(times[player], longest)}%",
                                            title: "{player}: {short_duration(times[player])}",
                                        }
                                    }
                                }
                            }
                            td { class: "time-number", "{short_duration(times.values().sum())}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TurnStats(stats: Arc<TimeStats>) -> Element {
    let gc = use_context::<GameContext>();
    let summaries = stats.turn_summaries();
    let players = gc.game_state().table_order.clone();

    rsx! {
        div { class: "card time-card",
            h2 { "Turns" }
            table { class: "time-table",
                thead {
                    tr {
                        th { "Player" }
                        th { class: "time-number", "Turns" }
                        th { class: "time-number", "Avg" }
                        th { class: "time-number", "Median" }
                        th { class: "time-number", "Max" }
                    }
                }
                tbody {
                    for (player, summary) in players.iter().filter_map(|p| summaries.get(p).map(|s| (p, s))) {
                        tr { key: "{player}",
                            td {
                                PlayerName { player: player.clone() }
                            }
                            td { class: "time-number", "{summary.count}" }
                            td { class: "time-number", "{short_duration(summary.average)}" }
                            td { class: "time-number", "{short_duration(summary.median)}" }
                            td { class: "time-number", "{short_duration(summary.longest)}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LongestTurns(stats: Arc<TimeStats>) -> Element {
    let turns = stats.longest_turns(5);

    rsx! {
        div { class: "card time-card",
            h2 { "Longest turns" }
            table { class: "time-table",
                thead {
                    tr {
                        th { "Player" }
                        th { "When" }
                        th { class: "time-number", "Time" }
                    }
                }
                tbody {
                    for (index, turn) in turns.iter().enumerate() {
                        tr { key: "{index}",
                            td {
                                PlayerName { player: turn.player.clone() }
                            }
                            td { "R{turn.round} {turn.phase}, {local_day_and_time(turn.start)}" }
                            td { class: "time-number", "{short_duration(turn.duration)}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PhaseDurations(stats: Arc<TimeStats>) -> Element {
    let mut rounds = stats.phase_durations();
    let setup: Duration = rounds.remove(&0).unwrap_or_default().values().sum();
    let paused = stats.paused_total();
    let phases = PhaseGroup::ALL
        .into_iter()
        .filter(|phase| rounds.values().any(|r| r.contains_key(phase)))
        .collect::<Vec<_>>();

    rsx! {
        div { class: "card time-card",
            h2 { "Phase durations" }
            table { class: "time-table",
                thead {
                    tr {
                        th { "Round" }
                        for phase in phases.iter() {
                            th { key: "{phase}", class: "time-number", "{phase}" }
                        }
                        th { class: "time-number", "Total" }
                    }
                }
                tbody {
                    for (round, durations) in rounds.iter() {
                        tr { key: "{round}",
                            td { "{round}" }
                            for phase in phases.iter() {
                                td { key: "{phase}", class: "time-number",
                                    {durations.get(phase).map(|d| short_duration(*d)).unwrap_or_default()}
                                }
                            }
                            td { class: "time-number", "{short_duration(durations.values().sum())}" }
                        }
                    }
                }
            }
            p { class: "margin-top",
                "Setup: {short_duration(setup)}, paused in total: {short_duration(paused)}"
            }
        }
    }
}
