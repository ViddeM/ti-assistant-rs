//! Manual adjustments to score, planets and techs, which are allowed in any phase.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{SIMPLE_PICKS, three_players};

scenarios![
    score_manual_adjustments,
    score_manual_planets_and_techs,
    score_manual_relics,
];

/// Every source of points, adjusted by hand.
pub fn score_manual_adjustments() -> TestGame {
    let mut g = three_players().pok().start();
    let support = |giver: &str, receiver: &str| Event::GiveSupportForTheThrone {
        giver: p(giver),
        receiver: p(receiver),
    };

    g.apply(Event::SetCustodians {
        player: Some(p("Alice")),
    })
    .apply(support("Bob", "Alice"))
    .apply(support("Carol", "Alice"))
    // Giving Support for the Throne again moves it.
    .apply(support("Bob", "Carol"))
    .apply(Event::AddExtraPoints {
        player: p("Carol"),
        value: 2,
    })
    .apply(Event::AddExtraPoints {
        player: p("Carol"),
        value: -1,
    })
    .apply(Event::AddImperial {
        player: p("Bob"),
        value: 1,
    });

    let score_public =
        |player: &str, objective: PublicObjective| Event::ScoreExtraPublicObjective {
            player: p(player),
            objective: objective.into(),
        };
    g.apply(score_public("Alice", PublicObjective::CornerTheMarket))
        .expect_rejected(
            score_public("Alice", PublicObjective::CornerTheMarket),
            "can't score a public objective twice",
        )
        .expect_rejected(
            score_public("Alice", PublicObjective::ErectAMonument),
            "can't score an unrevealed public objective",
        )
        .apply(Event::RevealExtraPublicObjective {
            objective: PublicObjective::ErectAMonument,
        })
        .expect_rejected(
            Event::RevealExtraPublicObjective {
                objective: PublicObjective::ErectAMonument,
            },
            "already been revealed",
        )
        .apply(score_public("Bob", PublicObjective::ErectAMonument))
        .apply(Event::UnscoreObjective {
            player: p("Alice"),
            objective: PublicObjective::CornerTheMarket.into(),
        })
        .expect_rejected(
            Event::UnscoreObjective {
                player: p("Alice"),
                objective: PublicObjective::SwayTheCouncil.into(),
            },
            "hasn't been revealed",
        );

    let score_secret = |player: &str, objective| Event::ScoreExtraSecretObjective {
        player: p(player),
        objective,
    };
    g.apply(score_secret("Bob", SecretObjective::BecomeTheGatekeeper))
        .expect_rejected(
            score_secret("Carol", SecretObjective::BecomeTheGatekeeper),
            "already been scored",
        )
        .apply(score_secret("Carol", SecretObjective::AdaptNewStrategies))
        .apply(Event::UnscoreSecretObjective {
            player: p("Carol"),
            objective: SecretObjective::AdaptNewStrategies,
        })
        .expect_rejected(
            Event::UnscoreSecretObjective {
                player: p("Alice"),
                objective: SecretObjective::AdaptNewStrategies,
            },
            "Player not in secret objectives map",
        );

    g.apply(Event::SetShardForTheThroneOwner {
        player: Some(p("Carol")),
    })
    .apply(Event::SetShardForTheThroneOwner {
        player: Some(p("Bob")),
    })
    .expect_rejected(
        Event::SetShardForTheThroneOwner {
            player: Some(p("Nobody")),
        },
        "Player doesn't exist",
    )
    .apply(Event::SetCrownOfEmphidiaOwner {
        player: Some(p("Bob")),
    })
    .apply(Event::SetCrownOfEmphidiaOwner { player: None });

    // Alice: custodians and Carol's support for the throne.
    // Bob: imperial, Erect a Monument, a secret and the shard.
    // Carol: Bob's support for the throne and one extra point.
    assert_eq!(
        [g.points("Alice"), g.points("Bob"), g.points("Carol")],
        [2, 4, 2]
    );
    assert!(g.player("Bob").relics.contains(&Relic::ShardOfTheThrone));
    assert!(!g.player("Carol").relics.contains(&Relic::ShardOfTheThrone));
    g
}

/// Moving planets and attachments around, and adding or removing techs.
pub fn score_manual_planets_and_techs() -> TestGame {
    let mut g = three_players().pok().codex().start();
    let set_owner = |player: Option<&str>, planet: Planet| Event::SetPlanetOwner {
        player: player.map(p),
        planet,
    };
    let add = |planet: Planet, attachment: PlanetAttachment| Event::AddPlanetAttachment {
        player: p("Alice"),
        planet,
        attachment,
    };
    let remove = |planet: Planet, attachment: PlanetAttachment| Event::RemovePlanetAttachment {
        player: p("Alice"),
        planet,
        attachment,
    };

    // Whoever first gets Mecatol Rex gets the custodians, and keeps it.
    g.apply(set_owner(Some("Bob"), Planet::MecatolRex))
        .apply(set_owner(Some("Carol"), Planet::MecatolRex))
        .apply(set_owner(None, Planet::MecatolRex))
        .expect_rejected(
            set_owner(Some("Nobody"), Planet::Lodor),
            "Player does not exist",
        );
    assert_eq!(g.state().score.custodians, Some(p("Bob")));

    g.apply(set_owner(Some("Alice"), Planet::Elysium))
        .apply(set_owner(Some("Alice"), Planet::Lodor))
        .apply(set_owner(Some("Alice"), Planet::Primor))
        .expect_rejected(
            add(Planet::Lodor, PlanetAttachment::Geoform),
            "can only be used on Elysium",
        )
        .apply(add(Planet::Elysium, PlanetAttachment::Geoform))
        .expect_rejected(
            add(Planet::Elysium, PlanetAttachment::Geoform),
            "already has the attachment",
        )
        .expect_rejected(
            add(Planet::Jord, PlanetAttachment::Terraform),
            "Cannot terraform home planet",
        )
        .expect_rejected(
            add(Planet::MecatolRex, PlanetAttachment::Terraform),
            "Player doesn't own planet",
        )
        .apply(add(Planet::Lodor, PlanetAttachment::Terraform))
        .expect_rejected(
            add(Planet::Primor, PlanetAttachment::NanoForge),
            "Cannot place Nano Forge on legendary planet",
        )
        .expect_rejected(
            add(Planet::Elysium, PlanetAttachment::DysonSphere),
            "planet_trait requirement not met",
        )
        .apply(add(Planet::Lodor, PlanetAttachment::DysonSphere))
        .apply(remove(Planet::Lodor, PlanetAttachment::Terraform))
        .expect_rejected(
            remove(Planet::Lodor, PlanetAttachment::Terraform),
            "Planet doesn't have that attachment",
        );

    g.apply(Event::AddTechToPlayer {
        player: p("Carol"),
        tech: Technology::GravityDrive,
    })
    .apply(Event::RemoveTechFromPlayer {
        player: p("Carol"),
        tech: Technology::GravitonLaserSystem,
    })
    .expect_rejected(
        Event::AddTechToPlayer {
            player: p("Nobody"),
            tech: Technology::GravityDrive,
        },
        "Player does not exist",
    );

    assert_eq!(
        g.player("Alice").planets[&Planet::Lodor]
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
        [PlanetAttachment::DysonSphere]
    );
    assert_eq!(
        g.player("Carol")
            .technologies
            .iter()
            .cloned()
            .collect::<Vec<_>>(),
        [Technology::GravityDrive]
    );
    g
}

/// Giving relics to players, or taking them away, outside of an action.
pub fn score_manual_relics() -> TestGame {
    let mut g = three_players().pok().start();
    let set_owner = |relic: Relic, player: Option<&str>| Event::SetRelicOwner {
        relic,
        player: player.map(p),
    };

    g.apply(set_owner(Relic::StellarConverter, Some("Alice")))
        .apply(set_owner(Relic::StellarConverter, Some("Bob")));
    assert!(!g.player("Alice").relics.contains(&Relic::StellarConverter));
    assert!(g.player("Bob").relics.contains(&Relic::StellarConverter));

    g.apply(set_owner(Relic::ShardOfTheThrone, Some("Carol")));
    assert_eq!(g.points("Carol"), 1);
    assert_eq!(g.state().score.shard_of_the_throne, Some(p("Carol")));
    g.apply(set_owner(Relic::ShardOfTheThrone, None));
    assert_eq!(g.points("Carol"), 0);
    assert!(!g.player("Carol").relics.contains(&Relic::ShardOfTheThrone));

    g.expect_rejected(
        set_owner(Relic::TheCodex, Some("Nobody")),
        "Player doesn't exist",
    )
    .expect_rejected(
        set_owner(Relic::Neuraloop, Some("Alice")),
        "is not available with the enabled expansions",
    );

    g.strategy_phase(SIMPLE_PICKS);
    g.turn("Alice").expect_rejected(
        Event::GainRelicAction {
            player: p("Alice"),
            relic: Relic::StellarConverter,
        },
        "already owned by another player",
    );
    g
}
