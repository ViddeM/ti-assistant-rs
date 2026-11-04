//! Galactic events.

use std::collections::BTreeSet;

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{SIMPLE_PICKS, action_phase, three_players};

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
/// phase: the first in their slot, the others as extra objectives.
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

    g.strategy_phase(SIMPLE_PICKS)
        .play_out_action_phase()
        .status_phase()
        .public("Alice", PublicObjective::CornerTheMarket)
        .apply(Event::ScoreExtraPublicObjective {
            player: p("Alice"),
            objective: PublicObjective::DevelopWeaponry.into(),
        })
        .apply(Event::ScoreExtraPublicObjective {
            player: p("Alice"),
            objective: PublicObjective::CentralizeGalacticTrade.into(),
        })
        .expect_rejected(
            Event::CompleteStatusPhase,
            "Not all players have made a decision regarding their public objectives",
        )
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
