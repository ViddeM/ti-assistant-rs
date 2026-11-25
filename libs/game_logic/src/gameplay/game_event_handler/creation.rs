//! Creation phase events.

use std::collections::BTreeSet;

use anyhow::{Context, Result, ensure};
use ti_helper_game_data::{
    common::{
        faction::Faction,
        game_settings::GameSettings,
        milty_data::{MiltyData, MiltyPlayer},
        player_id::PlayerId,
    },
    components::{
        phase::Phase,
        system::{SystemType, systems},
    },
    state::{
        expedition::TEExpedition,
        game_state::{GameState, MapData, MiltyInformation},
        player::NewPlayer,
    },
};

use crate::gameplay::color_assignment::assign_colors;

/// Handles `Event::ImportFromMilty`.
pub(super) fn import_from_milty(
    game_state: &mut GameState,
    max_points: u32,
    milty_data: MiltyData,
) -> Result<()> {
    game_state.assert_phase(Phase::Creation)?;

    game_state.game_settings = GameSettings {
        max_points,
        expansions: milty_data.expansions.clone(),
    };
    let mut hex_map = milty_data.hex_map;
    hex_map.place_crimson_rebellion_home_system();
    if milty_data.expansions.thunders_edge {
        hex_map.place_the_fracture();
    }
    game_state.map_data = MapData {
        milty_information: Some(MiltyInformation {
            hex_map,
            mirage_system: None,
        }),
        destroyed_planets: vec![],
        ever_owned_planets: BTreeSet::new(),
    };

    // Sorted so that the outcome (e.g. color ties) doesn't depend on HashMap order.
    let mut players: Vec<&MiltyPlayer> = milty_data.players.values().collect();
    players.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.name.cmp(&b.name)));
    game_state.table_order = players.iter().map(|p| p.name.clone().into()).collect();

    let colors_map = assign_colors(players.iter().map(|p| p.faction).collect())
        .context("Failed to assign colors to factions")?;
    let new_players = players
        .iter()
        .map(|p| {
            Ok(NewPlayer {
                name: p.name.clone(),
                faction: p.faction,
                color: *colors_map.get(&p.faction).ok_or_else(|| {
                    anyhow::anyhow!("Faction {:?} did not get a color?", p.faction)
                })?,
            })
        })
        .collect::<anyhow::Result<Vec<NewPlayer>>>()?;

    game_state.players = new_players
        .into_iter()
        .map(|p| (p.name.clone().into(), p.setup(&milty_data.expansions)))
        .collect();

    game_state.speaker = Some(
        game_state
            .table_order
            .first()
            .cloned()
            .context("Expected there to be players from milty?")?,
    );

    if game_state.game_settings.expansions.thunders_edge {
        game_state.te_expedition = Some(TEExpedition::default());
    }
    Ok(())
}

/// Handles `Event::AddPlayer`.
pub(super) fn add_player(game_state: &mut GameState, player: NewPlayer) -> Result<()> {
    game_state.assert_phase(Phase::Creation)?;
    game_state.assert_enabled(&player.faction)?;
    ensure!(
        game_state.map_data.milty_information.is_none(),
        "Game was imported from milty, can't add more players!"
    );
    ensure!(
        game_state.players.len() < game_state.max_players(),
        "can't have more than {} players",
        game_state.max_players()
    );

    let id: PlayerId = player.name.clone().into();

    ensure!(
        !game_state.players.contains_key(&id),
        "a player called {id:?} already exists",
    );

    ensure!(
        game_state
            .players
            .values()
            .all(|other| other.faction != player.faction),
        "two players can't play the same faction",
    );

    game_state.table_order.push(id.clone());
    game_state
        .players
        .insert(id, player.setup(&game_state.game_settings.expansions));
    Ok(())
}

/// Handles `Event::CreationDone`.
pub(super) fn creation_done(game_state: &mut GameState) -> Result<()> {
    game_state.assert_phase(Phase::Creation)?;
    ensure!(
        game_state.players.len() >= MIN_PLAYER_COUNT,
        "can't have less than {MIN_PLAYER_COUNT} players"
    );
    let systems = systems();
    if let Some(map_data) = game_state.map_data.milty_information.as_ref() {
        let selected_factions: Vec<Faction> = game_state
            .players
            .values()
            .map(|player| player.faction)
            .collect();

        ensure!(
            map_data
                .hex_map
                .tiles
                .iter()
                .filter_map(|tile| systems.get(&tile.system))
                .filter_map(|system| match system.system_type {
                    SystemType::HomeSystem(faction) => Some(faction),
                    _ => None,
                })
                .all(|faction| selected_factions.contains(&faction)),
            "Factions from the milty draft map have not yet been picked"
        );
    }

    // We do not call change_phase here as we should not track time / calculate turn order here.
    game_state.phase = Phase::Setup;
    Ok(())
}

const MIN_PLAYER_COUNT: usize = 3;
