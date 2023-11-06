use std::collections::{HashMap, HashSet};

use eyre::{bail, ensure};

use crate::{
    data::components::{phase::Phase, strategy_card::StrategyCard, system::System},
    gameplay::{
        event::{StrategicPrimaryAction, StrategicSecondaryAction},
        game_state::{ActionPhaseProgress, StrategicPrimaryProgress, StrategicProgress},
        player::PlayerId,
    },
};

use super::{
    error::GameError,
    event::Event,
    game_state::{GameState, TacticalProgress},
};

const MIN_PLAYER_COUNT: usize = 3;
const MAX_PLAYER_COUNT: usize = 8;

pub fn update_game_state(game_state: &mut GameState, event: Event) -> Result<(), GameError> {
    match event {
        Event::AddPlayer { player } => {
            game_state.assert_phase(Phase::Setup)?;
            ensure!(
                game_state.players.len() <= MAX_PLAYER_COUNT,
                "can't have more than {MAX_PLAYER_COUNT} players"
            );
            let id: PlayerId = player.name.clone().into();
            game_state.table_order.push(id.clone());
            game_state.players.insert(id, player.into());
        }
        Event::StartGame => {
            game_state.assert_phase(Phase::Setup)?;
            ensure!(
                game_state.players.len() >= MIN_PLAYER_COUNT,
                "can't have less than {MIN_PLAYER_COUNT} players"
            );

            // TODO: in the future we should set this in the frontend.
            game_state.speaker = game_state
                .players
                .iter()
                .fold(None, |_, (id, _)| Some(id.clone()));
            game_state.current_player = game_state.speaker.clone();
            game_state.calculate_turn_order_from_speaker()?;
            game_state.phase = Phase::Strategy;
        }
        Event::TakeStrategyCard { player, card } => {
            game_state.assert_phase(Phase::Strategy)?;
            ensure!(
                !game_state.strategy_card_holders.contains_key(&card),
                "strategy card can't be picked twice"
            );
            game_state.strategy_card_holders.insert(card, player);
            game_state.advance_turn()?;
        }
        Event::CompleteStrategyPhase => {
            let how_many_card_must_pick = match game_state.players.len() {
                3 => 6,
                4 => 8,
                n => n,
            };
            if game_state.strategy_card_holders.len() != how_many_card_must_pick {
                eyre::bail!(
                    "can't complete strategy phase, all players have not selected strategy cards"
                );
            }
            game_state.calculate_action_turn_order()?;
            game_state.phase = Phase::Action;
        }
        Event::TacticalActionBegin { player } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            game_state.phase = Phase::TacticalAction;
            game_state.action_progress = Some(ActionPhaseProgress::Tactical(TacticalProgress {
                activated_system: None,
                taken_planets: vec![],
            }));
        }
        Event::TacticalActionTakePlanet { player, planet } => {
            game_state.assert_phase(Phase::TacticalAction)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.action_progress.is_some(),
                "Must have action progress to perform take planet action."
            );

            let planet_system = System::for_planet(&planet)?;

            // In case someone else currently owns the planet, remove it from them.
            game_state
                .players
                .values_mut()
                .for_each(|p| p.remove_planet(&planet));

            let current_player = game_state.get_current_player()?;
            current_player.planets.insert(planet.clone());

            match game_state.action_progress.as_mut() {
                Some(ActionPhaseProgress::Tactical(tactical)) => {
                    tactical.activated_system = Some(planet_system.id);
                    tactical.taken_planets.push(planet);
                }
                other => {
                    bail!("Invalid game state, expected tactical action, got {other:?}")
                }
            };
        }
        Event::TacticalActionCommit { player } => {
            game_state.assert_phase(Phase::TacticalAction)?;
            game_state.assert_player_turn(&player)?;
            game_state.action_progress = None;
            game_state.phase = Phase::Action;
            game_state.advance_turn()?;
        }
        Event::StrategicActionBegin { player, card } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.strategy_card_holders.get(&card) == Some(&player),
                "current player must hold the strategy card",
            );
            ensure!(
                !game_state.spent_strategy_cards.contains(&card),
                "strategy card can't already be used",
            );
            ensure!(
                game_state.action_progress.is_none(),
                "we are already performing an action phase action",
            );
            game_state.phase = Phase::StrategicAction;
            game_state.action_progress = Some(ActionPhaseProgress::Strategic(StrategicProgress {
                card,
                primary: None,
                other_players: Default::default(),
            }));
            game_state.spent_strategy_cards.insert(card);
        }
        Event::StrategicActionPrimary { player, action } => {
            game_state.assert_phase(Phase::StrategicAction)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.action_progress.is_some(),
                "Invalid game state: expected action_progress not to be None"
            );
            let progress = game_state.action_progress.as_mut().unwrap();
            let progress = match progress {
                ActionPhaseProgress::Strategic(p) => p,
                _ => bail!("Invalid action phase progress '{:?}'", progress),
            };

            ensure!(
                progress.primary.is_none(),
                "Primary action has already been performed"
            );

            match (progress.card.clone(), action) {
                (StrategyCard::Technology, StrategicPrimaryAction::Technology { tech, extra }) => {
                    /* Set the progress */
                    progress.primary = Some(StrategicPrimaryProgress::Technology {
                        tech: tech.clone(),
                        extra: extra.clone(),
                    });

                    /* Give the tech to the current player */
                    let current_player = game_state.get_current_player()?;

                    current_player.take_tech(tech.clone())?;
                    if let Some(t) = extra.clone() {
                        current_player.take_tech(t.clone())?;
                    }
                }
                (StrategyCard::Politics, StrategicPrimaryAction::Politics { new_speaker }) => {
                    progress.primary = Some(StrategicPrimaryProgress::Politics {
                        new_speaker: new_speaker.clone(),
                    });

                    game_state.speaker = Some(new_speaker);
                }
                (card, action) => {
                    bail!("Mismatch between progress card {card:?} and action {action:?}")
                }
            }
        }
        Event::StrategicActionSecondary { player, action } => {
            game_state.assert_phase(Phase::StrategicAction)?;
            let current_player = game_state.current_player()?;
            ensure!(
                &player != current_player,
                "current player can't perform the secondary on a strategy card",
            );

            let Some(action_progress) = &mut game_state.action_progress else {
                bail!("no strategic action in progress");
            };

            match action_progress {
                ActionPhaseProgress::Tactical { .. } => {
                    bail!("cannot perform strategic actions during a tactical action")
                }
                ActionPhaseProgress::Strategic(progress) => {
                    ensure!(
                        action.is_for_card(progress.card),
                        "Mismatch between strategic progress {progress:?} and action {action:?}"
                    );

                    progress
                        .other_players
                        .insert(player.clone(), action.clone().into());

                    match action {
                        StrategicSecondaryAction::Technology { tech } => {
                            let player = game_state.players.get_mut(&player).unwrap();
                            player.technologies.insert(tech);
                        }
                        _ => {}
                    }
                }
            };
        }
        Event::StrategicActionCommit => {
            game_state.assert_phase(Phase::StrategicAction)?;
            ensure!(
                game_state.action_progress.is_some(),
                "not currently performing an action",
            );
            ensure!(
                game_state
                    .action_progress
                    .as_ref()
                    .unwrap()
                    .is_strategy_card(),
                "not currently performing a strategic action"
            );
            game_state.phase = Phase::Action;
            game_state.action_progress = None;
            game_state.advance_turn()?;
        }
        Event::ComponentAction {
            player,
            component: _,
        } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            game_state.advance_turn()?;
        }
        Event::PassAction { player } => {
            game_state.assert_player_turn(&player)?;

            let has_used_strategy_cards = game_state
                .strategy_card_holders
                .iter()
                .filter(|(card, _)| !game_state.spent_strategy_cards.contains(card))
                .all(|(_, holder)| *holder != player);

            ensure!(
                has_used_strategy_cards,
                "player must use all strategy cards before passing"
            );

            game_state.passed_players.insert(player);
            game_state.advance_turn()?;
        }
        Event::CompleteStatusPhase => {
            // TODO: Require objectives scored & revealed

            // TODO: Agenda phase

            // Reset state
            game_state.phase = Phase::Strategy;
            game_state.calculate_turn_order_from_speaker()?;
            game_state.current_player = Some(game_state.speaker()?.clone());
            game_state.strategy_card_holders = HashMap::new();
            game_state.passed_players = HashSet::new();
            game_state.spent_strategy_cards = HashSet::new();
        }
    }
    Ok(())
}
