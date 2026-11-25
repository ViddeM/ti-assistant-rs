//! Agenda phase events, and agendas added from the admin page.

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{
        agenda::{Agenda, AgendaElect, AgendaElectKind, AgendaKind},
        phase::Phase,
        relic::Relic,
    },
    state::{
        agenda::{AgendaRound, MinisterOfAntiquesProgress, Vote, VoteState},
        game_state::{AgendaOverrideState, GameState},
    },
};

/// Handles `Event::RevealAgenda`.
pub(super) fn reveal_agenda(game_state: &mut GameState, agenda: Agenda) -> Result<()> {
    game_state.assert_phase(Phase::Agenda)?;
    game_state.assert_enabled(&agenda)?;
    let vote = VoteState::new(agenda, game_state)?;
    let Some(state) = &mut game_state.agenda else {
        bail!("agenda state not initialized, this is a bug.");
    };
    ensure!(
        state.round < AgendaRound::Completed,
        "there are only 2 rounds of agenda"
    );
    ensure!(state.vote.is_none(), "an agenda is already revealed");

    state.vote = Some(vote);
    Ok(())
}

/// Handles `Event::CastAgendaVote`.
pub(super) fn cast_vote(
    game_state: &mut GameState,
    player: PlayerId,
    outcome: Option<AgendaElect>,
    votes: u16,
) -> Result<()> {
    game_state.assert_phase(Phase::Agenda)?;

    game_state
        .players
        .get(&player)
        .context("Player doesn't exist?")?;
    if let Some(restriction) = game_state.vote_restriction(&player) {
        bail!(restriction.reason());
    }

    let Some(state) = &mut game_state.agenda else {
        bail!("agenda state not initialized, this is a bug.");
    };

    let Some(vote) = &mut state.vote else {
        bail!("no agenda has been revealed yet");
    };

    if let Some(outcome) = outcome {
        let kind = AgendaElectKind::from(&outcome);
        ensure!(
            kind == vote.elect,
            "invalid vote kind, expected {:?}",
            vote.elect
        );

        vote.player_votes
            .insert(player, Some(Vote::new(votes, outcome)));
    } else {
        vote.player_votes.insert(player, None);
    }
    vote.tally_votes();
    Ok(())
}

/// Handles `Event::ResolveAgenda`.
pub(super) fn resolve_agenda(
    game_state: &mut GameState,
    outcome: Option<AgendaElect>,
) -> Result<()> {
    game_state.assert_phase(Phase::Agenda)?;
    let Some(state) = &mut game_state.agenda else {
        bail!("agenda state not initialized, this is a bug.");
    };

    let Some(vote) = state.vote.as_ref() else {
        bail!("no agenda has been revealed yet");
    };

    let vote = vote.clone();
    let minister_of_antiques = match (&vote.agenda, &outcome) {
        (Agenda::MinisterOfAntiques, Some(AgendaElect::Player(player))) => {
            Some(MinisterOfAntiquesProgress::GainRelic {
                player: player.clone(),
            })
        }
        _ => None,
    };

    game_state
        .resolve_agenda(vote, outcome)
        .context("Failed to resolve agenda")?;

    let Some(state) = &mut game_state.agenda else {
        bail!("agenda state not initialized, this is a bug.");
    };

    state.vote = None;
    state.round = state.round.next();
    if minister_of_antiques.is_some() {
        game_state.phase = Phase::AgendaEffect;
    }
    state.minister_of_antiques = minister_of_antiques;
    Ok(())
}

/// Handles `Event::CompleteAgendaPhase`.
pub(super) fn complete_agenda_phase(
    game_state: &mut GameState,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::Agenda)?;
    let Some(state) = &mut game_state.agenda else {
        bail!("agenda state not initialized, this is a bug.");
    };
    ensure!(
        state.round == AgendaRound::Completed,
        "need to complete 2 agenda rounds first"
    );

    game_state.change_phase(Phase::Strategy, timestamp)?;
    game_state.agenda = None;
    Ok(())
}

/// Handles `Event::RepealLaw`.
pub(super) fn repeal_law(game_state: &mut GameState, law: Agenda) -> Result<()> {
    if law.info().kind != AgendaKind::Law {
        bail!("Cannot repeal non-law agenda from list of laws");
    }

    if !game_state.laws.contains_key(&law) {
        bail!("Unable to repeal law that has not been enacted");
    }

    game_state.score.handle_law_repealed(&law);
    game_state.laws.remove(&law);
    Ok(())
}

/// Handles `Event::AddAgendaBegin`.
pub(super) fn add_agenda_begin(game_state: &mut GameState, agenda: Agenda) -> Result<()> {
    game_state.assert_enabled(&agenda)?;
    ensure!(
        game_state.agenda_override_state.is_none(),
        "There is already an agenda override in progress, cancel or resolve that before starting a new one."
    );

    game_state.agenda_override_state = Some(AgendaOverrideState {
        vote_state: VoteState::new(agenda, game_state).context("Failed to create vote state")?,
    });
    Ok(())
}

/// Handles `Event::AddAgendaPlayerVote`.
pub(super) fn add_agenda_player_vote(
    game_state: &mut GameState,
    player: PlayerId,
    outcome: AgendaElect,
    votes: u16,
) -> Result<()> {
    game_state
        .players
        .get(&player)
        .context("Player doesn't exist?")?;
    if let Some(restriction) = game_state.vote_restriction(&player) {
        bail!(restriction.reason());
    }

    let Some(agenda_override_state) = game_state.agenda_override_state.as_mut() else {
        bail!("No agenda override is in progress");
    };

    ensure!(
        !agenda_override_state
            .vote_state
            .player_votes
            .contains_key(&player),
        "Votes have already been cast by this player for this agenda"
    );

    let kind = AgendaElectKind::from(&outcome);
    ensure!(
        kind == agenda_override_state.vote_state.elect,
        "Invalid vote kind, expected {:?}",
        agenda_override_state.vote_state.elect
    );

    agenda_override_state
        .vote_state
        .player_votes
        .insert(player, Some(Vote::new(votes, outcome)));

    agenda_override_state.vote_state.tally_votes();
    Ok(())
}

/// Handles `Event::AddAgendaResolve`.
pub(super) fn add_agenda_resolve(
    game_state: &mut GameState,
    elected_outcome: AgendaElect,
) -> Result<()> {
    let Some(agenda_override_state) = game_state.agenda_override_state.as_mut() else {
        bail!("No agenda override is in progress");
    };

    let vote_state = agenda_override_state.vote_state.clone();

    game_state
        .resolve_agenda(vote_state, Some(elected_outcome))
        .context("Failed to resolve agenda")?;
    game_state.agenda_override_state = None;
    Ok(())
}

/// Handles `Event::GainMinisterOfAntiquesRelic`.
pub(super) fn gain_minister_of_antiques_relic(
    game_state: &mut GameState,
    player: PlayerId,
    relic: Option<Relic>,
) -> Result<()> {
    game_state.assert_phase(Phase::AgendaEffect)?;
    ensure!(
        matches!(
            game_state.agenda.as_ref().and_then(|agenda| agenda.minister_of_antiques.as_ref()),
            Some(MinisterOfAntiquesProgress::GainRelic { player: p }) if p == &player
        ),
        "{player} has no Minister of Antiques relic to gain"
    );

    if let Some(relic) = &relic {
        game_state.assert_enabled(relic)?;
        ensure!(
            !game_state
                .players
                .values()
                .any(|p| p.relics.contains(relic)),
            "Relic {relic:?} is already owned by another player."
        );
    }

    let next = (relic == Some(Relic::BookOfLatvinia)).then(|| {
        MinisterOfAntiquesProgress::BookOfLatviniaResearch {
            player: player.clone(),
        }
    });
    if next.is_none() {
        game_state.phase = Phase::Agenda;
    }
    game_state
        .agenda
        .as_mut()
        .context("agenda state not initialized, this is a bug.")?
        .minister_of_antiques = next;
    if let Some(relic) = relic {
        game_state.set_relic_owner(relic, Some(player))?;
    }
    Ok(())
}
