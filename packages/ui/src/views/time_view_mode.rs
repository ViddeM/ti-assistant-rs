use std::{collections::BTreeSet, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
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

fn local_time(time: DateTime<Utc>) -> String {
    time.with_timezone(&chrono::Local).format("%H:%M").to_string()
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

    let start = segments.first().map(|s| s.start).unwrap_or_default();
    let end = segments.last().map(|s| s.end).unwrap_or_default();
    let total = (end - start).num_seconds().max(1);
    let position = |time: DateTime<Utc>| {
        ((time - start).num_seconds() as f64 / total as f64 * 100.0).clamp(0.0, 100.0)
    };
    let bar_style = |bar: &Bar| {
        format!(
            "left: {}%; width: {}%",
            position(bar.start),
            position(bar.end) - position(bar.start)
        )
    };

    let visible = zoom().filter(|window| *window < total).unwrap_or(total);
    let width = total as f64 / visible as f64 * 100.0;
    let tick = TICK_INTERVALS
        .into_iter()
        .find(|interval| visible / interval <= 6)
        .unwrap_or(TICK_INTERVALS[TICK_INTERVALS.len() - 1]);
    let first_tick = (start.timestamp() / tick + 1) * tick;
    let ticks = (first_tick..end.timestamp())
        .step_by(tick as usize)
        .filter_map(|t| DateTime::from_timestamp(t, 0))
        .filter(|time| (end - *time).num_seconds() * 16 > visible)
        .collect::<Vec<_>>();

    let mut round_starts: Vec<(u32, DateTime<Utc>)> = Vec::new();
    for segment in &segments {
        if round_starts.last().is_none_or(|(round, _)| *round != segment.round) {
            round_starts.push((segment.round, segment.start));
        }
    }
    let round_markers = round_starts
        .iter()
        .enumerate()
        .map(|(index, (round, time))| {
            let next = round_starts.get(index + 1).map_or(end, |(_, next)| *next);
            let fits = (next - *time).num_seconds() * 8 > visible;
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
                    div { class: "time-timeline", style: "width: {width}%",
                        div { class: "time-row time-row-rounds time-track",
                            for (round, time, label) in round_markers.iter() {
                                span {
                                    key: "{round}",
                                    class: "time-marker",
                                    style: "left: {position(*time)}%",
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
                            for time in ticks.iter() {
                                span {
                                    key: "{time}",
                                    class: "time-marker",
                                    style: "left: {position(*time)}%",
                                    "{local_time(*time)}"
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
                            td { "R{turn.round} {turn.phase}, {local_time(turn.start)}" }
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
