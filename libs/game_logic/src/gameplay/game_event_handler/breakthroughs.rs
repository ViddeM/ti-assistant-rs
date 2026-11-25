//! Breakthrough and technology action events.

use anyhow::{Context, Result, bail, ensure};
use ti_helper_game_data::{
    actions::event::BreakthroughAction,
    common::player_id::PlayerId,
    components::{
        breakthrough::{Breakthrough, BreakthroughKind},
        phase::Phase,
        tech::{TechOrigin, TechType, Technology},
    },
    state::game_state::{ActionPhaseProgress, BreakthroughProgress, GameState},
};

/// Handles `Event::BreakthroughActionBegin`.
pub(super) fn begin(game_state: &mut GameState, player: PlayerId) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        game_state.action_progress.is_none(),
        "Invalid game state, action_progress not None during Action Phase"
    );
    ensure!(
        (game_state.players.get(&player))
            .and_then(|p| p.breakthrough.as_ref())
            .is_some_and(|b| b.info().kind == BreakthroughKind::Action),
        "{player} doesn't have a breakthrough with an action"
    );

    game_state.action_progress = Some(ActionPhaseProgress::Breakthrough(
        BreakthroughProgress::default(),
    ));
    game_state.phase = Phase::BreakthroughAction;
    Ok(())
}

/// Handles `Event::VisionariaSelectDecision`.
pub(super) fn visionaria_select_decision(
    game_state: &mut GameState,
    player: PlayerId,
    tech: Option<Technology>,
) -> Result<()> {
    game_state.assert_phase(Phase::BreakthroughAction)?;
    let deepwrought = game_state.current_player()?;
    ensure!(
        (game_state.players.get(&deepwrought))
            .is_some_and(|p| p.breakthrough == Some(Breakthrough::VisionariaSelect)),
        "{deepwrought} isn't performing Visionaria Select"
    );
    ensure!(
        player != deepwrought,
        "{player} can't research with their own Visionaria Select"
    );
    let researcher = (game_state.players.get(&player)).context("Player not found")?;
    if let Some(tech) = &tech {
        game_state.assert_enabled(tech)?;
        let info = tech.info();
        ensure!(
            info.origin == TechOrigin::Base,
            "{} is a faction technology",
            info.name
        );
        ensure!(
            info.tech_type != TechType::UnitUpgrade,
            "{} is a unit upgrade",
            info.name
        );
        researcher.assert_can_research(tech)?;
        ensure!(
            !researcher.has_tech(tech),
            "{player} already has tech {}",
            info.name
        );
    }

    let Some(ActionPhaseProgress::Breakthrough(progress)) = &mut game_state.action_progress else {
        bail!("Not currently performing a breakthrough action");
    };
    ensure!(
        !progress.visionaria_select.contains_key(&player),
        "{player} has already decided"
    );
    progress.visionaria_select.insert(player, tech);
    Ok(())
}

/// Handles `Event::BreakthroughActionCommit`.
pub(super) fn commit(
    game_state: &mut GameState,
    player: PlayerId,
    action: BreakthroughAction,
) -> Result<()> {
    game_state.assert_phase(Phase::BreakthroughAction)?;
    game_state.assert_player_turn(&player)?;
    let Some(ActionPhaseProgress::Breakthrough(progress)) = game_state.action_progress.clone()
    else {
        bail!("Not currently performing a breakthrough action");
    };
    let breakthrough = action.breakthrough();
    ensure!(
        game_state
            .players
            .get(&player)
            .is_some_and(|p| p.breakthrough.as_ref() == Some(&breakthrough)),
        "{player} doesn't have {}",
        breakthrough.info().name
    );
    game_state.action_progress = None;

    match action {
        BreakthroughAction::Psychospore
        | BreakthroughAction::DeorbitBarrage
        | BreakthroughAction::ResonanceGenerator => {}
        BreakthroughAction::VaultsOfTheHeir { purged_tech, relic } => {
            ensure!(
                game_state
                    .players
                    .get(&player)
                    .is_some_and(|p| p.has_tech(&purged_tech)),
                "{player} doesn't own {}",
                purged_tech.info().name
            );
            game_state.assert_enabled(&relic)?;
            ensure!(
                !game_state
                    .players
                    .values()
                    .any(|p| p.relics.contains(&relic)),
                "Relic {relic:?} is already owned by another player."
            );

            game_state
                .players
                .get_mut(&player)
                .context("Player not found")?
                .remove_tech(&purged_tech);
            game_state.gain_relic_as_action(player.clone(), relic)?;
        }
        BreakthroughAction::VisionariaSelect => {
            if let Some(undecided) = (game_state.table_order.iter()).find(|p| {
                **p != player
                    && game_state.players.get(*p).is_some_and(|p| p.can_research())
                    && !progress.visionaria_select.contains_key(*p)
            }) {
                bail!("Waiting for {undecided} to decide");
            }

            let deepwrought = (game_state.players.get(&player)).context("Player not found")?;
            let gains = progress.visionaria_select_gains(deepwrought);
            for (researcher, tech) in progress.visionaria_select {
                if let Some(tech) = tech {
                    game_state.research_tech(&researcher, tech)?;
                }
            }
            for tech in gains {
                game_state.gain_tech(&player, tech)?;
            }
        }
    }

    if game_state.action_progress.is_none() {
        game_state.phase = Phase::EndActionTurn;
    }
    Ok(())
}

/// Handles `Event::PlayTechAction`.
pub(super) fn play_tech_action(
    game_state: &mut GameState,
    player: PlayerId,
    tech: Technology,
) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        game_state.action_progress.is_none(),
        "Invalid game state, action_progress not None during Action Phase"
    );
    ensure!(
        tech.is_single_button_action(),
        "{} doesn't have a single button ACTION",
        tech.info().name
    );
    ensure!(
        game_state
            .players
            .get(&player)
            .is_some_and(|p| p.has_tech(&tech)),
        "{player} doesn't have {}",
        tech.info().name
    );

    game_state.phase = Phase::EndActionTurn;
    Ok(())
}
