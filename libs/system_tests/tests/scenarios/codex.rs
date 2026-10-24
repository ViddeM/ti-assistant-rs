//! Codex components, currently the Codex IV relics.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{SIMPLE_PICKS, action_phase, three_players};

scenarios![codex_iv_book_of_latvinia, codex_iv_neuraloop,];

/// Book of Latvinia gives a point to a player that controls all 4 technology specialties, and
/// the speaker token to anyone else.
pub fn codex_iv_book_of_latvinia() -> TestGame {
    let mut g = action_phase(three_players().pok().codex());
    for planet in [Planet::NewAlbion, Planet::Gral, Planet::Lazar, Planet::Abyz] {
        g.apply(Event::SetPlanetOwner {
            player: Some(p("Bob")),
            planet,
        });
    }
    // The warfare specialty comes from an attachment.
    g.apply(Event::AddPlanetAttachment {
        player: p("Bob"),
        planet: Planet::Abyz,
        attachment: PlanetAttachment::WarfareResearchFacility,
    });

    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob")
        .gain_relic(Relic::BookOfLatvinia)
        .take_another_turn()
        .relic(Relic::BookOfLatvinia, None)
        .end_turn();
    assert_eq!(g.state().score.book_of_latvinia, Some(p("Bob")));
    assert_eq!(g.points("Bob"), 1);
    assert_eq!(g.speaker(), Some("Alice"));
    assert!(g.player("Bob").relics.is_empty());

    // Purged relics aren't tracked, so it can be gained again.
    g.turn("Carol")
        .gain_relic(Relic::BookOfLatvinia)
        .take_another_turn()
        .relic(Relic::BookOfLatvinia, None)
        .end_turn();
    assert_eq!(g.state().score.book_of_latvinia, Some(p("Bob")));
    assert_eq!(g.points("Carol"), 0);
    assert_eq!(g.speaker(), Some("Carol"));
    assert!(g.player("Carol").relics.is_empty());
    g
}

/// Neuraloop replaces the objective revealed in the status phase, even with a secret objective,
/// which is then scored like a public one.
pub fn codex_iv_neuraloop() -> TestGame {
    let mut g = action_phase(three_players().pok().codex());
    let neuraloop =
        |player: &str, purged_relic: Relic, replacement: Objective| Event::PlayNeuraloop {
            player: p(player),
            purged_relic,
            replacement,
        };
    let replacement: Objective = SecretObjective::DestroyTheirGreatestShip.into();

    g.turn("Alice")
        .gain_relic(Relic::Neuraloop)
        .take_another_turn()
        .gain_relic(Relic::ShardOfTheThrone)
        .end_turn();
    assert_eq!(g.points("Alice"), 1);
    g.expect_rejected(
        neuraloop("Alice", Relic::ShardOfTheThrone, replacement),
        "expected Status, was Action",
    );

    g.play_out_action_phase()
        .status_phase()
        .secret("Bob", SecretObjective::BecomeTheGatekeeper)
        .expect_rejected(
            neuraloop("Alice", Relic::ShardOfTheThrone, replacement),
            "after a public objective has been revealed",
        )
        .reveal(PublicObjective::ExpandBorders)
        .expect_rejected(
            neuraloop("Bob", Relic::ShardOfTheThrone, replacement),
            "doesn't have the Neuraloop relic",
        )
        .expect_rejected(
            neuraloop("Alice", Relic::TheCodex, replacement),
            "doesn't have the TheCodex relic",
        )
        .expect_rejected(
            neuraloop(
                "Alice",
                Relic::ShardOfTheThrone,
                PublicObjective::CornerTheMarket.into(),
            ),
            "already been revealed",
        )
        .expect_rejected(
            neuraloop(
                "Alice",
                Relic::ShardOfTheThrone,
                SecretObjective::BecomeTheGatekeeper.into(),
            ),
            "already been scored",
        )
        .apply(neuraloop("Alice", Relic::ShardOfTheThrone, replacement))
        .complete();

    let revealed = &g.state().score.revealed_objectives;
    assert!(!revealed.contains_key(&PublicObjective::ExpandBorders.into()));
    assert!(revealed.contains_key(&replacement));
    assert!(g.player("Alice").relics.contains(&Relic::Neuraloop));
    assert!(!g.player("Alice").relics.contains(&Relic::ShardOfTheThrone));
    assert_eq!(g.state().score.shard_of_the_throne, None);
    assert_eq!(g.points("Alice"), 0);

    g.strategy_phase(SIMPLE_PICKS)
        .play_out_action_phase()
        .status_phase()
        .public("Alice", replacement)
        .expect_rejected(
            Event::ScoreSecretObjective {
                player: p("Carol"),
                objective: Some(SecretObjective::DestroyTheirGreatestShip),
            },
            "revealed as a public objective",
        );
    assert_eq!(g.points("Alice"), 1);
    g
}

#[test]
fn codex_iv_relics_need_codex_iv() {
    let mut g = action_phase(three_players().expansions(Expansions {
        prophecy_of_kings: true,
        codex_1: true,
        codex_2: true,
        codex_3: true,
        thunders_edge: true,
        ..Default::default()
    }));
    g.expect_rejected(
        Event::GainRelicAction {
            player: p("Alice"),
            relic: Relic::Neuraloop,
        },
        "Expansion is not enabled",
    );
}
