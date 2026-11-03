//! Galactic events.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{action_phase, three_players};

scenarios![
    galactic_events_age_of_fighters,
    galactic_events_advent_of_the_war_sun,
    galactic_events_zealous_orthodoxy,
    galactic_events_zealous_orthodoxy_initiative,
    galactic_events_total_war,
    galactic_events_mercenaries_for_hire,
];

/// Age of Fighters gives everyone Fighter II when the game starts, and the Naalu Hybrid Crystal
/// Fighter II instead. Setup techs are kept.
pub fn galactic_events_age_of_fighters() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .thunders_edge()
        .player("Alice", Faction::NaaluCollective)
        .player("Bob", Faction::Winnu)
        .player("Carol", Faction::FederationOfSol)
        .galactic_events([GalacticEvent::AgeOfFighters])
        .setup();

    g.auto_setup();
    for player in ["Alice", "Bob", "Carol"] {
        let technologies = &g.player(player).technologies;
        assert!(!technologies.contains(&Technology::FighterII));
        assert!(!technologies.contains(&Technology::HybridCrystalFighterII));
    }

    g.start_game();
    let alice = &g.player("Alice").technologies;
    assert!(alice.contains(&Technology::HybridCrystalFighterII));
    assert!(!alice.contains(&Technology::FighterII));
    assert!(g.player("Bob").technologies.contains(&Technology::FighterII));
    assert!(g.player("Carol").technologies.contains(&Technology::FighterII));
    assert!(
        g.player("Bob")
            .technologies
            .contains(&Technology::NeuralMotivator)
    );
    g
}

/// Advent of the War Sun gives everyone but Muaat War Sun when the game starts. Setup techs are
/// kept.
pub fn galactic_events_advent_of_the_war_sun() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .thunders_edge()
        .player("Alice", Faction::EmbersOfMuaat)
        .player("Bob", Faction::Winnu)
        .player("Carol", Faction::FederationOfSol)
        .galactic_events([GalacticEvent::AdventOfTheWarSun])
        .setup();

    g.auto_setup();
    for player in ["Alice", "Bob", "Carol"] {
        assert!(!g.player(player).technologies.contains(&Technology::WarSun));
    }

    g.start_game();
    assert!(!g.player("Alice").technologies.contains(&Technology::WarSun));
    assert!(g.player("Bob").technologies.contains(&Technology::WarSun));
    assert!(g.player("Carol").technologies.contains(&Technology::WarSun));
    assert!(
        g.player("Bob")
            .technologies
            .contains(&Technology::NeuralMotivator)
    );
    g
}

/// Zealous Orthodoxy gives 1 point to the first player to score 2 secret objectives. Outside the
/// status phase that's whoever reaches 2 first. The point is kept when a secret is unscored.
pub fn galactic_events_zealous_orthodoxy() -> TestGame {
    let mut g = action_phase(
        three_players()
            .thunders_edge()
            .galactic_events([GalacticEvent::ZealousOrthodoxy]),
    );

    g.apply(Event::ScoreExtraSecretObjective {
        player: p("Bob"),
        objective: SecretObjective::BecomeTheGatekeeper,
    });
    assert_eq!(g.state().score.galactic_events.zealous_orthodoxy, None);

    g.apply(Event::ScoreExtraSecretObjective {
        player: p("Bob"),
        objective: SecretObjective::AdaptNewStrategies,
    });
    assert_eq!(
        g.state().score.galactic_events.zealous_orthodoxy,
        Some(p("Bob"))
    );
    assert_eq!(g.points("Bob"), 3);

    g.apply(Event::ScoreExtraSecretObjective {
        player: p("Alice"),
        objective: SecretObjective::MonopolizeProduction,
    })
    .apply(Event::ScoreExtraSecretObjective {
        player: p("Alice"),
        objective: SecretObjective::ThreatenEnemies,
    });
    assert_eq!(
        g.state().score.galactic_events.zealous_orthodoxy,
        Some(p("Bob"))
    );
    assert_eq!(g.points("Alice"), 2);

    g.apply(Event::UnscoreSecretObjective {
        player: p("Bob"),
        objective: SecretObjective::AdaptNewStrategies,
    });
    assert_eq!(
        g.state().score.galactic_events.zealous_orthodoxy,
        Some(p("Bob"))
    );
    assert_eq!(g.points("Bob"), 2);
    g
}

/// Players reaching 2 secret objectives in the same status phase are tied by initiative, whatever
/// order their decisions are entered in. The point is only given once every player has decided.
pub fn galactic_events_zealous_orthodoxy_initiative() -> TestGame {
    let mut g = action_phase(
        three_players()
            .thunders_edge()
            .galactic_events([GalacticEvent::ZealousOrthodoxy]),
    );

    g.apply(Event::ScoreExtraSecretObjective {
        player: p("Alice"),
        objective: SecretObjective::AdaptNewStrategies,
    })
    .apply(Event::ScoreExtraSecretObjective {
        player: p("Bob"),
        objective: SecretObjective::BecomeTheGatekeeper,
    })
    .play_out_action_phase()
    .status_phase()
    .secret("Bob", SecretObjective::MonopolizeProduction)
    .secret("Alice", SecretObjective::ThreatenEnemies);
    assert_eq!(g.state().score.galactic_events.zealous_orthodoxy, None);

    g.status_phase().reveal(PublicObjective::ExpandBorders);
    assert_eq!(
        g.state().score.galactic_events.zealous_orthodoxy,
        Some(p("Alice"))
    );

    g.status_phase().complete();
    assert_eq!(g.points("Alice"), 3);
    assert_eq!(g.points("Bob"), 2);
    g
}

/// The Total War action gives 1 point and ends the turn. It can be played every turn, by every
/// player.
pub fn galactic_events_total_war() -> TestGame {
    let mut g = action_phase(
        three_players()
            .codex()
            .galactic_events([GalacticEvent::TotalWar]),
    );
    let total_war = |player: &str| Event::PlayGalacticEventAction {
        player: p(player),
        action: GalacticEventAction::TotalWar,
    };

    g.expect_rejected(total_war("Bob"), "wrong players turn");
    let alice = g
        .turn("Alice")
        .galactic_event(GalacticEventAction::TotalWar);
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    assert_eq!(alice.points("Alice"), 1);
    alice
        .expect_rejected(total_war("Alice"), "expected Action")
        .end_turn();

    g.turn("Bob")
        .galactic_event(GalacticEventAction::TotalWar)
        .end_turn();
    g.turn("Carol").play_card(StrategyCard::Politics);
    g.turn("Alice")
        .galactic_event(GalacticEventAction::TotalWar)
        .end_turn();

    let total_war = &g.state().score.galactic_events.total_war;
    assert_eq!(total_war.get(&p("Alice")), Some(&2));
    assert_eq!(total_war.get(&p("Bob")), Some(&1));
    assert_eq!(g.points("Alice"), 2);
    assert_eq!(g.points("Bob"), 1);
    assert_eq!(g.points("Carol"), 0);
    g
}

/// The Mercenaries for Hire action ends the turn and gives no points. It can be played again on a
/// later turn.
pub fn galactic_events_mercenaries_for_hire() -> TestGame {
    let mut g = action_phase(
        three_players()
            .pok()
            .thunders_edge()
            .galactic_events([GalacticEvent::MercenariesForHire]),
    );

    let alice = g
        .turn("Alice")
        .galactic_event(GalacticEventAction::MercenariesForHire);
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice.end_turn();

    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol").play_card(StrategyCard::Politics);
    g.turn("Alice")
        .galactic_event(GalacticEventAction::MercenariesForHire)
        .end_turn();

    for player in ["Alice", "Bob", "Carol"] {
        assert_eq!(g.points(player), 0);
    }
    g
}

#[test]
fn total_war_needs_the_event() {
    let mut g = action_phase(three_players().codex());
    g.expect_rejected(
        Event::PlayGalacticEventAction {
            player: p("Alice"),
            action: GalacticEventAction::TotalWar,
        },
        "Total War isn't in play",
    );
}
