//! Frontier card events.

use anyhow::{Context, Result, bail, ensure};

use ti_helper_game_data::components::frontier_card::FrontierCard;
use ti_helper_game_data::{
    actions::event::{FrontierCardAction, action_matches_frontier_card},
    common::player_id::PlayerId,
    components::{frontier_card::FrontierCardType, phase::Phase, planet::Planet},
    state::game_state::{ActionPhaseProgress, FrontierCardProgress, GameState},
};

/// Handles `Event::FrontierCardActionBegin`.
pub(super) fn begin(
    game_state: &mut GameState,
    player: PlayerId,
    card: FrontierCard,
) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    game_state.assert_enabled(&card)?;
    ensure!(
        card.info().frontier_type == FrontierCardType::Action,
        "Only able to play frontier card actions during action phase"
    );
    ensure!(
        game_state.action_progress.is_none(),
        "Invalid game state, action_progress not None during Action Phase"
    );

    game_state.action_progress = Some(ActionPhaseProgress::FrontierCard(FrontierCardProgress {
        card,
    }));
    game_state.phase = Phase::FrontierCardAction;
    Ok(())
}

/// Handles `Event::FrontierCardActionCommit`.
pub(super) fn commit(
    game_state: &mut GameState,
    player: PlayerId,
    data: Option<FrontierCardAction>,
) -> Result<()> {
    game_state.assert_phase(Phase::FrontierCardAction)?;
    game_state.assert_player_turn(&player)?;

    let Some(ActionPhaseProgress::FrontierCard(progress)) = game_state.action_progress.as_ref()
    else {
        bail!(
            "Invalid state, expected frontier card progress, got {:?}",
            game_state.action_progress
        );
    };

    let current_player_faction = game_state
        .players
        .iter()
        .find(|&(id, _)| &player == id)
        .context("Current player to exist")?
        .1
        .faction;
    ensure!(
        action_matches_frontier_card(
            &data,
            &progress.card,
            &current_player_faction,
            game_state.map_data.milty_information.is_some()
        ),
        "Data provided doesn't match the card being played."
    );

    if let Some(progress) = data {
        match progress {
            FrontierCardAction::EnigmaticDevice { tech } => {
                game_state.assert_enabled(&tech)?;
                game_state.research_tech(&player, tech)?;
            }
            FrontierCardAction::Mirage { system, attachment } => {
                ensure!(
                    !game_state
                        .players
                        .values()
                        .flat_map(|p| p.planets.keys())
                        .any(|p| p == &Planet::Mirage),
                    "A player already owns mirage and can only be takn using a tactical action"
                );
                ensure!(
                    game_state
                        .map_data
                        .milty_information
                        .as_ref()
                        .map(|data| data.mirage_system.is_none())
                        .unwrap_or(true),
                    "Mirage system is already set?"
                );
                let player_id = game_state
                    .current_player
                    .clone()
                    .context("No current player")?;
                game_state.set_planet_owner(Planet::Mirage, Some(player_id))?;
                if let Some(attachment) = attachment {
                    game_state
                        .get_current_player()?
                        .planets
                        .entry(Planet::Mirage)
                        .or_default()
                        .insert(attachment);
                }

                if let Some(milty_info) = game_state.map_data.milty_information.as_mut() {
                    milty_info.mirage_system =
                        Some(system.context("System to be chosen when milty info is enabled")?);
                }
            }
        }
    }

    game_state.action_progress = None;
    game_state.phase = Phase::EndActionTurn;
    Ok(())
}
