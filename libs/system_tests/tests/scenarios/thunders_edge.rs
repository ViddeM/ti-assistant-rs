//! Thunder's Edge: the expedition, The Obsidian, its planets and its action cards.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use ti_helper_game_data::{
    common::{expansions::Expansion, map::HexPosition, player_id::PlayerId},
    components::{
        breakthrough::Breakthrough,
        system::{SystemType, systems},
    },
    state::{
        faction_state::FirmamentSide,
        game_state::{
            ActionPhaseProgress, FractureProgress, FractureProgressState, PlanetLocation,
        },
    },
};
use ti_helper_system_tests::prelude::*;

use crate::fixtures::{
    action_phase, every_thunders_edge_tile_draft, five_player_draft, three_players,
};

scenarios![
    thunders_edge_commit_and_complete,
    thunders_edge_passing,
    thunders_edge_admin_flow,
    firmament_becomes_obsidian,
    thunders_edge_take_planets_without_tile,
    thunders_edge_styx_victory_point,
    thunders_edge_the_fracture_enters_play,
    thunders_edge_fracture_planet_relics,
    thunders_edge_setup_breakthroughs_roll_at_start,
    thunders_edge_breakthrough_planets,
    thunders_edge_action_cards,
    thunders_edge_overrule_and_strategize,
    thunders_edge_crisis_and_puppets_on_a_string,
    thunders_edge_quantumcore,
    thunders_edge_map_tiles,
    thunders_edge_breakthrough_actions,
    thunders_edge_vaults_of_the_heir,
    thunders_edge_visionaria_select,
    thunders_edge_entropic_scar,
    thunders_edge_warfare_primary_tactical_action,
    thunders_edge_overrule_warfare_tactical_action,
];

fn a_valid_system() -> SystemId {
    let mut systems: Vec<_> = System::valid_thunders_edge_systems().into_iter().collect();
    systems.sort();
    systems.remove(0)
}

#[test]
fn thunders_edge_can_be_placed_in_a_system_with_only_a_space_station() {
    let valid = System::valid_thunders_edge_systems();
    assert!(valid.contains("117"));
    assert!(!valid.contains("109"));
}

#[test]
fn thunders_edge_single_button_tech_action_ends_the_turn() {
    let mut g = action_phase(three_players().thunders_edge());
    let play = |player: &str, tech| Event::PlayTechAction {
        player: p(player),
        tech,
    };

    g.expect_rejected(
        play("Alice", Technology::Nanomachines),
        "Alice doesn't have Nanomachines",
    );
    g.apply(Event::AddTechToPlayer {
        player: p("Alice"),
        tech: Technology::Nanomachines,
    })
    .apply(Event::AddTechToPlayer {
        player: p("Alice"),
        tech: Technology::GravityDrive,
    });

    g.expect_rejected(
        play("Alice", Technology::GravityDrive),
        "doesn't have a single button ACTION",
    );
    g.expect_rejected(play("Bob", Technology::Nanomachines), "wrong players turn");

    g.apply(play("Alice", Technology::Nanomachines));
    assert_eq!(g.state().phase, Phase::EndActionTurn);
    g.expect_rejected(play("Alice", Technology::Nanomachines), "expected Action");
    g.apply(Event::EndTurn { player: p("Alice") });
    assert_eq!(g.current_player(), p("Bob"));
}

#[test]
fn coexistence_gives_deepwrought_up_to_five_oceans() {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::DeepwroughtScholarate)
            .player("Bob", Faction::ClanOfSaar)
            .player("Carol", Faction::XxchaKingdom)
            .pok()
            .thunders_edge(),
    );
    for planet in [
        Planet::Bellatrix,
        Planet::Tarana,
        Planet::Elnath,
        Planet::Capha,
        Planet::MecatolRexOmega,
        Planet::Industrex,
    ] {
        g.apply(Event::SetPlanetOwner {
            player: Some(p("Bob")),
            planet,
        });
    }
    let begin = |player: &str, planet| Event::CoexistenceBegin {
        player: p(player),
        planet,
    };
    let end = |player: &str, planet| Event::CoexistenceEnd {
        player: p(player),
        planet,
    };

    g.expect_rejected(
        begin("Alice", Planet::Ikatena),
        "Alice can't coexist on a planet they control",
    );
    g.expect_rejected(
        begin("Carol", Planet::Mirage),
        "isn't controlled by a player",
    );

    g.apply(begin("Carol", Planet::Bellatrix));
    assert_eq!(g.player("Carol").ocean_cards(), 0);
    g.apply(begin("Alice", Planet::Bellatrix));
    assert_eq!(g.player("Alice").ocean_cards(), 1);
    g.expect_rejected(
        begin("Alice", Planet::Bellatrix),
        "Alice is already coexisting on Bellatrix",
    );

    for planet in [
        Planet::Tarana,
        Planet::Elnath,
        Planet::Capha,
        Planet::MecatolRexOmega,
    ] {
        g.apply(begin("Alice", planet));
    }
    assert_eq!(g.player("Alice").ocean_cards(), 5);

    g.apply(begin("Alice", Planet::Industrex));
    assert_eq!(g.player("Alice").ocean_cards(), 5);
    assert_eq!(g.state().coexistences.len(), 7);

    g.expect_rejected(end("Alice", Planet::Ikatena), "isn't coexisting on Ikatena");
    g.apply(end("Alice", Planet::Bellatrix));
    assert_eq!(g.player("Alice").ocean_cards(), 5);
    g.apply(end("Alice", Planet::Tarana));
    assert_eq!(g.player("Alice").ocean_cards(), 4);
    assert_eq!(g.player("Carol").ocean_cards(), 0);
}

#[test]
fn helios_starts_on_ordinian_and_is_lost_with_the_planet() {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::LastBastion)
            .player("Bob", Faction::ClanOfSaar)
            .player("Carol", Faction::XxchaKingdom)
            .pok()
            .thunders_edge(),
    );
    let attach = |player: &str, planet, attachment| Event::AddPlanetAttachment {
        player: p(player),
        planet,
        attachment,
    };
    let remove = |player: &str, planet, attachment| Event::RemovePlanetAttachment {
        player: p(player),
        planet,
        attachment,
    };
    assert_eq!(
        g.state().players[&p("Alice")].planets[&Planet::Ordinian],
        BTreeSet::from([PlanetAttachment::HeliosV1])
    );

    g.expect_rejected(
        attach("Alice", Planet::Ordinian, PlanetAttachment::HeliosV2),
        "doesn't have the Helios V2 technology",
    );
    for planet in [Planet::Bellatrix, Planet::Tarana, Planet::Elnath] {
        g.apply(Event::SetPlanetOwner {
            player: Some(p("Alice")),
            planet,
        });
    }
    g.apply(attach(
        "Alice",
        Planet::Bellatrix,
        PlanetAttachment::HeliosV1,
    ));
    g.apply(attach("Alice", Planet::Tarana, PlanetAttachment::HeliosV1));
    g.expect_rejected(
        attach("Alice", Planet::Elnath, PlanetAttachment::HeliosV1),
        "at most 3 Helios",
    );
    g.expect_rejected(
        attach("Alice", Planet::Ordinian, PlanetAttachment::HeliosV1),
        "Planet already has a Helios",
    );

    for planet in [Planet::Ordinian, Planet::Bellatrix, Planet::Tarana] {
        g.apply(remove("Alice", planet, PlanetAttachment::HeliosV1));
    }
    g.apply(Event::AddTechToPlayer {
        player: p("Alice"),
        tech: Technology::HeliosV2,
    });
    g.apply(attach("Alice", Planet::Elnath, PlanetAttachment::HeliosV2));
    g.expect_rejected(
        attach("Alice", Planet::Bellatrix, PlanetAttachment::HeliosV1),
        "replaced by Helios V2",
    );

    g.apply(Event::SetPlanetOwner {
        player: Some(p("Bob")),
        planet: Planet::Elnath,
    });
    assert_eq!(
        g.state().players[&p("Bob")].planets[&Planet::Elnath],
        BTreeSet::new(),
        "Helios is lost when the planet changes owner"
    );
}

#[test]
fn helios_follows_the_v2_tech() {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::LastBastion)
            .player("Bob", Faction::ClanOfSaar)
            .player("Carol", Faction::XxchaKingdom)
            .pok()
            .thunders_edge(),
    );
    g.apply(Event::SetPlanetOwner {
        player: Some(p("Alice")),
        planet: Planet::Bellatrix,
    });
    g.apply(Event::AddPlanetAttachment {
        player: p("Alice"),
        planet: Planet::Bellatrix,
        attachment: PlanetAttachment::HeliosV1,
    });

    g.apply(Event::AddTechToPlayer {
        player: p("Alice"),
        tech: Technology::HeliosV2,
    });
    assert_eq!(
        g.state().players[&p("Alice")].planets[&Planet::Ordinian],
        BTreeSet::from([PlanetAttachment::HeliosV2])
    );
    assert_eq!(
        g.state().players[&p("Alice")].planets[&Planet::Bellatrix],
        BTreeSet::from([PlanetAttachment::HeliosV2])
    );

    g.apply(Event::RemoveTechFromPlayer {
        player: p("Alice"),
        tech: Technology::HeliosV2,
    });
    assert_eq!(
        g.state().players[&p("Alice")].planets[&Planet::Ordinian],
        BTreeSet::from([PlanetAttachment::HeliosV1])
    );
    assert_eq!(
        g.state().players[&p("Alice")].planets[&Planet::Bellatrix],
        BTreeSet::from([PlanetAttachment::HeliosV1])
    );
}

fn complete(player: &str, won_by: &str, placed_in: SystemId) -> Event {
    Event::CompleteThundersEdgeExpedition {
        player: p(player),
        placed_in,
        won_by: p(won_by),
    }
}

fn roll(player: &str, entered_play: bool) -> Event {
    Event::RollForTheFracture {
        player: p(player),
        entered_play,
    }
}

fn fracture_roll(g: &TestGame) -> Option<PlayerId> {
    match g.state().action_progress.as_ref()? {
        ActionPhaseProgress::Fracture(FractureProgressState {
            progress: FractureProgress::Roll(roll),
            ..
        }) => Some(roll.current_player().clone()),
        _ => None,
    }
}

/// Players claim slots at the end of their turns, the final claimer places Thunder's Edge.
pub fn thunders_edge_commit_and_complete() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());

    g.turn("Alice")
        .tactical()
        .commit()
        .commit_to_expedition(TEExpeditionSlot::Resources)
        .apply(roll("Alice", false))
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
        .apply(roll("Bob", false))
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
        .apply(roll("Carol", false))
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
        .apply(roll("Alice", false))
        .end_turn();
    assert!(g.state().passed_players.contains(&p("Alice")));

    g.turn("Bob").play_card(StrategyCard::Trade);
    g.turn("Carol").play_card(StrategyCard::Warfare);
    // Passing without committing is just ending the turn.
    g.turn("Bob").pass();
    g.turn("Carol").pass();

    assert_eq!(g.phase(), Phase::EntropicScar);
    g
}

/// Filling and completing the expedition from the admin page doesn't involve the players, and
/// gives no breakthroughs.
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
    assert!(g.state().players[&p("Alice")].breakthrough.is_none());
    assert!(g.state().players[&p("Bob")].breakthrough.is_none());
    assert_eq!(fracture_roll(&g), None);
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
    assert_eq!(fracture_roll(&g), Some(p("Alice")));
    assert_round_trips(&g);
    g.apply(roll("Alice", false));
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
        .commit_to_expedition(TEExpeditionSlot::TradeGoods)
        .apply(roll("Bob", false));
    let te = g.state().te_expedition.as_ref().unwrap();
    assert_eq!(te.claimed_by()[&TEExpeditionSlot::TradeGoods], p("Bob"));
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
    })
    .apply(roll("Alice", false));

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
    assert_eq!(g.phase(), Phase::EntropicScar);
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
    g.state()
        .action_progress
        .as_ref()
        .and_then(ActionPhaseProgress::tactical)
        .expect("no tactical action in progress")
        .activated_system
        .clone()
}

/// The Fracture planets are in their own systems, once the Fracture is in play. Planets without a
/// system tile can be taken once they are on the board: Thunder's Edge where the expedition
/// placed it, and Avernus once someone controls it. Custodia Vigilia only exists as a card.
/// Taking Thunder's Edge unlocks the breakthrough.
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
        .expect_rejected(take("Alice", Planet::Lethe), "isn't on the board")
        .apply(Event::SetFractureInPlay { in_play: true })
        .take(Planet::Lethe)
        .take(Planet::Phlegethon);
    assert_eq!(activated_system(&alice), Some("126".to_string()));
    alice
        .relic_for(Planet::Lethe, None)
        .relic_for(Planet::Phlegethon, None)
        .commit()
        .end_turn();

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

#[test]
fn thunders_edge_can_be_placed_with_a_space_station() {
    let mut g = action_phase(three_players().thunders_edge());
    g.apply(Event::SetThundersEdgeCompletion {
        new_completion: Some(TECompletion {
            won_by: p("Alice"),
            placed_in: "117".to_string(),
        }),
    });
    assert_eq!(
        g.state().planet_location(&Planet::ThundersEdge),
        PlanetLocation::Token(Some("117".to_string()))
    );
}

#[test]
fn thunders_edge_is_in_its_system_in_a_tactical_action() {
    for entered_play in [false, true] {
        let mut g = action_phase(three_players().thunders_edge());
        g.apply(Event::SetThundersEdgeCompletion {
            new_completion: Some(TECompletion {
                won_by: p("Bob"),
                placed_in: "117".to_string(),
            }),
        });

        let alice = g.turn("Alice").tactical().take(Planet::TheWatchtower);
        assert_eq!(activated_system(&alice), Some("117".to_string()));
        assert_eq!(
            alice.state().planets_in_system(&"117".to_string()),
            vec![Planet::TheWatchtower, Planet::ThundersEdge]
        );

        let alice = alice.take(Planet::ThundersEdge);
        assert_eq!(alice.state().phase, Phase::Fracture);
        assert_eq!(fracture_roll(&alice), Some(p("Alice")));
        let mut alice = alice.apply(roll("Alice", entered_play));
        if entered_play {
            alice = alice.apply(Event::FractureSpawned { player: p("Alice") });
        }

        assert_eq!(alice.state().phase, Phase::TacticalAction);
        assert_eq!(alice.state().fracture_in_play, entered_play);
        assert_eq!(activated_system(&alice), Some("117".to_string()));
        alice.commit().end_turn();

        let planets = &g.player("Alice").planets;
        assert!(planets.contains_key(&Planet::TheWatchtower));
        assert!(planets.contains_key(&Planet::ThundersEdge));
        assert!(g.player("Alice").breakthrough.is_some());
    }
}

/// Whoever controls Styx has 1 VP from it.
pub fn thunders_edge_styx_victory_point() -> TestGame {
    let mut g = action_phase(three_players().thunders_edge());
    let points = |g: &TestGame| [g.points("Alice"), g.points("Bob")];
    assert_eq!(points(&g), [0, 0]);

    g.apply(Event::SetFractureInPlay { in_play: true });

    g.turn("Alice")
        .tactical()
        .take(Planet::Styx)
        .relic_for(Planet::Styx, None)
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

/// The Fracture planets aren't on the board until the Fracture enters play. A player who gains
/// their breakthrough while it isn't in play must roll for it before anything else happens, and
/// it enters play on a 1 or 10. It can also be put in play from the admin page, and the Cabal's
/// Al'Raith Ix Ianovar puts it in play.
pub fn thunders_edge_the_fracture_enters_play() -> TestGame {
    let mut g = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::XxchaKingdom)
        .player("Dave", Faction::VuilRaithCabal)
        .pok()
        .codex()
        .thunders_edge()
        .start();
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
    let in_play = |g: &TestGame| g.state().fracture_in_play;
    let roll_first = "invalid game state, expected EndActionTurn, was Fracture";

    let alice = g
        .turn("Alice")
        .tactical()
        .expect_rejected(take("Alice", Planet::Cocytus), "isn't on the board")
        .commit()
        .commit_to_expedition(TEExpeditionSlot::Resources);
    assert_eq!(fracture_roll(&alice), Some(p("Alice")));
    alice
        .expect_rejected(Event::EndTurn { player: p("Alice") }, roll_first)
        .expect_rejected(
            roll("Bob", true),
            "Wrong player rolled, expected Alice got Bob",
        )
        .apply(roll("Alice", false))
        .end_turn();
    assert_eq!(fracture_roll(&g), None);
    assert!(!in_play(&g));

    g.expect_rejected(
        Event::SetFractureInPlay { in_play: false },
        "No change provided",
    );
    g.apply(Event::SetFractureInPlay { in_play: true });
    assert!(in_play(&g));
    g.apply(Event::SetFractureInPlay { in_play: false });
    assert!(!in_play(&g));

    let bob = g.turn("Bob").gain_relic(Relic::TheQuantumcore);
    assert_eq!(fracture_roll(&bob), Some(p("Bob")));
    bob.apply(roll("Bob", true))
        .apply(Event::FractureSpawned { player: p("Bob") })
        .end_turn();
    assert!(in_play(&g));
    assert_eq!(fracture_roll(&g), None);

    g.turn("Carol")
        .action_card(
            ActionCard::Brilliance,
            Some(ActionCardAction::Brilliance {
                player: Some(p("Carol")),
            }),
        )
        .end_turn();
    assert!(g.player("Carol").breakthrough.is_some());
    assert_eq!(fracture_roll(&g), None);

    g.apply(Event::SetFractureInPlay { in_play: false });
    g.apply(Event::SetBreakthrough {
        player: p("Dave"),
        breakthrough: Some(Breakthrough::AlRaithIxIanovar),
    });
    assert!(in_play(&g));
    assert_eq!(fracture_roll(&g), None);

    g.turn("Dave")
        .tactical()
        .take(Planet::Cocytus)
        .relic_for(Planet::Cocytus, None)
        .commit()
        .end_turn();
    assert!(g.player("Dave").planets.contains_key(&Planet::Cocytus));
    g
}

/// The first player to gain a planet in the Fracture gains a relic, with its on-gain effects,
/// before their tactical action can end. Gaining one that somebody has owned before gives no
/// relic and isn't explored, and planets given from the admin page never give a relic. Taking
/// legendary Styx from its owner takes Shard of the Throne with it.
pub fn thunders_edge_fracture_planet_relics() -> TestGame {
    let mut g = action_phase(three_players().pok().codex().thunders_edge());
    let gain = |player: &str, planet, relic| Event::TacticalActionGainRelic {
        player: p(player),
        planet,
        relic: Some(relic),
    };

    g.apply(Event::SetFractureInPlay { in_play: true })
        .apply(Event::SetPlanetOwner {
            player: Some(p("Carol")),
            planet: Planet::Lethe,
        })
        .apply(Event::SetPlanetOwner {
            player: Some(p("Carol")),
            planet: Planet::Centauri,
        })
        .apply(Event::SetPlanetOwner {
            player: None,
            planet: Planet::Lethe,
        })
        .apply(Event::SetPlanetOwner {
            player: None,
            planet: Planet::Centauri,
        });
    assert!(g.player("Carol").relics.is_empty());

    g.turn("Alice")
        .tactical()
        .take(Planet::Styx)
        .expect_rejected(
            Event::TacticalActionCommit { player: p("Alice") },
            "A relic must be gained for Styx first",
        )
        .expect_rejected(
            gain("Alice", Planet::Cocytus, Relic::ShardOfTheThrone),
            "Alice doesn't gain a relic for Cocytus",
        )
        .relic_for(Planet::Styx, Some(Relic::ShardOfTheThrone))
        .expect_rejected(
            gain("Alice", Planet::Styx, Relic::TheQuantumcore),
            "Alice doesn't gain a relic for Styx",
        )
        .commit()
        .end_turn();
    assert!(g.player("Alice").relics.contains(&Relic::ShardOfTheThrone));
    assert_eq!(g.points("Alice"), 2);

    g.turn("Bob")
        .tactical()
        .take(Planet::Styx)
        .expect_rejected(
            gain("Bob", Planet::Styx, Relic::TheQuantumcore),
            "Bob doesn't gain a relic for Styx",
        )
        .commit()
        .end_turn();
    assert_eq!(
        g.player("Bob").relics.iter().collect::<Vec<_>>(),
        [&Relic::ShardOfTheThrone]
    );
    assert_eq!([g.points("Alice"), g.points("Bob")], [0, 2]);

    g.turn("Carol")
        .tactical()
        .take(Planet::Lethe)
        .take(Planet::Centauri)
        .expect_rejected(
            gain("Carol", Planet::Lethe, Relic::TheQuantumcore),
            "Carol doesn't gain a relic for Lethe",
        )
        .expect_rejected(
            Event::TacticalActionAttachPlanetAttachment {
                player: p("Carol"),
                planet: Planet::Centauri,
                attachment: PlanetAttachment::ParadiseWorld,
            },
            "Centauri has been owned before, so it isn't explored",
        )
        .take(Planet::Phlegethon)
        .relic_for(Planet::Phlegethon, Some(Relic::BookOfLatvinia))
        .expect_rejected(
            Event::TacticalActionCommit { player: p("Carol") },
            "The Book of Latvinia must be researched with first",
        )
        .apply(Event::ResearchBookOfLatviniaTechs {
            player: p("Carol"),
            technologies: vec![Technology::SarweenTools],
        })
        .expect_rejected(
            Event::ResearchBookOfLatviniaTechs {
                player: p("Carol"),
                technologies: vec![],
            },
            "Carol has no Book of Latvinia research",
        )
        .commit()
        .end_turn();
    assert!(g.player("Carol").relics.contains(&Relic::BookOfLatvinia));
    assert!(g.player("Carol").has_tech(&Technology::SarweenTools));

    g.turn("Alice")
        .tactical()
        .take(Planet::Cocytus)
        .relic_for(Planet::Cocytus, Some(Relic::TheQuantumcore))
        .commit()
        .end_turn();
    assert!(g.player("Alice").relics.contains(&Relic::TheQuantumcore));
    assert!(g.player("Alice").breakthrough.is_some());
    g
}

/// Breakthroughs unlocked during setup don't roll straight away. When the game starts, every
/// player with a breakthrough rolls exactly once, one after another.
pub fn thunders_edge_setup_breakthroughs_roll_at_start() -> TestGame {
    let mut g = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::XxchaKingdom)
        .thunders_edge()
        .setup();

    g.apply(Event::SetBreakthrough {
        player: p("Bob"),
        breakthrough: Some(Breakthrough::Psychospore),
    })
    .apply(Event::SetBreakthrough {
        player: p("Carol"),
        breakthrough: Some(Breakthrough::DeorbitBarrage),
    });
    assert_eq!(g.phase(), Phase::Setup);
    assert_eq!(fracture_roll(&g), None);

    g.auto_setup().start_game();
    assert_eq!(g.phase(), Phase::Fracture);
    assert_eq!(fracture_roll(&g), Some(p("Bob")));
    g.apply(roll("Bob", false));
    assert_eq!(fracture_roll(&g), Some(p("Carol")));
    g.apply(roll("Carol", false));
    assert_eq!(fracture_roll(&g), None);
    assert_eq!(g.phase(), Phase::Strategy);
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
        .apply(roll("Alice", false))
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
    })
    .apply(roll("Bob", false));
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

/// Gaining The Quantumcore, as an action or by hand, unlocks the new owner's breakthrough. The
/// previous owner keeps theirs.
pub fn thunders_edge_quantumcore() -> TestGame {
    let mut g = action_phase(three_players().pok().codex().thunders_edge());

    g.turn("Alice")
        .gain_relic(Relic::TheQuantumcore)
        .apply(roll("Alice", false))
        .end_turn();
    assert_eq!(
        g.player("Alice").breakthrough,
        Some(Breakthrough::BellumGloriosum)
    );

    g.apply(Event::SetRelicOwner {
        relic: Relic::TheQuantumcore,
        player: Some(p("Bob")),
    })
    .apply(roll("Bob", false));
    assert!(!g.player("Alice").relics.contains(&Relic::TheQuantumcore));
    assert_eq!(
        g.player("Alice").breakthrough,
        Some(Breakthrough::BellumGloriosum)
    );
    assert_eq!(
        g.player("Bob").breakthrough,
        Some(Breakthrough::AutoFactories)
    );
    g
}

/// The Crimson Rebellion starts with its breakthrough unlocked, so it rolls for the Fracture
/// before the strategy phase can begin.
#[test]
fn the_crimson_rebellion_starts_with_its_breakthrough() {
    let mut g = TestGame::builder()
        .player("Alice", Faction::CrimsonRebellion)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::XxchaKingdom)
        .thunders_edge()
        .start();
    assert_eq!(
        g.player("Alice").breakthrough,
        Some(Breakthrough::ResonanceGenerator)
    );
    assert_eq!(g.player("Bob").breakthrough, None);
    assert_eq!(fracture_roll(&g), Some(p("Alice")));

    let take_leadership = Event::TakeStrategyCard {
        player: p("Alice"),
        card: StrategyCard::Leadership,
    };
    g.expect_rejected(
        take_leadership.clone(),
        "invalid game state, expected Strategy, was Fracture",
    );
    g.apply(roll("Alice", false));
    assert_eq!(fracture_roll(&g), None);
    g.apply(take_leadership);
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

/// Imports from before the Crimson Rebellion home system was placed have only its gate.
#[test]
fn imported_crimson_rebellion_gate_gets_its_home_system() {
    let mut draft = every_thunders_edge_tile_draft();
    draft.hex_map.tiles.retain(|tile| tile.system != "118");

    let g = TestGame::builder().milty(draft).create();

    let home_systems: Vec<HexPosition> = g
        .state()
        .map_data
        .milty_information
        .as_ref()
        .unwrap()
        .hex_map
        .tiles
        .iter()
        .filter(|tile| tile.system == "118")
        .map(|tile| tile.position.clone())
        .collect();
    assert_eq!(home_systems, vec![HexPosition::OutsideGalaxy]);
}

/// Imports from before the Fracture was placed don't have it.
#[test]
fn imported_thunders_edge_map_gets_the_fracture() {
    let mut draft = every_thunders_edge_tile_draft();
    draft
        .hex_map
        .tiles
        .retain(|tile| !["125", "126", "127"].contains(&tile.system.as_str()));

    let g = TestGame::builder().milty(draft).create();

    let fracture: Vec<(SystemId, HexPosition)> = g
        .state()
        .map_data
        .milty_information
        .as_ref()
        .unwrap()
        .hex_map
        .tiles
        .iter()
        .filter(|tile| ["125", "126", "127"].contains(&tile.system.as_str()))
        .map(|tile| (tile.system.clone(), tile.position.clone()))
        .collect();
    assert_eq!(
        fracture,
        vec![
            ("125".to_string(), HexPosition::OutsideGalaxy),
            ("126".to_string(), HexPosition::OutsideGalaxy),
            ("127".to_string(), HexPosition::OutsideGalaxy),
        ]
    );
}

/// Every Thunder's Edge system tile is on the map, each of their planets has an owner and
/// Thunder's Edge is placed in the entropic scar 114.
pub fn thunders_edge_map_tiles() -> TestGame {
    let players = ["Ann", "Ben", "Cid", "Dee", "Eve"];
    let mut g = TestGame::builder()
        .milty(every_thunders_edge_tile_draft())
        .create();
    g.finish_creation()
        .auto_setup()
        .start_game()
        .apply(roll("Cid", false));

    let systems = systems();
    let tiles = g
        .state()
        .map_data
        .milty_information
        .as_ref()
        .unwrap()
        .hex_map
        .tiles
        .clone();
    let planets = tiles
        .iter()
        .map(|tile| &systems[&tile.system])
        .filter(|system| system.expansion == Expansion::ThundersEdge)
        .filter(|system| !matches!(system.system_type, SystemType::HomeSystem(_)))
        .flat_map(|system| system.planets.clone());
    for (index, planet) in planets.enumerate() {
        g.apply(Event::SetPlanetOwner {
            player: Some(p(players[index % players.len()])),
            planet,
        });
    }

    g.apply(Event::SetThundersEdgeCompletion {
        new_completion: Some(TECompletion {
            won_by: p("Eve"),
            placed_in: "114".to_string(),
        }),
    });

    assert!(
        tiles
            .iter()
            .any(|tile| tile.system == "118" && tile.position == HexPosition::OutsideGalaxy)
    );
    assert_eq!(
        g.state().planet_location(&Planet::ThundersEdge),
        PlanetLocation::Token(Some("114".to_string()))
    );
    g
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

/// The primary of Warfare Ω is a tactical action, performed and ended with the strategic action.
pub fn thunders_edge_warfare_primary_tactical_action() -> TestGame {
    let mut g = action_phase(three_players().pok().thunders_edge());

    g.turn("Alice")
        .tactical()
        .take(Planet::Meer)
        .commit()
        .end_turn();
    g.turn("Bob").play_card(StrategyCard::Diplomacy);

    g.turn("Carol")
        .strategic(StrategyCard::Warfare)
        .take(Planet::Arinam)
        .attach(Planet::Arinam, PlanetAttachment::BioticResearchFacility)
        .take(Planet::Meer)
        .expect_rejected(
            Event::TacticalActionCommit { player: p("Carol") },
            "expected TacticalAction",
        )
        .expect_rejected(
            Event::StrategicActionPrimary {
                player: p("Carol"),
                action: StrategicPrimaryAction::Imperial {
                    score_objective: None,
                },
            },
            "already been performed",
        )
        .secondary("Alice", StrategicSecondaryAction::Warfare)
        .skip_rest()
        .commit();
    assert_eq!(g.phase(), Phase::EndActionTurn);

    let carol = g.player("Carol");
    assert!(carol.planets.contains_key(&Planet::Meer));
    assert!(carol.planets[&Planet::Arinam].contains(&PlanetAttachment::BioticResearchFacility));
    assert!(!g.player("Alice").planets.contains_key(&Planet::Meer));

    g.turn("Carol").end_turn();
    g
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
        .apply(roll("Bob", false))
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

/// Overrule on Warfare Ω performs its primary, a tactical action, and leaves Warfare readied.
pub fn thunders_edge_overrule_warfare_tactical_action() -> TestGame {
    let mut g = action_phase(three_players().pok().thunders_edge());

    g.turn("Alice").play_card(StrategyCard::Leadership);
    g.turn("Bob").play_card(StrategyCard::Diplomacy);

    g.turn("Carol")
        .action_card(
            ActionCard::Overrule,
            Some(ActionCardAction::Overrule {
                card: StrategyCard::Warfare,
                primary: None,
            }),
        )
        .tactical_in_progress()
        .take(Planet::Arinam)
        .commit();
    assert_eq!(g.phase(), Phase::EndActionTurn);
    assert!(g.player("Carol").planets.contains_key(&Planet::Arinam));
    assert!(
        !g.state()
            .spent_strategy_cards
            .contains(&StrategyCard::Warfare)
    );

    g.turn("Carol").end_turn();
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
    assert_eq!(g.phase(), Phase::EntropicScar);
    g
}

/// An ACTION breakthrough can only be played by the player who has it, on their turn, and it ends
/// the turn.
pub fn thunders_edge_breakthrough_actions() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::Arborec)
            .player("Bob", Faction::ClanOfSaar)
            .player("Carol", Faction::XxchaKingdom)
            .pok()
            .thunders_edge(),
    );
    let begin = |player: &str| Event::BreakthroughActionBegin { player: p(player) };
    let play = |player: &str, action| Event::BreakthroughActionCommit {
        player: p(player),
        action,
    };

    g.expect_rejected(
        begin("Alice"),
        "Alice doesn't have a breakthrough with an action",
    );
    g.apply(Event::SetBreakthrough {
        player: p("Alice"),
        breakthrough: Some(Breakthrough::Psychospore),
    })
    .apply(roll("Alice", false))
    .apply(Event::SetBreakthrough {
        player: p("Bob"),
        breakthrough: Some(Breakthrough::DeorbitBarrage),
    })
    .apply(roll("Bob", false));

    g.expect_rejected(begin("Bob"), "wrong players turn");
    let alice = g
        .turn("Alice")
        .expect_rejected(
            play("Alice", BreakthroughAction::Psychospore),
            "expected BreakthroughAction",
        )
        .apply(begin("Alice"));
    assert_eq!(alice.phase(), Phase::BreakthroughAction);
    let alice = alice.apply(play("Alice", BreakthroughAction::Psychospore));
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice
        .expect_rejected(begin("Alice"), "expected Action")
        .expect_rejected(
            play("Alice", BreakthroughAction::Psychospore),
            "expected BreakthroughAction",
        )
        .end_turn();

    g.turn("Bob")
        .apply(begin("Bob"))
        .expect_rejected(
            play("Bob", BreakthroughAction::Psychospore),
            "Bob doesn't have Psychospore",
        )
        .apply(play("Bob", BreakthroughAction::DeorbitBarrage))
        .end_turn();
    assert_eq!(g.current_player(), p("Carol"));
    g
}

/// Vaults of the Heir purges one of the player's technologies to gain a relic nobody owns.
pub fn thunders_edge_vaults_of_the_heir() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::MahactGeneSorcerers)
            .player("Bob", Faction::EmiratesOfHacan)
            .player("Carol", Faction::XxchaKingdom)
            .pok()
            .thunders_edge(),
    );
    let vaults = |purged_tech, relic| Event::BreakthroughActionCommit {
        player: p("Alice"),
        action: BreakthroughAction::VaultsOfTheHeir { purged_tech, relic },
    };

    g.apply(Event::SetBreakthrough {
        player: p("Alice"),
        breakthrough: Some(Breakthrough::VaultsOfTheHeir),
    })
    .apply(roll("Alice", false))
    .apply(Event::SetRelicOwner {
        relic: Relic::TheCodex,
        player: Some(p("Bob")),
    });

    let alice = g
        .turn("Alice")
        .apply(Event::BreakthroughActionBegin { player: p("Alice") })
        .expect_rejected(
            vaults(Technology::GravityDrive, Relic::ShardOfTheThrone),
            "Alice doesn't own Gravity Drive",
        )
        .expect_rejected(
            vaults(Technology::BioStims, Relic::TheCodex),
            "already owned",
        )
        .apply(vaults(Technology::BioStims, Relic::ShardOfTheThrone));
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice.end_turn();

    let alice = g.player("Alice");
    assert!(!alice.technologies().contains(&Technology::BioStims));
    assert!(
        alice
            .technologies()
            .contains(&Technology::PredictiveIntelligence)
    );
    assert!(alice.relics.contains(&Relic::ShardOfTheThrone));
    assert!(g.player("Bob").relics.contains(&Relic::TheCodex));
    assert_eq!(g.points("Alice"), 1);
    g
}

/// Visionaria Select lets each other player who pays research a non-faction, non-unit upgrade
/// technology, which the Deepwrought player gains as well. Nobody paying just ends the turn.
pub fn thunders_edge_visionaria_select() -> TestGame {
    let mut g = action_phase(
        TestGame::builder()
            .player("Alice", Faction::DeepwroughtScholarate)
            .player("Bob", Faction::MahactGeneSorcerers)
            .player("Carol", Faction::Arborec)
            .pok()
            .thunders_edge(),
    );
    for (player, breakthrough) in [
        ("Alice", Breakthrough::VisionariaSelect),
        ("Bob", Breakthrough::VaultsOfTheHeir),
        ("Carol", Breakthrough::Psychospore),
    ] {
        g.apply(Event::SetBreakthrough {
            player: p(player),
            breakthrough: Some(breakthrough),
        })
        .apply(roll(player, false));
    }
    let visionaria = |researched: &[(&str, Technology)]| Event::BreakthroughActionCommit {
        player: p("Alice"),
        action: BreakthroughAction::VisionariaSelect {
            researched: researched
                .iter()
                .map(|(player, tech)| (p(player), tech.clone()))
                .collect(),
        },
    };

    let alice = g
        .turn("Alice")
        .apply(Event::BreakthroughActionBegin { player: p("Alice") })
        .expect_rejected(
            visionaria(&[("Alice", Technology::GravityDrive)]),
            "Alice can't research with their own Visionaria Select",
        )
        .expect_rejected(
            visionaria(&[
                ("Bob", Technology::GravityDrive),
                ("Carol", Technology::CarrierII),
            ]),
            "Carrier II is a unit upgrade",
        )
        .expect_rejected(
            visionaria(&[("Bob", Technology::GeneticRecombination)]),
            "Genetic Recombination is a faction technology",
        )
        .expect_rejected(
            visionaria(&[("Bob", Technology::BioStims)]),
            "already has tech",
        );
    assert!(
        !alice
            .player("Bob")
            .technologies()
            .contains(&Technology::GravityDrive)
    );
    let alice = alice.apply(visionaria(&[
        ("Bob", Technology::GravityDrive),
        ("Carol", Technology::GravityDrive),
    ]));
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice.end_turn();
    for player in ["Alice", "Bob", "Carol"] {
        assert!(
            g.player(player)
                .technologies()
                .contains(&Technology::GravityDrive)
        );
    }

    g.turn("Bob").tactical().commit().end_turn();
    g.turn("Carol").tactical().commit().end_turn();
    let technologies = |g: &TestGame| {
        g.state()
            .players
            .values()
            .map(|p| p.technologies().clone())
            .collect::<Vec<_>>()
    };
    let before = technologies(&g);
    let alice = g
        .turn("Alice")
        .breakthrough(BreakthroughAction::VisionariaSelect {
            researched: BTreeMap::new(),
        });
    assert_eq!(alice.phase(), Phase::EndActionTurn);
    alice.end_turn();
    assert_eq!(technologies(&g), before);
    g
}

fn gain_scar_tech(player: &str, tech: Technology) -> Event {
    Event::GainEntropicScarTech {
        player: p(player),
        tech,
    }
}

/// In the entropic scar phase before the status phase, each player with ships in an entropic scar
/// may gain one of their own faction technologies, without prerequisites.
pub fn thunders_edge_entropic_scar() -> TestGame {
    let mut g = TestGame::builder()
        .milty(every_thunders_edge_tile_draft())
        .setup_techs(
            "Ben",
            [Technology::NeuralMotivator, Technology::RadicalAdvancement],
        )
        .create();
    g.finish_creation()
        .auto_setup()
        .start_game()
        .apply(roll("Cid", false));
    g.strategy_phase(&[
        ("Ann", StrategyCard::Leadership),
        ("Ben", StrategyCard::Diplomacy),
        ("Cid", StrategyCard::Politics),
        ("Dee", StrategyCard::Construction),
        ("Eve", StrategyCard::Trade),
    ]);
    g.expect_rejected(
        gain_scar_tech("Ben", Technology::HydrothermalMining),
        "expected EntropicScar",
    )
    .play_out_action_phase();
    assert_eq!(g.phase(), Phase::EntropicScar);

    g.expect_rejected(
        gain_scar_tech("Ben", Technology::NeuralParasite),
        "isn't one of",
    )
    .expect_rejected(
        gain_scar_tech("Ben", Technology::SarweenTools),
        "isn't one of",
    )
    .expect_rejected(
        gain_scar_tech("Ben", Technology::RadicalAdvancement),
        "already has tech",
    )
    .apply(gain_scar_tech("Ben", Technology::HydrothermalMining))
    .apply(gain_scar_tech("Ann", Technology::NeuralParasite))
    .expect_rejected(
        gain_scar_tech("Ann", Technology::PlaneSplitter),
        "already gained",
    )
    .apply(Event::CompleteEntropicScarPhase);

    let status = g
        .status_phase()
        .public("Dee", PublicObjective::CornerTheMarket)
        .expect_rejected(
            gain_scar_tech("Dee", Technology::Nanomachines),
            "expected EntropicScar",
        );
    assert_eq!(
        status
            .state()
            .status_phase_state
            .as_ref()
            .unwrap()
            .entropic_scar_techs,
        HashMap::from([
            (p("Ben"), Technology::HydrothermalMining),
            (p("Ann"), Technology::NeuralParasite),
        ])
    );
    status.reveal(PublicObjective::ExpandBorders).complete();

    assert!(g.player("Ben").has_tech(&Technology::HydrothermalMining));
    assert!(g.player("Ann").has_tech(&Technology::NeuralParasite));
    assert!(!g.player("Dee").has_tech(&Technology::Nanomachines));
    g
}

#[test]
fn entropic_scar_needs_a_scar_on_the_map() {
    let mut g = TestGame::builder().milty(five_player_draft()).start();
    g.strategy_phase(&[
        ("Ben", StrategyCard::Leadership),
        ("Cid", StrategyCard::Diplomacy),
        ("Ann", StrategyCard::Politics),
        ("Dee", StrategyCard::Construction),
        ("Eve", StrategyCard::Trade),
    ])
    .play_out_action_phase();
    assert_eq!(g.phase(), Phase::Status);
    g.expect_rejected(
        gain_scar_tech("Dee", Technology::HydrothermalMining),
        "expected EntropicScar",
    );
}

#[test]
fn entropic_scar_without_a_map() {
    let mut g = action_phase(three_players().pok().codex().thunders_edge());
    g.play_out_action_phase();
    assert_eq!(g.phase(), Phase::EntropicScar);
    g.apply(gain_scar_tech("Alice", Technology::AdvancedCarrierII));
    assert!(g.player("Alice").has_tech(&Technology::AdvancedCarrierII));

    let mut g = action_phase(three_players().pok().codex());
    g.play_out_action_phase();
    assert_eq!(g.phase(), Phase::Status);
    g.expect_rejected(
        gain_scar_tech("Alice", Technology::AdvancedCarrierII),
        "expected EntropicScar",
    );
}
