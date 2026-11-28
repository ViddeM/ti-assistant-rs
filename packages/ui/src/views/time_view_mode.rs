use std::{collections::BTreeSet, sync::Arc, time::Duration};

use dioxus::prelude::*;
use ti_helper_game_data::state::time_stats::{PhaseGroup, TimeStats};

use crate::{
    components::{button::Button, dropdown::Dropdown, spinner::Spinner},
    data::{game_context::GameContext, server_data::ServerDataContext},
    views::time_scale::{
        BREAK_WIDTH_PX, Bar, PlayerName, TimeScale, color_class, local_day_and_time, merge_bars,
        phase_class, round_name, short_duration,
    },
};

const TIME_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/time_view_mode.scss");

const ZOOM_LEVELS: [(&str, i64); 5] = [
    ("2 h", 2 * 3600),
    ("1 h", 3600),
    ("30 min", 30 * 60),
    ("10 min", 10 * 60),
    ("2 min", 2 * 60),
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

fn percent(part: Duration, whole: Duration) -> f64 {
    if whole.is_zero() {
        0.0
    } else {
        part.as_secs_f64() / whole.as_secs_f64() * 100.0
    }
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
    let ticks = scale.ticks(visible);
    let round_markers = scale.round_markers(&segments, visible);

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
