//! Tactical action events.

use anyhow::{Result, bail, ensure};
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{
        galactic_event::GalacticEvent,
        phase::Phase,
        planet::{OrbitalBodyType, Planet, PlanetTrait},
        planet_attachment::PlanetAttachment,
        relic::Relic,
        system::System,
        tech::Technology,
    },
    state::game_state::{
        ActionPhaseProgress, GameState, PlanetLocation, TacticalProgress, TakenFrom,
    },
};

/// Handles `Event::TacticalActionTakePlanet`.
pub(super) fn take_planet(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
) -> Result<()> {
    game_state.assert_player_turn(&player)?;
    let planet_system = match game_state.planet_location(&planet) {
        PlanetLocation::SystemTile => Some(System::for_planet(&planet)?.id),
        PlanetLocation::Token(system) => system,
        PlanetLocation::NotOnBoard => bail!("{} isn't on the board", planet.info().name),
    };
    game_state.assert_enabled(&planet)?;

    let has_been_owned = game_state.has_been_owned(&planet);
    let previous_owner = game_state.set_planet_owner(planet.clone(), Some(player.clone()))?;

    let taken_from = match previous_owner {
        Some(previous_owner) => TakenFrom::Player(previous_owner),
        None if has_been_owned => TakenFrom::Unowned,
        None => TakenFrom::NeverOwned,
    };
    let tactical = game_state.tactical_progress_mut()?;
    tactical.activated_system = planet_system;
    tactical.taken_planets.insert(planet.clone(), taken_from);

    if planet == Planet::ThundersEdge {
        game_state.gain_breakthrough(&player)?;
    }
    Ok(())
}

/// Handles `Event::TacticalActionAttachPlanetAttachment`.
pub(super) fn attach_planet_attachment(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
    attachment: PlanetAttachment,
) -> Result<()> {
    game_state.assert_player_turn(&player)?;
    let planet_traits = game_state.planet_traits(&planet);
    ensure!(
        planet.info().body_type != OrbitalBodyType::SpaceStation,
        "Space stations can't have attachments"
    );

    {
        let tactical = game_state.tactical_progress_mut()?;

        match tactical.taken_planets.get(&planet) {
            None => bail!("Can only attach attachment to planet taken this turn"),
            Some(TakenFrom::Player(_)) => {
                bail!("Cannot explore planet that was taken from another player")
            }
            Some(TakenFrom::NeverOwned | TakenFrom::Unowned) => {}
        }

        ensure!(
            !tactical.planet_attachments.contains_key(&planet),
            "Planet has already been explored this turn!"
        );

        let attachment_info = attachment.info();
        let Some(attachment_planet_trait) = &attachment_info.planet_trait else {
            // TODO: Somewhat of an exploit of the fact that currently,
            //       the attachments that have planet trait requirements match 100%
            //       with the ones that are available during exploration.
            bail!("Only attachments with planet_traits can be used during exploration")
        };
        ensure!(
            planet_traits.contains(attachment_planet_trait),
            "Planet does not meet the trait requirements for the attachment"
        );
    }

    let current_player = game_state.get_current_player()?;
    let Some(planet_attachments) = current_player.planets.get_mut(&planet) else {
        bail!("Player doesn't have planet that they just took, this is a bug!");
    };
    let actual_attachment = attachment.match_planet(&planet.info());
    planet_attachments.insert(actual_attachment.clone());

    game_state
        .tactical_progress_mut()?
        .planet_attachments
        .insert(planet, actual_attachment);
    Ok(())
}

/// Handles `Event::TacticalActionResearchTechnology`.
pub(super) fn research_technology(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
    tech: Technology,
) -> Result<()> {
    game_state.assert_player_turn(&player)?;
    game_state.assert_galactic_event(&GalacticEvent::DangerousWilds)?;
    let planet_traits = game_state.planet_traits(&planet);

    {
        let tactical = game_state.tactical_progress_mut()?;

        match tactical.taken_planets.get(&planet) {
            None => bail!("Can only research for a planet taken this turn"),
            Some(TakenFrom::Player(_)) => bail!(
                "{} was taken from another player, not the planet deck",
                planet.info().name
            ),
            Some(TakenFrom::Unowned) => bail!(
                "{} has been owned before, it wasn't taken from the planet deck",
                planet.info().name
            ),
            Some(TakenFrom::NeverOwned) => {}
        }

        ensure!(
            planet_traits.contains(&PlanetTrait::Hazardous),
            "{} isn't hazardous",
            planet.info().name
        );

        ensure!(
            !tactical.researched_techs.contains_key(&planet),
            "Player has already researched a technology for {}",
            planet.info().name
        );
    }

    game_state.assert_enabled(&tech)?;
    game_state.research_tech(&player, tech.clone())?;

    game_state
        .tactical_progress_mut()?
        .researched_techs
        .insert(planet, tech);
    Ok(())
}

/// Handles `Event::TacticalActionGainRelic`.
pub(super) fn gain_relic(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
    relic: Option<Relic>,
) -> Result<()> {
    game_state.assert_player_turn(&player)?;
    ensure!(
        game_state.tactical_progress_mut()?.owes_relic(&planet),
        "{player} doesn't gain a relic for {}",
        planet.info().name
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

    game_state
        .tactical_progress_mut()?
        .gained_relics
        .insert(planet, relic.clone());
    if let Some(relic) = relic {
        game_state.set_relic_owner(relic, Some(player))?;
    }
    Ok(())
}

/// Start a tactical action for the current player.
pub(super) fn start_tactical_action(game_state: &mut GameState) {
    game_state.phase = Phase::TacticalAction;
    game_state.action_progress = Some(ActionPhaseProgress::Tactical(TacticalProgress::default()));
}

/// Ensure the tactical action being performed, if any, has gained the relics it owes and has
/// researched with a Book of Latvinia gained during it.
pub(super) fn ensure_tactical_action_finished(game_state: &GameState) -> Result<()> {
    let Some(tactical) = game_state
        .action_progress
        .as_ref()
        .and_then(ActionPhaseProgress::tactical)
    else {
        return Ok(());
    };
    if let Some(planet) = tactical
        .taken_planets
        .keys()
        .find(|planet| tactical.owes_relic(planet))
    {
        bail!("A relic must be gained for {} first", planet.info().name);
    }
    ensure!(
        !tactical.owes_book_of_latvinia_research(),
        "The Book of Latvinia must be researched with first"
    );
    Ok(())
}
