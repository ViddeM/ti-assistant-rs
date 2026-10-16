//! The checks every registered scenario goes through, see [check_scenario].

use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use ti_helper_game_data::{actions::event::Event, state::game_state::GameState};
use ti_helper_game_logic::gameplay::game_event_handler::update_game_state;

use crate::game::TestGame;

/// A named scenario, registered with [crate::scenarios!].
pub struct Scenario {
    /// The name, also used for the snapshot files.
    pub name: &'static str,
    /// Builds the game.
    pub build: fn() -> TestGame,
}

/// Replay `history` from an empty game, returning the state after each event.
pub fn replay(history: &[(Event, DateTime<Utc>)]) -> anyhow::Result<Vec<GameState>> {
    let mut state = GameState::default();
    let mut states = Vec::with_capacity(history.len());
    for (index, (event, timestamp)) in history.iter().enumerate() {
        update_game_state(&mut state, event.clone(), *timestamp)
            .map_err(|e| e.context(format!("event #{index} {event:?}")))?;
        states.push(state.clone());
    }
    Ok(states)
}

/// Run all checks for a scenario:
///
/// 1. **Determinism**: building the scenario twice gives the same state and timeline. Each
///    build gets differently seeded HashMaps, so this catches results that depend on
///    iteration order.
/// 2. **Replay**: replaying the recorded history from scratch gives the same state.
/// 3. **Persistence**: the history survives a JSON round trip (like when stored in the DB)
///    and still replays to the same state.
/// 4. **Undo**: undoing the last few events gives the same states as before those events.
/// 5. **Snapshots**: the final state and the timeline match `tests/snapshots/<name>@*.snap`.
#[track_caller]
pub fn check_scenario(name: &str, build: fn() -> TestGame) {
    let game = build();
    let state = game.state();
    let history = game.history();

    let again = build();
    assert_same(
        state,
        again.state(),
        "building the scenario twice gave different states",
    );
    let timeline = game.timeline();
    assert_eq!(
        timeline,
        again.timeline(),
        "building the scenario twice gave different timelines"
    );

    let states = replay(history).expect("replaying the history failed");
    assert_same(
        state,
        states.last().unwrap_or(&GameState::default()),
        "replaying the history gave a different state",
    );

    let json = serde_json::to_value(history).expect("failed to serialize the history");
    let parsed: Vec<(Event, DateTime<Utc>)> =
        serde_json::from_value(json).expect("failed to deserialize the history");
    let parsed_states = replay(&parsed).expect("replaying the deserialized history failed");
    assert_same(
        state,
        parsed_states.last().unwrap_or(&GameState::default()),
        "replaying the JSON round-tripped history gave a different state",
    );

    let mut undo_game = TestGame::from_history(history.to_vec(), state.clone());
    for undone in 1..=history.len().min(3) {
        undo_game.undo();
        let remaining = history.len() - undone;
        let expected = match remaining {
            0 => GameState::default(),
            n => states[n - 1].clone(),
        };
        assert_same(
            undo_game.current.as_ref(),
            &expected,
            &format!("undoing {undone} event(s) gave a different state than before them"),
        );
    }

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots"));
    settings.set_prepend_module_to_snapshot(false);
    settings.set_omit_expression(true);
    settings.set_sort_maps(true);
    settings.set_snapshot_suffix("state");
    settings.bind(|| insta::assert_yaml_snapshot!(name, state));
    settings.set_snapshot_suffix("timeline");
    settings.bind(|| insta::assert_snapshot!(name, timeline));
}

/// Assert that two values are equal, both by `PartialEq` and by their serialized form (which
/// also catches differences in ordering). On failure, points out the first differing path.
#[track_caller]
pub fn assert_same<T: PartialEq + Serialize>(left: &T, right: &T, message: &str) {
    let left_json = serde_json::to_value(left).expect("failed to serialize");
    let right_json = serde_json::to_value(right).expect("failed to serialize");
    if left == right && left_json == right_json {
        return;
    }
    match first_difference(&left_json, &right_json, String::new()) {
        Some((path, l, r)) => panic!("{message}\n  at: {path}\n  left:  {l}\n  right: {r}"),
        None => panic!("{message}\n  (the serialized forms are equal, but `PartialEq` isn't)"),
    }
}

fn first_difference(left: &Value, right: &Value, path: String) -> Option<(String, Value, Value)> {
    match (left, right) {
        (Value::Object(l), Value::Object(r)) => {
            let mut keys: Vec<&String> = l.keys().chain(r.keys()).collect();
            keys.sort();
            keys.dedup();
            keys.into_iter().find_map(|key| {
                let l = l.get(key).unwrap_or(&Value::Null);
                let r = r.get(key).unwrap_or(&Value::Null);
                first_difference(l, r, format!("{path}.{key}"))
            })
        }
        (Value::Array(l), Value::Array(r)) if l.len() == r.len() => l
            .iter()
            .zip(r)
            .enumerate()
            .find_map(|(i, (l, r))| first_difference(l, r, format!("{path}[{i}]"))),
        (l, r) if l == r => None,
        (l, r) => Some((path, l.clone(), r.clone())),
    }
}
