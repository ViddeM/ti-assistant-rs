use std::collections::{HashMap, HashSet};

use eyre::{bail, ensure};

use crate::{
    data::components::{
        action_card::{ActionCard, ActionCardPlay},
        phase::Phase,
        planet::Planet,
        strategy_card::StrategyCard,
        system::System,
        tech::{TechOrigin, TechType},
    },
    gameplay::{
        event::{StrategicPrimaryAction, StrategicSecondaryAction},
        game_state::{
            ActionCardProgress, ActionPhaseProgress, StrategicPrimaryProgress, StrategicProgress,
        },
        player::PlayerId,
    },
};

use super::{
    error::GameError,
    event::{ActionCardInfo, Event},
    game_state::{ActionCardState, GameState, TacticalProgress},
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
            game_state.current_player = game_state.turn_order.first().cloned();
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

            let Some(ActionPhaseProgress::Tactical(tactical)) = &mut game_state.action_progress
            else {
                bail!(
                    "Invalid game state, expected tactical action, got {:?}",
                    game_state.action_progress
                );
            };

            let Some(current_player_id) = &game_state.current_player else {
                bail!("no current player");
            };

            let Some(current_player) = game_state.players.get_mut(current_player_id) else {
                bail!("invalid game state, expected current player (id: {current_player_id:?}) to be in the players map")
            };

            let planet_system = System::for_planet(&planet)?;

            current_player.planets.insert(planet.clone());

            // In case someone else currently owns the planet, remove it from them.
            game_state
                .players
                .iter_mut()
                .filter(|&(id, _)| id != current_player_id)
                .for_each(|(_, p)| p.remove_planet(&planet));

            // Give the current player Custodians if he is the first to take Mecatol Rex
            if let Planet::MecatolRex = planet {
                if game_state.score.custodians.is_none() {
                    game_state.score.custodians = Some(current_player_id.clone());
                }
            }

            tactical.activated_system = Some(planet_system.id);
            tactical.taken_planets.push(planet);
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

            match (progress.card, action) {
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
                ActionPhaseProgress::Strategic(progress) => {
                    ensure!(
                        action.is_for_card(progress.card),
                        "Mismatch between strategic progress {progress:?} and action {action:?}"
                    );

                    progress
                        .other_players
                        .insert(player.clone(), action.clone().into());

                    // TODO: add other relevant strategy cards (initially Imperial)
                    match action {
                        StrategicSecondaryAction::Technology { tech } => {
                            let player = game_state.players.get_mut(&player).unwrap();
                            player.take_tech(tech)?;
                        }
                        _ => {}
                    }
                }
                _ => bail!("cannot perform strategic actions during non-strategy actions"),
            };
        }
        Event::StrategicActionCommit => {
            game_state.assert_phase(Phase::StrategicAction)?;

            let Some(ActionPhaseProgress::Strategic(progress)) = &game_state.action_progress else {
                bail!("not currently performing a strategic action");
            };

            let has_primary = matches!(
                progress.card,
                StrategyCard::Politics | StrategyCard::Technology
            );

            if has_primary {
                ensure!(
                    progress.primary.is_some(),
                    "has not performed strategy card primary"
                );
            }

            game_state.phase = Phase::Action;
            game_state.action_progress = None;
            game_state.advance_turn()?;
        }
        Event::ActionCardActionBegin { player, card } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;

            let card_info = card.info();
            ensure!(
                card_info.play == ActionCardPlay::Action,
                "card cannot be played as an action"
            );

            game_state.action_progress =
                Some(ActionPhaseProgress::ActionCard(ActionCardProgress {
                    card,
                    state: None,
                }));
            game_state.phase = Phase::ActionCardAction;
        }
        Event::ActionCardActionPerform { player, data } => {
            game_state.assert_phase(Phase::ActionCardAction)?;
            game_state.assert_player_turn(&player)?;

            // TODO: Implement Plagiarize and DivertFunding
            if let Some(ActionPhaseProgress::ActionCard(progress)) = &game_state.action_progress {
                ensure!(
                    progress.state.is_none(),
                    "Action has already been performed"
                );

                ensure!(
                    data.is_for_card(&progress.card),
                    "Trying to perform action that does not match the current action card being played"
                );
            } else {
                bail!("Not currently performing an action card action");
            }

            let new_state = match data {
                ActionCardInfo::FocusedResearch { tech } => {
                    let current_player = game_state.get_current_player()?;
                    current_player.take_tech(tech.clone())?;
                    ActionCardState::FocusedResearch { tech: tech.clone() }
                }
                ActionCardInfo::DivertFunding {
                    remove_tech,
                    take_tech,
                } => {
                    let current_player = game_state.get_current_player()?;
                    ensure!(
                        current_player.has_tech(&remove_tech),
                        "Player doesn't have technology {remove_tech:?}"
                    );
                    ensure!(
                        !(current_player.has_tech(&take_tech) && take_tech != remove_tech),
                        "Player already has technology {take_tech:?}"
                    );
                    let remove_tech_info = remove_tech.info();
                    ensure!(
                        !matches!(remove_tech_info.origin, TechOrigin::Faction(..)),
                        "Cannot remove faction technology"
                    );
                    ensure!(
                        !matches!(remove_tech_info.tech_type, TechType::UnitUpgrade),
                        "Cannot remove unit upgrade technologies"
                    );
                    current_player.technologies.remove(&remove_tech);
                    current_player.take_tech(take_tech.clone())?;

                    ActionCardState::DivertFunding {
                        removed: remove_tech,
                        gained: take_tech,
                    }
                }
            };

            let Some(ActionPhaseProgress::ActionCard(progress)) = &mut game_state.action_progress
            else {
                bail!("Illegal state, no progress even though it existed a moment ago?");
            };
            progress.state = Some(new_state);
        }
        Event::ActionCardActionCommit { player } => {
            game_state.assert_phase(Phase::ActionCardAction)?;
            game_state.assert_player_turn(&player)?;

            let Some(ActionPhaseProgress::ActionCard(progress)) = &game_state.action_progress
            else {
                bail!("Not currently performing an action card action");
            };

            match progress.card {
                ActionCard::FocusedResearch => {
                    let Some(p) = &progress.state else {
                        bail!("Can't commit action before having performed the action");
                    };

                    ensure!(
                        matches!(p, ActionCardState::FocusedResearch { .. }),
                        "Illegal state: Invalid action performed for card?"
                    );
                }
                _ => { /* Action isn't tracked by us */ }
            }

            game_state.action_progress = None;
            game_state.phase = Phase::Action;
            game_state.advance_turn()?;
        }
        Event::PassAction { player } => {
            game_state.assert_phase(Phase::Action)?;
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
        Event::ScorePublicObjective { player, objective } => {
            // TODO: consider restricting to the correct phase, etc

            let Some(scorers) = game_state.score.revealed_objectives.get_mut(&objective) else {
                bail!("can't score an unrevealed public objective");
            };

            if !scorers.insert(player) {
                bail!("can't score a public objective twice");
            }
        }
        Event::ScoreSecretObjective { player, objective } => {
            // TODO: consider restricting to the correct phase, etc

            for scored in game_state.score.secret_objectives.values() {
                if scored.contains(&objective) {
                    bail!("secred objective has already been scored");
                }
            }

            game_state
                .score
                .secret_objectives
                .entry(player)
                .or_default()
                .insert(objective);
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

    // TODO: maybe not recalculate this all the time?
    game_state
        .score
        .update_player_points(&game_state.table_order);
    Ok(())
}
