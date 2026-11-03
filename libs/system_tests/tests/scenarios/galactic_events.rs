//! Galactic events.

use ti_helper_system_tests::prelude::*;

scenarios![
    galactic_events_age_of_fighters,
    galactic_events_advent_of_the_war_sun,
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
