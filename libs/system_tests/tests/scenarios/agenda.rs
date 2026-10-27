//! The agenda phase: voting, laws and directives, and agendas outside the agenda phase.

use ti_helper_system_tests::{TestGameBuilder, prelude::*};

use crate::fixtures::{SIMPLE_PICKS, action_phase, agenda_phase, three_players};

scenarios![
    agenda_laws_enact_and_repeal,
    agenda_veto_against_and_discard,
    agenda_mutiny_and_seed_for,
    agenda_mutiny_and_seed_against,
    agenda_elect_law_and_secret_objective,
    agenda_override_outside_agenda_phase,
    agenda_elect_planet_with_trait,
];

fn vote(player: &str, outcome: AgendaElect, votes: u16) -> Event {
    Event::CastAgendaVote {
        player: p(player),
        outcome: Some(outcome),
        votes,
    }
}

/// Laws stay in play (and keep giving points) until repealed.
pub fn agenda_laws_enact_and_repeal() -> TestGame {
    let mut g = agenda_phase(three_players().pok());
    // The speaker votes last.
    assert_eq!(g.turn_order(), ["Bob", "Carol", "Alice"]);

    g.expect_rejected(vote("Bob", FOR, 1), "no agenda has been revealed")
        .expect_rejected(
            Event::ResolveAgenda { outcome: None },
            "no agenda has been revealed",
        );

    g.agenda(Agenda::ConventionsOfWar)
        .vote_all(FOR, 2)
        .resolve_expected();
    assert_eq!(g.state().laws.get(&Agenda::ConventionsOfWar), Some(&FOR));

    g.agenda(Agenda::PoliticalCensure)
        .vote("Bob", elect("Carol"), 3)
        .vote("Carol", elect("Carol"), 1)
        .vote("Alice", elect("Alice"), 2)
        .resolve_expected();
    assert_eq!(g.points("Carol"), 1);

    g.complete_agenda_phase();
    assert_eq!((g.phase(), g.round()), (Phase::Strategy, 2));

    g.expect_rejected(
        Event::RepealLaw {
            law: Agenda::Mutiny,
        },
        "Cannot repeal non-law",
    )
    .expect_rejected(
        Event::RepealLaw {
            law: Agenda::MinisterOfWar,
        },
        "has not been enacted",
    )
    .apply(Event::RepealLaw {
        law: Agenda::PoliticalCensure,
    });
    assert_eq!(g.points("Carol"), 0);
    assert!(g.state().laws.contains_key(&Agenda::ConventionsOfWar));
    g
}

/// Vetoed agendas don't use up a round, discarded and rejected ones do.
pub fn agenda_veto_against_and_discard() -> TestGame {
    let mut g = agenda_phase(three_players());
    g.expect_rejected(
        Event::CompleteAgendaPhase,
        "need to complete 2 agenda rounds",
    )
    .expect_rejected(Event::VetoAgenda, "no agenda is revealed");

    g.agenda(Agenda::FleetRegulations).veto();
    g.agenda(Agenda::ConventionsOfWar)
        .expect_rejected(
            Event::RevealAgenda {
                agenda: Agenda::Mutiny,
            },
            "an agenda is already revealed",
        )
        .vote_all(AGAINST, 1)
        .resolve(Some(AGAINST));
    assert!(g.state().laws.is_empty());

    g.agenda(Agenda::MinisterOfWar)
        .expect_rejected(vote("Bob", FOR, 1), "invalid vote kind")
        .expect_rejected(
            Event::ResolveAgenda { outcome: Some(FOR) },
            "invalid elect kind",
        )
        .resolve(None);
    assert!(g.state().laws.is_empty());
    assert_eq!(g.state().agenda_vote_history.len(), 2);

    g.expect_rejected(
        Event::RevealAgenda {
            agenda: Agenda::Mutiny,
        },
        "there are only 2 rounds of agenda",
    )
    .complete_agenda_phase();
    g
}

/// Sol, Hacan and Nekro (who can't vote). Alice holds custodians, Bob has 2 extra points.
fn mutiny_and_seed_game() -> TestGame {
    let builder: TestGameBuilder = TestGame::builder()
        .player("Alice", Faction::FederationOfSol)
        .player("Bob", Faction::EmiratesOfHacan)
        .player("Carol", Faction::NekroVirus);
    let mut g = agenda_phase(builder);
    g.apply(Event::AddExtraPoints {
        player: p("Bob"),
        value: 2,
    });
    g
}

/// Mutiny passes, so those who voted for it gain a point. Seed of an Empire then gives a point
/// to everyone tied for the most points.
pub fn agenda_mutiny_and_seed_for() -> TestGame {
    let mut g = mutiny_and_seed_game();
    g.agenda(Agenda::Mutiny)
        .expect_rejected(vote("Carol", FOR, 1), "Nekro Virus cannot vote")
        .vote("Alice", FOR, 3)
        .vote("Bob", AGAINST, 1)
        .resolve_expected();
    assert_eq!((g.points("Alice"), g.points("Bob")), (2, 2));

    g.agenda(Agenda::SeedOfAnEmpire)
        .vote_all(FOR, 1)
        .resolve(Some(FOR));
    assert_eq!(
        [g.points("Alice"), g.points("Bob"), g.points("Carol")],
        [3, 3, 0]
    );
    g
}

/// Mutiny fails, so those who voted for it lose a point. Seed of an Empire then gives a point
/// to everyone tied for the fewest points.
pub fn agenda_mutiny_and_seed_against() -> TestGame {
    let mut g = mutiny_and_seed_game();
    g.agenda(Agenda::Mutiny)
        .vote("Alice", FOR, 1)
        .vote("Bob", AGAINST, 3)
        .resolve_expected();
    assert_eq!(g.points("Alice"), 0);

    g.agenda(Agenda::SeedOfAnEmpire)
        .vote_all(AGAINST, 1)
        .resolve(Some(AGAINST));
    assert_eq!(
        [g.points("Alice"), g.points("Bob"), g.points("Carol")],
        [1, 2, 1]
    );
    g
}

/// Agendas electing a scored secret objective or a law in play.
pub fn agenda_elect_law_and_secret_objective() -> TestGame {
    let mut g = agenda_phase(three_players().pok());
    g.apply(Event::ScoreExtraSecretObjective {
        player: p("Carol"),
        objective: SecretObjective::BecomeTheGatekeeper,
    })
    .apply(Event::ScoreExtraSecretObjective {
        player: p("Bob"),
        objective: SecretObjective::AdaptNewStrategies,
    })
    .expect_rejected(
        Event::RevealAgenda {
            agenda: Agenda::JudicialAbolishment,
        },
        "no candidates found",
    );

    g.agenda(Agenda::ConventionsOfWar)
        .vote_all(FOR, 1)
        .resolve_expected();
    g.agenda(Agenda::ClassifiedDocumentLeaks)
        .vote(
            "Bob",
            AgendaElect::SecretObjective(SecretObjective::BecomeTheGatekeeper),
            2,
        )
        .abstain("Carol")
        .abstain("Alice")
        .resolve_expected();
    g.complete_agenda_phase();

    // Classified Document Leaks delays stage II objectives by one.
    g.strategy_phase(SIMPLE_PICKS).play_out_action_phase();
    let status = g.state().status_phase_state.as_ref().unwrap();
    assert_eq!(status.expected_objectives_before_stage_two, 6);

    g.status_phase()
        .reveal(PublicObjective::ErectAMonument)
        .complete();
    g.agenda(Agenda::JudicialAbolishment)
        .vote_all(AgendaElect::Law(Agenda::ConventionsOfWar), 1)
        .resolve_expected();
    g
}

/// Agendas resolved outside of the agenda phase (e.g. through action cards).
pub fn agenda_override_outside_agenda_phase() -> TestGame {
    let mut g = action_phase(three_players());
    let vote = |player: &str, outcome: AgendaElect, votes: u16| Event::AddAgendaPlayerVote {
        player: p(player),
        outcome,
        votes,
    };

    g.expect_rejected(Event::AddAgendaCancel, "No agenda override is in progress")
        .apply(Event::AddAgendaBegin {
            agenda: Agenda::Mutiny,
        })
        .expect_rejected(
            Event::AddAgendaBegin {
                agenda: Agenda::Mutiny,
            },
            "already an agenda override in progress",
        )
        .apply(vote("Alice", FOR, 2))
        .expect_rejected(vote("Alice", AGAINST, 1), "already been cast")
        .expect_rejected(vote("Bob", elect("Bob"), 1), "Invalid vote kind")
        .apply(Event::AddAgendaCancel)
        .apply(Event::AddAgendaBegin {
            agenda: Agenda::MinisterOfWar,
        })
        .apply(vote("Bob", elect("Bob"), 2))
        .apply(Event::AddAgendaResolve {
            elected_outcome: elect("Bob"),
        });

    assert_eq!(g.phase(), Phase::Action);
    assert!(g.state().agenda_override_state.is_none());
    assert_eq!(
        g.state().laws.get(&Agenda::MinisterOfWar),
        Some(&elect("Bob"))
    );
    g
}

/// The candidates of the vote currently in progress.
fn candidates(g: &TestGame) -> Vec<AgendaElect> {
    g.state()
        .agenda
        .as_ref()
        .and_then(|a| a.vote.as_ref())
        .expect("no agenda is revealed")
        .candidates
        .clone()
}

/// Agendas that elect a planet with a trait only offer planets with that trait, including traits
/// gained from attachments, and the elected planet is recorded with the matching kind.
pub fn agenda_elect_planet_with_trait() -> TestGame {
    let mut g = agenda_phase(three_players());
    g.apply(Event::SetPlanetOwner {
        player: Some(p("Alice")),
        planet: Planet::Quann,
    })
    .apply(Event::SetPlanetOwner {
        player: Some(p("Bob")),
        planet: Planet::Arnor,
    })
    .apply(Event::AddPlanetAttachment {
        player: p("Bob"),
        planet: Planet::Arnor,
        attachment: PlanetAttachment::Terraform,
    })
    .apply(Event::SetPlanetOwner {
        player: Some(p("Carol")),
        planet: Planet::Wellon,
    });
    let alice_points = g.points("Alice");

    // Arnor is industrial, Terraform makes it cultural as well.
    let quann = AgendaElect::CulturalPlanet(Planet::Quann);
    g.agenda(Agenda::HolyPlanetOfIxth).expect_rejected(
        vote("Bob", AgendaElect::Planet(Planet::Quann), 1),
        "invalid vote kind",
    );
    assert_eq!(
        candidates(&g),
        [AgendaElect::CulturalPlanet(Planet::Arnor), quann.clone()]
    );
    // The frontend sends the candidates in their string form.
    for candidate in candidates(&g) {
        assert_eq!(AgendaElect::parse(&candidate.name()).unwrap(), candidate);
    }
    g.apply(vote("Bob", quann.clone(), 1))
        .apply(vote("Carol", quann.clone(), 1))
        .apply(vote("Alice", quann.clone(), 1))
        .apply(Event::ResolveAgenda {
            outcome: Some(quann.clone()),
        });
    assert_eq!(g.state().laws.get(&Agenda::HolyPlanetOfIxth), Some(&quann));
    assert_eq!(g.points("Alice"), alice_points + 1);
    g.apply(Event::SetPlanetOwner {
        player: None,
        planet: Planet::Quann,
    });
    assert_eq!(g.points("Alice"), alice_points);

    let wellon = AgendaElect::IndustrialPlanet(Planet::Wellon);
    g.agenda(Agenda::ResearchTeamCybernetic);
    assert_eq!(
        candidates(&g),
        [AgendaElect::IndustrialPlanet(Planet::Arnor), wellon.clone()]
    );
    g.apply(vote("Bob", wellon.clone(), 2))
        .apply(Event::ResolveAgenda {
            outcome: Some(wellon.clone()),
        });
    assert_eq!(
        g.state().laws.get(&Agenda::ResearchTeamCybernetic),
        Some(&wellon)
    );
    g
}
