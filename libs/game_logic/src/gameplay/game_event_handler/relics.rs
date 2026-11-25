//! Relic events.

use anyhow::{Context, Result, bail, ensure};
use ti_helper_game_data::{
    actions::event::{RelicAction, action_matches_relic},
    common::player_id::PlayerId,
    components::{
        galactic_event::GalacticEvent,
        phase::Phase,
        planet_attachment::PlanetAttachment,
        relic::{Relic, RelicPlay},
        tech::Technology,
    },
    state::{
        agenda::MinisterOfAntiquesProgress,
        game_state::{ActionPhaseProgress, GameState, RelicProgress, RelicStep, TacticalProgress},
    },
};

/// Handles `Event::RelicActionBegin`.
pub(super) fn begin(game_state: &mut GameState, player: PlayerId, relic: Relic) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    game_state.assert_enabled(&relic)?;
    ensure!(
        relic.info().play == RelicPlay::Action,
        "Only able to play relic actions during action phase"
    );
    ensure!(
        game_state.action_progress.is_none(),
        "Invalid game state, action_progress not None during Action Phase"
    );
    ensure!(
        game_state
            .players
            .get(&player)
            .ok_or_else(|| anyhow::anyhow!("Player doesn't exist? (This is a bug)"))?
            .relics
            .contains(&relic),
        "Player doesn't own the relic"
    );

    game_state.action_progress = Some(ActionPhaseProgress::Relic(RelicProgress {
        relic,
        step: RelicStep::Action,
    }));
    game_state.phase = Phase::RelicAction;
    Ok(())
}

/// Handles `Event::RelicActionCommit`.
pub(super) fn commit(
    game_state: &mut GameState,
    player: PlayerId,
    data: Option<RelicAction>,
) -> Result<()> {
    game_state.assert_phase(Phase::RelicAction)?;
    game_state.assert_player_turn(&player)?;

    let Some(ActionPhaseProgress::Relic(progress)) = game_state.action_progress.clone() else {
        bail!(
            "Invalid state, expected relic progress, got {:?}",
            game_state.action_progress
        )
    };
    ensure!(
        progress.step == RelicStep::Action,
        "{player} must research with the Book of Latvinia first"
    );

    ensure!(
        game_state
            .players
            .get(&player)
            .ok_or_else(|| anyhow::anyhow!("Player doesn't exist? (This is a bug)"))?
            .relics
            .contains(&progress.relic),
        "Player doesn't own the relic"
    );

    ensure!(
        action_matches_relic(&data, &progress.relic),
        "Data provided doesn't match the card being played."
    );

    if let Some(data) = data {
        match data {
            RelicAction::StellarConverter { planet } => {
                if !game_state
                    .galactic_events
                    .in_play
                    .contains(&GalacticEvent::WildWildGalaxy)
                {
                    ensure!(
                        !planet.is_mecatol_rex(),
                        "Cannot use stellar converter on mecatol rex"
                    );
                    ensure!(
                        !game_state.is_home_planet(&planet),
                        "Cannot use stellar converter on home system"
                    );
                    ensure!(
                        !planet.info().is_legendary,
                        "Cannot use stellar converter on legendary planet"
                    );
                    ensure!(
                        !game_state
                            .players
                            .values()
                            .filter_map(|p| p.planets.get(&planet))
                            .flatten()
                            .any(|a| a.info().set_legendary),
                        "Planet has attachment that makes it legendary and stellar converter cannot be played on legendary planets"
                    );
                }
                ensure!(
                    !game_state.map_data.destroyed_planets.contains(&planet),
                    "Cannot use stellar converter on a planet that has already been destroyed!"
                );

                game_state.destroy_planet(planet);
            }
            RelicAction::NanoForge { planet } => {
                ensure!(
                    !planet.info().is_legendary,
                    "Cannot place nano forge on legendary planet"
                );
                ensure!(
                    !game_state.is_home_planet(&planet),
                    "Cannot place Nano Forge on home planet"
                );
                let Some(player) = game_state.players.get_mut(&player) else {
                    bail!("Player doesn't exist?")
                };
                ensure!(
                    player.planets.contains_key(&planet),
                    "Player must own the planet in order to place nano-forges there"
                );
                let Some(attachments) = player.planets.get_mut(&planet) else {
                    bail!("Player no longer owns planet? (This is a bug)")
                };
                ensure!(
                    !attachments.contains(&PlanetAttachment::NanoForge),
                    "Planet already has attachment Nano-Forges"
                );
                attachments.insert(PlanetAttachment::NanoForge);
            }
        }
    }

    if progress.relic == Relic::BookOfLatvinia {
        let controls_all_specialties = game_state
            .players
            .get(&player)
            .ok_or_else(|| anyhow::anyhow!("Player doesn't exist? (This is a bug)"))?
            .controls_all_tech_specialties();

        if controls_all_specialties {
            game_state.score.book_of_latvinia = Some(player.clone());
        } else {
            game_state.speaker = Some(player.clone());
        }
    }

    if !progress.relic.is_exhausted_when_played() {
        game_state
            .players
            .get_mut(&player)
            .ok_or_else(|| anyhow::anyhow!("Player no longer exists? (This is a bug)"))?
            .relics
            .remove(&progress.relic);
    }

    game_state.action_progress = None;
    game_state.phase = Phase::EndActionTurn;
    Ok(())
}

/// Handles `Event::GainRelicAction`.
pub(super) fn gain_as_action(
    game_state: &mut GameState,
    player: PlayerId,
    relic: Relic,
) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    game_state.assert_enabled(&relic)?;
    ensure!(
        !game_state
            .players
            .values()
            .any(|p| p.relics.contains(&relic)),
        "Relic {relic:?} is already owned by another player."
    );

    game_state.gain_relic_as_action(player, relic)?;
    Ok(())
}

/// Handles `Event::PlayCrownOfEmphidia`.
pub(super) fn play_crown_of_emphidia(game_state: &mut GameState, player: PlayerId) -> Result<()> {
    game_state.assert_phase(Phase::Relics)?;

    game_state.assert_enabled(&Relic::TheCrownOfEmphidia)?;

    let Some(p) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist");
    };

    ensure!(
        p.relics.contains(&Relic::TheCrownOfEmphidia),
        "Player doesn't have the Crown of emphidia relic!"
    );

    ensure!(
        p.controls_tomb_of_emphidia(),
        "Planet must have the 'Tomb of Emphidia' planet attachment in order to play the Crown of Emphidia"
    );

    p.relics.remove(&Relic::TheCrownOfEmphidia);
    game_state.score.crown_of_emphidia = Some(player);
    Ok(())
}

/// Handles `Event::PlayMawOfWorlds`.
pub(super) fn play_maw_of_worlds(
    game_state: &mut GameState,
    player: PlayerId,
    tech: Technology,
) -> Result<()> {
    game_state.assert_phase(Phase::Relics)?;

    game_state.assert_enabled(&Relic::MawOfWorlds)?;
    game_state.assert_enabled(&tech)?;
    ensure!(
        game_state.is_agenda_enabled(),
        "The Maw of Worlds can only be played when there is an agenda phase (the custodians token is taken)"
    );

    let Some(p) = game_state.players.get(&player) else {
        bail!("Player doesn't exist");
    };

    ensure!(
        p.relics.contains(&Relic::MawOfWorlds),
        "Player doesn't have the Maw of Worlds relic!"
    );

    game_state.gain_tech(&player, tech)?;
    (game_state.players.get_mut(&player))
        .context("Player doesn't exist")?
        .relics
        .remove(&Relic::MawOfWorlds);
    Ok(())
}

/// Handles `Event::ResearchBookOfLatviniaTechs`.
pub(super) fn research_book_of_latvinia_techs(
    game_state: &mut GameState,
    player: PlayerId,
    technologies: Vec<Technology>,
) -> Result<()> {
    let minister_research = game_state.phase == Phase::AgendaEffect
        && matches!(
            game_state.agenda.as_ref().and_then(|agenda| agenda.minister_of_antiques.as_ref()),
            Some(MinisterOfAntiquesProgress::BookOfLatviniaResearch { player: p }) if p == &player
        );
    let action_research = game_state.phase == Phase::RelicAction
        && game_state.current_player.as_ref() == Some(&player)
        && matches!(
            &game_state.action_progress,
            Some(ActionPhaseProgress::Relic(RelicProgress {
                relic: Relic::BookOfLatvinia,
                step: RelicStep::Gain,
            }))
        );
    let tactical_research = game_state.current_player.as_ref() == Some(&player)
        && game_state
            .action_progress
            .as_ref()
            .and_then(ActionPhaseProgress::tactical)
            .is_some_and(TacticalProgress::owes_book_of_latvinia_research);
    ensure!(
        minister_research || action_research || tactical_research,
        "{player} has no Book of Latvinia research"
    );
    ensure!(
        technologies.len() <= 2,
        "The Book of Latvinia researches at most 2 technologies"
    );
    let researcher = game_state
        .players
        .get(&player)
        .context("Player doesn't exist")?;
    for (i, tech) in technologies.iter().enumerate() {
        game_state.assert_enabled(tech)?;
        researcher.assert_can_research(tech)?;
        ensure!(
            tech.info().requirements.is_empty(),
            "{tech:?} has prerequisites"
        );
        ensure!(
            !researcher.has_tech(tech),
            "{player} already owns {}",
            tech.info().name
        );
        ensure!(
            !technologies[..i].contains(tech),
            "{} can't be researched twice",
            tech.info().name
        );
    }

    for tech in &technologies {
        game_state.research_tech(&player, tech.clone())?;
    }
    if minister_research {
        game_state
            .agenda
            .as_mut()
            .context("agenda state not initialized, this is a bug.")?
            .minister_of_antiques = None;
        game_state.phase = Phase::Agenda;
    } else if tactical_research {
        game_state.tactical_progress_mut()?.book_of_latvinia_techs = Some(technologies);
    } else {
        game_state.action_progress = None;
        game_state.phase = Phase::EndActionTurn;
    }
    Ok(())
}

/// Handles `Event::SetCrownOfEmphidiaOwner`.
pub(super) fn set_crown_of_emphidia_owner(
    game_state: &mut GameState,
    player: Option<PlayerId>,
) -> Result<()> {
    if let Some(p) = player.as_ref() {
        ensure!(
            game_state.players.contains_key(p),
            "Player doesn't exist '{p}'"
        );
    }

    // Should any player currently own the relic, remove it from them.
    game_state.players.values_mut().for_each(|p| {
        p.relics.remove(&Relic::TheCrownOfEmphidia);
    });

    game_state.score.crown_of_emphidia = player;
    Ok(())
}

/// Handles `Event::SetBookOfLatviniaOwner`.
pub(super) fn set_book_of_latvinia_owner(
    game_state: &mut GameState,
    player: Option<PlayerId>,
) -> Result<()> {
    if let Some(p) = player.as_ref() {
        ensure!(
            game_state.players.contains_key(p),
            "Player doesn't exist '{p}'"
        );
    }

    game_state.players.values_mut().for_each(|p| {
        p.relics.remove(&Relic::BookOfLatvinia);
    });

    game_state.score.book_of_latvinia = player;
    Ok(())
}
