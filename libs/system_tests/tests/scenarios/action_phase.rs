//! The action phase: turn order, passing and the different kinds of actions.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{action_phase, three_players};

scenarios![
    action_turn_order_and_passing,
    action_tactical_planets_and_attachments,
    action_strategic_technology,
    action_strategic_technology_nekro,
    action_strategic_politics_and_imperial,
    action_cards,
    action_leaders,
    action_frontier_cards,
    action_relics,
];

fn primary(player: &str, action: StrategicPrimaryAction) -> Event {
    Event::StrategicActionPrimary {
        player: p(player),
        action,
    }
}

fn secondary(player: &str, action: StrategicSecondaryAction) -> Event {
    Event::StrategicActionSecondary {
        player: p(player),
        action,
    }
}

/// Players take turns in initiative order and must use their strategy cards before passing.
pub fn action_turn_order_and_passing() -> TestGame {
    let mut g = three_players().start();
    g.strategy_phase(&[
        ("Alice", StrategyCard::Leadership),
        ("Bob", StrategyCard::Diplomacy),
        ("Carol", StrategyCard::Construction),
        ("Alice", StrategyCard::Trade),
        ("Bob", StrategyCard::Warfare),
        ("Carol", StrategyCard::Technology),
    ]);
    assert_eq!(g.turn_order(), ["Alice", "Bob", "Carol"]);

    g.expect_rejected(
        Event::PassAction { player: p("Alice") },
        "must use all strategy cards before passing",
    )
    .expect_rejected(
        Event::TacticalActionBegin { player: p("Bob") },
        "wrong players turn",
    )
    .expect_rejected(
        Event::EndTurn { player: p("Alice") },
        "expected EndActionTurn, was Action",
    )
    .expect_rejected(
        Event::StrategicActionBegin {
            player: p("Alice"),
            card: StrategyCard::Diplomacy,
        },
        "must hold the strategy card",
    );

    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol").play_card(StrategyCard::Construction);
    // Alice takes two actions in one turn.
    g.turn("Alice")
        .strategic(StrategyCard::Trade)
        .commit()
        .take_another_turn()
        .tactical()
        .commit()
        .end_turn();
    g.turn("Bob").play_card(StrategyCard::Warfare);
    g.turn("Carol")
        .strategic(StrategyCard::Technology)
        .expect_rejected(
            Event::StrategicActionCommit,
            "has not performed strategy card primary",
        )
        .primary(StrategicPrimaryAction::Technology {
            first_tech: None,
            second_tech: None,
        })
        .commit()
        .end_turn();

    g.turn("Alice").pass();
    g.expect_rejected(
        Event::StrategicActionBegin {
            player: p("Bob"),
            card: StrategyCard::Warfare,
        },
        "strategy card can't already be used",
    );
    g.turn("Bob").pass();
    // Passed players are skipped.
    g.turn("Carol").tactical().commit().end_turn();
    g.turn("Carol").pass();

    assert_eq!(g.phase(), Phase::Status);
    assert!(g.state().passed_players.is_empty());
    let status = g.state().status_phase_state.as_ref().unwrap();
    assert_eq!(status.expected_objectives_before_stage_two, 5);
    g
}

/// Taking planets (also from other players) and exploring them.
pub fn action_tactical_planets_and_attachments() -> TestGame {
    let mut g = action_phase(three_players().pok());
    let attach = |player: &str, planet: Planet, attachment: PlanetAttachment| {
        Event::TacticalActionAttachPlanetAttachment {
            player: p(player),
            planet,
            attachment,
        }
    };

    g.turn("Alice")
        .tactical()
        .expect_rejected(
            attach("Alice", Planet::Lodor, PlanetAttachment::DysonSphere),
            "Can only attach attachment to planet taken this turn",
        )
        .take(Planet::Centauri)
        .attach(Planet::Centauri, PlanetAttachment::DysonSphere)
        .expect_rejected(
            attach("Alice", Planet::Centauri, PlanetAttachment::ParadiseWorld),
            "already been explored",
        )
        // Gral has a tech specialty, so the research facility gives resources instead.
        .take(Planet::Gral)
        .attach(Planet::Gral, PlanetAttachment::BioticResearchFacility)
        .take(Planet::Arinam)
        .expect_rejected(
            attach("Alice", Planet::Arinam, PlanetAttachment::DysonSphere),
            "does not meet the trait requirements",
        )
        .expect_rejected(
            attach("Alice", Planet::Arinam, PlanetAttachment::Geoform),
            "Only attachments with planet_traits",
        )
        .commit()
        .end_turn();

    // Planets taken from other players keep their attachments but can't be explored.
    g.turn("Bob")
        .tactical()
        .take(Planet::Centauri)
        .expect_rejected(
            attach("Bob", Planet::Centauri, PlanetAttachment::ParadiseWorld),
            "taken from another player",
        )
        .take(Planet::MecatolRex)
        .commit()
        .end_turn();

    g.turn("Carol")
        .tactical()
        .expect_rejected(
            Event::TacticalActionTakePlanet {
                player: p("Carol"),
                planet: Planet::Mirage,
            },
            "isn't on the board",
        )
        .commit()
        .end_turn();

    assert!(!g.player("Alice").planets.contains_key(&Planet::Centauri));
    assert!(g.player("Bob").planets[&Planet::Centauri].contains(&PlanetAttachment::DysonSphere));
    assert!(
        g.player("Alice").planets[&Planet::Gral]
            .contains(&PlanetAttachment::BioticResearchFacilityResources)
    );
    assert_eq!(g.state().score.custodians, Some(p("Bob")));
    assert_eq!(g.points("Bob"), 1);
    g
}

/// The Technology card: primary, secondaries and the Jol-Nar and Nekro special cases.
pub fn action_strategic_technology() -> TestGame {
    let mut g = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::UniversitiesOfJolNar)
        .player("Carol", Faction::NekroVirus)
        .start();
    g.strategy_phase(&[
        ("Alice", StrategyCard::Technology),
        ("Bob", StrategyCard::Leadership),
        ("Carol", StrategyCard::Diplomacy),
        ("Alice", StrategyCard::Construction),
        ("Bob", StrategyCard::Trade),
        ("Carol", StrategyCard::Warfare),
    ]);
    g.turn("Bob").play_card(StrategyCard::Leadership);
    g.turn("Carol").play_card(StrategyCard::Diplomacy);

    use StrategicPrimaryAction::Technology as Primary;
    g.turn("Alice")
        .strategic(StrategyCard::Technology)
        .expect_rejected(
            primary(
                "Alice",
                Primary {
                    first_tech: None,
                    second_tech: Some(Technology::GravityDrive),
                },
            ),
            "Cannot take a second tech without a first tech",
        )
        .expect_rejected(
            primary(
                "Alice",
                Primary {
                    first_tech: Some(Technology::NeuralMotivator),
                    second_tech: None,
                },
            ),
            "already has tech",
        )
        .expect_rejected(
            primary(
                "Alice",
                Primary {
                    first_tech: Some(Technology::SpacialConduitCylinder),
                    second_tech: None,
                },
            ),
            "another faction's technology",
        )
        .expect_rejected(
            primary(
                "Alice",
                StrategicPrimaryAction::Politics {
                    new_speaker: p("Bob"),
                },
            ),
            "Mismatch between progress card",
        )
        .primary(Primary {
            first_tech: Some(Technology::GravityDrive),
            second_tech: Some(Technology::PlasmaScoring),
        })
        .expect_rejected(
            secondary(
                "Alice",
                StrategicSecondaryAction::Technology {
                    tech: Technology::SarweenTools,
                },
            ),
            "current player can't perform the secondary",
        )
        .expect_rejected(
            secondary(
                "Carol",
                StrategicSecondaryAction::Technology {
                    tech: Technology::SarweenTools,
                },
            ),
            "Nekro Virus cannot research",
        )
        .expect_rejected(
            secondary(
                "Carol",
                StrategicSecondaryAction::TechnologyJolNar {
                    first_tech: Technology::SarweenTools,
                    second_tech: None,
                },
            ),
            "Only the universities of Jol-Nar",
        )
        .expect_rejected(
            secondary("Bob", StrategicSecondaryAction::Leadership),
            "Mismatch between strategic progress",
        )
        // Jol-Nar may perform the primary as their secondary.
        .secondary(
            "Bob",
            StrategicSecondaryAction::TechnologyJolNar {
                first_tech: Technology::GravityDrive,
                second_tech: Some(Technology::DacxiveAnimators),
            },
        )
        .skip_rest()
        .commit()
        .end_turn();

    assert!(
        g.player("Alice")
            .technologies
            .contains(&Technology::GravityDrive)
    );
    assert!(
        g.player("Alice")
            .technologies
            .contains(&Technology::PlasmaScoring)
    );
    assert!(
        g.player("Bob")
            .technologies
            .contains(&Technology::DacxiveAnimators)
    );
    g
}

/// Nekro can't research, so their Technology primary is done without choosing anything.
pub fn action_strategic_technology_nekro() -> TestGame {
    let mut g = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::UniversitiesOfJolNar)
        .player("Carol", Faction::NekroVirus)
        .start();
    g.strategy_phase(&[
        ("Alice", StrategyCard::Leadership),
        ("Bob", StrategyCard::Diplomacy),
        ("Carol", StrategyCard::Technology),
        ("Alice", StrategyCard::Construction),
        ("Bob", StrategyCard::Trade),
        ("Carol", StrategyCard::Warfare),
    ]);
    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol")
        .strategic(StrategyCard::Technology)
        .expect_rejected(
            primary(
                "Carol",
                StrategicPrimaryAction::Technology {
                    first_tech: Some(Technology::GravityDrive),
                    second_tech: None,
                },
            ),
            "Primary action has already been performed",
        )
        .secondary(
            "Alice",
            StrategicSecondaryAction::Technology {
                tech: Technology::GravityDrive,
            },
        )
        .commit()
        .end_turn();
    g
}

/// Politics changes the speaker, Imperial scores an objective (plus a point for Mecatol Rex).
pub fn action_strategic_politics_and_imperial() -> TestGame {
    let mut g = three_players().start();
    g.strategy_phase(&[
        ("Alice", StrategyCard::Politics),
        ("Bob", StrategyCard::Imperial),
        ("Carol", StrategyCard::Leadership),
        ("Alice", StrategyCard::Diplomacy),
        ("Bob", StrategyCard::Construction),
        ("Carol", StrategyCard::Trade),
    ]);
    assert_eq!(g.turn_order(), ["Carol", "Alice", "Bob"]);

    g.turn("Carol").play_card(StrategyCard::Leadership);
    g.turn("Alice")
        .strategic(StrategyCard::Politics)
        .expect_rejected(
            primary(
                "Alice",
                StrategicPrimaryAction::Imperial {
                    score_objective: None,
                },
            ),
            "Mismatch between progress card",
        )
        .primary(StrategicPrimaryAction::Politics {
            new_speaker: p("Bob"),
        })
        .secondary("Carol", StrategicSecondaryAction::Politics)
        .skip_rest()
        .commit()
        .end_turn();
    assert_eq!(g.speaker(), Some("Bob"));

    g.turn("Bob")
        .tactical()
        .take(Planet::MecatolRex)
        .commit()
        .end_turn();
    g.turn("Carol").play_card(StrategyCard::Trade);
    g.turn("Alice").play_card(StrategyCard::Diplomacy);
    g.turn("Bob")
        .strategic(StrategyCard::Imperial)
        .expect_rejected(
            primary(
                "Bob",
                StrategicPrimaryAction::Imperial {
                    score_objective: Some(PublicObjective::ErectAMonument.into()),
                },
            ),
            "has not been revealed",
        )
        .primary(StrategicPrimaryAction::Imperial {
            score_objective: Some(PublicObjective::CornerTheMarket.into()),
        })
        .commit()
        .end_turn();

    // Custodians, Corner the Market and Imperial (holding Mecatol Rex).
    assert_eq!(g.points("Bob"), 3);
    g
}

/// Action cards that change technologies, plus cards that can't be played as actions or need
/// Thunder's Edge.
pub fn action_cards() -> TestGame {
    let mut g = action_phase(three_players().pok().codex());
    let begin = |player: &str, card: ActionCard| Event::ActionCardActionBegin {
        player: p(player),
        card,
    };
    let commit = |player: &str, data: ActionCardAction| Event::ActionCardActionCommit {
        player: p(player),
        data: Some(data),
    };

    g.turn("Alice")
        .expect_rejected(
            begin("Alice", ActionCard::Sabotage),
            "cannot be played as an action",
        )
        .expect_rejected(
            begin("Alice", ActionCard::FighterConscriptionOmega),
            "not available",
        )
        .action_card(
            ActionCard::FocusedResearch,
            Some(ActionCardAction::FocusedResearch {
                tech: Technology::GravityDrive,
            }),
        )
        .expect_rejected(Event::PlayCrisis { player: p("Bob") }, "not available")
        .end_turn();

    g.apply(Event::AddTechToPlayer {
        player: p("Bob"),
        tech: Technology::ProductionBiomes,
    });
    let divert = |remove_tech, take_tech| ActionCardAction::DivertFunding {
        remove_tech,
        take_tech,
    };
    g.turn("Bob")
        .apply(begin("Bob", ActionCard::DivertFunding))
        .expect_rejected(
            commit(
                "Bob",
                ActionCardAction::FocusedResearch {
                    tech: Technology::GravityDrive,
                },
            ),
            "does not match the selected action card",
        )
        .expect_rejected(
            commit(
                "Bob",
                divert(Technology::PlasmaScoring, Technology::GravityDrive),
            ),
            "doesn't have technology",
        )
        .expect_rejected(
            commit(
                "Bob",
                divert(Technology::SarweenTools, Technology::AntimassDeflectors),
            ),
            "already has technology",
        )
        .expect_rejected(
            commit(
                "Bob",
                divert(Technology::ProductionBiomes, Technology::GravityDrive),
            ),
            "Cannot remove faction technology",
        )
        .apply(commit(
            "Bob",
            divert(Technology::SarweenTools, Technology::GravityDrive),
        ))
        .end_turn();

    g.turn("Carol")
        .apply(begin("Carol", ActionCard::Plagiarize))
        .expect_rejected(
            commit(
                "Carol",
                ActionCardAction::Plagiarize {
                    tech: Technology::PlasmaScoring,
                },
            ),
            "Unable to take tech",
        )
        .apply(commit(
            "Carol",
            ActionCardAction::Plagiarize {
                tech: Technology::NeuralMotivator,
            },
        ))
        .end_turn();

    g.turn("Alice")
        .action_card(ActionCard::UnstablePlanet, None)
        .end_turn();

    assert!(
        g.player("Alice")
            .technologies
            .contains(&Technology::GravityDrive)
    );
    assert!(
        !g.player("Bob")
            .technologies
            .contains(&Technology::SarweenTools)
    );
    assert!(
        g.player("Carol")
            .technologies
            .contains(&Technology::NeuralMotivator)
    );
    g
}

/// Leaders with action abilities, and heroes unlocking after three scored objectives.
pub fn action_leaders() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .pok()
            .player("Alice", Faction::Arborec)
            .player("Bob", Faction::EmiratesOfHacan)
            .player("Carol", Faction::XxchaKingdom),
    );
    let hero = Leader::Hero(Hero::LetaniMiasmiala);
    assert!(!g.state().available_leaders[&p("Alice")].contains(&hero));

    g.turn("Alice")
        .expect_rejected(
            Event::LeaderActionBegin {
                player: p("Alice"),
                leader: Leader::Agent(Agent::ViscountUnlenn),
            },
            "can't be played as an action",
        )
        .leader(Leader::Agent(Agent::LetaniOspha))
        .expect_rejected(
            Event::LeaderActionCommit {
                player: p("Alice"),
                leader: Leader::Agent(Agent::LetaniOspha),
            },
            "expected LeaderAction, was EndActionTurn",
        )
        .end_turn();

    g.apply(Event::ScoreExtraPublicObjective {
        player: p("Alice"),
        objective: PublicObjective::CornerTheMarket.into(),
    })
    .apply(Event::ScoreExtraPublicObjective {
        player: p("Alice"),
        objective: PublicObjective::DevelopWeaponry.into(),
    })
    .apply(Event::ScoreExtraSecretObjective {
        player: p("Alice"),
        objective: SecretObjective::BecomeTheGatekeeper,
    });
    assert!(g.state().available_leaders[&p("Alice")].contains(&hero));

    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol")
        .leader(Leader::Agent(Agent::GgrucotoRinn))
        .end_turn();
    g.turn("Alice").leader(hero).end_turn();
    g
}

/// Frontier cards, including spawning Mirage (outside of a milty game).
pub fn action_frontier_cards() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .pok()
            .player("Alice", Faction::FederationOfSol)
            .player("Bob", Faction::NekroVirus)
            .player("Carol", Faction::XxchaKingdom),
    );
    let begin = |player: &str, card: FrontierCard| Event::FrontierCardActionBegin {
        player: p(player),
        card,
    };
    let commit = |player: &str, data: Option<FrontierCardAction>| Event::FrontierCardActionCommit {
        player: p(player),
        data,
    };

    g.turn("Alice")
        .expect_rejected(
            begin("Alice", FrontierCard::GammaRelay),
            "Only able to play frontier card actions",
        )
        .frontier(
            FrontierCard::EnigmaticDevice,
            Some(FrontierCardAction::EnigmaticDevice {
                tech: Technology::GravityDrive,
            }),
        )
        .end_turn();

    // Nekro can't research, so Enigmatic Device does nothing for them.
    g.turn("Bob")
        .apply(begin("Bob", FrontierCard::EnigmaticDevice))
        .expect_rejected(
            commit(
                "Bob",
                Some(FrontierCardAction::EnigmaticDevice {
                    tech: Technology::GravityDrive,
                }),
            ),
            "doesn't match the card",
        )
        .apply(commit("Bob", None))
        .end_turn();

    g.turn("Carol")
        .frontier(
            FrontierCard::Mirage,
            Some(FrontierCardAction::Mirage {
                system: None,
                attachment: Some(PlanetAttachment::ParadiseWorld),
            }),
        )
        .end_turn();
    assert!(g.player("Carol").planets[&Planet::Mirage].contains(&PlanetAttachment::ParadiseWorld));

    // Once spawned, Mirage is taken like any other planet.
    g.turn("Alice")
        .tactical()
        .take(Planet::Mirage)
        .commit()
        .end_turn();
    g.turn("Bob")
        .apply(begin("Bob", FrontierCard::Mirage))
        .expect_rejected(
            commit(
                "Bob",
                Some(FrontierCardAction::Mirage {
                    system: None,
                    attachment: None,
                }),
            ),
            "A player already owns mirage",
        );
    g.undo();
    g.turn("Bob").play_card(StrategyCard::Diplomacy);

    assert!(g.player("Alice").planets.contains_key(&Planet::Mirage));
    g
}

/// Gaining and using relics (purged, or exhausted and kept), and Shard of the Throne moving with
/// legendary planets and the holder's home planets.
pub fn action_relics() -> TestGame {
    let mut g = action_phase(three_players().pok().codex());
    let set_owner = |player: &str, planet: Planet| Event::SetPlanetOwner {
        player: Some(p(player)),
        planet,
    };
    let begin = |player: &str, relic: Relic| Event::RelicActionBegin {
        player: p(player),
        relic,
    };
    let commit = |player: &str, data: RelicAction| Event::RelicActionCommit {
        player: p(player),
        data: Some(data),
    };
    g.apply(set_owner("Alice", Planet::Primor))
        .apply(set_owner("Alice", Planet::Centauri))
        .apply(set_owner("Bob", Planet::Lodor));

    g.turn("Alice")
        .expect_rejected(
            begin("Alice", Relic::StellarConverter),
            "doesn't own the relic",
        )
        .gain_relic(Relic::ShardOfTheThrone)
        .end_turn();
    assert_eq!(g.points("Alice"), 1);

    // Taking Primor (legendary) from Alice also takes the Shard of the Throne.
    g.turn("Bob")
        .expect_rejected(
            Event::GainRelicAction {
                player: p("Bob"),
                relic: Relic::ShardOfTheThrone,
            },
            "already owned by another player",
        )
        .tactical()
        .take(Planet::Primor)
        .commit()
        .end_turn();
    assert_eq!(g.state().score.shard_of_the_throne, Some(p("Bob")));
    assert_eq!((g.points("Alice"), g.points("Bob")), (0, 1));

    g.turn("Carol")
        .gain_relic(Relic::StellarConverter)
        .end_turn();
    g.turn("Alice").gain_relic(Relic::NanoForge).end_turn();
    g.turn("Bob").play_card(StrategyCard::Diplomacy);

    let stellar = |planet| RelicAction::StellarConverter { planet };
    g.turn("Carol")
        .expect_rejected(
            begin("Carol", Relic::MawOfWorlds),
            "Only able to play relic actions during action phase",
        )
        .apply(begin("Carol", Relic::StellarConverter))
        .expect_rejected(commit("Carol", stellar(Planet::MecatolRex)), "mecatol rex")
        .expect_rejected(commit("Carol", stellar(Planet::Jord)), "home system")
        .expect_rejected(commit("Carol", stellar(Planet::Primor)), "legendary planet")
        .expect_rejected(
            commit(
                "Carol",
                RelicAction::NanoForge {
                    planet: Planet::Lodor,
                },
            ),
            "doesn't match the card",
        )
        .apply(commit("Carol", stellar(Planet::Lodor)))
        .end_turn();
    assert!(!g.player("Bob").planets.contains_key(&Planet::Lodor));
    assert_eq!(
        g.state().map_data.stellar_converter_destroyed_planets,
        [Planet::Lodor]
    );
    assert!(g.player("Carol").relics.is_empty());

    let nano_forge = |planet| RelicAction::NanoForge { planet };
    g.turn("Alice")
        .apply(begin("Alice", Relic::NanoForge))
        .expect_rejected(commit("Alice", nano_forge(Planet::Jord)), "home planet")
        .expect_rejected(
            commit("Alice", nano_forge(Planet::Primor)),
            "legendary planet",
        )
        .expect_rejected(
            commit("Alice", nano_forge(Planet::Lodor)),
            "must own the planet",
        )
        .apply(commit("Alice", nano_forge(Planet::Centauri)))
        .end_turn();
    assert!(g.player("Alice").planets[&Planet::Centauri].contains(&PlanetAttachment::NanoForge));

    // JR-XS455-O and Circlet of the Void are exhausted, not purged, when played.
    g.turn("Bob").gain_relic(Relic::JrXs4550).end_turn();
    g.turn("Carol")
        .gain_relic(Relic::CircletOfTheVoid)
        .end_turn();
    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob").relic(Relic::JrXs4550, None).end_turn();
    g.turn("Carol")
        .relic(Relic::CircletOfTheVoid, None)
        .end_turn();
    assert!(g.player("Bob").relics.contains(&Relic::JrXs4550));
    assert!(g.player("Carol").relics.contains(&Relic::CircletOfTheVoid));

    // Taking a planet in the holder's home system also takes the Shard of the Throne.
    g.turn("Alice")
        .tactical()
        .take(Planet::Arretze)
        .commit()
        .end_turn();
    assert_eq!(g.state().score.shard_of_the_throne, Some(p("Alice")));
    assert_eq!((g.points("Alice"), g.points("Bob")), (1, 0));

    // Arretze isn't in Alice's home system, so Bob taking it back leaves the Shard with her.
    g.turn("Bob")
        .tactical()
        .take(Planet::Arretze)
        .commit()
        .end_turn();
    assert!(g.player("Alice").relics.contains(&Relic::ShardOfTheThrone));
    assert_eq!(g.state().score.shard_of_the_throne, Some(p("Alice")));
    g
}
