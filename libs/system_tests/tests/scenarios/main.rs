//! All scenarios, in one test binary so that it is linked once.
//!
//! Each module registers its scenarios with `scenarios!`, which runs the standard checks and
//! snapshots on them (see `ti_helper_system_tests::check_scenario`). Plain `#[test]`s in the
//! modules cover rules that are easier to assert directly.

mod action_phase;
mod agenda;
mod codex;
mod fixtures;
mod full_games;
mod score;
#[cfg(feature = "seed-db")]
mod seed_db;
mod setup;
mod status;
mod strategy;
mod thunders_edge;
mod time;
mod undo;

use ti_helper_system_tests::Scenario;

/// Every registered scenario, grouped by module.
pub const ALL_SCENARIOS: &[&[Scenario]] = &[
    setup::SCENARIOS,
    strategy::SCENARIOS,
    action_phase::SCENARIOS,
    status::SCENARIOS,
    agenda::SCENARIOS,
    score::SCENARIOS,
    thunders_edge::SCENARIOS,
    codex::SCENARIOS,
    time::SCENARIOS,
    full_games::SCENARIOS,
];

#[test]
fn scenario_names_are_unique() {
    let mut names: Vec<&str> = ALL_SCENARIOS
        .iter()
        .flat_map(|scenarios| scenarios.iter().map(|s| s.name))
        .collect();
    names.sort();
    let total = names.len();
    names.dedup();
    assert_eq!(
        total,
        names.len(),
        "scenario names are used for snapshots and must be unique"
    );
}
