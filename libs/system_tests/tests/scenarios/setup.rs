//! Creating and setting up games.

use std::collections::BTreeSet;

use ti_helper_game_data::{
    common::map::{Coordinate, HexPosition},
    state::player::NewPlayer,
};
use ti_helper_system_tests::{milty::milty_data_on_map, prelude::*};

use crate::fixtures::five_player_draft;

scenarios![
    setup_base_three_players,
    setup_special_factions,
    setup_milty_import,
    setup_galactic_events,
];

/// The plainest possible game: three base game factions.
pub fn setup_base_three_players() -> TestGame {
    let g = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::XxchaKingdom)
        .start();

    assert_eq!(g.phase(), Phase::Strategy);
    assert_eq!(g.round(), 1);
    assert_eq!(g.speaker(), Some("Alice"));
    assert_eq!(g.current_player(), p("Alice"));
    assert_eq!(g.state().score.revealed_objectives.len(), 2);
    assert!(g.player("Alice").planets.contains_key(&Planet::Jord));
    assert_eq!(
        g.player("Alice")
            .technologies
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
        [Technology::AntimassDeflectors, Technology::NeuralMotivator],
    );
    g
}

/// Factions that have to choose techs and planets during setup.
pub fn setup_special_factions() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .codex()
        .player("Wendy", Faction::Winnu)
        .player("Argus", Faction::ArgentFlight)
        .player("Kel", Faction::CouncilKeleres)
        .setup();

    let techs = |player: &str, technologies: Vec<Technology>| Event::SetupPlayerTechs {
        player: p(player),
        technologies,
    };

    g.expect_rejected(
        techs(
            "Wendy",
            vec![Technology::NeuralMotivator, Technology::SarweenTools],
        ),
        "Invalid amount of technologies",
    )
    // Gravity Drive has a prerequisite, so Winnu can't start with it.
    .expect_rejected(
        techs("Wendy", vec![Technology::GravityDrive]),
        "Invalid technology selection",
    )
    .apply(techs("Wendy", vec![Technology::NeuralMotivator]))
    .expect_rejected(
        techs("Wendy", vec![Technology::SarweenTools]),
        "already performed all necessary initialization",
    )
    .expect_rejected(
        techs(
            "Argus",
            vec![Technology::GravityDrive, Technology::SarweenTools],
        ),
        "Invalid technology selection",
    )
    .apply(techs(
        "Argus",
        vec![Technology::SarweenTools, Technology::PlasmaScoring],
    ))
    .expect_rejected(
        Event::SetupTheTribunii {
            player: p("Wendy"),
            faction: Faction::XxchaKingdom,
        },
        "Player already has planets selected",
    )
    .expect_rejected(
        Event::SetupTheTribunii {
            player: p("Kel"),
            faction: Faction::FederationOfSol,
        },
        "Selected faction must be one of",
    )
    .expect_rejected(Event::StartGame, "No speaker has been set")
    .expect_rejected(
        Event::SetupSpeaker {
            player: p("Nobody"),
        },
        "Player does not exist",
    )
    .apply(Event::SetupSpeaker { player: p("Kel") })
    .expect_rejected(Event::StartGame, "No objectives have been revealed")
    .expect_rejected(
        Event::RevealInitialObjectives {
            first_objective: PublicObjective::CentralizeGalacticTrade.into(),
            second_objective: PublicObjective::CornerTheMarket.into(),
        },
        "Invalid starting objective",
    )
    .apply(Event::RevealInitialObjectives {
        first_objective: PublicObjective::CornerTheMarket.into(),
        second_objective: PublicObjective::BuildDefenses.into(),
    })
    .expect_rejected(
        Event::StartGame,
        "All initialization has not yet been completed",
    )
    .apply(Event::SetupTheTribunii {
        player: p("Kel"),
        faction: Faction::XxchaKingdom,
    })
    // Keleres can only pick techs that the other players start with.
    .expect_rejected(
        techs(
            "Kel",
            vec![Technology::NeuralMotivator, Technology::GravityDrive],
        ),
        "Invalid technology selection",
    )
    .apply(techs(
        "Kel",
        vec![Technology::NeuralMotivator, Technology::PlasmaScoring],
    ))
    .expect_rejected(
        Event::RevealInitialObjectives {
            first_objective: PublicObjective::DevelopWeaponry.into(),
            second_objective: PublicObjective::ErectAMonument.into(),
        },
        "Objectives have already been revealed",
    )
    .apply(Event::StartGame);

    let kel = g.player("Kel");
    assert!(kel.planets.contains_key(&Planet::ArchonRen));
    assert!(kel.planets.contains_key(&Planet::ArchonTau));
    assert_eq!(kel.technologies.len(), 2);
    assert_eq!(g.speaker(), Some("Kel"));
    assert_eq!(g.turn_order(), ["Kel", "Wendy", "Argus"]);
    g
}

/// A game imported from milty: players, table order, speaker and colors come from the draft.
pub fn setup_milty_import() -> TestGame {
    let mut g = TestGame::builder().milty(five_player_draft()).create();

    g.expect_rejected(
        Event::AddPlayer {
            player: NewPlayer {
                name: "Late".into(),
                faction: Faction::Winnu,
                color: Color::Black,
            },
        },
        "imported from milty",
    );
    g.finish_creation().auto_setup().start_game();

    assert_eq!(
        g.state().table_order,
        ["Ben", "Cid", "Ann", "Dee", "Eve"].map(p)
    );
    assert_eq!(g.speaker(), Some("Ben"));
    // Naaz-Rokha and Arborec both want green the most, the one earlier in table order gets it.
    assert_eq!(g.player("Ben").color, Color::Green);
    assert_ne!(g.player("Ann").color, Color::Green);
    assert!(g.state().map_data.milty_information.is_some());

    // 5 players pick one card each.
    g.strategy_phase(&[
        ("Ben", StrategyCard::Leadership),
        ("Cid", StrategyCard::Diplomacy),
        ("Ann", StrategyCard::Politics),
        ("Dee", StrategyCard::Construction),
        ("Eve", StrategyCard::Trade),
    ]);

    // In milty games, Mirage must be placed in a system on the map.
    g.turn("Ben")
        .apply(Event::FrontierCardActionBegin {
            player: p("Ben"),
            card: FrontierCard::Mirage,
        })
        .expect_rejected(
            Event::FrontierCardActionCommit {
                player: p("Ben"),
                data: Some(FrontierCardAction::Mirage {
                    system: None,
                    attachment: Some(PlanetAttachment::ParadiseWorld),
                }),
            },
            "doesn't match the card",
        )
        .apply(Event::FrontierCardActionCommit {
            player: p("Ben"),
            data: Some(FrontierCardAction::Mirage {
                system: Some("65".into()),
                attachment: Some(PlanetAttachment::ParadiseWorld),
            }),
        })
        .end_turn();

    let milty = g.state().map_data.milty_information.as_ref().unwrap();
    assert_eq!(milty.mirage_system.as_deref(), Some("65"));
    assert!(g.player("Ben").planets.contains_key(&Planet::Mirage));
    g
}

/// A game imported from milty with galactic events chosen before it starts.
pub fn setup_galactic_events() -> TestGame {
    let g = TestGame::builder()
        .milty(five_player_draft())
        .galactic_events([GalacticEvent::TotalWar, GalacticEvent::StellarAtomics])
        .start();

    assert_eq!(
        g.state().galactic_events.in_play,
        BTreeSet::from([GalacticEvent::TotalWar, GalacticEvent::StellarAtomics])
    );
    g
}

#[test]
fn keleres_home_system_is_placed_on_the_milty_map() {
    let draft = milty_data_on_map(
        "65 74 41 85A0 113 100 63 39 102 28 36 87A2 78 87A0 59 34 32 68 96a 47 49 95 50 64 94 24 83A0 85A0 83A1 35 93 80 67 0 107 110",
        &[
            ("Ann", Faction::FirmamentObsidian, 0),
            ("Ben", Faction::DeepwroughtScholarate, 1),
            ("Cid", Faction::CrimsonRebellion, 2),
            ("Dee", Faction::RalNelConsortium, 3),
            ("Kel", Faction::CouncilKeleres, 4),
        ],
    );
    let mut g = TestGame::builder()
        .milty(draft)
        .tribunii(Faction::XxchaKingdom)
        .create();

    g.finish_creation().auto_setup();

    let hex_map = &g
        .state()
        .map_data
        .milty_information
        .as_ref()
        .unwrap()
        .hex_map;
    let keleres_slot = HexPosition::Pos(Coordinate {
        ring: 3,
        position: 15,
        rotation: 0,
    });
    assert!(
        hex_map
            .tiles
            .iter()
            .any(|tile| tile.system == "14" && tile.position == keleres_slot)
    );
    assert!(hex_map.empty_galaxy_slots().is_empty());
    assert!(g.player("Kel").planets.contains_key(&Planet::ArchonRen));
}

#[test]
fn galactic_event_rules() {
    use GalacticEvent::*;

    let builder = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::XxchaKingdom);
    let choose = |events: &[GalacticEvent]| Event::SetGalacticEvents {
        events: events.iter().copied().collect(),
    };

    builder
        .clone()
        .pok()
        .thunders_edge()
        .create()
        .expect_rejected(
            choose(&[TotalWar]),
            "is not available with the enabled expansions",
        );

    let mut g = builder
        .expansions(Expansions {
            codex_4: true,
            ..Default::default()
        })
        .create();
    g.expect_rejected(
        choose(&[AgeOfExploration]),
        "is not available with the enabled expansions",
    )
    .apply(choose(&[MinorFactions]));
    assert_eq!(
        g.state().galactic_events.in_play,
        BTreeSet::from([MinorFactions])
    );

    g.apply(choose(&[]));
    assert!(g.state().galactic_events.in_play.is_empty());

    g.finish_creation()
        .expect_rejected(choose(&[MinorFactions]), "expected Creation, was Setup");
}

#[test]
fn creation_rules() {
    let builder = || {
        TestGame::builder()
            .player("Alice", Faction::FederationOfSol)
            .player("Bob", Faction::EmiratesOfHacan)
    };
    let add = |name: &str, faction: Faction| Event::AddPlayer {
        player: NewPlayer {
            name: name.into(),
            faction,
            color: Color::Black,
        },
    };

    builder()
        .create()
        .expect_rejected(Event::CreationDone, "can't have less than 3 players")
        .expect_rejected(add("Alice", Faction::Winnu), "already exists")
        .expect_rejected(add("Carol", Faction::FederationOfSol), "same faction")
        .expect_rejected(
            add("Carol", Faction::ArgentFlight),
            "is not available with the enabled expansions",
        )
        .expect_rejected(
            Event::TakeStrategyCard {
                player: p("Alice"),
                card: StrategyCard::Leadership,
            },
            "expected Strategy, was Creation",
        );

    // The base game allows 6 players, PoK allows 8.
    let mut g = builder()
        .player("Carol", Faction::XxchaKingdom)
        .player("Dave", Faction::Arborec)
        .player("Erin", Faction::Winnu)
        .player("Finn", Faction::NekroVirus)
        .create();
    g.expect_rejected(
        add("Gus", Faction::YssarilTribes),
        "can't have more than 6 players",
    );

    let mut g = builder().pok().create();
    for (name, faction) in [
        ("Carol", Faction::XxchaKingdom),
        ("Dave", Faction::Arborec),
        ("Erin", Faction::Winnu),
        ("Finn", Faction::NekroVirus),
        ("Gus", Faction::YssarilTribes),
        ("Hal", Faction::MentakCoalition),
    ] {
        g.apply(add(name, faction));
    }
    g.expect_rejected(
        add("Ida", Faction::NaaluCollective),
        "can't have more than 8 players",
    )
    .finish_creation()
    .expect_rejected(
        add("Ida", Faction::NaaluCollective),
        "expected Creation, was Setup",
    );
}
