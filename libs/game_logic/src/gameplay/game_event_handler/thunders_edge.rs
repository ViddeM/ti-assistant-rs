//! Thunder's Edge events: the expedition, the Fracture, breakthroughs and coexistence.

use anyhow::{Context, Result, bail, ensure};

use ti_helper_game_data::components::system::SystemId;
use ti_helper_game_data::state::expedition::TEExpeditionSlot;
use ti_helper_game_data::{
    common::{expansions::Expansion, faction::Faction, player_id::PlayerId},
    components::{breakthrough::Breakthrough, phase::Phase, planet::Planet, system::System},
    state::{
        coexistence::Coexistence,
        expedition::{ExpeditionClaim, TECompletion},
        faction_state::DeepwroughtState,
        game_state::{ActionPhaseProgress, FractureProgress, GameState},
    },
};

/// Handles `Event::ClaimThundersEdgeExpeditionSlot`.
pub(super) fn claim_expedition_slot(
    game_state: &mut GameState,
    player: Option<PlayerId>,
    slot: TEExpeditionSlot,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;

    let round = game_state.round;
    let turn = game_state.turn_number;
    let te = game_state
        .te_expedition
        .as_mut()
        .expect("Thunders Edge enabled but not initialized");

    ensure!(
        te.progress.get(&slot).map(|claim| &claim.player) != player.as_ref(),
        "No change provided in event"
    );

    if let Some(player) = player {
        te.progress.insert(
            slot,
            ExpeditionClaim {
                player,
                round,
                turn,
                committed: false,
            },
        );
    } else {
        te.progress.remove(&slot);
        te.pending_placement = None;
    }
    Ok(())
}

/// Handles `Event::CommitToThundersEdgeExpedition`.
pub(super) fn commit_to_expedition(
    game_state: &mut GameState,
    player: PlayerId,
    slot: TEExpeditionSlot,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    game_state.assert_phase(Phase::EndActionTurn)?;
    game_state.assert_player_turn(&player)?;

    ensure!(
        !game_state.has_committed_to_thunders_edge(&player),
        "Player has already committed to the expedition this turn"
    );

    let round = game_state.round;
    let turn = game_state.turn_number;
    let te = game_state
        .te_expedition
        .as_mut()
        .expect("Thunders Edge enabled but not initialized");

    ensure!(
        !te.progress.contains_key(&slot),
        "Expedition slot is already claimed"
    );

    te.progress.insert(
        slot,
        ExpeditionClaim {
            player: player.clone(),
            round,
            turn,
            committed: true,
        },
    );
    if te.is_full() {
        te.pending_placement = Some(player.clone());
    }

    game_state.gain_breakthrough(&player)?;
    Ok(())
}

/// Handles `Event::CompleteThundersEdgeExpedition`.
pub(super) fn complete_expedition(
    game_state: &mut GameState,
    player: PlayerId,
    placed_in: SystemId,
    won_by: PlayerId,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    game_state.assert_phase(Phase::EndActionTurn)?;
    game_state.assert_player_turn(&player)?;

    let te = game_state
        .te_expedition
        .as_mut()
        .expect("Thunders Edge enabled but not initialized");

    ensure!(
        te.is_awaiting_placement(),
        "Thunder's Edge is not waiting to be placed"
    );
    ensure!(
        System::valid_thunders_edge_systems().contains(&placed_in),
        "System is not a valid candidate for Thunder's Edge"
    );

    let top_contributors = te
        .get_top_expedition_contributors()
        .context("Not all expedition slots are claimed")?;
    ensure!(
        top_contributors.contains(&won_by),
        "Only a player with the most claimed slots can win the expedition"
    );

    te.completed = Some(TECompletion {
        won_by: won_by.clone(),
        placed_in,
    });
    te.pending_placement = None;
    game_state.set_planet_owner(Planet::ThundersEdge, Some(won_by.clone()))?;
    game_state.gain_breakthrough(&won_by)?;
    Ok(())
}

/// Handles `Event::SetThundersEdgeCompletion`.
pub(super) fn set_completion(
    game_state: &mut GameState,
    new_completion: Option<TECompletion>,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    let te = game_state
        .te_expedition
        .as_mut()
        .expect("Thunders Edge enabled but not initialized");

    ensure!(
        te.completed != new_completion,
        "No change provided in event"
    );

    if let Some(comp) = new_completion.as_ref() {
        // Validate player & system.
        let valid_systems = System::valid_thunders_edge_systems();
        ensure!(
            valid_systems.contains(&comp.placed_in),
            "System is not a valid candidate for Thunder's Edge"
        );

        ensure!(
            game_state.players.contains_key(&comp.won_by),
            "Player not found"
        );
    } else {
        ensure!(
            !te.is_full(),
            "Cannot uncomplete Thunders Edge expedition with all slots claimed"
        );
    }

    let changed_winner =
        (te.completed.as_ref()).map(|c| &c.won_by) != (new_completion.as_ref()).map(|c| &c.won_by);
    te.completed = new_completion.clone();
    if changed_winner {
        game_state.set_planet_owner(Planet::ThundersEdge, new_completion.map(|c| c.won_by))?;
    }
    Ok(())
}

/// Handles `Event::SetBreakthrough`.
pub(super) fn set_breakthrough(
    game_state: &mut GameState,
    player: PlayerId,
    breakthrough: Option<Breakthrough>,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    if let Some(b) = &breakthrough {
        game_state.assert_enabled(b)?;
    }
    let player_state = game_state
        .players
        .get_mut(&player)
        .context("Player not found")?;

    ensure!(
        breakthrough != player_state.breakthrough,
        "Cannot set breakthrough to None"
    );

    let previous = std::mem::replace(&mut player_state.breakthrough, breakthrough.clone());
    if previous.is_none()
        && let Some(b) = &breakthrough
    {
        game_state.apply_breakthrough_gained(&player, b)?;
    }
    Ok(())
}

/// Handles `Event::RollForTheFracture`.
pub(super) fn roll_for_the_fracture(
    game_state: &mut GameState,
    player: PlayerId,
    entered_play: bool,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;

    let Some(progress) = &mut game_state.action_progress else {
        bail!("No action in progress for the Fracture roll");
    };

    let ActionPhaseProgress::Fracture(state) = progress else {
        bail!("Action progress is not fracture roll");
    };

    let FractureProgress::Roll(roll_state) = &mut state.progress else {
        bail!("Got roll for fracture during non-roll fracture state");
    };

    ensure!(
        roll_state.current_player() == &player,
        "Wrong player rolled, expected {} got {player}",
        roll_state.current_player()
    );

    if entered_play {
        game_state.fracture_in_play = true;
        state.progress = FractureProgress::Spawned { player };
    } else if roll_state.next_player().is_none() {
        let state = state.clone();
        game_state.end_fracture(state);
    }
    Ok(())
}

/// Handles `Event::FractureSpawned`.
pub(super) fn fracture_spawned(game_state: &mut GameState, player: PlayerId) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;

    let Some(progress) = game_state.action_progress.as_ref() else {
        bail!("No action in progress for the Fracture roll");
    };

    let ActionPhaseProgress::Fracture(state) = progress else {
        bail!("Action progress is not fracture roll");
    };

    let FractureProgress::Spawned { player: spawner } = &state.progress else {
        bail!("Invalid fracture progress");
    };

    ensure!(
        spawner == &player,
        "Wrong player spawns the fracture, expected {spawner} got {player}"
    );

    let state = state.clone();
    game_state.end_fracture(state);
    Ok(())
}

/// Handles `Event::CoexistenceBegin`.
pub(super) fn coexistence_begin(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    let owner = (game_state.players.iter())
        .find(|(_, p)| p.planets.contains_key(&planet))
        .map(|(id, _)| id.clone());
    ensure!(
        owner.is_some(),
        "{} isn't controlled by a player",
        planet.info().name
    );
    ensure!(
        owner.as_ref() != Some(&player),
        "{player} can't coexist on a planet they control"
    );
    let coexistence = Coexistence {
        planet: planet.clone(),
        player: player.clone(),
    };
    ensure!(
        !game_state.coexistences.contains(&coexistence),
        "{player} is already coexisting on {}",
        planet.info().name
    );

    let coexisting_player = game_state
        .players
        .get_mut(&player)
        .context("Player not found")?;
    if coexisting_player.faction == Faction::DeepwroughtScholarate {
        let oceans = coexisting_player.ocean_cards();
        let max = DeepwroughtState::MAX_OCEAN_CARDS;
        coexisting_player.set_ocean_cards((oceans + 1).min(max))?;
    }
    game_state.coexistences.insert(coexistence);
    Ok(())
}

/// Handles `Event::CoexistenceEnd`.
pub(super) fn coexistence_end(
    game_state: &mut GameState,
    player: PlayerId,
    planet: Planet,
) -> Result<()> {
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    let coexistence = Coexistence {
        planet: planet.clone(),
        player: player.clone(),
    };
    ensure!(
        game_state.coexistences.contains(&coexistence),
        "{player} isn't coexisting on {}",
        planet.info().name
    );
    game_state.coexistences.remove(&coexistence);

    let remaining = (game_state.coexistences.iter())
        .filter(|c| c.player == player)
        .count() as u8;
    let coexisting_player = game_state
        .players
        .get_mut(&player)
        .context("Player not found")?;
    if coexisting_player.faction == Faction::DeepwroughtScholarate {
        let oceans = coexisting_player.ocean_cards();
        if oceans > remaining {
            coexisting_player.set_ocean_cards(remaining)?;
        }
    }
    Ok(())
}
