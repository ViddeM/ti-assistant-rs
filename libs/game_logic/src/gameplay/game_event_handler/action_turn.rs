//! Events that end or pass an action phase turn.

use anyhow::{Result, ensure};
use chrono::{DateTime, Utc};

use super::{end_action_turn, is_awaiting_thunders_edge_placement};
use ti_helper_game_data::{
    common::{component::Component, player_id::PlayerId},
    components::{action_card::ActionCard, phase::Phase},
    state::game_state::GameState,
};

/// Handles `Event::TakeAnotherTurn`.
pub(super) fn take_another_turn(game_state: &mut GameState, player: PlayerId) -> Result<()> {
    game_state.assert_phase(Phase::EndActionTurn)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        !game_state.passed_players.contains(&player),
        "A player that has passed cannot take another turn"
    );

    ensure!(
        !is_awaiting_thunders_edge_placement(game_state),
        "Thunder's Edge must be placed before taking another turn"
    );
    game_state.action_progress = None;
    game_state.turn_number += 1;

    game_state.phase = Phase::Action;
    Ok(())
}

/// Handles `Event::EndTurn`.
pub(super) fn end_turn(
    game_state: &mut GameState,
    player: PlayerId,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::EndActionTurn)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        game_state.action_progress.is_none(),
        "Cannot end turn while an action is in progress"
    );

    end_action_turn(game_state)?;
    game_state.advance_turn(timestamp)?;
    Ok(())
}

/// Handles `Event::PassAction`.
pub(super) fn pass_action(
    game_state: &mut GameState,
    player: PlayerId,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        !game_state.passed_players.contains(&player),
        "{player} has already passed"
    );

    let has_used_strategy_cards = game_state
        .strategy_card_holders
        .iter()
        .filter(|(card, _)| !game_state.spent_strategy_cards.contains(card))
        .all(|(_, holder)| *holder != player);

    ensure!(
        has_used_strategy_cards,
        "player must use all strategy cards before passing"
    );

    game_state.passed_players.insert(player);

    // A passing player may still commit to the expedition at the end of their turn.
    let can_commit = (game_state.te_expedition.as_ref()).is_some_and(|te| !te.is_full());
    let has_end_of_turn_cards = [ActionCard::Crisis, ActionCard::PuppetsOnAString]
        .iter()
        .any(|card| card.is_enabled_in(&game_state.game_settings.expansions));
    if can_commit || has_end_of_turn_cards {
        game_state.phase = Phase::EndActionTurn;
    } else {
        game_state.advance_turn(timestamp)?;
    }
    Ok(())
}
