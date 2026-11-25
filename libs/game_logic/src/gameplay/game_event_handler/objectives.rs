//! Objective events.

use std::collections::BTreeSet;

use anyhow::{Result, bail, ensure};

use ti_helper_game_data::components::objectives::public::PublicObjective;
use ti_helper_game_data::components::objectives::secret::SecretObjective;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{
        objectives::{Objective, ObjectiveKind},
        phase::Phase,
    },
    state::game_state::GameState,
};

/// Handles `Event::ScorePublicObjective`.
pub(super) fn score_public(
    game_state: &mut GameState,
    player: PlayerId,
    objective: Option<Objective>,
) -> Result<()> {
    game_state.assert_phase(Phase::Status)?;

    if let Some(obj) = &objective {
        game_state.assert_enabled(obj)?;
    }

    let scores_any_number = game_state.status_phase_scores_any_number_of_public_objectives();

    let Some(status_state) = game_state.status_phase_state.as_mut() else {
        bail!("No status phase state!");
    };

    if let Some(scored) = status_state.scored_public_objectives.get(&player)
        && !(scores_any_number && objective.is_some() && !scored.is_empty())
    {
        bail!("Player has already taken a public objective scoring decision");
    }

    status_state
        .scored_public_objectives
        .entry(player.clone())
        .or_default()
        .extend(objective);

    if let Some(obj) = objective {
        ensure!(
            !game_state.is_former_public_objective(&obj),
            "{} is no longer a public objective",
            obj.info().name
        );
        let Some(scorers) = game_state.score.revealed_objectives.get_mut(&obj) else {
            bail!("can't score an unrevealed public objective");
        };

        if !scorers.insert(player) {
            bail!("can't score a public objective twice");
        }
    }
    Ok(())
}

/// Handles `Event::ScoreSecretObjective`.
pub(super) fn score_secret(
    game_state: &mut GameState,
    player: PlayerId,
    objective: Option<SecretObjective>,
) -> Result<()> {
    game_state.assert_phase(Phase::Status)?;

    if let Some(obj) = &objective {
        game_state.assert_enabled(&Objective::from(*obj))?;
    }

    let Some(status_state) = game_state.status_phase_state.as_mut() else {
        bail!("No status phase state?")
    };

    if status_state.scored_secret_objectives.contains_key(&player) {
        bail!("Player has already taken a secret objective scoring decision");
    }

    status_state
        .scored_secret_objectives
        .insert(player.clone(), objective);
    let all_decided = status_state.scored_secret_objectives.len() == game_state.players.len();

    if let Some(obj) = objective {
        for scored in game_state.score.secret_objectives.values() {
            if scored.contains(&obj) {
                bail!("secred objective has already been scored");
            }
        }

        ensure!(
            !game_state
                .score
                .revealed_objectives
                .contains_key(&Objective::Secret(obj)),
            "secret objective has been revealed as a public objective"
        );

        game_state
            .score
            .secret_objectives
            .entry(player)
            .or_default()
            .insert(obj);
    }

    if all_decided {
        let turn_order = game_state.turn_order.clone();
        game_state.gain_zealous_orthodoxy(&turn_order);
    }
    Ok(())
}

/// Handles `Event::RevealPublicObjective`.
pub(super) fn reveal_public(game_state: &mut GameState, objective: PublicObjective) -> Result<()> {
    game_state.assert_phase(Phase::Status)?;
    ensure!(
        game_state.status_phase_reveals_objective(),
        "Under Civilized Society all public objectives are revealed at setup"
    );

    let pub_obj = Objective::Public(objective);
    ensure!(
        !game_state.score.revealed_objectives.contains_key(&pub_obj),
        "Objective has already been revealed!"
    );

    game_state.assert_enabled(&pub_obj)?;

    let stage_two = game_state.status_phase_reveals_stage_two();
    let Some(status_phase_state) = game_state.status_phase_state.as_mut() else {
        bail!("Status phase state not set!")
    };

    match (pub_obj.info().kind, stage_two) {
        (ObjectiveKind::StageI, false) | (ObjectiveKind::StageII, true) => {}
        (ObjectiveKind::StageI, true) => {
            bail!("Already revealed enough stage I objective, expected stage II");
        }
        (ObjectiveKind::StageII, false) => {
            bail!("Haven't finished revealing stage I obejctives, cannot reveal stage II yet")
        }
        (ObjectiveKind::Secret { .. }, _) => {
            bail!("Cannot reveal secret objective as public objective")
        }
    }

    if !status_phase_state.can_reveal_objective(game_state.players.len()) {
        bail!("Cannot reveal objective until all players have finished scoring their objectives");
    }

    status_phase_state.revealed_objective = Some(pub_obj);

    game_state
        .score
        .revealed_objectives
        .insert(pub_obj, BTreeSet::new());
    Ok(())
}

/// Handles `Event::ScoreExtraPublicObjective`.
pub(super) fn score_extra_public(
    game_state: &mut GameState,
    player: PlayerId,
    objective: Objective,
) -> Result<()> {
    game_state.assert_enabled(&objective)?;

    ensure!(
        !game_state.is_former_public_objective(&objective),
        "{} is no longer a public objective",
        objective.info().name
    );
    let Some(scorers) = game_state.score.revealed_objectives.get_mut(&objective) else {
        bail!("can't score an unrevealed public objective");
    };

    if !scorers.insert(player) {
        bail!("can't score a public objective twice");
    }
    Ok(())
}

/// Handles `Event::ScoreExtraSecretObjective`.
pub(super) fn score_extra_secret(
    game_state: &mut GameState,
    player: PlayerId,
    objective: SecretObjective,
) -> Result<()> {
    game_state.assert_enabled(&Objective::from(objective))?;

    for scored in game_state.score.secret_objectives.values() {
        if scored.contains(&objective) {
            bail!("secred objective has already been scored");
        }
    }

    ensure!(
        !game_state
            .score
            .revealed_objectives
            .contains_key(&Objective::Secret(objective)),
        "secret objective has been revealed as a public objective"
    );

    game_state
        .score
        .secret_objectives
        .entry(player.clone())
        .or_default()
        .insert(objective);
    game_state.gain_zealous_orthodoxy(&[player]);
    Ok(())
}

/// Handles `Event::RevealExtraPublicObjective`.
pub(super) fn reveal_extra_public(
    game_state: &mut GameState,
    objective: PublicObjective,
) -> Result<()> {
    let pub_obj = Objective::Public(objective);
    ensure!(
        !game_state.score.revealed_objectives.contains_key(&pub_obj),
        "Objective has already been revealed!"
    );
    game_state.assert_enabled(&pub_obj)?;

    game_state
        .score
        .revealed_objectives
        .insert(pub_obj, BTreeSet::new());
    Ok(())
}
