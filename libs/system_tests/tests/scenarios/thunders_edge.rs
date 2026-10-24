//! Thunder's Edge: committing to the expedition at the end of a turn and completing it.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{action_phase, three_players};

scenarios![
    thunders_edge_commit_and_complete,
    thunders_edge_passing,
    thunders_edge_admin_flow,
];

fn a_valid_system() -> SystemId {
    let mut systems: Vec<_> = System::valid_thunders_edge_systems().into_iter().collect();
    systems.sort();
    systems.remove(0)
}

fn complete(player: &str, won_by: &str, placed_in: SystemId) -> Event {
    Event::CompleteThundersEdgeExpedition {
        player: p(player),
        placed_in,
        won_by: p(won_by),
    }
}

/// Players claim slots at the end of their turns, the final claimer places Thunder's Edge.
pub fn thunders_edge_commit_and_complete() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());

    g.turn("Alice")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::Resources)
        .expect_rejected(
            Event::CommitToThundersEdgeExpedition {
                player: p("Alice"),
                slot: TEExpeditionSlot::Influence,
            },
            "already committed",
        )
        .take_another_turn()
        .tactical()
        .commit()
        .expect_rejected(
            Event::CommitToThundersEdgeExpedition {
                player: p("Alice"),
                slot: TEExpeditionSlot::Resources,
            },
            "already claimed",
        )
        .commit_to_expedition(TEExpeditionSlot::Influence)
        .end_turn();
    assert!(g.state().action_progress.is_none());
    assert!(g.state().players[&p("Alice")].breakthrough.is_some());
    assert!(g.state().players[&p("Bob")].breakthrough.is_none());

    g.turn("Bob")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::ActionCards)
        .end_turn();
    g.turn("Carol").tactical().commit().end_turn();
    g.turn("Alice")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::TradeGoods)
        .end_turn();
    g.turn("Bob")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::SecretObjective)
        .end_turn();

    // Commits are only possible at the end of the turn, by the current player.
    g.expect_rejected(
        Event::CommitToThundersEdgeExpedition {
            player: p("Carol"),
            slot: TEExpeditionSlot::TechSpecialityPlanet,
        },
        "expected EndActionTurn",
    );

    let system = a_valid_system();
    g.turn("Carol")
        .tactical()
        .commit()
        .expect_rejected(
            complete("Carol", "Alice", system.clone()),
            "not waiting to be placed",
        )
        .commit_to_expedition(TEExpeditionSlot::TechSpecialityPlanet)
        .expect_rejected(
            Event::CommitToThundersEdgeExpedition {
                player: p("Carol"),
                slot: TEExpeditionSlot::TradeGoods,
            },
            "already committed",
        )
        .expect_rejected(
            Event::TakeAnotherTurn { player: p("Carol") },
            "must be placed",
        )
        .expect_rejected(Event::EndTurn { player: p("Carol") }, "must be placed")
        .expect_rejected(
            complete("Carol", "Bob", system.clone()),
            "most claimed slots",
        )
        .expect_rejected(
            complete("Carol", "Carol", system.clone()),
            "most claimed slots",
        )
        .expect_rejected(
            complete("Carol", "Alice", "18".to_string()),
            "not a valid candidate",
        )
        .expect_rejected(
            complete("Alice", "Alice", system.clone()),
            "wrong players turn",
        )
        .apply(complete("Carol", "Alice", system.clone()))
        .expect_rejected(
            complete("Carol", "Alice", system.clone()),
            "not waiting to be placed",
        )
        .end_turn();

    let te = g.state().te_expedition.as_ref().unwrap();
    assert_eq!(
        te.completed,
        Some(TECompletion {
            won_by: p("Alice"),
            placed_in: system,
        })
    );
    assert!(
        g.state().players[&p("Alice")]
            .planets
            .contains_key(&Planet::ThundersEdge)
    );
    assert!(g.state().players[&p("Carol")].breakthrough.is_some());
    g
}

/// A player that passes can still commit, but can't take another turn.
pub fn thunders_edge_passing() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());
    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol").play_card(StrategyCard::Politics);

    g.turn("Alice")
        .strategic(StrategyCard::Construction)
        .commit()
        .take_another_turn()
        .pass_and_stay()
        .expect_rejected(Event::TakeAnotherTurn { player: p("Alice") }, "has passed")
        .commit_to_expedition(TEExpeditionSlot::Resources)
        .end_turn();
    assert!(g.state().passed_players.contains(&p("Alice")));

    g.turn("Bob").play_card(StrategyCard::Trade);
    g.turn("Carol").play_card(StrategyCard::Warfare);
    // Passing without committing is just ending the turn.
    g.turn("Bob").pass();
    g.turn("Carol").pass();

    assert_eq!(g.phase(), Phase::Status);
    g
}

/// Filling and completing the expedition from the admin page doesn't involve the players.
pub fn thunders_edge_admin_flow() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());
    for slot in [
        TEExpeditionSlot::Resources,
        TEExpeditionSlot::ActionCards,
        TEExpeditionSlot::Influence,
        TEExpeditionSlot::SecretObjective,
        TEExpeditionSlot::TechSpecialityPlanet,
        TEExpeditionSlot::TradeGoods,
    ] {
        g.apply(Event::ClaimThundersEdgeExpeditionSlot {
            player: Some(p("Alice")),
            slot,
        });
    }
    assert!(g.state().action_progress.is_none());

    // Nothing is required of the players.
    g.turn("Alice").tactical().commit().end_turn();

    let system = a_valid_system();
    g.apply(Event::SetThundersEdgeCompletion {
        new_completion: Some(TECompletion {
            won_by: p("Alice"),
            placed_in: system.clone(),
        }),
    });
    assert!(
        g.state().players[&p("Alice")]
            .planets
            .contains_key(&Planet::ThundersEdge)
    );

    g.apply(Event::SetThundersEdgeCompletion {
        new_completion: Some(TECompletion {
            won_by: p("Bob"),
            placed_in: system,
        }),
    });
    assert!(
        !g.state().players[&p("Alice")]
            .planets
            .contains_key(&Planet::ThundersEdge)
    );
    assert!(
        g.state().players[&p("Bob")]
            .planets
            .contains_key(&Planet::ThundersEdge)
    );
    assert!(g.state().players[&p("Bob")].breakthrough.is_some());
    g
}
