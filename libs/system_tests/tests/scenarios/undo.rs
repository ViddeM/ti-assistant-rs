//! Undo. Every scenario also checks that undoing its last events works, these cover specific
//! situations.

use ti_helper_game_data::state::game_state::GameState;
use ti_helper_system_tests::prelude::*;

use crate::fixtures::{action_phase, three_players};

#[test]
fn undo_inside_a_tactical_action() {
    let mut g = action_phase(three_players());
    let before = g.state().clone();

    g.turn("Alice").tactical().take(Planet::Centauri);
    g.undo();
    assert_eq!(g.phase(), Phase::TacticalAction);
    assert!(!g.player("Alice").planets.contains_key(&Planet::Centauri));

    g.undo();
    assert_eq!(*g.state(), before);

    // A different action can be taken instead.
    g.turn("Alice").play_card(StrategyCard::Leadership);
    assert_eq!(g.current_player(), p("Bob"));
}

#[test]
fn undo_start_game_returns_to_setup() {
    let mut g = three_players().start();
    g.undo();
    assert_eq!(g.phase(), Phase::Setup);
    assert_eq!(g.round(), 0);

    g.start_game();
    assert_eq!(g.phase(), Phase::Strategy);
}

#[test]
fn undo_everything_gives_an_empty_game() {
    let mut g = three_players().create();
    for _ in 0..g.history().len() {
        g.undo();
    }
    assert_eq!(*g.state(), GameState::default());

    // Undoing an empty game does nothing.
    g.undo();
    assert_eq!(*g.state(), GameState::default());
}
