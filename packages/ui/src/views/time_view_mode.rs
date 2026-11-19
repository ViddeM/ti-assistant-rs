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
    format::format_std_duration,
    views::history_view_mode::HistoryPlayer,
};

const TIME_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/time_view_mode.scss");

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
                    Leaderboard { stats: stats.clone() }
                    TimePerRound { stats: stats.clone() }
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

fn percent(part: Duration, whole: Duration) -> f64 {
    if whole.is_zero() {
        0.0
    } else {
        part.as_secs_f64() / whole.as_secs_f64() * 100.0
    }
}

const ZOOM_LEVELS: [(&str, i64); 5] = [
    ("2 h", 2 * 3600),
    ("1 h", 3600),
    ("30 min", 30 * 60),
    ("10 min", 10 * 60),
    ("2 min", 2 * 60),
];

const TICK_INTERVALS: [i64; 9] = [60, 2 * 60, 5 * 60, 10 * 60, 15 * 60, 30 * 60, 3600, 2 * 3600, 4 * 3600];

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
    let in_range = |segment: &&TimeSegment| round().is_none_or(|r| segment.round == r);
    let segments = stats.segments.iter().filter(in_range).collect::<Vec<_>>();

    let start = segments.first().map(|s| s.start).unwrap_or_default();
    let end = segments.last().map(|s| s.end).unwrap_or_default();
    let total = (end - start).num_seconds().max(1);
    let position = |time: DateTime<Utc>| ((time - start).num_seconds() as f64 / total as f64 * 100.0).clamp(0.0, 100.0);
    let bar_style = |segment: &TimeSegment| {
        format!(
            "left: {}%; width: {}%",
            position(segment.start),
            position(segment.end) - position(segment.start)
        )
    };

    let visible = zoom().filter(|window| *window < total).unwrap_or(total);
    let width = total as f64 / visible as f64 * 100.0;
    let tick = TICK_INTERVALS
        .into_iter()
        .find(|interval| visible / interval <= 8)
        .unwrap_or(TICK_INTERVALS[TICK_INTERVALS.len() - 1]);
    let first_tick = (start.timestamp() / tick + 1) * tick;
    let ticks = (first_tick..end.timestamp())
        .step_by(tick as usize)
        .filter_map(|t| DateTime::from_timestamp(t, 0))
        .collect::<Vec<_>>();

    let mut round_starts = Vec::new();
    for segment in &segments {
        if PhaseGroup::from(segment.phase) == PhaseGroup::Strategy
            && round_starts.last().is_none_or(|(round, _)| *round != segment.round)
        {
            round_starts.push((segment.round, segment.start));
        }
    }

    rsx! {
        div { class: "card time-card",
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
                        option { key: "{r}", value: "{r}",
                            if *r == 0 {
                                "Setup"
                            } else {
                                "Round {r}"
                            }
                        }
                    }
                }
            }
            div { class: "time-legend",
                for phase in PhaseGroup::ALL {
                    span { key: "{phase}",
                        span { class: "time-legend-swatch {phase_class(phase)}" }
                        "{phase}"
                    }
                }
                span {
                    span { class: "time-legend-swatch time-paused" }
                    "Paused"
                }
            }
            div { class: "time-timeline-scroll",
                div { class: "time-timeline", style: "width: {width}%",
                    div { class: "time-lane",
                        span { class: "time-lane-label", "Phase" }
                        div { class: "time-lane-track",
                            for (index, segment) in segments.iter().enumerate() {
                                div {
                                    key: "{index}",
                                    class: if segment.paused { "time-bar time-paused" } else { "time-bar {phase_class(segment.phase.into())}" },
                                    style: bar_style(segment),
                                    title: "Round {segment.round}, {segment.phase}: {format_std_duration(segment.duration())}",
                                }
                            }
                            for (round, time) in round_starts.iter() {
                                div {
                                    key: "round-{round}",
                                    class: "time-round-marker",
                                    style: "left: {position(*time)}%",
                                    "R{round}"
                                }
                            }
                        }
                    }
                    for player in players.iter() {
                        div { key: "{player}", class: "time-lane",
                            span { class: "time-lane-label",
                                HistoryPlayer { player: player.clone() }
                            }
                            div { class: "time-lane-track",
                                for (index, segment) in segments
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, s)| s.tracked && s.player.as_ref() == Some(player))
                                {
                                    div {
                                        key: "{index}",
                                        class: "time-bar time-player-bar {color_class(&gc, player)}",
                                        style: bar_style(segment),
                                        title: "{player}, round {segment.round}, {segment.phase}: {format_std_duration(segment.duration())}",
                                    }
                                }
                            }
                        }
                    }
                    div { class: "time-lane time-axis",
                        span { class: "time-lane-label" }
                        div { class: "time-lane-track",
                            for time in ticks.iter() {
                                span {
                                    key: "{time}",
                                    class: "time-tick",
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
                                HistoryPlayer { player: player.clone() }
                            }
                            td { class: "time-bar-cell",
                                div {
                                    class: "time-hbar {color_class(&gc, player)}",
                                    style: "width: {percent(*total, longest)}%",
                                }
                            }
                            td { "{format_std_duration(*total)}" }
                            td { {format!("{:.0}%", percent(*total, sum))} }
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
        div { class: "card time-card",
            h2 { "Time per round" }
            table { class: "time-table",
                tbody {
                    for (round, times) in rounds.iter() {
                        tr { key: "{round}",
                            td { "Round {round}" }
                            td { class: "time-bar-cell",
                                div { class: "time-stacked-bar",
                                    for player in players.iter().filter(|p| times.contains_key(*p)) {
                                        div {
                                            key: "{player}",
                                            class: "time-hbar {color_class(&gc, player)}",
                                            style: "width: {percent(times[player], longest)}%",
                                            title: "{player}: {format_std_duration(times[player])}",
                                        }
                                    }
                                }
                            }
                            td { "{format_std_duration(times.values().sum())}" }
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
                        th { "Turns" }
                        th { "Average" }
                        th { "Median" }
                        th { "Longest" }
                    }
                }
                tbody {
                    for (player, summary) in players.iter().filter_map(|p| summaries.get(p).map(|s| (p, s))) {
                        tr { key: "{player}",
                            td {
                                HistoryPlayer { player: player.clone() }
                            }
                            td { "{summary.count}" }
                            td { "{format_std_duration(summary.average)}" }
                            td { "{format_std_duration(summary.median)}" }
                            td { "{format_std_duration(summary.longest)}" }
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
                        th { "Round" }
                        th { "Phase" }
                        th { "Started" }
                        th { "Time" }
                    }
                }
                tbody {
                    for (index, turn) in turns.iter().enumerate() {
                        tr { key: "{index}",
                            td {
                                HistoryPlayer { player: turn.player.clone() }
                            }
                            td { "{turn.round}" }
                            td { "{turn.phase}" }
                            td { "{local_time(turn.start)}" }
                            td { "{format_std_duration(turn.duration)}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PhaseDurations(stats: Arc<TimeStats>) -> Element {
    let rounds = stats.phase_durations();
    let paused = stats.paused_total();

    rsx! {
        div { class: "card time-card",
            h2 { "Phase durations" }
            table { class: "time-table",
                thead {
                    tr {
                        th { "Round" }
                        for phase in PhaseGroup::ALL {
                            th { key: "{phase}", "{phase}" }
                        }
                        th { "Total" }
                    }
                }
                tbody {
                    for (round, phases) in rounds.iter() {
                        tr { key: "{round}",
                            td {
                                if *round == 0 {
                                    "Setup"
                                } else {
                                    "{round}"
                                }
                            }
                            for phase in PhaseGroup::ALL {
                                td { key: "{phase}",
                                    {phases.get(&phase).map(|d| format_std_duration(*d)).unwrap_or_default()}
                                }
                            }
                            td { "{format_std_duration(phases.values().sum())}" }
                        }
                    }
                }
            }
            p { class: "margin-top", "Paused in total: {format_std_duration(paused)}" }
        }
    }
}
