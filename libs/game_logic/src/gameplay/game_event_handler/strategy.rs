//! Strategy phase and strategic action events.

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};

use super::{
    ensure_tactical_action_finished, perform_strategic_primary, perform_strategic_secondary,
};
use ti_helper_game_data::{
    actions::strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
    common::player_id::PlayerId,
    components::{phase::Phase, strategy_card::StrategyCard},
    state::game_state::{
        ActionPhaseProgress, GameState, StrategicPrimaryProgress, StrategicProgress,
    },
};

/// Handles `Event::TakeStrategyCard`.
pub(super) fn take_strategy_card(
    game_state: &mut GameState,
    player: PlayerId,
    card: StrategyCard,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::Strategy)?;
    ensure!(
        !game_state.strategy_card_holders.contains_key(&card),
        "strategy card can't be picked twice"
    );
    game_state.strategy_card_holders.insert(card, player);
    game_state.advance_turn(timestamp)?;
    Ok(())
}

/// Handles `Event::PlayGiftOfPrescience`.
pub(super) fn play_gift_of_prescience(game_state: &mut GameState, player: PlayerId) -> Result<()> {
    game_state.assert_phase(Phase::Strategy)?;
    ensure!(
        game_state.players.contains_key(&player),
        "unknown player: {player:?}",
    );
    game_state.naalu_telepathy = Some(player);
    Ok(())
}

/// Handles `Event::CompleteStrategyPhase`.
pub(super) fn complete_strategy_phase(
    game_state: &mut GameState,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    let how_many_card_must_pick = match game_state.players.len() {
        3 => 6,
        4 => 8,
        n => n,
    };
    if game_state.strategy_card_holders.len() != how_many_card_must_pick {
        anyhow::bail!(
            "can't complete strategy phase, all players have not selected strategy cards"
        );
    }
    game_state.change_phase(Phase::Action, timestamp)?;
    Ok(())
}

/// Handles `Event::StrategicActionBegin`.
pub(super) fn strategic_action_begin(
    game_state: &mut GameState,
    player: PlayerId,
    card: StrategyCard,
) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        game_state.strategy_card_holders.get(&card) == Some(&player),
        "current player must hold the strategy card",
    );
    ensure!(
        !game_state.spent_strategy_cards.contains(&card),
        "strategy card can't already be used",
    );
    ensure!(
        game_state.action_progress.is_none(),
        "we are already performing an action phase action",
    );
    game_state.phase = Phase::StrategicAction;

    let player_faction = game_state
        .players
        .get(&player)
        .context("Expected player to exist")?
        .faction;

    game_state.action_progress = Some(ActionPhaseProgress::Strategic(StrategicProgress {
        card,
        primary: StrategicPrimaryProgress::default_for(
            card.version(&game_state.game_settings.expansions),
            player_faction,
        ),
        other_players: Default::default(),
    }));
    game_state.spent_strategy_cards.insert(card);
    Ok(())
}

/// Handles `Event::StrategicActionPrimary`.
pub(super) fn strategic_action_primary(
    game_state: &mut GameState,
    player: PlayerId,
    action: StrategicPrimaryAction,
) -> Result<()> {
    game_state.assert_phase(Phase::StrategicAction)?;
    game_state.assert_player_turn(&player)?;
    let Some(progress) = &game_state.action_progress else {
        bail!("Invalid game state: expected action_progress not to be None");
    };
    let ActionPhaseProgress::Strategic(progress) = progress else {
        bail!("Invalid action phase progress '{:?}'", progress);
    };

    ensure!(
        progress.primary.is_none(),
        "Primary action has already been performed"
    );
    let card = progress.card;

    let primary = perform_strategic_primary(game_state, &player, card, action)?;

    let Some(ActionPhaseProgress::Strategic(progress)) = &mut game_state.action_progress else {
        bail!("not currently performing a strategic action");
    };
    progress.primary = Some(primary);
    Ok(())
}

/// Handles `Event::StrategicActionSecondary`.
pub(super) fn strategic_action_secondary(
    game_state: &mut GameState,
    player: PlayerId,
    action: StrategicSecondaryAction,
) -> Result<()> {
    game_state.assert_phase(Phase::StrategicAction)?;
    let current_player = game_state.current_player()?;
    ensure!(
        player != current_player,
        "current player can't perform the secondary on a strategy card",
    );

    let Some(action_progress) = &game_state.action_progress else {
        bail!("no strategic action in progress");
    };
    let ActionPhaseProgress::Strategic(progress) = action_progress else {
        bail!("cannot perform strategic actions during non-strategy actions");
    };
    ensure!(
        action.is_for_card(progress.card),
        "Mismatch between strategic progress {progress:?} and action {action:?}"
    );

    perform_strategic_secondary(game_state, &player, &action)?;

    let Some(ActionPhaseProgress::Strategic(progress)) = &mut game_state.action_progress else {
        bail!("no strategic action in progress");
    };
    progress.other_players.insert(player, action.into());
    Ok(())
}

/// Handles `Event::StrategicActionCommit`.
pub(super) fn strategic_action_commit(game_state: &mut GameState) -> Result<()> {
    game_state.assert_phase(Phase::StrategicAction)?;

    let Some(ActionPhaseProgress::Strategic(progress)) = &game_state.action_progress else {
        bail!("not currently performing a strategic action");
    };

    if progress.card.has_tracked_primary() {
        ensure!(
            progress.primary.is_some(),
            "has not performed strategy card primary"
        );
    }
    ensure_tactical_action_finished(game_state)?;

    game_state.action_progress = None;
    game_state.phase = Phase::EndActionTurn;
    Ok(())
}
