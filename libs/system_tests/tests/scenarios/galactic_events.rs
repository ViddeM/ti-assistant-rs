//! Galactic events.

use std::collections::BTreeSet;

use ti_helper_game_data::{
    common::map::{Coordinate, HexPosition},
    components::planet::PlanetTrait,
};
use ti_helper_system_tests::prelude::*;

use crate::fixtures::{SIMPLE_PICKS, action_phase, five_player_draft, three_players};

scenarios![
    galactic_events_age_of_fighters,
    galactic_events_advent_of_the_war_sun,
    galactic_events_zealous_orthodoxy,
    galactic_events_zealous_orthodoxy_initiative,
    galactic_events_total_war,
    galactic_events_mercenaries_for_hire,
    galactic_events_stellar_atomics,
    galactic_events_wild_wild_galaxy,
    galactic_events_conventions_of_war_abandoned,
    galactic_events_dangerous_wilds,
    galactic_events_civilized_society,
    galactic_events_cultural_exchange_program,
    galactic_events_monuments_to_the_ages,
    galactic_events_age_of_exploration,
    galactic_events_rapid_mobilization,
    galactic_events_minor_factions,
    galactic_events_minor_factions_on_a_milty_map,
    galactic_events_admin,
];

const STAGE_ONE: [PublicObjective; 5] = [
    PublicObjective::CornerTheMarket,
    PublicObjective::DevelopWeaponry,
    PublicObjective::DiversifyResearch,
    PublicObjective::ErectAMonument,
    PublicObjective::ExpandBorders,
];

const STAGE_TWO: [PublicObjective; 5] = [
    PublicObjective::CentralizeGalacticTrade,
    PublicObjective::ConquerTheWeak,
    PublicObjective::FormGalacticBrainTrust,
    PublicObjective::FoundAGoldenAge,
    PublicObjective::GalvanizeThePeople,
];

fn objectives(objectives: &[PublicObjective]) -> BTreeSet<Objective> {
    objectives.iter().map(|&o| Objective::from(o)).collect()
}

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

/// The Stellar Atomics action ends the turn and can only be played once per player. A player who
/// has used it can't vote, in the agenda phase or outside it, and the other players' votes are
/// enough to resolve the agenda.
pub fn galactic_events_stellar_atomics() -> TestGame {
    let mut g = action_phase(
        three_players()
            .pok()
            .thunders_edge()
            .galactic_events([GalacticEvent::StellarAtomics]),
    );
    let stellar_atomics = Event::PlayGalacticEventAction {
        player: p("Alice"),
        action: GalacticEventAction::StellarAtomics,
    };

    let alice = g
        .turn("Alice")
        .galactic_event(GalacticEventAction::StellarAtomics);
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice.end_turn();
    assert_eq!(g.state().stellar_atomics_used, BTreeSet::from([p("Alice")]));

    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol").play_card(StrategyCard::Politics);
    g.expect_rejected(stellar_atomics, "already used")
        .apply(Event::AddAgendaBegin {
            agenda: Agenda::Mutiny,
        })
        .expect_rejected(
            Event::AddAgendaPlayerVote {
                player: p("Alice"),
                outcome: FOR,
                votes: 1,
            },
            "Stellar Atomics",
        )
        .apply(Event::AddAgendaCancel)
        .apply(Event::SetCustodians {
            player: Some(p("Alice")),
        })
        .play_out_action_phase()
        .status_phase()
        .reveal(PublicObjective::ExpandBorders)
        .complete();
    assert_eq!(g.phase(), Phase::Agenda);

    g.agenda(Agenda::Mutiny)
        .expect_rejected(
            Event::CastAgendaVote {
                player: p("Alice"),
                outcome: Some(FOR),
                votes: 1,
            },
            "Stellar Atomics token used, cannot vote on agendas",
        )
        .expect_rejected(
            Event::CastAgendaVote {
                player: p("Alice"),
                outcome: None,
                votes: 0,
            },
            "Stellar Atomics",
        )
        .vote_all(FOR, 1)
        .resolve_expected();
    assert_eq!(
        [g.points("Alice"), g.points("Bob"), g.points("Carol")],
        [1, 1, 1]
    );
    g
}

/// With Wild, Wild Galaxy the Stellar Converter can destroy Mecatol Rex, home planets and
/// legendary planets, but not a planet that is already destroyed.
pub fn galactic_events_wild_wild_galaxy() -> TestGame {
    let mut g = action_phase(
        three_players()
            .pok()
            .thunders_edge()
            .galactic_events([GalacticEvent::WildWildGalaxy]),
    );
    let give_converter = |player: &str| Event::SetRelicOwner {
        relic: Relic::StellarConverter,
        player: Some(p(player)),
    };
    let stellar = |planet| Some(RelicAction::StellarConverter { planet });
    g.apply(Event::SetPlanetOwner {
        player: Some(p("Bob")),
        planet: Planet::Primor,
    });
    assert!(g.player("Alice").planets.contains_key(&Planet::Jord));

    g.apply(give_converter("Alice"));
    g.turn("Alice")
        .relic(Relic::StellarConverter, stellar(Planet::MecatolRex))
        .end_turn();

    g.apply(give_converter("Bob"));
    g.turn("Bob")
        .relic(Relic::StellarConverter, stellar(Planet::Jord))
        .end_turn();
    assert!(!g.player("Alice").planets.contains_key(&Planet::Jord));

    g.apply(give_converter("Carol"));
    g.turn("Carol")
        .apply(Event::RelicActionBegin {
            player: p("Carol"),
            relic: Relic::StellarConverter,
        })
        .expect_rejected(
            Event::RelicActionCommit {
                player: p("Carol"),
                data: stellar(Planet::MecatolRex),
            },
            "already been destroyed",
        )
        .apply(Event::RelicActionCommit {
            player: p("Carol"),
            data: stellar(Planet::Primor),
        })
        .end_turn();
    assert!(!g.player("Bob").planets.contains_key(&Planet::Primor));
    assert_eq!(
        g.state().map_data.destroyed_planets,
        [Planet::MecatolRex, Planet::Jord, Planet::Primor]
    );
    g
}

/// The Conventions of War Abandoned action needs X-89 Bacterial Weapon and ends the turn. It
/// purges any planet on the board, home planets included, and the planet leaves its owner. A
/// purged planet can't be purged again.
pub fn galactic_events_conventions_of_war_abandoned() -> TestGame {
    let mut g = action_phase(
        three_players()
            .pok()
            .thunders_edge()
            .galactic_events([GalacticEvent::ConventionsOfWarAbandoned]),
    );
    let purge = |planet| GalacticEventAction::ConventionsOfWarAbandoned { planet };
    let play = |planet| Event::PlayGalacticEventAction {
        player: p("Alice"),
        action: purge(planet),
    };
    assert!(g.player("Bob").planets.contains_key(&Planet::Arretze));

    let alice = g
        .turn("Alice")
        .expect_rejected(play(Planet::Arretze), "X-89")
        .apply(Event::AddTechToPlayer {
            player: p("Alice"),
            tech: Technology::X89BacterialWeapon,
        })
        .galactic_event(purge(Planet::Arretze));
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice.end_turn();
    assert!(!g.player("Bob").planets.contains_key(&Planet::Arretze));
    assert_eq!(g.state().map_data.destroyed_planets, [Planet::Arretze]);

    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol").play_card(StrategyCard::Politics);
    g.turn("Alice")
        .expect_rejected(play(Planet::Arretze), "isn't on the board")
        .galactic_event(purge(Planet::Jord))
        .end_turn();
    assert!(!g.player("Alice").planets.contains_key(&Planet::Jord));
    assert_eq!(
        g.state().map_data.destroyed_planets,
        [Planet::Arretze, Planet::Jord]
    );
    g
}

/// With Dangerous Wilds a player who takes a hazardous planet from the planet deck may research 1
/// technology for it, ignoring prerequisites. It is once per planet, and not for a planet without
/// the hazardous trait, a planet taken from another player or a technology the player can't
/// research.
pub fn galactic_events_dangerous_wilds() -> TestGame {
    let mut g = action_phase(
        three_players()
            .pok()
            .thunders_edge()
            .galactic_events([GalacticEvent::DangerousWilds]),
    );
    let research = |player: &str, planet, tech| Event::TacticalActionResearchTechnology {
        player: p(player),
        planet,
        tech,
    };

    g.turn("Alice")
        .tactical()
        .expect_rejected(
            research("Alice", Planet::Lodor, Technology::SarweenTools),
            "taken this turn",
        )
        .take(Planet::Centauri)
        .expect_rejected(
            research("Alice", Planet::Centauri, Technology::SarweenTools),
            "isn't hazardous",
        )
        .take(Planet::Abyz)
        .research(Planet::Abyz, Technology::FleetLogistics)
        .expect_rejected(
            research("Alice", Planet::Abyz, Technology::SarweenTools),
            "already researched",
        )
        .take(Planet::Fria)
        .expect_rejected(
            research("Alice", Planet::Fria, Technology::NeuralMotivator),
            "already has tech",
        )
        .expect_rejected(
            research("Alice", Planet::Fria, Technology::QuantumDatahubNode),
            "another faction's technology",
        )
        .research(Planet::Fria, Technology::SarweenTools)
        .commit()
        .end_turn();
    let alice = &g.player("Alice").technologies;
    assert!(alice.contains(&Technology::FleetLogistics));
    assert!(alice.contains(&Technology::SarweenTools));

    g.turn("Bob")
        .tactical()
        .take(Planet::Abyz)
        .expect_rejected(
            research("Bob", Planet::Abyz, Technology::NeuralMotivator),
            "taken from another player",
        )
        .commit()
        .end_turn();
    assert!(g.player("Bob").planets.contains_key(&Planet::Abyz));
    g
}

/// Civilized Society reveals all five stage I and five stage II objectives at setup, and the
/// status phase reveals none. A player scores any number of public objectives in the status
/// phase, all through the status phase scoring.
pub fn galactic_events_civilized_society() -> TestGame {
    let mut g = three_players()
        .thunders_edge()
        .galactic_events([GalacticEvent::CivilizedSociety])
        .setup();

    g.expect_rejected(
        Event::RevealInitialObjectives {
            first_objective: PublicObjective::CornerTheMarket.into(),
            second_objective: PublicObjective::DevelopWeaponry.into(),
        },
        "Civilized Society reveals all public objectives at setup",
    )
    .expect_rejected(
        Event::RevealAllPublicObjectives {
            stage_one: objectives(&STAGE_ONE[..4]),
            stage_two: objectives(&STAGE_TWO),
        },
        "Expected 5 stage I objectives, got 4",
    )
    .expect_rejected(
        Event::RevealAllPublicObjectives {
            stage_one: objectives(&[&STAGE_ONE[..4], &STAGE_TWO[..1]].concat()),
            stage_two: objectives(&STAGE_TWO),
        },
        "Centralize Galactic Trade isn't a stage I objective",
    );

    g.auto_setup().start_game();
    let revealed = (g.state().score.revealed_objectives.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    assert_eq!(revealed, objectives(&[STAGE_ONE, STAGE_TWO].concat()));

    let status = g
        .strategy_phase(SIMPLE_PICKS)
        .play_out_action_phase()
        .status_phase()
        .public("Alice", PublicObjective::CornerTheMarket)
        .public("Alice", PublicObjective::DevelopWeaponry)
        .public("Alice", PublicObjective::CentralizeGalacticTrade)
        .expect_rejected(
            Event::ScorePublicObjective {
                player: p("Alice"),
                objective: None,
            },
            "already taken a public objective scoring decision",
        )
        .expect_rejected(
            Event::ScorePublicObjective {
                player: p("Alice"),
                objective: Some(PublicObjective::DevelopWeaponry.into()),
            },
            "can't score a public objective twice",
        )
        .apply(Event::ScorePublicObjective {
            player: p("Bob"),
            objective: None,
        })
        .expect_rejected(
            Event::ScorePublicObjective {
                player: p("Bob"),
                objective: Some(PublicObjective::CornerTheMarket.into()),
            },
            "already taken a public objective scoring decision",
        )
        .expect_rejected(
            Event::CompleteStatusPhase,
            "Not all players have made a decision regarding their public objectives",
        );
    assert_eq!(
        status
            .state()
            .status_phase_state
            .as_ref()
            .unwrap()
            .scored_public_objectives[&p("Alice")],
        [
            PublicObjective::CornerTheMarket,
            PublicObjective::DevelopWeaponry,
            PublicObjective::CentralizeGalacticTrade,
        ]
        .map(Objective::from)
    );
    status
        .skip_undecided()
        .expect_rejected(
            Event::RevealPublicObjective {
                objective: PublicObjective::FoundResearchOutposts,
            },
            "Under Civilized Society all public objectives are revealed at setup",
        )
        .complete();
    assert_eq!(g.points("Alice"), 4);
    assert_eq!((g.round(), g.phase()), (2, Phase::Strategy));
    g
}

/// Cultural Exchange Program: at the end of setup each player draws the faction of a player in
/// play and holds its leaders instead of their own. The Firmament doesn't take part and starts with
/// The Obsidian. The Yssaril agent follows the player who drew the Yssaril.
pub fn galactic_events_cultural_exchange_program() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .thunders_edge()
        .player("Alice", Faction::Arborec)
        .player("Bob", Faction::XxchaKingdom)
        .player("Carol", Faction::YssarilTribes)
        .player("Dave", Faction::FirmamentObsidian)
        .galactic_events([GalacticEvent::CulturalExchangeProgram])
        .setup();
    let draw = |player: &str, faction: Option<Faction>| Event::SetupCulturalExchange {
        player: p(player),
        faction,
    };

    g.expect_rejected(
        draw("Dave", Some(Faction::Arborec)),
        "The Firmament doesn't take part in the Cultural Exchange Program",
    )
    .expect_rejected(
        draw("Alice", Some(Faction::FirmamentObsidian)),
        "The Firmament doesn't take part in the Cultural Exchange Program",
    )
    .expect_rejected(
        draw("Alice", Some(Faction::FederationOfSol)),
        "The Federation of Sol isn't in play",
    )
    .apply(draw("Alice", Some(Faction::Arborec)))
    .expect_rejected(
        draw("Carol", Some(Faction::Arborec)),
        "The Arborec has already been drawn by Alice",
    )
    .apply(draw("Alice", None));
    assert_eq!(g.player("Alice").leaders_from, None);

    g.auto_setup();
    assert_eq!(g.player("Alice").leaders_from, Some(Faction::XxchaKingdom));
    assert_eq!(g.player("Bob").leaders_from, Some(Faction::YssarilTribes));
    assert_eq!(g.player("Carol").leaders_from, Some(Faction::Arborec));
    assert_eq!(g.player("Dave").leaders_from, None);

    g.apply(draw("Bob", None))
        .expect_rejected(
            Event::StartGame,
            "All initialization has not yet been completed",
        )
        .apply(draw("Bob", Some(Faction::YssarilTribes)))
        .start_game();
    assert!(g.player("Dave").relics.contains(&Relic::TheObsidian));

    let leaders = |g: &TestGame, player: &str| {
        (g.state().available_leaders[&p(player)].iter())
            .copied()
            .collect::<BTreeSet<_>>()
    };
    let rinn = Leader::Agent(Agent::GgrucotoRinn);
    let ospha = Leader::Agent(Agent::LetaniOspha);
    assert_eq!(leaders(&g, "Alice"), BTreeSet::from([rinn]));
    assert_eq!(leaders(&g, "Bob"), BTreeSet::from([rinn, ospha]));
    assert_eq!(leaders(&g, "Carol"), BTreeSet::from([ospha]));
    assert_eq!(leaders(&g, "Dave"), BTreeSet::new());

    g.strategy_phase(&[
        ("Alice", StrategyCard::Leadership),
        ("Bob", StrategyCard::Diplomacy),
        ("Carol", StrategyCard::Politics),
        ("Dave", StrategyCard::Construction),
        ("Alice", StrategyCard::Trade),
        ("Bob", StrategyCard::Warfare),
        ("Carol", StrategyCard::Technology),
        ("Dave", StrategyCard::Imperial),
    ]);
    g.turn("Alice")
        .expect_rejected(
            Event::LeaderActionBegin {
                player: p("Alice"),
                leader: ospha,
            },
            "Letani Ospha isn't available to Alice",
        )
        .leader(rinn)
        .end_turn();
    g.turn("Bob").leader(ospha);
    g
}

/// Monuments to the Ages: monuments on planets the players control. Each gets a commodity at the
/// start of every status phase and gives 1 VP per 3 commodities to whoever controls its planet.
/// Building and the commodities aren't limited by the card's rules, so that any recorded state can
/// be fixed. Destroying the planet destroys its monument.
pub fn galactic_events_monuments_to_the_ages() -> TestGame {
    let mut g = action_phase(
        three_players()
            .thunders_edge()
            .galactic_events([GalacticEvent::MonumentsToTheAges]),
    );
    let build = |player: &str, planet: Planet| Event::BuildMonument {
        player: p(player),
        planet,
    };
    let set_commodities = |planet: Planet, commodities| Event::SetMonumentCommodities {
        planet,
        commodities,
    };
    let commodities = |g: &TestGame, planet: Planet| g.state().monument_commodities(&planet);

    g.expect_rejected(build("Alice", Planet::Abyz), "Alice doesn't control Abyz")
        .expect_rejected(
            build("Nobody", Planet::Abyz),
            "Player doesn't exist 'Nobody'",
        )
        .expect_rejected(
            Event::DestroyMonument {
                planet: Planet::Abyz,
            },
            "Abyz has no monument",
        )
        .apply(build("Alice", Planet::Jord))
        .apply(Event::DestroyMonument {
            planet: Planet::Jord,
        })
        .apply(Event::SetPlanetOwner {
            player: Some(p("Alice")),
            planet: Planet::ThundersEdge,
        })
        .apply(build("Alice", Planet::ThundersEdge))
        .apply(Event::DestroyMonument {
            planet: Planet::ThundersEdge,
        })
        .apply(Event::SetPlanetOwner {
            player: None,
            planet: Planet::ThundersEdge,
        });

    g.turn("Alice")
        .tactical()
        .take(Planet::Abyz)
        .take(Planet::Fria)
        .commit()
        .end_turn();
    g.turn("Bob")
        .tactical()
        .take(Planet::Arinam)
        .take(Planet::Meer)
        .commit()
        .end_turn();
    g.apply(build("Alice", Planet::Abyz))
        .apply(build("Alice", Planet::Fria))
        .apply(build("Bob", Planet::Arinam))
        .apply(build("Bob", Planet::Meer))
        .apply(Event::DestroyMonument {
            planet: Planet::Meer,
        })
        .expect_rejected(build("Alice", Planet::Abyz), "Abyz already has a monument");
    assert_eq!(commodities(&g, Planet::Abyz), Some(0));
    assert_eq!(commodities(&g, Planet::Meer), None);

    g.play_out_action_phase();
    assert_eq!(commodities(&g, Planet::Abyz), Some(1));
    assert_eq!(commodities(&g, Planet::Arinam), Some(1));
    g.apply(set_commodities(Planet::Abyz, 5));
    assert_eq!(commodities(&g, Planet::Abyz), Some(5));
    g.apply(set_commodities(Planet::Abyz, 1))
        .expect_rejected(set_commodities(Planet::Abyz, 1), "No change")
        .expect_rejected(set_commodities(Planet::Meer, 0), "Meer has no monument")
        .apply(set_commodities(Planet::Fria, 0));
    assert_eq!(commodities(&g, Planet::Fria), Some(0));
    g.status_phase()
        .reveal(PublicObjective::DiversifyResearch)
        .complete();
    assert_eq!((g.round(), g.phase()), (2, Phase::Strategy));
    assert_eq!(commodities(&g, Planet::Abyz), Some(1));

    g.strategy_phase(SIMPLE_PICKS).play_out_action_phase();
    assert_eq!(commodities(&g, Planet::Abyz), Some(2));
    assert_eq!(commodities(&g, Planet::Fria), Some(1));
    assert!(g.state().monument_points().is_empty());
    g.status_phase()
        .reveal(PublicObjective::ErectAMonument)
        .complete();

    g.strategy_phase(SIMPLE_PICKS);
    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob")
        .tactical()
        .take(Planet::Abyz)
        .commit()
        .end_turn();
    g.play_out_action_phase();
    assert_eq!(commodities(&g, Planet::Abyz), Some(3));
    assert_eq!(commodities(&g, Planet::Fria), Some(2));
    assert_eq!([g.points("Alice"), g.points("Bob")], [0, 2]);

    g.apply(set_commodities(Planet::Fria, 3));
    assert_eq!(g.points("Alice"), 1);
    g.apply(Event::SetPlanetDestroyed {
        planet: Planet::Arinam,
        destroyed: true,
    });
    assert_eq!(commodities(&g, Planet::Arinam), None);
    assert_eq!(g.points("Bob"), 1);
    g.apply(Event::DestroyMonument {
        planet: Planet::Abyz,
    })
    .apply(build("Bob", Planet::Meer));
    assert_eq!(g.points("Bob"), 0);
    assert_eq!(commodities(&g, Planet::Meer), Some(0));
    g
}

fn explore(system: &str, ring: u32, position: u32) -> GalacticEventAction {
    GalacticEventAction::AgeOfExploration {
        tile: Some((
            system.into(),
            Coordinate {
                ring,
                position,
                rotation: 0,
            },
        )),
    }
}

/// The Age of Exploration action needs Dark Energy Tap and ends the turn. On a milty map the
/// drawn tile must be an unused red or blue tile, placed in an empty spot touching at least 2
/// non-home systems. It is added to the map, and its planets can be taken.
pub fn galactic_events_age_of_exploration() -> TestGame {
    let mut g = TestGame::builder()
        .milty(five_player_draft())
        .galactic_events([GalacticEvent::AgeOfExploration])
        .start();
    g.strategy_phase(&[
        ("Ben", StrategyCard::Leadership),
        ("Cid", StrategyCard::Diplomacy),
        ("Ann", StrategyCard::Politics),
        ("Dee", StrategyCard::Construction),
        ("Eve", StrategyCard::Trade),
    ]);
    let play = |action| Event::PlayGalacticEventAction {
        player: p("Ben"),
        action,
    };

    let ben = g
        .turn("Ben")
        .expect_rejected(play(explore("26", 4, 2)), "doesn't own Dark Energy Tap")
        .apply(Event::AddTechToPlayer {
            player: p("Ben"),
            tech: Technology::DarkEnergyTap,
        })
        .expect_rejected(
            play(GalacticEventAction::AgeOfExploration { tile: None }),
            "Choose the drawn tile",
        )
        .expect_rejected(
            play(explore("65", 4, 2)),
            "isn't an unused red or blue tile",
        )
        .expect_rejected(
            play(explore("82", 4, 2)),
            "isn't an unused red or blue tile",
        )
        .expect_rejected(play(explore("26", 2, 0)), "isn't an empty spot")
        .expect_rejected(play(explore("26", 4, 1)), "isn't an empty spot")
        .galactic_event(explore("26", 4, 2));
    assert_eq!(ben.phase(), Phase::EndActionTurn);
    ben.end_turn();

    let hex_map = &g
        .state()
        .map_data
        .milty_information
        .as_ref()
        .unwrap()
        .hex_map;
    let lodor = hex_map.tiles.iter().find(|tile| tile.system == "26");
    assert_eq!(
        lodor.map(|tile| &tile.position),
        Some(&HexPosition::Pos(Coordinate {
            ring: 4,
            position: 2,
            rotation: 0,
        }))
    );

    g.turn("Cid")
        .tactical()
        .take(Planet::Lodor)
        .commit()
        .end_turn();
    assert!(g.player("Cid").planets.contains_key(&Planet::Lodor));
    g
}

/// Rapid Mobilization: Start Game puts the Fracture into play, gives everyone their breakthrough
/// and starts the post-setup phase. There each player gains control of the planets they pick,
/// unowned non-home planets on the board, and researches 1 technology: 2 for the Nomad and none
/// for the Nekro Virus. Once everyone has resolved it, the strategy phase of round 1 starts.
pub fn galactic_events_rapid_mobilization() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .thunders_edge()
        .player("Alice", Faction::Nomad)
        .player("Bob", Faction::NekroVirus)
        .player("Carol", Faction::EmbersOfMuaat)
        .galactic_events([GalacticEvent::RapidMobilization])
        .start();
    assert_eq!(g.phase(), Phase::PostSetup);
    let state = g.state();
    assert!(state.fracture_in_play);
    assert_eq!(state.fracture_roll, None);
    for player in ["Alice", "Bob", "Carol"] {
        assert!(g.player(player).breakthrough.is_some());
    }
    assert!(g.player("Carol").planets.contains_key(&Planet::Avernus));
    assert_eq!(
        g.state().rapid_mobilization,
        BTreeSet::from([p("Alice"), p("Bob"), p("Carol")])
    );

    let resolve = |player: &str, planets: &[Planet], technologies: &[Technology]| {
        Event::ResolveRapidMobilization {
            player: p(player),
            planets: planets.iter().cloned().collect(),
            technologies: technologies.iter().cloned().collect(),
        }
    };

    g.expect_rejected(
        Event::TakeStrategyCard {
            player: p("Alice"),
            card: StrategyCard::Leadership,
        },
        "expected Strategy",
    )
    .expect_rejected(
        Event::CompletePostSetupPhase,
        "Not all players have resolved Rapid Mobilization",
    )
    .expect_rejected(
        resolve("Alice", &[Planet::Abyz], &[Technology::GravityDrive]),
        "Alice researches 2 technologies with Rapid Mobilization",
    )
    .expect_rejected(
        resolve("Bob", &[Planet::Lodor], &[Technology::GravityDrive]),
        "Bob researches 0 technologies with Rapid Mobilization",
    )
    .expect_rejected(
        resolve("Carol", &[Planet::Jord], &[Technology::GravityDrive]),
        "Jord isn't an unowned non-home planet on the board",
    )
    .expect_rejected(
        resolve("Carol", &[Planet::Mirage], &[Technology::GravityDrive]),
        "Mirage isn't an unowned non-home planet on the board",
    )
    .expect_rejected(
        resolve(
            "Alice",
            &[Planet::Abyz],
            &[Technology::SlingRelay, Technology::GravityDrive],
        ),
        "Alice already owns Sling Relay",
    )
    .apply(resolve(
        "Carol",
        &[Planet::Quann],
        &[Technology::AntimassDeflectors],
    ))
    .expect_rejected(
        resolve("Carol", &[], &[Technology::GravityDrive]),
        "Carol has no Rapid Mobilization to resolve",
    )
    .expect_rejected(
        resolve(
            "Alice",
            &[Planet::Quann],
            &[Technology::GravityDrive, Technology::NeuralMotivator],
        ),
        "Quann isn't an unowned non-home planet on the board",
    )
    .apply(resolve(
        "Alice",
        &[Planet::Abyz, Planet::Fria],
        &[Technology::GravityDrive, Technology::NeuralMotivator],
    ))
    .apply(resolve("Bob", &[Planet::Lodor, Planet::MeharXull], &[]));

    let owns =
        |g: &TestGame, player: &str, planet: Planet| g.player(player).planets.contains_key(&planet);
    assert!(owns(&g, "Alice", Planet::Abyz));
    assert!(owns(&g, "Alice", Planet::Fria));
    assert!(owns(&g, "Bob", Planet::Lodor));
    assert!(owns(&g, "Bob", Planet::MeharXull));
    assert!(owns(&g, "Carol", Planet::Quann));
    let alice = &g.player("Alice").technologies;
    assert!(alice.contains(&Technology::GravityDrive));
    assert!(alice.contains(&Technology::NeuralMotivator));
    assert!(
        g.player("Carol")
            .technologies
            .contains(&Technology::AntimassDeflectors)
    );
    assert!(g.state().rapid_mobilization.is_empty());

    g.apply(Event::CompletePostSetupPhase);
    assert_eq!((g.round(), g.phase()), (1, Phase::Strategy));
    g.strategy_phase(SIMPLE_PICKS);
    assert_eq!(g.phase(), Phase::Action);
    g
}

/// Minor Factions: each player is dealt a faction that isn't being played, whose home system
/// becomes a minor faction system. Its planets aren't home planets and have all three traits, and
/// controlling all of them gives the player that faction's alliance card.
pub fn galactic_events_minor_factions() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .codex()
        .thunders_edge()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::CouncilKeleres)
        .tribunii(Faction::XxchaKingdom)
        .galactic_events([GalacticEvent::MinorFactions, GalacticEvent::DangerousWilds])
        .setup();
    let deal = |player: &str, faction: Option<Faction>| Event::SetupMinorFaction {
        player: p(player),
        faction,
        replaced_system: None,
    };
    let minor_faction = |g: &TestGame, player: &str| g.player(player).minor_faction;
    let alliance =
        |g: &TestGame, player: &str, planet| g.state().minor_faction_alliance(&p(player), &planet);

    g.expect_rejected(
        deal("Alice", Some(Faction::EmiratesOfHacan)),
        "The Emirates of Hacan can't be Alice's minor faction",
    )
    .expect_rejected(
        deal("Alice", Some(Faction::CouncilKeleres)),
        "The Council Keleres can't be Alice's minor faction",
    )
    .expect_rejected(
        Event::SetupMinorFaction {
            player: p("Alice"),
            faction: Some(Faction::MentakCoalition),
            replaced_system: Some("19".into()),
        },
        "Only games imported from milty have a map",
    )
    .apply(deal("Alice", Some(Faction::MentakCoalition)))
    .expect_rejected(
        deal("Bob", Some(Faction::MentakCoalition)),
        "The Mentak Coalition can't be Bob's minor faction",
    )
    .expect_rejected(
        Event::SetupTheTribunii {
            player: p("Carol"),
            faction: Faction::MentakCoalition,
        },
        "The Mentak Coalition is Alice's minor faction",
    );

    g.auto_setup();
    assert_eq!(minor_faction(&g, "Alice"), Some(Faction::MentakCoalition));
    assert_eq!(minor_faction(&g, "Bob"), Some(Faction::Arborec));
    assert_eq!(minor_faction(&g, "Carol"), Some(Faction::BaronyOfLetnev));

    g.apply(deal("Bob", None))
        .expect_rejected(
            Event::StartGame,
            "All initialization has not yet been completed",
        )
        .expect_rejected(
            deal("Bob", Some(Faction::XxchaKingdom)),
            "The Xxcha Kingdom can't be Bob's minor faction",
        )
        .apply(deal("Bob", Some(Faction::NaaluCollective)))
        .start_game()
        .strategy_phase(SIMPLE_PICKS);

    assert!(!g.state().is_home_planet(&Planet::MollPrimus));
    assert!(g.state().is_home_planet(&Planet::Jord));
    assert_eq!(
        g.state().planet_traits(&Planet::MollPrimus),
        BTreeSet::from([
            PlanetTrait::Cultural,
            PlanetTrait::Hazardous,
            PlanetTrait::Industrial,
        ])
    );
    assert_eq!(alliance(&g, "Alice", Planet::MollPrimus), None);

    g.turn("Alice")
        .tactical()
        .take(Planet::MollPrimus)
        .attach(Planet::MollPrimus, PlanetAttachment::DysonSphere)
        .research(Planet::MollPrimus, Technology::GravityDrive)
        .commit()
        .end_turn();
    assert_eq!(
        alliance(&g, "Alice", Planet::MollPrimus),
        Some(Faction::MentakCoalition)
    );
    assert!(
        g.player("Alice")
            .technologies
            .contains(&Technology::GravityDrive)
    );

    let terraform = |planet| Event::AddPlanetAttachment {
        player: p("Alice"),
        planet,
        attachment: PlanetAttachment::Terraform,
    };
    g.apply(terraform(Planet::MollPrimus))
        .expect_rejected(terraform(Planet::Jord), "Cannot terraform home planet")
        .apply(Event::AddAgendaBegin {
            agenda: Agenda::ColonialRedistribution,
        });
    let candidates = &(g.state().agenda_override_state.as_ref())
        .expect("Colonial Redistribution to be in progress")
        .vote_state
        .candidates;
    assert!(candidates.contains(&AgendaElect::NonHomePlanet(Planet::MollPrimus)));
    g.apply(Event::AddAgendaCancel);

    g.turn("Bob")
        .tactical()
        .take(Planet::Maaluuk)
        .commit()
        .end_turn();
    assert_eq!(alliance(&g, "Bob", Planet::Maaluuk), None);

    g.apply(Event::SetRelicOwner {
        relic: Relic::StellarConverter,
        player: Some(p("Carol")),
    });
    g.turn("Carol")
        .relic(
            Relic::StellarConverter,
            Some(RelicAction::StellarConverter {
                planet: Planet::MollPrimus,
            }),
        )
        .end_turn();
    assert!(!g.player("Alice").planets.contains_key(&Planet::MollPrimus));

    g.turn("Alice").tactical().commit().end_turn();
    g.turn("Bob")
        .tactical()
        .take(Planet::Druaa)
        .commit()
        .end_turn();
    assert_eq!(
        alliance(&g, "Bob", Planet::Druaa),
        Some(Faction::NaaluCollective)
    );
    assert_eq!(
        alliance(&g, "Bob", Planet::Maaluuk),
        Some(Faction::NaaluCollective)
    );
    g
}

/// Minor Factions on a milty map: the minor faction system replaces a system on the map that isn't
/// Mecatol Rex, a home system, a hyperlane or a gate, and only Undo changes it. The Ghosts of Creuss
/// put the Creuss gate on the map and the Crimson Rebellion put their gate on the map, with their
/// home systems outside the galaxy.
pub fn galactic_events_minor_factions_on_a_milty_map() -> TestGame {
    let mut g = TestGame::builder()
        .milty(five_player_draft())
        .galactic_events([GalacticEvent::MinorFactions])
        .setup();
    let deal =
        |player: &str, faction: Option<Faction>, replaced: Option<&str>| Event::SetupMinorFaction {
            player: p(player),
            faction,
            replaced_system: replaced.map(String::from),
        };
    let positions = |g: &TestGame, system: &str| {
        (g.state().map_data.milty_information.as_ref())
            .expect("a milty map")
            .hex_map
            .tiles
            .iter()
            .filter(|tile| tile.system == system)
            .map(|tile| tile.position.clone())
            .collect::<Vec<_>>()
    };
    let mehar_xull = positions(&g, "24");
    let tequran_torkan = positions(&g, "28");
    let dal_bootha_xxehan = positions(&g, "32");
    let centauri_gral = positions(&g, "34");
    for system in [
        &mehar_xull,
        &tequran_torkan,
        &dal_bootha_xxehan,
        &centauri_gral,
    ] {
        assert_eq!(system.len(), 1);
    }

    let not_replaceable = "isn't a system on the map that a minor faction system can replace";
    g.expect_rejected(
        deal("Ann", Some(Faction::MentakCoalition), None),
        "Choose the minor faction and the system it replaces on the map",
    )
    .expect_rejected(
        deal("Ann", None, Some("24")),
        "Choose the minor faction and the system it replaces on the map",
    )
    .expect_rejected(
        deal("Ann", Some(Faction::MentakCoalition), Some("112")),
        not_replaceable,
    )
    .expect_rejected(
        deal("Ann", Some(Faction::MentakCoalition), Some("85A")),
        not_replaceable,
    )
    .expect_rejected(
        deal("Ann", Some(Faction::MentakCoalition), Some("95")),
        not_replaceable,
    )
    .expect_rejected(
        deal("Ann", Some(Faction::MentakCoalition), Some("26")),
        not_replaceable,
    )
    .apply(deal("Ann", Some(Faction::MentakCoalition), Some("24")))
    .expect_rejected(
        deal("Ann", Some(Faction::NaaluCollective), Some("28")),
        "The Mentak Coalition is already on the map as Ann's minor faction, use Undo to change it",
    )
    .expect_rejected(
        deal("Ann", None, None),
        "Choose the minor faction and the system it replaces on the map",
    )
    .expect_rejected(
        deal("Ben", Some(Faction::NaaluCollective), Some("2")),
        not_replaceable,
    )
    .apply(deal("Ben", Some(Faction::GhostsOfCreuss), Some("28")))
    .expect_rejected(
        deal("Cid", Some(Faction::NaaluCollective), Some("17")),
        not_replaceable,
    )
    .apply(deal("Cid", Some(Faction::CrimsonRebellion), Some("32")));

    assert_eq!(positions(&g, "2"), mehar_xull);
    assert_eq!(positions(&g, "24"), vec![]);
    assert_eq!(positions(&g, "17"), tequran_torkan);
    assert_eq!(positions(&g, "28"), vec![]);
    assert_eq!(positions(&g, "51"), vec![HexPosition::OutsideGalaxy]);
    assert_eq!(positions(&g, "94"), dal_bootha_xxehan);
    assert_eq!(positions(&g, "32"), vec![]);
    assert_eq!(positions(&g, "118"), vec![HexPosition::OutsideGalaxy]);
    for planet in [Planet::MollPrimus, Planet::Creuss, Planet::AhkCreuxx] {
        assert!(!g.state().is_home_planet(&planet), "{planet:?}");
    }

    g.auto_setup().start_game();
    assert_eq!(g.phase(), Phase::Strategy);
    assert_eq!(g.player("Dee").minor_faction, Some(Faction::BaronyOfLetnev));
    assert_eq!(positions(&g, "10"), centauri_gral);
    assert_eq!(positions(&g, "34"), vec![]);
    g
}

/// The admin events fix the tracked state of the galactic events in play, in any phase: a used
/// Stellar Atomics token (the player can't vote while it's used), the Zealous Orthodoxy holder and
/// the Total War points. Each needs an existing player and a change.
pub fn galactic_events_admin() -> TestGame {
    let mut g = three_players()
        .pok()
        .thunders_edge()
        .codex()
        .galactic_events([
            GalacticEvent::StellarAtomics,
            GalacticEvent::ZealousOrthodoxy,
            GalacticEvent::TotalWar,
            GalacticEvent::ConventionsOfWarAbandoned,
        ])
        .start();
    assert_eq!(g.phase(), Phase::Strategy);
    let stellar_atomics = |player: &str, used| Event::SetStellarAtomicsUsed {
        player: p(player),
        used,
    };
    let zealous_orthodoxy = |player: Option<&str>| Event::SetZealousOrthodoxyOwner {
        player: player.map(p),
    };
    let total_war = |player: &str, points| Event::SetTotalWarPoints {
        player: p(player),
        points,
    };
    let alice_votes = Event::AddAgendaPlayerVote {
        player: p("Alice"),
        outcome: FOR,
        votes: 1,
    };

    g.apply(stellar_atomics("Alice", true))
        .apply(Event::AddAgendaBegin {
            agenda: Agenda::Mutiny,
        })
        .expect_rejected(alice_votes.clone(), "Stellar Atomics token used")
        .apply(stellar_atomics("Alice", false))
        .apply(alice_votes)
        .apply(Event::AddAgendaCancel);
    assert!(g.state().stellar_atomics_used.is_empty());

    g.apply(zealous_orthodoxy(Some("Bob")));
    assert_eq!(g.points("Bob"), 1);
    g.apply(zealous_orthodoxy(Some("Carol")));
    assert_eq!([g.points("Bob"), g.points("Carol")], [0, 1]);
    g.apply(zealous_orthodoxy(None));
    assert_eq!(g.points("Carol"), 0);

    g.apply(total_war("Alice", 3))
        .apply(total_war("Alice", 2))
        .apply(total_war("Bob", 1));
    assert_eq!([g.points("Alice"), g.points("Bob")], [2, 1]);
    g.apply(total_war("Bob", 0));
    assert_eq!(g.points("Bob"), 0);
    assert!(!(g.state().score.galactic_events.total_war).contains_key(&p("Bob")));

    g.expect_rejected(
        stellar_atomics("Nobody", true),
        "Player doesn't exist 'Nobody'",
    )
    .expect_rejected(
        zealous_orthodoxy(Some("Nobody")),
        "Player doesn't exist 'Nobody'",
    )
    .expect_rejected(total_war("Nobody", 1), "Player doesn't exist 'Nobody'")
    .expect_rejected(stellar_atomics("Alice", false), "No change")
    .expect_rejected(zealous_orthodoxy(None), "No change")
    .expect_rejected(total_war("Alice", 2), "No change")
    .expect_rejected(total_war("Alice", -1), "can't be negative");
    g
}

#[test]
fn galactic_events_admin_needs_the_events() {
    let mut g = three_players().pok().thunders_edge().codex().start();
    g.expect_rejected(
        Event::SetStellarAtomicsUsed {
            player: p("Alice"),
            used: true,
        },
        "Stellar Atomics isn't in play",
    )
    .expect_rejected(
        Event::SetZealousOrthodoxyOwner {
            player: Some(p("Alice")),
        },
        "Zealous Orthodoxy isn't in play",
    )
    .expect_rejected(
        Event::SetTotalWarPoints {
            player: p("Alice"),
            points: 1,
        },
        "Total War isn't in play",
    );
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

#[test]
fn dangerous_wilds_needs_the_event() {
    let mut g = action_phase(three_players().pok().thunders_edge());
    g.turn("Alice")
        .tactical()
        .take(Planet::Abyz)
        .expect_rejected(
            Event::TacticalActionResearchTechnology {
                player: p("Alice"),
                planet: Planet::Abyz,
                tech: Technology::SarweenTools,
            },
            "Dangerous Wilds isn't in play",
        );
}

#[test]
fn revealing_all_objectives_needs_civilized_society() {
    let mut g = three_players().thunders_edge().setup();
    g.expect_rejected(
        Event::RevealAllPublicObjectives {
            stage_one: objectives(&STAGE_ONE),
            stage_two: objectives(&STAGE_TWO),
        },
        "Civilized Society isn't in play",
    );
}

#[test]
fn cultural_exchange_program_needs_the_event() {
    let mut g = three_players().pok().thunders_edge().setup();
    g.expect_rejected(
        Event::SetupCulturalExchange {
            player: p("Alice"),
            faction: Some(Faction::XxchaKingdom),
        },
        "Cultural Exchange Program isn't in play",
    );
}

#[test]
fn cultural_exchange_program_needs_pok() {
    let mut g = three_players().thunders_edge().create();
    g.expect_rejected(
        Event::SetGalacticEvents {
            events: BTreeSet::from([GalacticEvent::CulturalExchangeProgram]),
        },
        "is not available with the enabled expansions",
    );
}

#[test]
fn minor_factions_needs_the_event() {
    let mut g = three_players().codex().setup();
    g.expect_rejected(
        Event::SetupMinorFaction {
            player: p("Alice"),
            faction: Some(Faction::MentakCoalition),
            replaced_system: None,
        },
        "Minor Factions isn't in play",
    );
}

#[test]
fn monuments_to_the_ages_needs_the_event() {
    let mut g = three_players().thunders_edge().start();
    g.expect_rejected(
        Event::BuildMonument {
            player: p("Alice"),
            planet: Planet::Jord,
        },
        "Monuments to the Ages isn't in play",
    )
    .expect_rejected(
        Event::DestroyMonument {
            planet: Planet::Jord,
        },
        "Monuments to the Ages isn't in play",
    )
    .expect_rejected(
        Event::SetMonumentCommodities {
            planet: Planet::Jord,
            commodities: 1,
        },
        "Monuments to the Ages isn't in play",
    );
}

#[test]
fn rapid_mobilization_needs_the_event() {
    let mut g = three_players().pok().thunders_edge().start();
    assert_eq!(g.phase(), Phase::Strategy);
    assert!(!g.state().fracture_in_play);
    assert!(g.state().rapid_mobilization.is_empty());
    g.expect_rejected(
        Event::ResolveRapidMobilization {
            player: p("Alice"),
            planets: BTreeSet::from([Planet::Abyz]),
            technologies: BTreeSet::from([Technology::GravityDrive]),
        },
        "expected PostSetup",
    )
    .expect_rejected(Event::CompletePostSetupPhase, "expected PostSetup");
}

#[test]
fn age_of_exploration_without_a_milty_map() {
    let mut g = action_phase(
        three_players()
            .pok()
            .codex()
            .galactic_events([GalacticEvent::AgeOfExploration]),
    );
    g.turn("Alice")
        .apply(Event::AddTechToPlayer {
            player: p("Alice"),
            tech: Technology::DarkEnergyTap,
        })
        .expect_rejected(
            Event::PlayGalacticEventAction {
                player: p("Alice"),
                action: explore("26", 4, 2),
            },
            "Only games imported from milty",
        )
        .galactic_event(GalacticEventAction::AgeOfExploration { tile: None });
    assert_eq!(g.phase(), Phase::EndActionTurn);
}
