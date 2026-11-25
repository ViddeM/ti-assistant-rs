//! Planet, attachment and technology events.

use anyhow::{Result, bail, ensure};

use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{
        planet::{OrbitalBodyType, Planet},
        planet_attachment::PlanetAttachment,
        tech::Technology,
    },
    state::game_state::{GameState, PlanetLocation},
};

/// Handles `Event::SetPlanetOwner`.
pub(super) fn set_planet_owner(
    game_state: &mut GameState,
    player: Option<PlayerId>,
    planet: Planet,
) -> Result<()> {
    game_state.assert_enabled(&planet)?;
    ensure!(
        player.is_none() || !game_state.map_data.destroyed_planets.contains(&planet),
        "{} has been destroyed",
        planet.info().name
    );

    game_state.set_planet_owner(planet, player)?;
    Ok(())
}

/// Handles `Event::SetPlanetDestroyed`.
pub(super) fn set_planet_destroyed(
    game_state: &mut GameState,
    planet: Planet,
    destroyed: bool,
) -> Result<()> {
    game_state.assert_enabled(&planet)?;
    let name = planet.info().name;
    let is_destroyed = game_state.map_data.destroyed_planets.contains(&planet);
    if destroyed {
        ensure!(!is_destroyed, "{name} is already destroyed");
        ensure!(
            game_state.planet_location(&planet) != PlanetLocation::NotOnBoard,
            "{name} isn't on the board"
        );
        game_state.destroy_planet(planet);
    } else {
        ensure!(is_destroyed, "{name} isn't destroyed");
        game_state
            .map_data
            .destroyed_planets
            .retain(|p| p != &planet);
    }
    Ok(())
}

/// Handles `Event::AddPlanetAttachment`.
pub(super) fn add_planet_attachment(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
    attachment: PlanetAttachment,
) -> Result<()> {
    let home_planet = game_state.is_home_planet(&planet);
    let planet_traits = game_state.planet_traits(&planet);
    let Some(player) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist");
    };

    if attachment.is_helios() {
        let has_v2_tech = player.has_tech(&Technology::HeliosV2);
        ensure!(
            attachment != PlanetAttachment::HeliosV2 || has_v2_tech,
            "{} doesn't have the Helios V2 technology",
            player.name
        );
        ensure!(
            attachment != PlanetAttachment::HeliosV1 || !has_v2_tech,
            "Helios V1 is replaced by Helios V2"
        );
        ensure!(
            !player
                .planets
                .get(&planet)
                .is_some_and(|a| a.iter().any(|a| a.is_helios())),
            "Planet already has a Helios"
        );
        let helios_count = (player.planets.values())
            .flatten()
            .filter(|a| a.is_helios())
            .count();
        ensure!(helios_count < 3, "A player can have at most 3 Helios");
        ensure!(
            !(player.planets.values())
                .flatten()
                .any(|a| a.is_helios() && *a != attachment),
            "A player can only have one Helios version at a time"
        );
    }

    let Some(attachments) = player.planets.get_mut(&planet) else {
        bail!("Player doesn't own planet");
    };

    ensure!(
        !attachments.contains(&attachment),
        "Planet already has the attachment"
    );
    ensure!(
        planet.info().body_type != OrbitalBodyType::SpaceStation,
        "Space stations can't have attachments"
    );

    match &attachment {
        PlanetAttachment::Geoform => {
            ensure!(
                planet == Planet::Elysium,
                "UI The Progenitor can only be used on Elysium"
            );
        }
        PlanetAttachment::Terraform => {
            ensure!(!planet.is_mecatol_rex(), "Cannot terraform Mecatol Rex");
            ensure!(!home_planet, "Cannot terraform home planet");
        }
        PlanetAttachment::NanoForge => {
            ensure!(
                !planet.info().is_legendary,
                "Cannot place Nano Forge on legendary planet"
            );
            ensure!(!home_planet, "Cannot place Nano Forge on home planet");
        }
        v => {
            if let Some(t) = v.info().planet_trait {
                ensure!(
                    planet_traits.contains(&t),
                    "Attachment is not allowed to be placed on this planet, planet_trait requirement not met."
                );
            }
        }
    }

    attachments.insert(attachment.match_planet(&planet.info()));
    Ok(())
}

/// Handles `Event::RemovePlanetAttachment`.
pub(super) fn remove_planet_attachment(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
    attachment: PlanetAttachment,
) -> Result<()> {
    let Some(player) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist");
    };

    let Some(attachments) = player.planets.get_mut(&planet) else {
        bail!("Player doesn't own planet");
    };

    ensure!(
        attachments.contains(&attachment),
        "Planet doesn't have that attachment"
    );

    attachments.remove(&attachment);
    Ok(())
}

/// Handles `Event::AddTechToPlayer`.
pub(super) fn add_tech_to_player(
    game_state: &mut GameState,
    player: PlayerId,
    tech: Technology,
) -> Result<()> {
    game_state.assert_enabled(&tech)?;
    ensure!(
        game_state.players.contains_key(&player),
        "Player does not exist?"
    );
    game_state.grant_tech(&player, tech)?;
    Ok(())
}

/// Handles `Event::RemoveTechFromPlayer`.
pub(super) fn remove_tech_from_player(
    game_state: &mut GameState,
    player: PlayerId,
    tech: Technology,
) -> Result<()> {
    let Some(p) = game_state.players.get_mut(&player) else {
        bail!("Player does not exist?");
    };

    p.remove_tech(&tech);
    Ok(())
}
