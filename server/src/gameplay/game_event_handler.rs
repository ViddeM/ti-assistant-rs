use std::collections::{HashMap, HashSet};

use eyre::{bail, ensure, eyre};

use crate::{
    data::components::{
        action_card::{ActionCard, ActionCardPlay},
        objectives::Objective,
        phase::Phase,
        planet::Planet,
        strategy_card::StrategyCard,
        system::System,
        tech::{TechOrigin, TechType, Technology},
    },
    gameplay::{
        event::{action_matches_action_card, StrategicPrimaryAction, StrategicSecondaryAction},
        game_state::{
            ActionCardProgress, ActionPhaseProgress, StrategicPrimaryProgress, StrategicProgress,
        },
        player::PlayerId,
    },
};

use super::{
    error::GameError,
    event::{ActionCardInfo, Event},
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
                (StrategyCard::Imperial, StrategicPrimaryAction::Imperial { score_objective }) => {
                    let player_id = game_state.current_player()?;
                    let player = game_state
                        .players
                        .get(&player_id)
                        .ok_or(eyre!("no player state"))?;

                    let Some(players) = game_state
                        .score
                        .revealed_objectives
                        .get_mut(&score_objective)
                    else {
                        bail!("objective {score_objective:?} has not been revealed");
                    };

                    if !players.insert(player_id.clone()) {
                        bail!("player {player_id:?} has already scored {score_objective:?}");
                    }

                    if player.planets.contains(&Planet::MecatolRex) {
                        let imperial_points =
                            game_state.score.imperial.entry(player_id).or_default();
                        *imperial_points = imperial_points.saturating_add(1);
                    }
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
                player != current_player,
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
                StrategyCard::Politics | StrategyCard::Technology | StrategyCard::Imperial
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

            match card {
                ActionCard::Plagiarize => {
                    ensure!(
                        !get_plagiarize_available_techs(game_state)?.is_empty(),
                        "There are no techs available for the player to steal."
                    )
                }
                _ => {}
            }

            game_state.action_progress =
                Some(ActionPhaseProgress::ActionCard(ActionCardProgress { card }));
            game_state.phase = Phase::ActionCardAction;
        }
        Event::ActionCardActionCommit { player, data } => {
            game_state.assert_phase(Phase::ActionCardAction)?;
            game_state.assert_player_turn(&player)?;

            if let Some(ActionPhaseProgress::ActionCard(progress)) = &game_state.action_progress {
                ensure!(
                    action_matches_action_card(&data, &progress.card),
                    "Trying to perform an action that does not match the selected action card."
                );
            } else {
                bail!("Not currently performing an action card action");
            };

            if let Some(data) = data {
                match data {
                    ActionCardInfo::FocusedResearch { tech } => {
                        let current_player = game_state.get_current_player()?;
                        current_player.take_tech(tech.clone())?;
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
                    }
                    ActionCardInfo::Plagiarize { tech } => {
                        let available_techs = get_plagiarize_available_techs(game_state)?;
                        ensure!(
                            available_techs.contains(&tech),
                            "Unable to take tech {tech:?} for this action card"
                        );

                        let current_player = game_state.get_current_player()?;
                        current_player.take_tech(tech)?;
                    }
                }
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
        Event::RevealPublicObjective { objective } => {
            let pub_obj = Objective::Public(objective);
            ensure!(
                !game_state.score.revealed_objectives.contains_key(&pub_obj),
                "Objective has already been revealed!"
            );

            game_state
                .score
                .revealed_objectives
                .insert(pub_obj, HashSet::new());
        }
        Event::CompleteStatusPhase => {
            game_state.assert_phase(Phase::Status)?;
            // TODO: Require objectives scored & revealed

            // Reset state
            game_state.strategy_card_holders = HashMap::new();
            game_state.passed_players = HashSet::new();
            game_state.spent_strategy_cards = HashSet::new();
            game_state.calculate_turn_order_from_speaker()?;
            game_state.current_player = Some(game_state.speaker()?.clone());

            // TODO: Agenda phase
            game_state.phase = Phase::Strategy;
            //if game_state.score.custodians.is_some() {
            //    game_state.phase = Phase::Agenda;
            //} else {
            //    game_state.phase = Phase::Strategy;
            //}
        }
        Event::RevealAgenda { agenda: _ } => {
            todo!("implement RevealAgenda")
        }
        Event::ResolveAgenda { outcome: _ } => {
            todo!("implement ResolveAgenda")
        }
        Event::CompleteAgendaPhase => {
            game_state.assert_phase(Phase::Status)?;
            // TODO: Require 2 agendas to have been resolved

            game_state.calculate_turn_order_from_speaker()?;
            game_state.current_player = Some(game_state.speaker()?.clone());
            game_state.phase = Phase::Strategy;
        }
        Event::GiveSupportForTheThrone { giver, receiver } => {
            let score = &mut game_state.score;

            score.support_for_the_throne.insert(giver, receiver);
        }
        Event::SetExtraPoints { player, value } => {
            game_state.score.extra_points.insert(player, value);
        }
        Event::AddExtraPoints { player, value } => {
            let extra = game_state.score.extra_points.entry(player).or_default();
            *extra = extra.saturating_add(value);
        }
        Event::SetCustodians { player } => game_state.score.custodians = player,
        Event::AddImperial { player, value } => {
            let imperial = game_state.score.imperial.entry(player).or_default();
            *imperial = imperial.saturating_add(value);
        }
        Event::UnscoreObjective { player, objective } => {
            let Some(scorers) = game_state.score.revealed_objectives.get_mut(&objective) else {
                bail!("Cannot un-score objective that hasn't been revealed");
            };

            scorers.remove(&player);
        }
        Event::UnscoreSecretObjective { player, objective } => {
            let Some(objectives) = game_state.score.secret_objectives.get_mut(&player) else {
                bail!("Player not in secret objectives map?");
            };

            objectives.remove(&objective);
        }
    }

    // TODO: maybe not recalculate this all the time?
    game_state
        .score
        .update_player_points(&game_state.table_order);
    Ok(())
}

fn get_plagiarize_available_techs(
    game_state: &GameState,
) -> Result<HashSet<&Technology>, GameError> {
    let current_player_id = game_state.current_player()?;
    let current_player = game_state
        .players
        .get(&current_player_id)
        .ok_or_else(|| eyre::eyre!("Invalid game state, current player not in players map"))?;

    let available_techs = game_state
        .players
        .iter()
        .filter(|&(id, _)| id != &current_player_id)
        .flat_map(|(_, player)| player.technologies.iter())
        .filter(|tech| !matches!(tech.info().origin, TechOrigin::Faction(..)))
        .filter(|tech| !current_player.has_tech(tech))
        .collect::<HashSet<&Technology>>();

    Ok(available_techs)
}
