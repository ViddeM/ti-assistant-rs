//! Galactic event events.

use std::collections::BTreeSet;

use anyhow::{Context, Result, bail, ensure};

use ti_helper_game_data::{
    actions::event::GalacticEventAction,
    common::{
        component::Component,
        map::{Coordinate, HexPosition, Tile},
        player_id::PlayerId,
    },
    components::{
        galactic_event::GalacticEvent, phase::Phase, planet::Planet, system::systems,
        tech::Technology,
    },
    state::{
        galactic_events::RapidMobilizationResolution,
        game_state::{GameState, PlanetLocation},
    },
};

/// Handles `Event::PlayGalacticEventAction`.
pub(super) fn play_action(
    game_state: &mut GameState,
    player: PlayerId,
    action: GalacticEventAction,
) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    ensure!(
        game_state.action_progress.is_none(),
        "Invalid game state, action_progress not None during Action Phase"
    );
    game_state.assert_galactic_event(&action.event())?;

    match action {
        GalacticEventAction::TotalWar => {
            let points = game_state
                .score
                .galactic_events
                .total_war
                .entry(player)
                .or_default();
            *points = points.saturating_add(1);
        }
        GalacticEventAction::MercenariesForHire => {}
        GalacticEventAction::StellarAtomics => {
            ensure!(
                !game_state
                    .galactic_events
                    .stellar_atomics_used
                    .contains(&player),
                "{player} has already used their Stellar Atomics token"
            );
            game_state
                .galactic_events
                .stellar_atomics_used
                .insert(player);
        }
        GalacticEventAction::ConventionsOfWarAbandoned { planet } => {
            game_state.assert_enabled(&planet)?;
            ensure!(
                game_state
                    .players
                    .get(&player)
                    .is_some_and(|p| p.has_x89_bacterial_weapon()),
                "{player} doesn't own X-89 Bacterial Weapon"
            );
            ensure!(
                game_state.planet_location(&planet) != PlanetLocation::NotOnBoard,
                "{} isn't on the board",
                planet.info().name
            );
            game_state.destroy_planet(planet);
        }
        GalacticEventAction::AgeOfExploration { tile } => {
            ensure!(
                game_state
                    .players
                    .get(&player)
                    .is_some_and(|p| p.has_tech(&Technology::DarkEnergyTap)),
                "{player} doesn't own Dark Energy Tap"
            );
            let minor_factions = game_state.minor_factions();
            match (game_state.map_data.milty_information.as_mut(), tile) {
                (Some(milty), Some((system, coord))) => {
                    ensure!(
                        systems().get(&system).is_some_and(|s| {
                            s.is_enabled_in(&game_state.game_settings.expansions)
                                && milty.hex_map.is_unused_red_or_blue_tile(s)
                        }),
                        "System {system} isn't an unused red or blue tile"
                    );
                    let coord = Coordinate {
                        rotation: 0,
                        ..coord
                    };
                    ensure!(
                        milty
                            .hex_map
                            .exploration_positions(&minor_factions)
                            .contains(&coord),
                        "Ring {}, position {} isn't an empty spot touching at least 2 \
                         non-home systems",
                        coord.ring,
                        coord.position
                    );
                    milty.hex_map.tiles.push(Tile {
                        system,
                        position: HexPosition::Pos(coord),
                    });
                }
                (Some(_), None) => {
                    bail!("Choose the drawn tile and where it goes on the map")
                }
                (None, Some(_)) => {
                    bail!("Only games imported from milty have a map to place the tile on")
                }
                (None, None) => {}
            }
        }
    }

    game_state.phase = Phase::EndActionTurn;
    Ok(())
}

/// Handles `Event::ResolveRapidMobilization`.
pub(super) fn resolve_rapid_mobilization(
    game_state: &mut GameState,
    player: PlayerId,
    planets: BTreeSet<Planet>,
    technologies: BTreeSet<Technology>,
) -> Result<()> {
    game_state.assert_phase(Phase::PostSetup)?;
    ensure!(
        game_state
            .galactic_events
            .rapid_mobilization
            .contains(&player),
        "{player} has no Rapid Mobilization to resolve"
    );
    for planet in &planets {
        game_state.assert_enabled(planet)?;
        ensure!(
            game_state.is_rapid_mobilization_planet(planet),
            "{} isn't an unowned non-home planet on the board",
            planet.info().name
        );
    }
    let researcher = game_state
        .players
        .get(&player)
        .context("Player doesn't exist")?;
    let count = researcher.faction.rapid_mobilization_research_count();
    ensure!(
        technologies.len() == count,
        "{player} researches {count} technologies with Rapid Mobilization"
    );
    for tech in &technologies {
        game_state.assert_enabled(tech)?;
        researcher.assert_can_research(tech)?;
        ensure!(
            !researcher.has_tech(tech),
            "{player} already owns {}",
            tech.info().name
        );
    }

    let resolution = RapidMobilizationResolution {
        planets: planets.iter().cloned().collect(),
        technologies: technologies.iter().cloned().collect(),
    };
    for planet in planets {
        game_state.set_planet_owner(planet, Some(player.clone()))?;
    }
    for tech in technologies {
        game_state.research_tech(&player, tech)?;
    }
    game_state
        .galactic_events
        .rapid_mobilization
        .remove(&player);
    game_state
        .galactic_events
        .rapid_mobilization_resolved
        .insert(player, resolution);
    Ok(())
}

/// Handles `Event::SetStellarAtomicsUsed`.
pub(super) fn set_stellar_atomics_used(
    game_state: &mut GameState,
    player: PlayerId,
    used: bool,
) -> Result<()> {
    game_state.assert_galactic_event(&GalacticEvent::StellarAtomics)?;
    ensure!(
        game_state.players.contains_key(&player),
        "Player doesn't exist '{player}'"
    );
    let changed = if used {
        game_state
            .galactic_events
            .stellar_atomics_used
            .insert(player)
    } else {
        game_state
            .galactic_events
            .stellar_atomics_used
            .remove(&player)
    };
    ensure!(changed, "No change provided in event");
    Ok(())
}

/// Handles `Event::SetZealousOrthodoxyOwner`.
pub(super) fn set_zealous_orthodoxy_owner(
    game_state: &mut GameState,
    player: Option<PlayerId>,
) -> Result<()> {
    game_state.assert_galactic_event(&GalacticEvent::ZealousOrthodoxy)?;
    if let Some(p) = player.as_ref() {
        ensure!(
            game_state.players.contains_key(p),
            "Player doesn't exist '{p}'"
        );
    }
    let zealous_orthodoxy = &mut game_state.score.galactic_events.zealous_orthodoxy;
    ensure!(*zealous_orthodoxy != player, "No change provided in event");
    *zealous_orthodoxy = player;
    Ok(())
}

/// Handles `Event::SetTotalWarPoints`.
pub(super) fn set_total_war_points(
    game_state: &mut GameState,
    player: PlayerId,
    points: i8,
) -> Result<()> {
    game_state.assert_galactic_event(&GalacticEvent::TotalWar)?;
    ensure!(
        game_state.players.contains_key(&player),
        "Player doesn't exist '{player}'"
    );
    ensure!(points >= 0, "Total War points can't be negative");
    let total_war = &mut game_state.score.galactic_events.total_war;
    ensure!(
        total_war.get(&player).copied().unwrap_or(0) != points,
        "No change provided in event"
    );
    if points == 0 {
        total_war.remove(&player);
    } else {
        total_war.insert(player, points);
    }
    Ok(())
}

/// Handles `Event::BuildMonument`.
pub(super) fn build_monument(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
) -> Result<()> {
    game_state.assert_galactic_event(&GalacticEvent::MonumentsToTheAges)?;
    let Some(player_state) = game_state.players.get(&player) else {
        bail!("Player doesn't exist '{player}'");
    };
    let name = planet.info().name;
    ensure!(
        player_state.planets.contains_key(&planet),
        "{player} doesn't control {name}"
    );
    ensure!(
        !game_state.galactic_events.monuments.contains_key(&planet),
        "{name} already has a monument"
    );

    let started = game_state.status_phases_started();
    game_state
        .galactic_events
        .monuments
        .insert(planet, i64::from(started));
    Ok(())
}

/// Handles `Event::DestroyMonument`.
pub(super) fn destroy_monument(game_state: &mut GameState, planet: Planet) -> Result<()> {
    game_state.assert_galactic_event(&GalacticEvent::MonumentsToTheAges)?;
    ensure!(
        game_state
            .galactic_events
            .monuments
            .remove(&planet)
            .is_some(),
        "{} has no monument",
        planet.info().name
    );
    Ok(())
}

/// Handles `Event::SetMonumentCommodities`.
pub(super) fn set_monument_commodities(
    game_state: &mut GameState,
    planet: Planet,
    commodities: u32,
) -> Result<()> {
    game_state.assert_galactic_event(&GalacticEvent::MonumentsToTheAges)?;
    let name = planet.info().name;
    let Some(current) = game_state.monument_commodities(&planet) else {
        bail!("{name} has no monument");
    };
    ensure!(current != commodities, "No change provided in event");
    let started = game_state.status_phases_started();
    game_state
        .galactic_events
        .monuments
        .insert(planet, i64::from(started) - i64::from(commodities));
    Ok(())
}
