//! Creating and setting up games.

use ti_helper_game_data::state::player::NewPlayer;
use ti_helper_system_tests::prelude::*;

use crate::fixtures::five_player_draft;

scenarios![
    setup_base_three_players,
    setup_special_factions,
    setup_milty_import,
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
            "Expansion is not enabled",
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
