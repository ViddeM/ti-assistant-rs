//! Shared starting points for scenarios.

use ti_helper_system_tests::{TestGameBuilder, prelude::*};

/// Alice (Sol), Bob (Hacan) and Carol (Xxcha), Alice is speaker.
pub fn three_players() -> TestGameBuilder {
    TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::XxchaKingdom)
}

/// Strategy picks for [three_players]. Politics is the only card with a primary action.
///
/// Initiative order becomes Alice (1), Bob (2), Carol (3).
pub const SIMPLE_PICKS: &[(&str, StrategyCard)] = &[
    ("Alice", StrategyCard::Leadership),
    ("Bob", StrategyCard::Diplomacy),
    ("Carol", StrategyCard::Politics),
    ("Alice", StrategyCard::Construction),
    ("Bob", StrategyCard::Trade),
    ("Carol", StrategyCard::Warfare),
];

/// A [three_players] game in the action phase of round 1, using [SIMPLE_PICKS].
pub fn action_phase(builder: TestGameBuilder) -> TestGame {
    let mut g = builder.start();
    g.strategy_phase(SIMPLE_PICKS);
    g
}

/// A [three_players] game in the agenda phase of round 1: Alice holds the custodians token
/// and nobody has scored anything.
pub fn agenda_phase(builder: TestGameBuilder) -> TestGame {
    let mut g = action_phase(builder);
    g.apply(Event::SetCustodians {
        player: Some(p("Alice")),
    })
    .play_out_action_phase()
    .status_phase()
    .reveal(PublicObjective::ExpandBorders)
    .complete();
    assert_eq!(g.phase(), Phase::Agenda);
    g
}
