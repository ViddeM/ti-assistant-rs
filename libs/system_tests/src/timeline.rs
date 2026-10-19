//! A compact, line-per-event description of how a game progressed.
//!
//! Each line shows the event, the round, phase and current player after the event, plus
//! whatever changed among the speaker, turn order, custodians and points. A rule change that
//! affects the flow of the game shows up as a readable diff of this text.
//!
//! ```text
//!   8 StartGame                            r1 Strategy          Alice   speaker=Alice order=[Alice Bob Carol]
//!  15 CompleteStrategyPhase                r1 Action            Bob     order=[Bob Alice Carol]
//!  22 TacticalActionTakePlanet(Bob)        r1 TacticalAction    Bob     custodians=Bob Bob=1
//! ```

use std::fmt::Write;

use chrono::{DateTime, Utc};
use serde_json::Value;
use ti_helper_game_data::{actions::event::Event, state::game_state::GameState};

use crate::check::replay;

/// Render the timeline for the provided history.
pub fn render(history: &[(Event, DateTime<Utc>)]) -> String {
    let states = replay(history).unwrap_or_else(|e| panic!("history doesn't replay: {e:#}"));

    let mut out = String::new();
    let mut previous = GameState::default();
    for (index, ((event, _), state)) in history.iter().zip(&states).enumerate() {
        let name: &'static str = event.into();
        let event = match actor(event) {
            Some(actor) => format!("{name}({actor})"),
            None => name.to_string(),
        };
        let current = state.current_player.as_deref().unwrap_or("-");
        let mut line = format!(
            "{index:>4} {event:<40} r{} {:<18} {current:<10}",
            state.round,
            format!("{:?}", state.phase),
        );

        for change in changes(&previous, state) {
            write!(line, " {change}").unwrap();
        }
        writeln!(out, "{}", line.trim_end()).unwrap();
        previous = state.clone();
    }
    out
}

/// The player performing the event, if any.
fn actor(event: &Event) -> Option<String> {
    let Ok(Value::Object(outer)) = serde_json::to_value(event) else {
        return None;
    };
    let (_, Value::Object(fields)) = outer.into_iter().next()? else {
        return None;
    };
    let actor = fields.get("player").or_else(|| fields.get("giver"))?;
    match actor {
        Value::String(name) => Some(name.clone()),
        Value::Object(player) => player.get("name")?.as_str().map(str::to_string),
        _ => None,
    }
}

fn changes(before: &GameState, after: &GameState) -> Vec<String> {
    let mut changes = vec![];
    if before.speaker != after.speaker {
        changes.push(format!(
            "speaker={}",
            after.speaker.as_deref().unwrap_or("-")
        ));
    }
    if before.turn_order != after.turn_order && !after.turn_order.is_empty() {
        let order: Vec<&str> = after.turn_order.iter().map(|p| &**p).collect();
        changes.push(format!("order=[{}]", order.join(" ")));
    }
    if before.score.custodians != after.score.custodians {
        let custodians = after.score.custodians.as_deref().unwrap_or("-");
        changes.push(format!("custodians={custodians}"));
    }
    for player in &after.table_order {
        let points = after.score.player_points.get(player).copied().unwrap_or(0);
        let previous = before.score.player_points.get(player).copied().unwrap_or(0);
        if points != previous {
            changes.push(format!("{player}={points}"));
        }
    }
    changes
}
