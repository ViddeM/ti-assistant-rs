//! Status phase events.

use std::collections::BTreeSet;

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{
        objectives::Objective,
        phase::Phase,
        relic::Relic,
        tech::{TechOrigin, Technology},
    },
    state::game_state::GameState,
};

/// Handles `Event::CompleteStatusPhase`.
pub(super) fn complete_status_phase(
    game_state: &mut GameState,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::Status)?;
    // TODO: Require objectives scored & revealed

    let Some(state) = game_state.status_phase_state.as_ref() else {
        bail!("Expected status phase state");
    };

    ensure!(
        state.revealed_objective.is_some() || !game_state.status_phase_reveals_objective(),
        "No public objective has been revealed!"
    );
    ensure!(
        state.scored_public_objectives.len() == game_state.players.len(),
        "Not all players have made a decision regarding their public objectives"
    );
    ensure!(
        state.scored_secret_objectives.len() == game_state.players.len(),
        "Not all players have made a decision regarding their secret objectives"
    );

    // Reset state
    game_state.strategy_card_holders = Default::default();
    game_state.passed_players = Default::default();
    game_state.spent_strategy_cards = Default::default();

    let phase = get_phase_after_status(game_state);
    game_state.change_phase(phase, timestamp)?;
    Ok(())
}

/// Handles `Event::PlayNeuraloop`.
pub(super) fn play_neuraloop(
    game_state: &mut GameState,
    player: PlayerId,
    purged_relic: Relic,
    replacement: Objective,
) -> Result<()> {
    game_state.assert_phase(Phase::Status)?;
    game_state.assert_enabled(&Relic::Neuraloop)?;
    game_state.assert_enabled(&replacement)?;

    let Some(p) = game_state.players.get(&player) else {
        bail!("Player doesn't exist");
    };

    ensure!(
        p.relics.contains(&Relic::Neuraloop),
        "Player doesn't have the Neuraloop relic!"
    );
    ensure!(
        p.relics.contains(&purged_relic),
        "Player doesn't have the {purged_relic:?} relic to purge!"
    );

    let Some(status_phase_state) = game_state.status_phase_state.as_mut() else {
        bail!("Status phase state not set!")
    };

    let Some(revealed) = status_phase_state.revealed_objective else {
        bail!("Neuraloop can only be played after a public objective has been revealed");
    };

    ensure!(
        !game_state
            .score
            .revealed_objectives
            .contains_key(&replacement),
        "Objective has already been revealed!"
    );

    if let Objective::Secret(secret) = &replacement {
        ensure!(
            !game_state
                .score
                .secret_objectives
                .values()
                .any(|scored| scored.contains(secret)),
            "Secret objective has already been scored"
        );
    }

    game_state.score.revealed_objectives.remove(&revealed);
    game_state
        .score
        .revealed_objectives
        .insert(replacement, BTreeSet::new());
    status_phase_state.revealed_objective = Some(replacement);

    let Some(p) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist");
    };
    p.relics.remove(&purged_relic);
    if purged_relic == Relic::ShardOfTheThrone {
        game_state.score.shard_of_the_throne = None;
    }
    Ok(())
}

/// Handles `Event::GainEntropicScarTech`.
pub(super) fn gain_entropic_scar_tech(
    game_state: &mut GameState,
    player: PlayerId,
    tech: Technology,
) -> Result<()> {
    game_state.assert_phase(Phase::EntropicScar)?;
    game_state.assert_enabled(&tech)?;

    let Some(status_state) = game_state.status_phase_state.as_ref() else {
        bail!("Status phase state not set!")
    };
    ensure!(
        !status_state.entropic_scar_techs.contains_key(&player),
        "{player} has already gained a technology from an entropic scar this status phase"
    );

    let p = game_state
        .players
        .get(&player)
        .context("Player doesn't exist")?;
    ensure!(
        tech.info().origin == TechOrigin::Faction(p.faction),
        "{} isn't one of {}'s faction technologies",
        tech.info().name,
        p.faction.name()
    );
    game_state.gain_tech(&player, tech.clone())?;
    (game_state.status_phase_state.as_mut())
        .context("Status phase state not set!")?
        .entropic_scar_techs
        .insert(player, tech);
    Ok(())
}

/// Handles `Event::CompleteRelicsPhase`.
pub(super) fn complete_relics_phase(
    game_state: &mut GameState,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::Relics)?;
    let phase = if game_state.is_agenda_enabled() {
        Phase::Agenda
    } else {
        Phase::Strategy
    };
    game_state.change_phase(phase, timestamp)?;
    Ok(())
}

fn get_phase_after_status(game_state: &GameState) -> Phase {
    if game_state
        .players
        .values()
        .any(|p| p.relics.contains(&Relic::TheCrownOfEmphidia) && p.controls_tomb_of_emphidia())
    {
        return Phase::Relics;
    }

    if game_state.is_agenda_enabled() {
        return if game_state
            .players
            .values()
            .any(|p| p.relics.contains(&Relic::MawOfWorlds))
        {
            Phase::Relics
        } else {
            Phase::Agenda
        };
    }

    Phase::Strategy
}
