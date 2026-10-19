//! The status phase (scoring and revealing objectives) and the relics phase.

use ti_helper_system_tests::prelude::*;

use crate::fixtures::{action_phase, three_players};

scenarios![
    status_scoring_and_reveal,
    status_stage_two_unlock,
    status_relics_phase_maw_and_crown,
];

/// Every player makes a public and a secret scoring decision before an objective is revealed.
pub fn status_scoring_and_reveal() -> TestGame {
    let mut g = action_phase(three_players());
    g.play_out_action_phase();

    g.expect_rejected(
        Event::RevealPublicObjective {
            objective: PublicObjective::ExpandBorders,
        },
        "until all players have finished",
    )
    .expect_rejected(
        Event::RevealPublicObjective {
            objective: PublicObjective::CentralizeGalacticTrade,
        },
        "Haven't finished revealing stage I",
    )
    .expect_rejected(
        Event::RevealPublicObjective {
            objective: PublicObjective::CornerTheMarket,
        },
        "already been revealed",
    )
    .expect_rejected(
        Event::CompleteStatusPhase,
        "No public objective has been revealed",
    )
    .expect_rejected(
        Event::ScorePublicObjective {
            player: p("Alice"),
            objective: Some(PublicObjective::ErectAMonument.into()),
        },
        "can't score an unrevealed public objective",
    );

    g.status_phase()
        .public("Alice", PublicObjective::CornerTheMarket)
        .expect_rejected(
            Event::ScorePublicObjective {
                player: p("Alice"),
                objective: None,
            },
            "already taken a public objective scoring decision",
        )
        .secret("Bob", SecretObjective::BecomeTheGatekeeper)
        .expect_rejected(
            Event::ScoreSecretObjective {
                player: p("Carol"),
                objective: Some(SecretObjective::BecomeTheGatekeeper),
            },
            "already been scored",
        )
        .public("Carol", PublicObjective::CornerTheMarket)
        .reveal(PublicObjective::ExpandBorders)
        .complete();

    // Without custodians there is no agenda phase.
    assert_eq!(g.phase(), Phase::Strategy);
    assert_eq!(g.round(), 2);
    assert!(g.state().strategy_card_holders.is_empty());
    assert_eq!(g.turn_order(), ["Alice", "Bob", "Carol"]);
    assert_eq!(
        [g.points("Alice"), g.points("Bob"), g.points("Carol")],
        [1, 1, 1]
    );
    g
}

/// Stage II objectives are revealed once five objectives are out.
pub fn status_stage_two_unlock() -> TestGame {
    let mut g = action_phase(three_players());
    for objective in [
        PublicObjective::ErectAMonument,
        PublicObjective::ExpandBorders,
        PublicObjective::SwayTheCouncil,
    ] {
        g.apply(Event::RevealExtraPublicObjective { objective });
    }
    g.play_out_action_phase()
        .expect_rejected(
            Event::RevealPublicObjective {
                objective: PublicObjective::NegotiateTradeRoutes,
            },
            "Already revealed enough stage I objective",
        )
        .status_phase()
        .reveal(PublicObjective::CentralizeGalacticTrade)
        .complete();
    g
}

/// With custodians taken, Maw of Worlds or the Crown of Emphidia (with the Tomb) add a relics
/// phase between the status and agenda phases.
pub fn status_relics_phase_maw_and_crown() -> TestGame {
    let mut g = action_phase(three_players().pok());
    g.apply(Event::SetCustodians {
        player: Some(p("Alice")),
    })
    .apply(Event::SetPlanetOwner {
        player: Some(p("Bob")),
        planet: Planet::Centauri,
    })
    .apply(Event::AddPlanetAttachment {
        player: p("Bob"),
        planet: Planet::Centauri,
        attachment: PlanetAttachment::TombOfEmphidia,
    });
    g.turn("Alice").gain_relic(Relic::MawOfWorlds).end_turn();
    g.turn("Bob")
        .gain_relic(Relic::TheCrownOfEmphidia)
        .end_turn();
    g.expect_rejected(
        Event::PlayCrownOfEmphidia { player: p("Bob") },
        "expected Relics, was Action",
    );

    g.play_out_action_phase()
        .status_phase()
        .reveal(PublicObjective::ExpandBorders)
        .complete();
    assert_eq!(g.phase(), Phase::Relics);

    g.expect_rejected(
        Event::PlayCrownOfEmphidia { player: p("Carol") },
        "doesn't have the Crown of emphidia",
    )
    .expect_rejected(
        Event::PlayMawOfWorlds {
            player: p("Carol"),
            tech: Technology::GravityDrive,
        },
        "doesn't have the Maw of Worlds",
    )
    .apply(Event::PlayMawOfWorlds {
        player: p("Alice"),
        tech: Technology::GravityDrive,
    })
    .apply(Event::PlayCrownOfEmphidia { player: p("Bob") })
    .apply(Event::CompleteRelicsPhase);

    assert_eq!(g.phase(), Phase::Agenda);
    assert!(
        g.player("Alice")
            .technologies
            .contains(&Technology::GravityDrive)
    );
    assert!(g.player("Alice").relics.is_empty());
    assert!(g.player("Bob").relics.is_empty());
    assert_eq!(g.state().score.crown_of_emphidia, Some(p("Bob")));
    assert_eq!(g.points("Bob"), 1);
    g
}
