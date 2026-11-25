//! Action card events.

use std::collections::HashSet;

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};
use strum::IntoEnumIterator;

use crate::gameplay::error::GameError;

use super::{
    end_action_turn, perform_strategic_primary, perform_strategic_secondary, start_tactical_action,
};
use ti_helper_game_data::{
    actions::event::{ActionCardAction, action_matches_action_card},
    common::player_id::PlayerId,
    components::{
        action_card::{ActionCard, ActionCardPlay},
        phase::Phase,
        strategy_card::StrategyCard,
        tech::{TechOrigin, TechType, Technology},
    },
    state::game_state::{ActionCardProgress, ActionPhaseProgress, GameState, PlanetLocation},
};

/// Handles `Event::ActionCardActionBegin`.
pub(super) fn begin(game_state: &mut GameState, player: PlayerId, card: ActionCard) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    game_state.assert_enabled(&card)?;

    let card_info = card.info();
    ensure!(
        card_info.play == ActionCardPlay::Action,
        "card cannot be played as an action"
    );

    match card {
        ActionCard::Plagiarize => {
            ensure!(
                !get_plagiarize_available_techs(game_state)?.is_empty(),
                "There are no techs available for the player to steal."
            )
        }
        ActionCard::Overrule | ActionCard::Strategize => {
            ensure!(
                game_state.spent_strategy_cards.len() < StrategyCard::iter().count(),
                "There are no readied or unchosen strategy cards."
            )
        }
        _ => {}
    }

    game_state.action_progress = Some(ActionPhaseProgress::ActionCard(ActionCardProgress { card }));
    game_state.phase = Phase::ActionCardAction;
    Ok(())
}

/// Handles `Event::ActionCardActionCommit`.
pub(super) fn commit(
    game_state: &mut GameState,
    player: PlayerId,
    data: Option<ActionCardAction>,
) -> Result<()> {
    game_state.assert_phase(Phase::ActionCardAction)?;
    game_state.assert_player_turn(&player)?;

    if let Some(ActionPhaseProgress::ActionCard(progress)) = &game_state.action_progress {
        ensure!(
            action_matches_action_card(&data, &progress.card),
            "Trying to perform an action that does not match the selected action card."
        );
    } else {
        bail!("Not currently performing an action card action");
    };

    let starts_tactical_action = match &data {
        Some(ActionCardAction::Overrule { card, .. }) => card
            .version(&game_state.game_settings.expansions)
            .primary_is_tactical_action(),
        _ => false,
    };

    let mut brilliance_target = None;
    if let Some(data) = data {
        match data {
            ActionCardAction::FocusedResearch { tech } => {
                game_state.assert_enabled(&tech)?;
                game_state.research_tech(&player, tech)?;
            }
            ActionCardAction::DivertFunding {
                remove_tech,
                take_tech,
            } => {
                game_state.assert_enabled(&take_tech)?;

                let current_player = game_state.get_current_player()?;
                ensure!(
                    current_player.has_tech(&remove_tech),
                    "Player doesn't have technology {remove_tech:?}"
                );
                ensure!(
                    !(current_player.has_tech(&take_tech) && take_tech != remove_tech),
                    "Player already has technology {take_tech:?}"
                );
                let remove_tech_info = remove_tech.info();
                ensure!(
                    !matches!(remove_tech_info.origin, TechOrigin::Faction(..)),
                    "Cannot remove faction technology"
                );
                ensure!(
                    !matches!(remove_tech_info.tech_type, TechType::UnitUpgrade),
                    "Cannot remove unit upgrade technologies"
                );

                if take_tech != remove_tech {
                    game_state.research_tech(&player, take_tech)?;
                    game_state.get_current_player()?.remove_tech(&remove_tech);
                }
            }
            ActionCardAction::Plagiarize { tech } => {
                let available_techs = get_plagiarize_available_techs(game_state)?;
                ensure!(
                    available_techs.contains(&tech),
                    "Unable to take tech {tech:?} for this action card"
                );

                game_state.gain_tech(&player, tech)?;
            }
            ActionCardAction::Brilliance { player: target } => {
                if let Some(target) = target {
                    let target_player = game_state
                        .players
                        .get(&target)
                        .context("Player not found")?;
                    ensure!(
                        target_player.breakthrough.is_none(),
                        "{target} has already gained their breakthrough"
                    );
                    brilliance_target = Some(target);
                }
            }
            ActionCardAction::MercenaryContract { planet } => {
                if let Some(planet) = planet {
                    ensure!(
                        !game_state.is_home_planet(&planet),
                        "Cannot use Mercenary Contract on a home planet"
                    );
                    ensure!(
                        game_state.planet_location(&planet) != PlanetLocation::NotOnBoard,
                        "{} is not on the board",
                        planet.info().name
                    );
                    let Some((owner, owner_player)) = game_state
                        .players
                        .iter_mut()
                        .find(|(_, p)| p.planets.contains_key(&planet))
                    else {
                        bail!("{} is not owned by any player", planet.info().name);
                    };
                    ensure!(
                        *owner != player,
                        "Players keep their own planets when using Mercenary Contract"
                    );
                    owner_player.remove_planet(&planet);
                }
            }
            ActionCardAction::Overrule { card, primary } => {
                assert_readied_or_unchosen(game_state, card)?;
                match primary {
                    Some(primary) => {
                        perform_strategic_primary(game_state, &player, card, primary)?;
                    }
                    None => ensure!(
                        !card.has_tracked_primary(),
                        "The primary ability of {card} must be performed"
                    ),
                }
            }
            ActionCardAction::Strategize { card, secondary } => {
                assert_readied_or_unchosen(game_state, card)?;
                ensure!(
                    !secondary.skipped(),
                    "The secondary ability of {card} must be performed"
                );
                ensure!(
                    secondary.is_for_card(card),
                    "Mismatch between strategy card {card:?} and secondary {secondary:?}"
                );
                perform_strategic_secondary(game_state, &player, &secondary)?;
            }
        }
    }

    if starts_tactical_action {
        start_tactical_action(game_state);
    } else {
        game_state.action_progress = None;
        game_state.phase = Phase::EndActionTurn;
    }
    if let Some(target) = brilliance_target {
        game_state.gain_breakthrough(&target)?;
    }
    Ok(())
}

/// Handles `Event::PlayCrisis`.
pub(super) fn play_crisis(
    game_state: &mut GameState,
    player: PlayerId,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::EndActionTurn)?;
    game_state.assert_enabled(&ActionCard::Crisis)?;
    ensure!(
        game_state.players.contains_key(&player),
        "Player {player} not found"
    );

    let players_not_passed = (game_state.players.keys())
        .filter(|p| !game_state.passed_players.contains(*p))
        .count();
    ensure!(
        players_not_passed >= 2,
        "Crisis can only be played while at least 2 players haven't passed"
    );

    end_action_turn(game_state)?;
    game_state.advance_turn(timestamp)?;
    game_state.advance_turn(timestamp)?;
    Ok(())
}

/// Handles `Event::PlayPuppetsOnAString`.
pub(super) fn play_puppets_on_a_string(
    game_state: &mut GameState,
    player: PlayerId,
    timestamp: DateTime<Utc>,
) -> Result<()> {
    game_state.assert_phase(Phase::EndActionTurn)?;
    game_state.assert_enabled(&ActionCard::PuppetsOnAString)?;
    ensure!(
        game_state.passed_players.contains(&player),
        "Only a player that has passed can play Puppets on a String"
    );
    ensure!(
        game_state.interrupted_turn.is_none(),
        "A passed player is already performing an action"
    );

    end_action_turn(game_state)?;
    let next_player = game_state.next_player_after(&game_state.current_player()?)?;
    if game_state.commit_turn_time(timestamp)? {
        game_state.current_turn_start_time = Some(timestamp);
    }
    game_state.interrupted_turn = next_player;
    game_state.current_player = Some(player);
    Ok(())
}

fn get_plagiarize_available_techs(
    game_state: &GameState,
) -> Result<HashSet<&Technology>, GameError> {
    let current_player_id = game_state.current_player()?;
    let current_player = game_state
        .players
        .get(&current_player_id)
        .ok_or_else(|| anyhow::anyhow!("Invalid game state, current player not in players map"))?;

    let available_techs = game_state
        .players
        .iter()
        .filter(|&(id, _)| id != &current_player_id)
        .flat_map(|(_, player)| player.technologies().iter())
        .filter(|tech| !matches!(tech.info().origin, TechOrigin::Faction(..)))
        .filter(|tech| !current_player.has_tech(tech))
        .collect::<HashSet<&Technology>>();

    Ok(available_techs)
}

/// Asserts that `card` is readied or unchosen, i.e. that its primary ability hasn't been used
/// this round.
fn assert_readied_or_unchosen(game_state: &GameState, card: StrategyCard) -> Result<()> {
    ensure!(
        !game_state.spent_strategy_cards.contains(&card),
        "{card} has been exhausted, only readied or unchosen strategy cards can be used"
    );
    Ok(())
}
