//! Thunder's Edge: the expedition, The Obsidian, its planets and its action cards.

use ti_helper_game_data::{
    components::breakthrough::Breakthrough,
    state::{
        faction_state::FirmamentSide,
        game_state::{ActionPhaseProgress, PlanetLocation, TacticalProgress},
    },
};
use ti_helper_system_tests::prelude::*;

use crate::fixtures::{action_phase, five_player_draft, three_players};

scenarios![
    thunders_edge_commit_and_complete,
    thunders_edge_passing,
    thunders_edge_admin_flow,
    firmament_becomes_obsidian,
    thunders_edge_take_planets_without_tile,
    thunders_edge_styx_victory_point,
    thunders_edge_breakthrough_planets,
    thunders_edge_action_cards,
    thunders_edge_overrule_and_strategize,
    thunders_edge_crisis_and_puppets_on_a_string,
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

/// The state is sent to clients as JSON, also while the expedition progress is set.
#[test]
fn thunders_edge_progress_survives_json() {
    use ti_helper_game_data::state::game_state::GameState;

    fn assert_round_trips(g: &TestGame) {
        let state = g.state().clone();
        let json = serde_json::to_value(&state).expect("failed to serialize the state");
        let back: GameState = serde_json::from_value(json.clone())
            .unwrap_or_else(|e| panic!("failed to deserialize the state: {e}\n{json}"));
        assert!(back == state, "the state changed in the JSON round trip");
    }

    let mut g = action_phase(three_players().thunders_edge());
    g.turn("Alice")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::Resources);
    assert!(g.state().action_progress.is_some());
    assert_round_trips(&g);
    g.current().end_turn();

    for slot in [
        TEExpeditionSlot::ActionCards,
        TEExpeditionSlot::Influence,
        TEExpeditionSlot::SecretObjective,
        TEExpeditionSlot::TechSpecialityPlanet,
    ] {
        g.apply(Event::ClaimThundersEdgeExpeditionSlot {
            player: Some(p("Bob")),
            slot,
        });
    }
    g.turn("Bob")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::TradeGoods);
    assert!(g.state().action_progress.is_some());
    assert_round_trips(&g);
}

/// The Firmament flips to The Obsidian using an action, which flips their home system planets
/// and breakthrough.
pub fn firmament_becomes_obsidian() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::FirmamentObsidian)
            .player("Bob", Faction::EmiratesOfHacan)
            .player("Carol", Faction::XxchaKingdom)
            .thunders_edge(),
    );
    assert_eq!(
        g.player("Alice").firmament_side(),
        Some(FirmamentSide::Firmament)
    );
    assert_eq!(g.player("Bob").firmament_side(), None);
    assert_eq!(g.player("Alice").faction_name(), "The Firmament");
    let mut planets: Vec<_> = g.player("Alice").planets.keys().cloned().collect();
    planets.sort();
    assert_eq!(planets, [Planet::Cronos, Planet::Tallin]);

    g.apply(Event::SetBreakthrough {
        player: p("Alice"),
        breakthrough: Some(Breakthrough::TheSowing),
    });

    g.expect_rejected(
        Event::PuppetsOfTheBlade { player: p("Bob") },
        "wrong players turn",
    );
    g.apply(Event::PuppetsOfTheBlade { player: p("Alice") });
    assert_eq!(g.phase(), Phase::EndActionTurn);
    g.expect_rejected(
        Event::PuppetsOfTheBlade { player: p("Alice") },
        "expected Action",
    );

    let alice = g.player("Alice");
    assert_eq!(alice.firmament_side(), Some(FirmamentSide::Obsidian));
    assert_eq!(alice.faction_name(), "The Obsidian");
    assert_eq!(alice.breakthrough, Some(Breakthrough::TheReaping));
    assert!(!alice.planets.contains_key(&Planet::Cronos));
    assert!(!alice.planets.contains_key(&Planet::Tallin));
    assert!(alice.planets.contains_key(&Planet::TallinHollow));
    assert!(alice.planets.contains_key(&Planet::CronosHollow));

    g.turn("Alice").end_turn();
    g.play_out_action_phase();
    assert_eq!(g.phase(), Phase::Status);
    g
}

#[test]
fn only_the_firmament_can_become_the_obsidian() {
    let mut g = action_phase(three_players().thunders_edge());
    g.expect_rejected(
        Event::PuppetsOfTheBlade { player: p("Alice") },
        "Only The Firmament",
    );
}

fn take(player: &str, planet: Planet) -> Event {
    Event::TacticalActionTakePlanet {
        player: p(player),
        planet,
    }
}

fn activated_system(g: &TestGame) -> Option<SystemId> {
    let Some(ActionPhaseProgress::Tactical(TacticalProgress {
        activated_system, ..
    })) = &g.state().action_progress
    else {
        panic!("no tactical action in progress");
    };
    activated_system.clone()
}

/// The Fracture planets are in their own systems. Planets without a system tile can be taken
/// once they are on the board: Thunder's Edge where the expedition placed it, and Avernus once
/// someone controls it. Custodia Vigilia only exists as a card. Taking Thunder's Edge unlocks
/// the breakthrough.
pub fn thunders_edge_take_planets_without_tile() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());
    let system = a_valid_system();

    let alice = g
        .turn("Alice")
        .tactical()
        .expect_rejected(take("Alice", Planet::ThundersEdge), "isn't on the board")
        .expect_rejected(take("Alice", Planet::Avernus), "isn't on the board")
        .expect_rejected(
            take("Alice", Planet::CustodiaVigiliaOmega),
            "isn't on the board",
        )
        .take(Planet::Lethe)
        .take(Planet::Phlegethon);
    assert_eq!(activated_system(&alice), Some("126".to_string()));
    alice.commit().end_turn();

    g.apply(Event::SetThundersEdgeCompletion {
        new_completion: Some(TECompletion {
            won_by: p("Alice"),
            placed_in: system.clone(),
        }),
    })
    .apply(Event::SetPlanetOwner {
        player: Some(p("Alice")),
        planet: Planet::Avernus,
    });

    let bob = g.turn("Bob").tactical().take(Planet::ThundersEdge);
    assert_eq!(activated_system(&bob), Some(system));
    bob.commit().end_turn();

    let carol = g.turn("Carol").tactical().take(Planet::Avernus);
    assert_eq!(activated_system(&carol), None);
    carol.commit().end_turn();

    assert!(g.player("Alice").planets.contains_key(&Planet::Lethe));
    assert!(g.player("Alice").planets.contains_key(&Planet::Phlegethon));
    assert!(
        !g.player("Alice")
            .planets
            .contains_key(&Planet::ThundersEdge)
    );
    assert!(g.player("Bob").planets.contains_key(&Planet::ThundersEdge));
    assert_eq!(
        g.player("Bob").breakthrough,
        Some(Breakthrough::AutoFactories)
    );
    assert!(g.player("Carol").planets.contains_key(&Planet::Avernus));
    g
}

/// Whoever controls Styx has 1 VP from it.
pub fn thunders_edge_styx_victory_point() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());
    let points = |g: &TestGame| [g.points("Alice"), g.points("Bob")];
    assert_eq!(points(&g), [0, 0]);

    g.turn("Alice")
        .tactical()
        .take(Planet::Styx)
        .commit()
        .end_turn();
    assert_eq!(points(&g), [1, 0]);

    g.turn("Bob")
        .tactical()
        .take(Planet::Styx)
        .commit()
        .end_turn();
    assert_eq!(points(&g), [0, 1]);

    g.apply(Event::SetPlanetOwner {
        player: None,
        planet: Planet::Styx,
    });
    assert_eq!(points(&g), [0, 0]);
    g
}

/// Gaining a breakthrough applies its on-gain effects: I.I.H.Q. Modernization gives the Keleres
/// Custodia Vigilia, and Stellar Genesis gives Muaat Avernus, which other players can then take.
pub fn thunders_edge_breakthrough_planets() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::CouncilKeleres)
            .player("Bob", Faction::EmbersOfMuaat)
            .player("Carol", Faction::XxchaKingdom)
            .pok()
            .codex()
            .thunders_edge(),
    );

    g.turn("Alice")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::Resources)
        .end_turn();
    assert_eq!(
        g.player("Alice").breakthrough,
        Some(Breakthrough::IIHQModernization)
    );
    assert!(
        g.player("Alice")
            .planets
            .contains_key(&Planet::CustodiaVigiliaOmega)
    );

    g.apply(Event::SetBreakthrough {
        player: p("Bob"),
        breakthrough: Some(Breakthrough::StellarGenesis),
    });
    assert!(g.player("Bob").planets.contains_key(&Planet::Avernus));
    g.turn("Bob").tactical().commit().end_turn();

    g.turn("Carol")
        .tactical()
        .expect_rejected(
            take("Carol", Planet::CustodiaVigiliaOmega),
            "isn't on the board",
        )
        .take(Planet::Avernus)
        .commit()
        .end_turn();
    assert!(!g.player("Bob").planets.contains_key(&Planet::Avernus));
    assert!(g.player("Carol").planets.contains_key(&Planet::Avernus));
    assert!(
        g.player("Alice")
            .planets
            .contains_key(&Planet::CustodiaVigiliaOmega)
    );
    g
}

/// The Crimson Rebellion starts with its breakthrough unlocked.
#[test]
fn the_crimson_rebellion_starts_with_its_breakthrough() {
    let g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::CrimsonRebellion)
            .player("Bob", Faction::EmiratesOfHacan)
            .player("Carol", Faction::XxchaKingdom)
            .thunders_edge(),
    );
    assert_eq!(
        g.player("Alice").breakthrough,
        Some(Breakthrough::ResonanceGenerator)
    );
    assert_eq!(g.player("Bob").breakthrough, None);
}

/// Only the planets on the shown side of the Firmament / The Obsidian home system tile are on
/// the board, so the other side can't be taken.
#[test]
fn only_the_shown_side_of_the_firmament_home_system_is_on_the_board() {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::FirmamentObsidian)
            .player("Bob", Faction::EmiratesOfHacan)
            .player("Carol", Faction::XxchaKingdom)
            .thunders_edge(),
    );
    let assert_locations = |g: &TestGame, on_board: [Planet; 2], hidden: [Planet; 2]| {
        for planet in on_board {
            assert_eq!(
                g.state().planet_location(&planet),
                PlanetLocation::SystemTile,
                "{planet:?}"
            );
        }
        for planet in hidden {
            assert_eq!(
                g.state().planet_location(&planet),
                PlanetLocation::NotOnBoard,
                "{planet:?}"
            );
        }
    };

    assert_locations(
        &g,
        [Planet::Cronos, Planet::Tallin],
        [Planet::CronosHollow, Planet::TallinHollow],
    );

    g.apply(Event::PuppetsOfTheBlade { player: p("Alice") });
    g.turn("Alice").end_turn();
    assert_locations(
        &g,
        [Planet::CronosHollow, Planet::TallinHollow],
        [Planet::Cronos, Planet::Tallin],
    );

    let bob = g
        .turn("Bob")
        .tactical()
        .expect_rejected(take("Bob", Planet::Cronos), "isn't on the board")
        .take(Planet::CronosHollow);
    assert_eq!(activated_system(&bob), Some("96B".to_string()));
    bob.commit().end_turn();

    assert!(g.player("Bob").planets.contains_key(&Planet::CronosHollow));
    assert!(
        !g.player("Alice")
            .planets
            .contains_key(&Planet::CronosHollow)
    );
}

/// In milty games the home system tile on the map is flipped as well.
#[test]
fn the_obsidian_flips_the_map_tile() {
    let mut g = TestGame::builder().milty(five_player_draft()).create();
    g.finish_creation().auto_setup().start_game();
    g.strategy_phase(&[
        ("Ben", StrategyCard::Diplomacy),
        ("Cid", StrategyCard::Politics),
        ("Ann", StrategyCard::Construction),
        ("Dee", StrategyCard::Trade),
        ("Eve", StrategyCard::Leadership),
    ]);

    let systems = |g: &TestGame| -> Vec<SystemId> {
        let milty = g.state().map_data.milty_information.as_ref().unwrap();
        milty
            .hex_map
            .tiles
            .iter()
            .map(|t| t.system.clone())
            .collect()
    };
    assert!(systems(&g).contains(&"96A".to_string()));
    assert!(!systems(&g).contains(&"96B".to_string()));

    g.apply(Event::PuppetsOfTheBlade { player: p("Eve") });
    assert!(!systems(&g).contains(&"96A".to_string()));
    assert!(systems(&g).contains(&"96B".to_string()));
}

fn begin_action_card(player: &str, card: ActionCard) -> Event {
    Event::ActionCardActionBegin {
        player: p(player),
        card,
    }
}

fn commit_action_card(player: &str, data: ActionCardAction) -> Event {
    Event::ActionCardActionCommit {
        player: p(player),
        data: Some(data),
    }
}

/// Brilliance gives a player their breakthrough, Mercenary Contract returns another player's
/// planet card, and the Ω versions of the Codex cards replace the old ones.
pub fn thunders_edge_action_cards() -> TestGame {
    let mut g = action_phase(three_players().pok().codex().thunders_edge());
    g.apply(Event::SetPlanetOwner {
        player: Some(p("Bob")),
        planet: Planet::Abyz,
    })
    .apply(Event::SetPlanetOwner {
        player: Some(p("Carol")),
        planet: Planet::Lazar,
    });
    let brilliance = |player: Option<&str>| ActionCardAction::Brilliance {
        player: player.map(p),
    };
    let mercenary_contract = |planet| ActionCardAction::MercenaryContract {
        planet: Some(planet),
    };

    g.turn("Alice")
        .expect_rejected(
            begin_action_card("Alice", ActionCard::FighterConscription),
            "not available",
        )
        .action_card(ActionCard::FighterConscriptionOmega, None)
        .take_another_turn()
        .action_card(ActionCard::Brilliance, Some(brilliance(Some("Bob"))))
        .end_turn();
    assert_eq!(
        g.player("Bob").breakthrough,
        Some(Breakthrough::AutoFactories)
    );

    g.turn("Bob")
        .apply(begin_action_card("Bob", ActionCard::Brilliance))
        .expect_rejected(
            commit_action_card("Bob", brilliance(Some("Bob"))),
            "already gained their breakthrough",
        )
        .apply(commit_action_card("Bob", brilliance(None)))
        .end_turn();

    g.turn("Carol")
        .apply(begin_action_card("Carol", ActionCard::MercenaryContract))
        .expect_rejected(
            commit_action_card("Carol", mercenary_contract(Planet::Arretze)),
            "home planet",
        )
        .expect_rejected(
            commit_action_card("Carol", mercenary_contract(Planet::Lazar)),
            "keep their own planets",
        )
        .expect_rejected(
            commit_action_card("Carol", mercenary_contract(Planet::Gral)),
            "not owned by any player",
        )
        .apply(commit_action_card(
            "Carol",
            mercenary_contract(Planet::Abyz),
        ))
        .end_turn();

    assert!(!g.player("Bob").planets.contains_key(&Planet::Abyz));
    assert!(g.player("Bob").planets.contains_key(&Planet::Arretze));
    assert!(g.player("Carol").planets.contains_key(&Planet::Lazar));
    assert_eq!(g.player("Alice").breakthrough, None);
    g
}

/// Overrule and Strategize perform the primary and secondary abilities of strategy cards that
/// are readied or unchosen, without exhausting them.
pub fn thunders_edge_overrule_and_strategize() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());
    let overrule = |card, primary| ActionCardAction::Overrule { card, primary };
    let strategize = |card, secondary| ActionCardAction::Strategize { card, secondary };

    g.turn("Alice").play_card(StrategyCard::Leadership);

    g.turn("Bob")
        .apply(begin_action_card("Bob", ActionCard::Overrule))
        .expect_rejected(
            commit_action_card("Bob", overrule(StrategyCard::Leadership, None)),
            "exhausted",
        )
        .expect_rejected(
            commit_action_card("Bob", overrule(StrategyCard::Technology, None)),
            "must be performed",
        )
        .expect_rejected(
            commit_action_card(
                "Bob",
                overrule(
                    StrategyCard::Diplomacy,
                    Some(StrategicPrimaryAction::Politics {
                        new_speaker: p("Bob"),
                    }),
                ),
            ),
            "Mismatch",
        )
        .apply(commit_action_card(
            "Bob",
            overrule(
                StrategyCard::Technology,
                Some(StrategicPrimaryAction::Technology {
                    first_tech: Some(Technology::GravityDrive),
                    second_tech: None,
                }),
            ),
        ))
        .end_turn();

    g.turn("Carol")
        .apply(begin_action_card("Carol", ActionCard::Strategize))
        .expect_rejected(
            commit_action_card(
                "Carol",
                strategize(
                    StrategyCard::Leadership,
                    StrategicSecondaryAction::Leadership,
                ),
            ),
            "exhausted",
        )
        .expect_rejected(
            commit_action_card(
                "Carol",
                strategize(StrategyCard::Imperial, StrategicSecondaryAction::Skip),
            ),
            "must be performed",
        )
        .expect_rejected(
            commit_action_card(
                "Carol",
                strategize(
                    StrategyCard::Imperial,
                    StrategicSecondaryAction::Technology {
                        tech: Technology::PlasmaScoring,
                    },
                ),
            ),
            "Mismatch",
        )
        .apply(commit_action_card(
            "Carol",
            strategize(
                StrategyCard::Technology,
                StrategicSecondaryAction::Technology {
                    tech: Technology::PlasmaScoring,
                },
            ),
        ))
        .end_turn();

    g.turn("Alice")
        .action_card(
            ActionCard::Overrule,
            Some(overrule(
                StrategyCard::Imperial,
                Some(StrategicPrimaryAction::Imperial {
                    score_objective: Some(PublicObjective::CornerTheMarket.into()),
                }),
            )),
        )
        .end_turn();

    g.turn("Bob")
        .action_card(
            ActionCard::Overrule,
            Some(overrule(
                StrategyCard::Politics,
                Some(StrategicPrimaryAction::Politics {
                    new_speaker: p("Bob"),
                }),
            )),
        )
        .end_turn();

    assert!(g.player("Bob").has_tech(&Technology::GravityDrive));
    assert!(g.player("Carol").has_tech(&Technology::PlasmaScoring));
    assert_eq!(g.points("Alice"), 1);
    assert_eq!(g.speaker(), Some("Bob"));
    assert_eq!(
        g.state().spent_strategy_cards,
        [StrategyCard::Leadership].into()
    );

    g.turn("Carol").play_card(StrategyCard::Politics);
    g
}

/// At the end of a turn, Crisis skips the next player's turn, and Puppets on a String lets a
/// passed player perform 1 action, also after the last player has passed. The expedition is full,
/// so passing only ends at the end of the turn because of these cards.
pub fn thunders_edge_crisis_and_puppets_on_a_string() -> TestGame {
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

    g.turn("Alice")
        .strategic(StrategyCard::Leadership)
        .commit()
        .expect_rejected(
            Event::PlayPuppetsOnAString { player: p("Carol") },
            "Only a player that has passed",
        )
        .apply(Event::PlayCrisis { player: p("Carol") });
    g.turn("Carol").play_card(StrategyCard::Politics);
    g.turn("Alice").play_card(StrategyCard::Construction);
    g.turn("Bob").play_card(StrategyCard::Diplomacy);
    g.turn("Carol").play_card(StrategyCard::Warfare);
    g.turn("Alice").pass();

    g.turn("Bob")
        .strategic(StrategyCard::Trade)
        .commit()
        .apply(Event::PlayPuppetsOnAString { player: p("Alice") });
    assert_eq!(g.state().interrupted_turn, Some(p("Carol")));

    g.turn("Alice")
        .expect_rejected(Event::PassAction { player: p("Alice") }, "already passed")
        .tactical()
        .commit()
        .expect_rejected(
            Event::TakeAnotherTurn { player: p("Alice") },
            "cannot take another turn",
        )
        .expect_rejected(
            Event::PlayPuppetsOnAString { player: p("Alice") },
            "already performing an action",
        )
        .end_turn();
    assert_eq!(g.state().interrupted_turn, None);

    g.turn("Carol").pass();
    g.turn("Bob")
        .pass_and_stay()
        .expect_rejected(
            Event::PlayCrisis { player: p("Alice") },
            "at least 2 players",
        )
        .apply(Event::PlayPuppetsOnAString { player: p("Carol") });
    assert_eq!(g.state().interrupted_turn, None);

    g.turn("Carol").tactical().commit().end_turn();
    assert_eq!(g.phase(), Phase::Status);
    g
}
