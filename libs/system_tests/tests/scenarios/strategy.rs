//! The strategy phase and the resulting initiative order.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::three_players;

scenarios![
    strategy_three_players_pick_two_each,
    strategy_naalu_telepathy,
    strategy_gift_of_prescience,
];

/// With three players everyone picks two cards, picking in speaker order twice.
pub fn strategy_three_players_pick_two_each() -> TestGame {
    let mut g = three_players().start();
    g.pick_strategy_card("Alice", StrategyCard::Warfare)
        .expect_rejected(
            Event::TakeStrategyCard {
                player: p("Bob"),
                card: StrategyCard::Warfare,
            },
            "can't be picked twice",
        )
        .pick_strategy_card("Bob", StrategyCard::Leadership)
        .pick_strategy_card("Carol", StrategyCard::Imperial)
        .expect_rejected(
            Event::CompleteStrategyPhase,
            "all players have not selected strategy cards",
        )
        .pick_strategy_card("Alice", StrategyCard::Trade)
        .pick_strategy_card("Bob", StrategyCard::Technology)
        .pick_strategy_card("Carol", StrategyCard::Diplomacy)
        .apply(Event::CompleteStrategyPhase);

    // Initiative is the lowest card held: Bob 1, Carol 2, Alice 5.
    assert_eq!(g.phase(), Phase::Action);
    assert_eq!(g.turn_order(), ["Bob", "Carol", "Alice"]);
    assert_eq!(g.current_player(), p("Bob"));
    g
}

/// Naalu's Telepathic ability gives them initiative 0, whatever cards they pick.
pub fn strategy_naalu_telepathy() -> TestGame {
    let mut g = TestGame::builder()
        .player("Alice", Faction::NaaluCollective)
        .player("Bob", Faction::FederationOfSol)
        .player("Carol", Faction::EmiratesOfHacan)
        .start();
    assert_eq!(g.state().naalu_telepathy, Some(p("Alice")));

    g.strategy_phase(&[
        ("Alice", StrategyCard::Imperial),
        ("Bob", StrategyCard::Leadership),
        ("Carol", StrategyCard::Diplomacy),
        ("Alice", StrategyCard::Warfare),
        ("Bob", StrategyCard::Trade),
        ("Carol", StrategyCard::Construction),
    ]);
    assert_eq!(g.turn_order(), ["Alice", "Bob", "Carol"]);

    // The status phase uses the same order.
    g.play_out_action_phase();
    assert_eq!(g.phase(), Phase::Status);
    assert_eq!(g.turn_order(), ["Alice", "Bob", "Carol"]);
    g
}

/// Naalu's promissory note gives initiative 0 to whoever plays it, for one round.
pub fn strategy_gift_of_prescience() -> TestGame {
    let mut g = TestGame::builder()
        .player("Alice", Faction::NaaluCollective)
        .player("Bob", Faction::FederationOfSol)
        .player("Carol", Faction::EmiratesOfHacan)
        .start();

    g.expect_rejected(
        Event::PlayGiftOfPrescience {
            player: p("Nobody"),
        },
        "unknown player",
    )
    .apply(Event::PlayGiftOfPrescience { player: p("Bob") })
    .strategy_phase(&[
        ("Alice", StrategyCard::Leadership),
        ("Bob", StrategyCard::Imperial),
        ("Carol", StrategyCard::Diplomacy),
        ("Alice", StrategyCard::Warfare),
        ("Bob", StrategyCard::Trade),
        ("Carol", StrategyCard::Construction),
    ]);
    assert_eq!(g.turn_order(), ["Bob", "Alice", "Carol"]);
    g.expect_rejected(
        Event::PlayGiftOfPrescience { player: p("Bob") },
        "expected Strategy, was Action",
    );

    // Next round, the telepathy is back with Naalu.
    g.play_out_action_phase()
        .status_phase()
        .reveal(PublicObjective::ErectAMonument)
        .complete();
    assert_eq!(g.round(), 2);
    assert_eq!(g.state().naalu_telepathy, Some(p("Alice")));
    g
}
