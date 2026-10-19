//! Longer games that go through several rounds, mixing most features. These are also good
//! starting points for manual testing (see the `seed_database` test).

use ti_helper_system_tests::prelude::*;

use crate::fixtures::three_players;

scenarios![full_game_base_three_players, full_game_pok_six_players];

/// Four rounds of a base game with three players.
pub fn full_game_base_three_players() -> TestGame {
    let mut g = three_players().start();

    /* -- Round 1 -- */
    g.strategy_phase(&[
        ("Alice", StrategyCard::Leadership),
        ("Bob", StrategyCard::Trade),
        ("Carol", StrategyCard::Politics),
        ("Alice", StrategyCard::Warfare),
        ("Bob", StrategyCard::Imperial),
        ("Carol", StrategyCard::Technology),
    ]);
    g.turn("Alice")
        .tactical()
        .take(Planet::Centauri)
        .take(Planet::Lodor)
        .take(Planet::Quann)
        .commit()
        .end_turn();
    g.turn("Carol")
        .strategic(StrategyCard::Technology)
        .primary(StrategicPrimaryAction::Technology {
            first_tech: Some(Technology::GravityDrive),
            second_tech: None,
        })
        .secondary(
            "Alice",
            StrategicSecondaryAction::Technology {
                tech: Technology::SarweenTools,
            },
        )
        .skip_rest()
        .commit()
        .end_turn();
    g.turn("Bob")
        .tactical()
        .take(Planet::MecatolRex)
        .commit()
        .end_turn();
    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Carol")
        .strategic(StrategyCard::Politics)
        .primary(StrategicPrimaryAction::Politics {
            new_speaker: p("Bob"),
        })
        .skip_rest()
        .commit()
        .end_turn();
    g.turn("Bob")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: None,
        })
        .commit()
        .end_turn();
    g.play_out_action_phase()
        .status_phase()
        .public("Alice", PublicObjective::CornerTheMarket)
        .secret("Bob", SecretObjective::BecomeTheGatekeeper)
        .reveal(PublicObjective::ErectAMonument)
        .complete();

    g.agenda(Agenda::ConventionsOfWar)
        .vote("Carol", FOR, 2)
        .vote("Alice", AGAINST, 3)
        .vote("Bob", FOR, 2)
        .resolve_expected();
    g.agenda(Agenda::Mutiny)
        .vote("Carol", FOR, 1)
        .vote("Alice", FOR, 1)
        .vote("Bob", AGAINST, 3)
        .resolve_expected();
    g.complete_agenda_phase();
    // Bob: custodians, imperial and a secret. Mutiny failed, costing Alice and Carol a point.
    assert_eq!(
        [g.points("Alice"), g.points("Bob"), g.points("Carol")],
        [0, 3, -1]
    );

    /* -- Round 2 -- */
    g.strategy_phase(&[
        ("Bob", StrategyCard::Leadership),
        ("Carol", StrategyCard::Imperial),
        ("Alice", StrategyCard::Technology),
        ("Bob", StrategyCard::Diplomacy),
        ("Carol", StrategyCard::Construction),
        ("Alice", StrategyCard::Trade),
    ]);
    g.turn("Bob")
        .tactical()
        .take(Planet::Centauri)
        .take(Planet::Gral)
        .commit()
        .end_turn();
    g.turn("Carol")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::ErectAMonument.into()),
        })
        .commit()
        .end_turn();
    g.turn("Alice")
        .strategic(StrategyCard::Technology)
        .primary(StrategicPrimaryAction::Technology {
            first_tech: Some(Technology::DacxiveAnimators),
            second_tech: Some(Technology::HyperMetabolism),
        })
        .secondary(
            "Bob",
            StrategicSecondaryAction::Technology {
                tech: Technology::GravityDrive,
            },
        )
        .skip_rest()
        .commit()
        .end_turn();
    g.play_out_action_phase()
        .status_phase()
        .public("Alice", PublicObjective::ErectAMonument)
        .public("Bob", PublicObjective::CornerTheMarket)
        .secret("Carol", SecretObjective::MonopolizeProduction)
        .reveal(PublicObjective::SwayTheCouncil)
        .complete();

    g.agenda(Agenda::SeedOfAnEmpire)
        .vote_all(FOR, 2)
        .resolve(Some(FOR));
    g.agenda(Agenda::MinisterOfWar)
        .vote("Carol", elect("Alice"), 2)
        .vote("Alice", elect("Alice"), 2)
        .vote("Bob", elect("Bob"), 3)
        .resolve_expected();
    g.complete_agenda_phase();
    assert_eq!(
        g.state().laws.get(&Agenda::MinisterOfWar),
        Some(&elect("Alice"))
    );

    /* -- Round 3 -- */
    g.strategy_phase(&[
        ("Bob", StrategyCard::Imperial),
        ("Carol", StrategyCard::Leadership),
        ("Alice", StrategyCard::Politics),
        ("Bob", StrategyCard::Warfare),
        ("Carol", StrategyCard::Diplomacy),
        ("Alice", StrategyCard::Construction),
    ]);
    // Carol takes Mecatol Rex, but Bob keeps the custodians.
    g.turn("Carol")
        .tactical()
        .take(Planet::MecatolRex)
        .commit()
        .end_turn();
    g.turn("Alice")
        .strategic(StrategyCard::Politics)
        .primary(StrategicPrimaryAction::Politics {
            new_speaker: p("Alice"),
        })
        .skip_rest()
        .commit()
        .end_turn();
    // Without Mecatol Rex, Imperial only scores the objective.
    g.turn("Bob")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::SwayTheCouncil.into()),
        })
        .commit()
        .end_turn();
    g.apply(Event::GiveSupportForTheThrone {
        giver: p("Carol"),
        receiver: p("Alice"),
    });
    g.play_out_action_phase()
        .status_phase()
        .public("Carol", PublicObjective::CornerTheMarket)
        .public("Alice", PublicObjective::SwayTheCouncil)
        .reveal(PublicObjective::ExpandBorders)
        .complete();

    g.agenda(Agenda::FleetRegulations).veto();
    g.agenda(Agenda::SharedResearch)
        .vote_all(AGAINST, 1)
        .resolve_expected();
    g.agenda(Agenda::IncentiveProgram)
        .vote_all(FOR, 1)
        .resolve_expected();
    g.complete_agenda_phase();

    /* -- Round 4 -- */
    g.strategy_phase(&[
        ("Alice", StrategyCard::Imperial),
        ("Bob", StrategyCard::Leadership),
        ("Carol", StrategyCard::Trade),
        ("Alice", StrategyCard::Diplomacy),
        ("Bob", StrategyCard::Construction),
        ("Carol", StrategyCard::Warfare),
    ]);
    g.turn("Bob")
        .tactical()
        .take(Planet::MecatolRex)
        .commit()
        .end_turn();
    g.turn("Alice")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::ExpandBorders.into()),
        })
        .commit()
        .end_turn();
    g.apply(Event::RepealLaw {
        law: Agenda::MinisterOfWar,
    });
    g.play_out_action_phase()
        .status_phase()
        .public("Bob", PublicObjective::ExpandBorders)
        .public("Alice", PublicObjective::DevelopWeaponry)
        .secret("Alice", SecretObjective::ThreatenEnemies)
        .reveal(PublicObjective::CentralizeGalacticTrade)
        .complete();

    g.agenda(Agenda::HomelandDefenseAct)
        .vote_all(FOR, 3)
        .resolve_expected();
    g.agenda(Agenda::PublicExecution)
        .vote_all(elect("Carol"), 1)
        .resolve_expected();
    g.complete_agenda_phase();

    assert_eq!((g.phase(), g.round()), (Phase::Strategy, 5));
    assert_eq!(g.speaker(), Some("Alice"));
    assert_eq!(g.state().score.custodians, Some(p("Bob")));
    assert_eq!(g.state().score.revealed_objectives.len(), 6);
    assert!(!g.state().laws.contains_key(&Agenda::MinisterOfWar));
    g
}

/// Three rounds of a six player game with the expansions, using leaders, relics, frontier
/// cards, action cards, Naalu's promissory note and pausing the time tracking.
pub fn full_game_pok_six_players() -> TestGame {
    let mut g = TestGame::builder()
        .pok()
        .codex()
        .player("Alice", Faction::NaaluCollective)
        .player("Bob", Faction::UniversitiesOfJolNar)
        .player("Carol", Faction::NekroVirus)
        .player("Dave", Faction::Winnu)
        .player("Erin", Faction::ArgentFlight)
        .player("Finn", Faction::CouncilKeleres)
        .start();

    /* -- Round 1 -- */
    g.apply(Event::PlayGiftOfPrescience { player: p("Dave") })
        .strategy_phase(&[
            ("Alice", StrategyCard::Imperial),
            ("Bob", StrategyCard::Technology),
            ("Carol", StrategyCard::Warfare),
            ("Dave", StrategyCard::Trade),
            ("Erin", StrategyCard::Leadership),
            ("Finn", StrategyCard::Diplomacy),
        ]);
    assert_eq!(
        g.turn_order(),
        ["Dave", "Erin", "Finn", "Carol", "Bob", "Alice"]
    );

    g.turn("Dave")
        .frontier(
            FrontierCard::EnigmaticDevice,
            Some(FrontierCardAction::EnigmaticDevice {
                tech: Technology::GravityDrive,
            }),
        )
        .end_turn();
    g.turn("Erin")
        .gain_relic(Relic::ShardOfTheThrone)
        .end_turn();
    g.turn("Finn")
        .tactical()
        .take(Planet::MecatolRex)
        .take(Planet::Primor)
        .commit()
        .end_turn();
    g.turn("Carol")
        .action_card(
            ActionCard::Plagiarize,
            Some(ActionCardAction::Plagiarize {
                tech: Technology::GravityDrive,
            }),
        )
        .end_turn();
    g.turn("Bob")
        .strategic(StrategyCard::Technology)
        .primary(StrategicPrimaryAction::Technology {
            first_tech: Some(Technology::GravityDrive),
            second_tech: Some(Technology::HyperMetabolism),
        })
        .secondary(
            "Dave",
            StrategicSecondaryAction::Technology {
                tech: Technology::SarweenTools,
            },
        )
        .skip_rest()
        .commit()
        .end_turn();
    // The players take a break.
    g.apply(Event::TrackTime { paused: true })
        .advance(TimeDelta::minutes(15))
        .apply(Event::TrackTime { paused: false });
    g.turn("Alice")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::DevelopWeaponry.into()),
        })
        .commit()
        .end_turn();
    g.apply(Event::GiveSupportForTheThrone {
        giver: p("Bob"),
        receiver: p("Finn"),
    });
    g.play_out_action_phase()
        .status_phase()
        .public("Finn", PublicObjective::CornerTheMarket)
        .secret("Erin", SecretObjective::BecomeTheGatekeeper)
        .reveal(PublicObjective::ErectAMonument)
        .complete();

    // Argent Flight always votes first, and Nekro can't vote.
    assert_eq!(
        g.turn_order(),
        ["Erin", "Bob", "Carol", "Dave", "Finn", "Alice"]
    );
    g.agenda(Agenda::Mutiny)
        .vote("Erin", FOR, 3)
        .vote("Bob", AGAINST, 2)
        .vote("Dave", FOR, 1)
        .vote("Finn", AGAINST, 1)
        .abstain("Alice")
        .resolve_expected();
    g.agenda(Agenda::PoliticalCensure)
        .vote_all(elect("Bob"), 1)
        .resolve_expected();
    g.complete_agenda_phase();

    /* -- Round 2 -- */
    assert_eq!(g.state().naalu_telepathy, Some(p("Alice")));
    g.strategy_phase(&[
        ("Alice", StrategyCard::Leadership),
        ("Bob", StrategyCard::Politics),
        ("Carol", StrategyCard::Technology),
        ("Dave", StrategyCard::Imperial),
        ("Erin", StrategyCard::Construction),
        ("Finn", StrategyCard::Trade),
    ]);
    g.turn("Alice")
        .leader(Leader::Agent(Agent::ZeuCxIII))
        .end_turn();
    g.turn("Bob")
        .strategic(StrategyCard::Politics)
        .primary(StrategicPrimaryAction::Politics {
            new_speaker: p("Erin"),
        })
        .skip_rest()
        .commit()
        .end_turn();
    g.turn("Erin")
        .frontier(
            FrontierCard::Mirage,
            Some(FrontierCardAction::Mirage {
                system: None,
                attachment: Some(PlanetAttachment::ParadiseWorld),
            }),
        )
        .end_turn();
    g.turn("Finn")
        .tactical()
        .take(Planet::Mirage)
        .commit()
        .end_turn();
    g.turn("Carol")
        .strategic(StrategyCard::Technology)
        .secondary(
            "Bob",
            StrategicSecondaryAction::TechnologyJolNar {
                first_tech: Technology::LightWaveDeflector,
                second_tech: None,
            },
        )
        .skip_rest()
        .commit()
        .end_turn();
    g.turn("Dave")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::ErectAMonument.into()),
        })
        .commit()
        .end_turn();
    g.play_out_action_phase()
        .status_phase()
        .public("Dave", PublicObjective::CornerTheMarket)
        .secret("Alice", SecretObjective::ThreatenEnemies)
        .reveal(PublicObjective::SwayTheCouncil)
        .complete();

    g.agenda(Agenda::SeedOfAnEmpire)
        .vote_all(AGAINST, 1)
        .resolve(Some(AGAINST));
    g.agenda(Agenda::JudicialAbolishment)
        .vote_all(AgendaElect::Law(Agenda::PoliticalCensure), 1)
        .resolve_expected();
    g.complete_agenda_phase();

    /* -- Round 3 -- */
    g.strategy_phase(&[
        ("Erin", StrategyCard::Leadership),
        ("Finn", StrategyCard::Imperial),
        ("Alice", StrategyCard::Diplomacy),
        ("Bob", StrategyCard::Technology),
        ("Carol", StrategyCard::Warfare),
        ("Dave", StrategyCard::Construction),
    ]);
    g.turn("Alice")
        .tactical()
        .take(Planet::Lodor)
        .commit()
        .end_turn();
    g.turn("Erin").play_card(StrategyCard::Leadership);
    g.turn("Dave").play_card(StrategyCard::Construction);
    g.turn("Carol").play_card(StrategyCard::Warfare);
    g.turn("Bob").play_card(StrategyCard::Technology);
    g.turn("Finn")
        .strategic(StrategyCard::Imperial)
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::ErectAMonument.into()),
        })
        .commit()
        .end_turn();
    g.play_out_action_phase();

    assert_eq!((g.phase(), g.round()), (Phase::Status, 3));
    assert_eq!(g.speaker(), Some("Erin"));
    assert_eq!(g.state().score.custodians, Some(p("Finn")));
    assert!(g.player("Finn").planets.contains_key(&Planet::Mirage));
    assert!(
        g.player("Carol")
            .technologies
            .contains(&Technology::GravityDrive)
    );
    g
}
