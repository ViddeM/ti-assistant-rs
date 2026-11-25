//! Setup phase events.

use std::{borrow::BorrowMut, collections::BTreeSet};

use anyhow::{Result, bail, ensure};
use chrono::{DateTime, Utc};

use crate::gameplay::error::GameError;

use ti_helper_game_data::{
    common::{faction::Faction, map::HexMap, player_id::PlayerId},
    components::{
        galactic_event::GalacticEvent,
        objectives::{Objective, ObjectiveKind, PUBLIC_OBJECTIVES_PER_STAGE},
        phase::Phase,
        relic::Relic,
        system::{SystemId, SystemType, systems},
        tech::Technology,
    },
    state::game_state::GameState,
};

/// Handles `Event::SetupTheTribunii`.
pub(super) fn setup_the_tribunii(
    game_state: &mut GameState,
    player: PlayerId,
    faction: Faction,
) -> Result<()> {
    game_state.assert_phase(Phase::Setup)?;
    let holder =
        (game_state.players.iter()).find(|(_, other)| other.minor_faction == Some(faction));
    if let Some((holder, _)) = holder {
        bail!("{} is {holder}'s minor faction", faction.name());
    }
    let Some(player) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist!")
    };

    ensure!(
        player.planets.is_empty(),
        "Player already has planets selected!"
    );

    ensure!(
        player.faction == Faction::CouncilKeleres,
        "The Tribunii can only be performed by The Council Keleres"
    );

    let available_factions = [
        Faction::MentakCoalition,
        Faction::XxchaKingdom,
        Faction::ArgentFlight,
    ];

    ensure!(
        available_factions.contains(&faction),
        "Selected faction must be one of [{available_factions:?}]"
    );

    // TODO: Also give the player the correct agent
    for planet in faction.get_starting_planets().into_iter() {
        player.planets.insert(planet, BTreeSet::new());
    }
    player.home_system_from = Some(faction);

    if let Some(milty) = game_state.map_data.milty_information.as_mut() {
        let home_system = systems()
            .into_values()
            .find(|system| system.system_type == SystemType::HomeSystem(faction));
        if let Some(home_system) = home_system {
            milty.hex_map.place_keleres_home_system(home_system.id);
        }
    }
    Ok(())
}

/// Handles `Event::SetupCulturalExchange`.
pub(super) fn setup_cultural_exchange(
    game_state: &mut GameState,
    player: PlayerId,
    faction: Faction,
) -> Result<()> {
    game_state.assert_phase(Phase::Setup)?;
    game_state.assert_galactic_event(&GalacticEvent::CulturalExchangeProgram)?;
    let Some(current) = game_state.players.get(&player) else {
        bail!("Player doesn't exist '{player}'");
    };
    if let Some(drawn) = current.leaders_from {
        bail!(
            "{player} has already drawn {}, use Undo to change it",
            drawn.name()
        );
    }
    ensure!(
        current.faction != Faction::FirmamentObsidian && faction != Faction::FirmamentObsidian,
        "The Firmament doesn't take part in the Cultural Exchange Program"
    );
    ensure!(
        game_state.players.values().any(|p| p.faction == faction),
        "{} isn't in play",
        faction.name()
    );
    let holder = (game_state.players.iter()).find(|(_, p)| p.leaders_from == Some(faction));
    if let Some((holder, _)) = holder {
        bail!("{} has already been drawn by {holder}", faction.name());
    }

    let Some(player) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist '{player}'");
    };
    player.leaders_from = Some(faction);
    Ok(())
}

/// Handles `Event::SetupMinorFaction`.
pub(super) fn setup_minor_faction(
    game_state: &mut GameState,
    player: PlayerId,
    faction: Faction,
    replaced_system: Option<SystemId>,
) -> Result<()> {
    game_state.assert_phase(Phase::Setup)?;
    game_state.assert_galactic_event(&GalacticEvent::MinorFactions)?;
    let Some(current) = game_state.players.get(&player) else {
        bail!("Player doesn't exist '{player}'");
    };
    if let Some(current) = current.minor_faction {
        bail!(
            "{} is already {player}'s minor faction, use Undo to change it",
            current.name()
        );
    }
    game_state.assert_enabled(&faction)?;
    ensure!(
        game_state.is_minor_faction_available(&player, faction),
        "{} can't be {player}'s minor faction: it has to be out of play, not another player's minor faction and have a home system with no controlled planets",
        faction.name()
    );

    match (
        game_state.map_data.milty_information.as_mut(),
        replaced_system,
    ) {
        (Some(milty), Some(replaced_system)) => {
            ensure!(
                milty
                    .hex_map
                    .minor_faction_slots()
                    .contains(&replaced_system),
                "System {replaced_system} isn't a system on the map that a minor faction system can replace"
            );
            ensure!(
                HexMap::minor_faction_system(faction).is_some(),
                "{} has no system to put on the map",
                faction.name()
            );
            milty
                .hex_map
                .place_minor_faction_system(faction, &replaced_system);
        }
        (Some(_), None) => {
            bail!("Choose the system the minor faction system replaces on the map")
        }
        (None, Some(_)) => {
            bail!("Only games imported from milty have a map to place the minor faction system on")
        }
        (None, None) => {}
    }

    let Some(player) = game_state.players.get_mut(&player) else {
        bail!("Player doesn't exist '{player}'");
    };
    player.minor_faction = Some(faction);
    Ok(())
}

/// Handles `Event::SetupPlayerTechs`.
pub(super) fn setup_player_techs(
    game_state: &mut GameState,
    player: PlayerId,
    technologies: Vec<Technology>,
) -> Result<()> {
    let player_id = player;

    game_state.assert_phase(Phase::Setup)?;
    ensure!(
        !game_state.player_initialization_finished(&player_id)?,
        "Player has already performed all necessary initialization"
    );

    let Some(player) = game_state.players.get(&player_id) else {
        bail!("Player does not exist!");
    };

    let allowed_techs = player.faction.get_starting_tech_choice_count();
    ensure!(
        technologies.len() == allowed_techs,
        "Invalid amount of technologies selected for faction"
    );

    let possible_techs = player.faction.get_starting_tech_options(&game_state);
    if let Some(possible_techs) = possible_techs {
        ensure!(
            technologies.iter().all(|t| possible_techs.contains(t)),
            "Invalid technology selection for players faction"
        );
    }

    ensure!(
        player.technologies().is_empty(),
        "Player already has technologies?"
    );

    for tech in technologies.into_iter() {
        game_state.gain_tech(&player_id, tech)?;
    }
    Ok(())
}

/// Handles `Event::RevealInitialObjectives`.
pub(super) fn reveal_initial_objectives(
    game_state: &mut GameState,
    first_objective: Objective,
    second_objective: Objective,
) -> Result<()> {
    game_state.assert_phase(Phase::Setup)?;
    ensure!(
        !game_state
            .galactic_events
            .in_play
            .contains(&GalacticEvent::CivilizedSociety),
        "Civilized Society reveals all public objectives at setup"
    );
    ensure!(
        game_state.score.revealed_objectives.is_empty(),
        "Objectives have already been revealed"
    );

    ensure!(
        first_objective.info().kind == ObjectiveKind::StageI,
        "Invalid starting objective"
    );
    game_state.assert_enabled(&first_objective)?;
    ensure!(
        second_objective.info().kind == ObjectiveKind::StageI,
        "Invalid starting objective"
    );
    game_state.assert_enabled(&second_objective)?;

    let score = game_state.score.borrow_mut();
    score
        .revealed_objectives
        .insert(first_objective, BTreeSet::new());
    score
        .revealed_objectives
        .insert(second_objective, BTreeSet::new());
    Ok(())
}

/// Handles `Event::RevealAllPublicObjectives`.
pub(super) fn reveal_all_public_objectives(
    game_state: &mut GameState,
    stage_one: BTreeSet<Objective>,
    stage_two: BTreeSet<Objective>,
) -> Result<()> {
    game_state.assert_phase(Phase::Setup)?;
    game_state.assert_galactic_event(&GalacticEvent::CivilizedSociety)?;
    ensure!(
        game_state.score.revealed_objectives.is_empty(),
        "Objectives have already been revealed"
    );

    for (objectives, kind, stage) in [
        (&stage_one, ObjectiveKind::StageI, "I"),
        (&stage_two, ObjectiveKind::StageII, "II"),
    ] {
        ensure!(
            objectives.len() == PUBLIC_OBJECTIVES_PER_STAGE,
            "Expected {PUBLIC_OBJECTIVES_PER_STAGE} stage {stage} objectives, got {}",
            objectives.len()
        );
        for objective in objectives {
            game_state.assert_enabled(objective)?;
            ensure!(
                objective.info().kind == kind,
                "{} isn't a stage {stage} objective",
                objective.info().name
            );
        }
    }

    for objective in stage_one.into_iter().chain(stage_two) {
        game_state
            .score
            .revealed_objectives
            .insert(objective, BTreeSet::new());
    }
    Ok(())
}

/// Handles `Event::StartGame`.
pub(super) fn start_game(game_state: &mut GameState, timestamp: DateTime<Utc>) -> Result<()> {
    game_state.assert_phase(Phase::Setup)?;

    ensure!(game_state.speaker.is_some(), "No speaker has been set!");
    ensure!(
        !game_state.score.revealed_objectives.is_empty(),
        "No objectives have been revealed"
    );

    let uninitialized_players = game_state
        .players
        .keys()
        .map(|p| {
            game_state
                .player_initialization_finished(p)
                .map(|ok| if ok { None } else { Some(p) })
        })
        .collect::<Result<Vec<Option<&PlayerId>>, GameError>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<&PlayerId>>();

    ensure!(
        uninitialized_players.is_empty(),
        format!(
            "All initialization has not yet been completed for players {uninitialized_players:?}"
        )
    );

    let age_of_fighters = game_state
        .galactic_events
        .in_play
        .contains(&GalacticEvent::AgeOfFighters);
    let advent_of_the_war_sun = game_state
        .galactic_events
        .in_play
        .contains(&GalacticEvent::AdventOfTheWarSun);
    let event_techs = (game_state.players.iter())
        .flat_map(|(id, player)| {
            let fighter = match player.faction {
                Faction::NaaluCollective => Technology::HybridCrystalFighterII,
                _ => Technology::FighterII,
            };
            let war_sun = player.faction != Faction::EmbersOfMuaat;
            (age_of_fighters.then_some(fighter))
                .into_iter()
                .chain((advent_of_the_war_sun && war_sun).then_some(Technology::WarSun))
                .map(|tech| (id.clone(), tech))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for (player, tech) in event_techs {
        game_state.grant_tech(&player, tech)?;
    }

    if game_state
        .galactic_events
        .in_play
        .contains(&GalacticEvent::CulturalExchangeProgram)
    {
        let firmament = (game_state.players.iter())
            .find(|(_, player)| player.faction == Faction::FirmamentObsidian)
            .map(|(id, _)| id.clone());
        if let Some(firmament) = firmament {
            game_state.set_relic_owner(Relic::TheObsidian, Some(firmament))?;
        }
    }

    if game_state
        .galactic_events
        .in_play
        .contains(&GalacticEvent::RapidMobilization)
    {
        game_state.fracture_in_play = true;
        let players = game_state.players.keys().cloned().collect::<Vec<_>>();
        for player in &players {
            game_state.gain_breakthrough(player)?;
        }
        game_state.galactic_events.rapid_mobilization = players.into_iter().collect();
    }

    if game_state.has_post_setup_phase() {
        game_state.phase = Phase::PostSetup;
    } else {
        game_state.change_phase(Phase::Strategy, timestamp)?;
    }

    let with_breakthrough = (game_state.players.iter())
        .filter(|(_, player)| player.breakthrough.is_some())
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();
    for player in with_breakthrough {
        game_state.roll_for_the_fracture(&player)?;
    }
    Ok(())
}
