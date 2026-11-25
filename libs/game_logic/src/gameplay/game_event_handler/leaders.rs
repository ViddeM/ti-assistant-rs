//! Leader events.

use anyhow::{Context, Result, bail, ensure};
use ti_helper_game_data::{
    common::{expansions::Expansion, player_id::PlayerId},
    components::{
        leaders::{Leader, LeaderAbilityKind},
        phase::Phase,
    },
    state::game_state::{ActionPhaseProgress, GameState, LeaderProgress},
};

/// Handles `Event::LeaderActionBegin`.
pub(super) fn begin(game_state: &mut GameState, player: PlayerId, leader: Leader) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    game_state.assert_enabled(&leader)?;
    ensure!(
        leader.info().ability_kind() == LeaderAbilityKind::Action,
        "{}'s ability can't be played as an action",
        leader.info().name(),
    );
    ensure!(
        game_state
            .available_leaders
            .get(&player)
            .is_some_and(|leaders| leaders.contains(&leader)),
        "{} isn't available to {player}",
        leader.info().name(),
    );

    // TODO: the following leaders may need special handling:
    //   XxekirGrom - discard law from play (sure, absolutely)
    //   UlTheProgenitor - attach to planet (mhm)
    //   UnitDsgnFlayesh - take a planet (ok, easy enough)
    //   DannelOfTheTenthCxIII - take several planets (ehm, what)
    //   RinTheMastersLegacy - switch out techs (oh god)
    //   ConservatorProcyon - explore frontier tokens (yes, plural...)
    //   MathisMathinus - perform a strategic action (WHAT)
    //   ZeuCxIII - any one player gets to take a tactical acton (WTF)
    //   HeshAndPrit - draw relic and do <=2 secondary strategy actions (NO! GOD NO!)
    //
    // TODO: when implementing this, add a LeaderAction phase
    //game_state.action_progress =
    //    Some(ActionPhaseProgress::Leader(LeaderProgress::Nothing {
    //        leader,
    //    }));
    //game_state.phase = Phase::LeaderAction;
    game_state.phase = Phase::EndActionTurn;
    Ok(())
}

/// Handles `Event::LeaderActionCommit`.
pub(super) fn commit(game_state: &mut GameState, player: PlayerId, leader: Leader) -> Result<()> {
    game_state.assert_phase(Phase::LeaderAction)?;
    game_state.assert_player_turn(&player)?;
    let Some(ActionPhaseProgress::Leader(LeaderProgress::Nothing {
        leader: previous_leader,
    })) = &game_state.action_progress
    else {
        bail!("action_progress not set to Leader, this is a bug");
    };
    if &leader != previous_leader {
        bail!("invalid leader {leader:?}, expected {previous_leader:?}");
    }

    game_state.action_progress = None;
    game_state.phase = Phase::EndActionTurn;
    Ok(())
}

/// Handles `Event::PuppetsOfTheBlade`.
pub(super) fn puppets_of_the_blade(game_state: &mut GameState, player: PlayerId) -> Result<()> {
    game_state.assert_phase(Phase::Action)?;
    game_state.assert_player_turn(&player)?;
    game_state.assert_expansion(&Expansion::ThundersEdge)?;
    ensure!(
        game_state.action_progress.is_none(),
        "Invalid game state, action_progress not None during Action Phase"
    );

    game_state
        .players
        .get_mut(&player)
        .context("Player not found")?
        .become_obsidian()?;

    // The planets are flipped no matter who controls them.
    for player in game_state.players.values_mut() {
        player.flip_home_system_planets();
    }

    // The home system tile is flipped as well.
    if let Some(milty) = game_state.map_data.milty_information.as_mut() {
        for tile in milty.hex_map.tiles.iter_mut().filter(|t| t.system == "96A") {
            tile.system = "96B".to_string();
        }
    }

    game_state.update_available_leaders();
    game_state.phase = Phase::EndActionTurn;
    Ok(())
}
