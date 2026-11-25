use std::collections::BTreeSet;

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};

use super::error::GameError;

use ti_helper_game_data::{
    actions::{
        event::Event,
        strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
    },
    common::{expansions::Expansion, faction::Faction, player_id::PlayerId},
    components::{
        agenda::{Agenda, AgendaElect, AgendaElectKind, AgendaKind},
        objectives::{Objective, ObjectiveKind},
        phase::Phase,
        relic::Relic,
        strategy_card::StrategyCard,
        tech::TechOrigin,
    },
    state::{
        agenda::{AgendaRound, MinisterOfAntiquesProgress, Vote, VoteState},
        expedition::TEExpedition,
        game_state::{
            ActionPhaseProgress, AgendaOverrideState, GameState, StrategicPrimaryProgress,
            TacticalProgress,
        },
    },
};

mod action_cards;
mod action_turn;
mod breakthroughs;
mod creation;
mod frontier;
mod galactic_events;
mod leaders;
mod map;
mod relics;
mod setup;
mod strategy;
mod tactical;
mod thunders_edge;

/// Update the game_state with the provided event, using the timestamp for time-keeping purposes.
pub fn update_game_state(
    game_state: &mut GameState,
    event: Event,
    timestamp: DateTime<Utc>,
) -> Result<(), GameError> {
    let mut new_game_state = game_state.clone();

    try_update_game_state(&mut new_game_state, event.clone(), timestamp)?;

    new_game_state.last_event = Some((event, timestamp));
    *game_state = new_game_state;
    Ok(())
}

fn try_update_game_state(
    game_state: &mut GameState,
    event: Event,
    timestamp: DateTime<Utc>,
) -> Result<(), GameError> {
    match event {
        Event::SetSettings { settings } => {
            game_state.assert_phase(Phase::Creation)?;
            game_state.game_settings = settings;

            if game_state.game_settings.expansions.thunders_edge {
                game_state.te_expedition = Some(TEExpedition::default());
            }
        }
        Event::ImportFromMilty {
            max_points,
            milty_data,
        } => creation::import_from_milty(game_state, max_points, milty_data)?,
        Event::SetGalacticEvents { events } => {
            game_state.assert_phase(Phase::Creation)?;
            for event in &events {
                game_state.assert_enabled(event)?;
            }
            game_state.galactic_events.in_play = events;
        }
        Event::AddPlayer { player } => creation::add_player(game_state, player)?,
        Event::CreationDone => creation::creation_done(game_state)?,
        Event::SetupTheTribunii { player, faction } => {
            setup::setup_the_tribunii(game_state, player, faction)?
        }
        Event::SetupCulturalExchange { player, faction } => {
            setup::setup_cultural_exchange(game_state, player, faction)?
        }
        Event::SetupMinorFaction {
            player,
            faction,
            replaced_system,
        } => setup::setup_minor_faction(game_state, player, faction, replaced_system)?,
        Event::SetupPlayerTechs {
            player,
            technologies,
        } => setup::setup_player_techs(game_state, player, technologies)?,
        Event::SetupSpeaker { player } => {
            game_state.assert_phase(Phase::Setup)?;
            ensure!(
                game_state.players.contains_key(&player),
                "Player does not exist"
            );
            game_state.speaker = Some(player);
        }
        Event::RevealInitialObjectives {
            first_objective,
            second_objective,
        } => setup::reveal_initial_objectives(game_state, first_objective, second_objective)?,
        Event::RevealAllPublicObjectives {
            stage_one,
            stage_two,
        } => setup::reveal_all_public_objectives(game_state, stage_one, stage_two)?,
        Event::StartGame => setup::start_game(game_state, timestamp)?,
        Event::TakeStrategyCard { player, card } => {
            strategy::take_strategy_card(game_state, player, card, timestamp)?
        }
        Event::PlayGiftOfPrescience { player } => {
            strategy::play_gift_of_prescience(game_state, player)?
        }
        Event::CompleteStrategyPhase => strategy::complete_strategy_phase(game_state, timestamp)?,
        Event::TacticalActionBegin { player } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            start_tactical_action(game_state);
        }
        Event::TacticalActionTakePlanet { player, planet } => {
            tactical::take_planet(game_state, player, planet)?
        }
        Event::TacticalActionAttachPlanetAttachment {
            player,
            planet,
            attachment,
        } => tactical::attach_planet_attachment(game_state, player, planet, attachment)?,
        Event::TacticalActionResearchTechnology {
            player,
            planet,
            tech,
        } => tactical::research_technology(game_state, player, planet, tech)?,
        Event::TacticalActionGainRelic {
            player,
            planet,
            relic,
        } => tactical::gain_relic(game_state, player, planet, relic)?,
        Event::TacticalActionCommit { player } => {
            game_state.assert_phase(Phase::TacticalAction)?;
            game_state.assert_player_turn(&player)?;
            ensure_tactical_action_finished(game_state)?;
            game_state.action_progress = None;
            game_state.phase = Phase::EndActionTurn;
        }
        Event::StrategicActionBegin { player, card } => {
            strategy::strategic_action_begin(game_state, player, card)?
        }
        Event::StrategicActionPrimary { player, action } => {
            strategy::strategic_action_primary(game_state, player, action)?
        }
        Event::StrategicActionSecondary { player, action } => {
            strategy::strategic_action_secondary(game_state, player, action)?
        }
        Event::StrategicActionCommit => strategy::strategic_action_commit(game_state)?,
        Event::ActionCardActionBegin { player, card } => {
            action_cards::begin(game_state, player, card)?
        }
        Event::ActionCardActionCommit { player, data } => {
            action_cards::commit(game_state, player, data)?
        }
        Event::LeaderActionBegin { player, leader } => leaders::begin(game_state, player, leader)?,
        Event::LeaderActionCommit { player, leader } => {
            leaders::commit(game_state, player, leader)?
        }
        Event::PuppetsOfTheBlade { player } => leaders::puppets_of_the_blade(game_state, player)?,
        Event::PlayGalacticEventAction { player, action } => {
            galactic_events::play_action(game_state, player, action)?
        }
        Event::BreakthroughActionBegin { player } => breakthroughs::begin(game_state, player)?,
        Event::VisionariaSelectDecision { player, tech } => {
            breakthroughs::visionaria_select_decision(game_state, player, tech)?
        }
        Event::BreakthroughActionCommit { player, action } => {
            breakthroughs::commit(game_state, player, action)?
        }
        Event::PlayTechAction { player, tech } => {
            breakthroughs::play_tech_action(game_state, player, tech)?
        }
        Event::CoexistenceBegin { player, planet } => {
            thunders_edge::coexistence_begin(game_state, player, planet)?
        }
        Event::CoexistenceEnd { player, planet } => {
            thunders_edge::coexistence_end(game_state, player, planet)?
        }
        Event::FrontierCardActionBegin { player, card } => {
            frontier::begin(game_state, player, card)?
        }
        Event::FrontierCardActionCommit { player, data } => {
            frontier::commit(game_state, player, data)?
        }
        Event::GainRelicAction { player, relic } => {
            relics::gain_as_action(game_state, player, relic)?
        }
        Event::RelicActionBegin { player, relic } => relics::begin(game_state, player, relic)?,
        Event::RelicActionCommit { player, data } => relics::commit(game_state, player, data)?,
        Event::TakeAnotherTurn { player } => action_turn::take_another_turn(game_state, player)?,
        Event::EndTurn { player } => action_turn::end_turn(game_state, player, timestamp)?,
        Event::PlayCrisis { player } => action_cards::play_crisis(game_state, player, timestamp)?,
        Event::PlayPuppetsOnAString { player } => {
            action_cards::play_puppets_on_a_string(game_state, player, timestamp)?
        }
        Event::PassAction { player } => action_turn::pass_action(game_state, player, timestamp)?,
        /* Status phase events */
        Event::ScorePublicObjective { player, objective } => {
            game_state.assert_phase(Phase::Status)?;

            if let Some(obj) = &objective {
                game_state.assert_enabled(obj)?;
            }

            let scores_any_number =
                game_state.status_phase_scores_any_number_of_public_objectives();

            let Some(status_state) = game_state.status_phase_state.as_mut() else {
                bail!("No status phase state!");
            };

            if let Some(scored) = status_state.scored_public_objectives.get(&player)
                && !(scores_any_number && objective.is_some() && !scored.is_empty())
            {
                bail!("Player has already taken a public objective scoring decision");
            }

            status_state
                .scored_public_objectives
                .entry(player.clone())
                .or_default()
                .extend(objective);

            if let Some(obj) = objective {
                ensure!(
                    !game_state.is_former_public_objective(&obj),
                    "{} is no longer a public objective",
                    obj.info().name
                );
                let Some(scorers) = game_state.score.revealed_objectives.get_mut(&obj) else {
                    bail!("can't score an unrevealed public objective");
                };

                if !scorers.insert(player) {
                    bail!("can't score a public objective twice");
                }
            }
        }
        Event::ScoreSecretObjective { player, objective } => {
            game_state.assert_phase(Phase::Status)?;

            if let Some(obj) = &objective {
                game_state.assert_enabled(&Objective::from(*obj))?;
            }

            let Some(status_state) = game_state.status_phase_state.as_mut() else {
                bail!("No status phase state?")
            };

            if status_state.scored_secret_objectives.contains_key(&player) {
                bail!("Player has already taken a secret objective scoring decision");
            }

            status_state
                .scored_secret_objectives
                .insert(player.clone(), objective);
            let all_decided =
                status_state.scored_secret_objectives.len() == game_state.players.len();

            if let Some(obj) = objective {
                for scored in game_state.score.secret_objectives.values() {
                    if scored.contains(&obj) {
                        bail!("secred objective has already been scored");
                    }
                }

                ensure!(
                    !game_state
                        .score
                        .revealed_objectives
                        .contains_key(&Objective::Secret(obj)),
                    "secret objective has been revealed as a public objective"
                );

                game_state
                    .score
                    .secret_objectives
                    .entry(player)
                    .or_default()
                    .insert(obj);
            }

            if all_decided {
                let turn_order = game_state.turn_order.clone();
                game_state.gain_zealous_orthodoxy(&turn_order);
            }
        }
        Event::RevealPublicObjective { objective } => {
            game_state.assert_phase(Phase::Status)?;
            ensure!(
                game_state.status_phase_reveals_objective(),
                "Under Civilized Society all public objectives are revealed at setup"
            );

            let pub_obj = Objective::Public(objective);
            ensure!(
                !game_state.score.revealed_objectives.contains_key(&pub_obj),
                "Objective has already been revealed!"
            );

            game_state.assert_enabled(&pub_obj)?;

            let stage_two = game_state.status_phase_reveals_stage_two();
            let Some(status_phase_state) = game_state.status_phase_state.as_mut() else {
                bail!("Status phase state not set!")
            };

            match (pub_obj.info().kind, stage_two) {
                (ObjectiveKind::StageI, false) | (ObjectiveKind::StageII, true) => {}
                (ObjectiveKind::StageI, true) => {
                    bail!("Already revealed enough stage I objective, expected stage II");
                }
                (ObjectiveKind::StageII, false) => {
                    bail!(
                        "Haven't finished revealing stage I obejctives, cannot reveal stage II yet"
                    )
                }
                (ObjectiveKind::Secret { .. }, _) => {
                    bail!("Cannot reveal secret objective as public objective")
                }
            }

            if !status_phase_state.can_reveal_objective(game_state.players.len()) {
                bail!(
                    "Cannot reveal objective until all players have finished scoring their objectives"
                );
            }

            status_phase_state.revealed_objective = Some(pub_obj);

            game_state
                .score
                .revealed_objectives
                .insert(pub_obj, BTreeSet::new());
        }
        Event::CompleteStatusPhase => {
            game_state.assert_phase(Phase::Status)?;
            // TODO: Require objectives scored & revealed

            let Some(state) = game_state.status_phase_state.as_ref() else {
                bail!("Expected status phase state");
            };

            ensure!(
                state.revealed_objective.is_some() || !game_state.status_phase_reveals_objective(),
                "No public objective has been revealed!"
            );
            ensure!(
                state.scored_public_objectives.len() == game_state.players.len(),
                "Not all players have made a decision regarding their public objectives"
            );
            ensure!(
                state.scored_secret_objectives.len() == game_state.players.len(),
                "Not all players have made a decision regarding their secret objectives"
            );

            // Reset state
            game_state.strategy_card_holders = Default::default();
            game_state.passed_players = Default::default();
            game_state.spent_strategy_cards = Default::default();

            let phase = get_phase_after_status(game_state);
            game_state.change_phase(phase, timestamp)?;
        }
        Event::PlayNeuraloop {
            player,
            purged_relic,
            replacement,
        } => {
            game_state.assert_phase(Phase::Status)?;
            game_state.assert_enabled(&Relic::Neuraloop)?;
            game_state.assert_enabled(&replacement)?;

            let Some(p) = game_state.players.get(&player) else {
                bail!("Player doesn't exist");
            };

            ensure!(
                p.relics.contains(&Relic::Neuraloop),
                "Player doesn't have the Neuraloop relic!"
            );
            ensure!(
                p.relics.contains(&purged_relic),
                "Player doesn't have the {purged_relic:?} relic to purge!"
            );

            let Some(status_phase_state) = game_state.status_phase_state.as_mut() else {
                bail!("Status phase state not set!")
            };

            let Some(revealed) = status_phase_state.revealed_objective else {
                bail!("Neuraloop can only be played after a public objective has been revealed");
            };

            ensure!(
                !game_state
                    .score
                    .revealed_objectives
                    .contains_key(&replacement),
                "Objective has already been revealed!"
            );

            if let Objective::Secret(secret) = &replacement {
                ensure!(
                    !game_state
                        .score
                        .secret_objectives
                        .values()
                        .any(|scored| scored.contains(secret)),
                    "Secret objective has already been scored"
                );
            }

            game_state.score.revealed_objectives.remove(&revealed);
            game_state
                .score
                .revealed_objectives
                .insert(replacement, BTreeSet::new());
            status_phase_state.revealed_objective = Some(replacement);

            let Some(p) = game_state.players.get_mut(&player) else {
                bail!("Player doesn't exist");
            };
            p.relics.remove(&purged_relic);
            if purged_relic == Relic::ShardOfTheThrone {
                game_state.score.shard_of_the_throne = None;
            }
        }
        Event::GainEntropicScarTech { player, tech } => {
            game_state.assert_phase(Phase::EntropicScar)?;
            game_state.assert_enabled(&tech)?;

            let Some(status_state) = game_state.status_phase_state.as_ref() else {
                bail!("Status phase state not set!")
            };
            ensure!(
                !status_state.entropic_scar_techs.contains_key(&player),
                "{player} has already gained a technology from an entropic scar this status phase"
            );

            let p = game_state
                .players
                .get(&player)
                .context("Player doesn't exist")?;
            ensure!(
                tech.info().origin == TechOrigin::Faction(p.faction),
                "{} isn't one of {}'s faction technologies",
                tech.info().name,
                p.faction.name()
            );
            game_state.gain_tech(&player, tech.clone())?;
            (game_state.status_phase_state.as_mut())
                .context("Status phase state not set!")?
                .entropic_scar_techs
                .insert(player, tech);
        }
        Event::CompleteEntropicScarPhase => {
            game_state.assert_phase(Phase::EntropicScar)?;
            game_state.phase = Phase::Status;
        }
        Event::CompletePostSetupPhase => {
            game_state.assert_phase(Phase::PostSetup)?;
            ensure!(
                game_state.galactic_events.rapid_mobilization.is_empty(),
                "Not all players have resolved Rapid Mobilization"
            );
            game_state.change_phase(Phase::Strategy, timestamp)?;
        }
        Event::PlayCrownOfEmphidia { player } => {
            relics::play_crown_of_emphidia(game_state, player)?
        }
        /* Maw of Worlds relic phase */
        Event::PlayMawOfWorlds { player, tech } => {
            relics::play_maw_of_worlds(game_state, player, tech)?
        }
        Event::CompleteRelicsPhase => {
            game_state.assert_phase(Phase::Relics)?;
            let phase = if game_state.is_agenda_enabled() {
                Phase::Agenda
            } else {
                Phase::Strategy
            };
            game_state.change_phase(phase, timestamp)?;
        }
        /* Agenda phase events */
        Event::RevealAgenda { agenda } => {
            game_state.assert_phase(Phase::Agenda)?;
            game_state.assert_enabled(&agenda)?;
            let vote = VoteState::new(agenda, game_state)?;
            let Some(state) = &mut game_state.agenda else {
                bail!("agenda state not initialized, this is a bug.");
            };
            ensure!(
                state.round < AgendaRound::Completed,
                "there are only 2 rounds of agenda"
            );
            ensure!(state.vote.is_none(), "an agenda is already revealed");

            state.vote = Some(vote);
        }
        Event::VetoAgenda => {
            game_state.assert_phase(Phase::Agenda)?;
            let Some(state) = &mut game_state.agenda else {
                bail!("agenda state not initialized, this is a bug.");
            };
            ensure!(state.vote.is_some(), "no agenda is revealed");
            state.vote = None;
        }
        Event::CastAgendaVote {
            player,
            outcome,
            votes,
        } => {
            game_state.assert_phase(Phase::Agenda)?;

            game_state
                .players
                .get(&player)
                .context("Player doesn't exist?")?;
            if let Some(restriction) = game_state.vote_restriction(&player) {
                bail!(restriction.reason());
            }

            let Some(state) = &mut game_state.agenda else {
                bail!("agenda state not initialized, this is a bug.");
            };

            let Some(vote) = &mut state.vote else {
                bail!("no agenda has been revealed yet");
            };

            if let Some(outcome) = outcome {
                let kind = AgendaElectKind::from(&outcome);
                ensure!(
                    kind == vote.elect,
                    "invalid vote kind, expected {:?}",
                    vote.elect
                );

                vote.player_votes
                    .insert(player, Some(Vote::new(votes, outcome)));
            } else {
                vote.player_votes.insert(player, None);
            }
            vote.tally_votes();
        }
        Event::ResolveAgenda { outcome } => {
            game_state.assert_phase(Phase::Agenda)?;
            let Some(state) = &mut game_state.agenda else {
                bail!("agenda state not initialized, this is a bug.");
            };

            let Some(vote) = state.vote.as_ref() else {
                bail!("no agenda has been revealed yet");
            };

            let vote = vote.clone();
            let minister_of_antiques = match (&vote.agenda, &outcome) {
                (Agenda::MinisterOfAntiques, Some(AgendaElect::Player(player))) => {
                    Some(MinisterOfAntiquesProgress::GainRelic {
                        player: player.clone(),
                    })
                }
                _ => None,
            };

            game_state
                .resolve_agenda(vote, outcome)
                .context("Failed to resolve agenda")?;

            let Some(state) = &mut game_state.agenda else {
                bail!("agenda state not initialized, this is a bug.");
            };

            state.vote = None;
            state.round = state.round.next();
            if minister_of_antiques.is_some() {
                game_state.phase = Phase::AgendaEffect;
            }
            state.minister_of_antiques = minister_of_antiques;
        }
        Event::CompleteAgendaPhase => {
            game_state.assert_phase(Phase::Agenda)?;
            let Some(state) = &mut game_state.agenda else {
                bail!("agenda state not initialized, this is a bug.");
            };
            ensure!(
                state.round == AgendaRound::Completed,
                "need to complete 2 agenda rounds first"
            );

            game_state.change_phase(Phase::Strategy, timestamp)?;
            game_state.agenda = None;
        }
        Event::GiveSupportForTheThrone { giver, receiver } => {
            let score = &mut game_state.score;
            score.support_for_the_throne.insert(giver, receiver);
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
        Event::ScoreExtraPublicObjective { player, objective } => {
            game_state.assert_enabled(&objective)?;

            ensure!(
                !game_state.is_former_public_objective(&objective),
                "{} is no longer a public objective",
                objective.info().name
            );
            let Some(scorers) = game_state.score.revealed_objectives.get_mut(&objective) else {
                bail!("can't score an unrevealed public objective");
            };

            if !scorers.insert(player) {
                bail!("can't score a public objective twice");
            }
        }
        Event::ScoreExtraSecretObjective { player, objective } => {
            game_state.assert_enabled(&Objective::from(objective))?;

            for scored in game_state.score.secret_objectives.values() {
                if scored.contains(&objective) {
                    bail!("secred objective has already been scored");
                }
            }

            ensure!(
                !game_state
                    .score
                    .revealed_objectives
                    .contains_key(&Objective::Secret(objective)),
                "secret objective has been revealed as a public objective"
            );

            game_state
                .score
                .secret_objectives
                .entry(player.clone())
                .or_default()
                .insert(objective);
            game_state.gain_zealous_orthodoxy(&[player]);
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
        Event::RevealExtraPublicObjective { objective } => {
            let pub_obj = Objective::Public(objective);
            ensure!(
                !game_state.score.revealed_objectives.contains_key(&pub_obj),
                "Objective has already been revealed!"
            );
            game_state.assert_enabled(&pub_obj)?;

            game_state
                .score
                .revealed_objectives
                .insert(pub_obj, BTreeSet::new());
        }
        Event::SetShardForTheThroneOwner { player } => {
            game_state.set_relic_owner(Relic::ShardOfTheThrone, player)?;
        }
        Event::SetCrownOfEmphidiaOwner { player } => {
            relics::set_crown_of_emphidia_owner(game_state, player)?
        }
        Event::SetBookOfLatviniaOwner { player } => {
            relics::set_book_of_latvinia_owner(game_state, player)?
        }
        Event::SetRelicOwner { relic, player } => {
            game_state.assert_enabled(&relic)?;
            game_state.set_relic_owner(relic, player)?;
        }
        Event::ResearchBookOfLatviniaTechs {
            player,
            technologies,
        } => relics::research_book_of_latvinia_techs(game_state, player, technologies)?,
        Event::AddTechToPlayer { player, tech } => {
            map::add_tech_to_player(game_state, player, tech)?
        }
        Event::RemoveTechFromPlayer { player, tech } => {
            map::remove_tech_from_player(game_state, player, tech)?
        }
        Event::RepealLaw { law } => {
            if law.info().kind != AgendaKind::Law {
                bail!("Cannot repeal non-law agenda from list of laws");
            }

            if !game_state.laws.contains_key(&law) {
                bail!("Unable to repeal law that has not been enacted");
            }

            game_state.score.handle_law_repealed(&law);
            game_state.laws.remove(&law);
        }
        Event::TrackTime { paused } => {
            game_state.time_tracking_paused = paused;
            if paused {
                game_state.commit_turn_time(timestamp)?;
            } else if should_track_time_in(game_state.phase) {
                game_state.current_turn_start_time = Some(timestamp);
            }
        }
        Event::SetPlanetOwner { player, planet } => {
            map::set_planet_owner(game_state, player, planet)?
        }
        Event::SetPlanetDestroyed { planet, destroyed } => {
            map::set_planet_destroyed(game_state, planet, destroyed)?
        }
        Event::AddPlanetAttachment {
            player,
            planet,
            attachment,
        } => map::add_planet_attachment(game_state, player, planet, attachment)?,
        Event::RemovePlanetAttachment {
            player,
            planet,
            attachment,
        } => map::remove_planet_attachment(game_state, player, planet, attachment)?,

        Event::AddAgendaBegin { agenda } => {
            game_state.assert_enabled(&agenda)?;
            ensure!(
                game_state.agenda_override_state.is_none(),
                "There is already an agenda override in progress, cancel or resolve that before starting a new one."
            );

            game_state.agenda_override_state = Some(AgendaOverrideState {
                vote_state: VoteState::new(agenda, game_state)
                    .context("Failed to create vote state")?,
            });
        }

        Event::AddAgendaPlayerVote {
            player,
            outcome,
            votes,
        } => {
            game_state
                .players
                .get(&player)
                .context("Player doesn't exist?")?;
            if let Some(restriction) = game_state.vote_restriction(&player) {
                bail!(restriction.reason());
            }

            let Some(agenda_override_state) = game_state.agenda_override_state.as_mut() else {
                bail!("No agenda override is in progress");
            };

            ensure!(
                !agenda_override_state
                    .vote_state
                    .player_votes
                    .contains_key(&player),
                "Votes have already been cast by this player for this agenda"
            );

            let kind = AgendaElectKind::from(&outcome);
            ensure!(
                kind == agenda_override_state.vote_state.elect,
                "Invalid vote kind, expected {:?}",
                agenda_override_state.vote_state.elect
            );

            agenda_override_state
                .vote_state
                .player_votes
                .insert(player, Some(Vote::new(votes, outcome)));

            agenda_override_state.vote_state.tally_votes();
        }

        Event::AddAgendaCancel => {
            ensure!(
                game_state.agenda_override_state.is_some(),
                "No agenda override is in progress"
            );

            game_state.agenda_override_state = None;
        }

        Event::AddAgendaResolve { elected_outcome } => {
            let Some(agenda_override_state) = game_state.agenda_override_state.as_mut() else {
                bail!("No agenda override is in progress");
            };

            let vote_state = agenda_override_state.vote_state.clone();

            game_state
                .resolve_agenda(vote_state, Some(elected_outcome))
                .context("Failed to resolve agenda")?;
            game_state.agenda_override_state = None;
        }
        Event::ClaimThundersEdgeExpeditionSlot { player, slot } => {
            thunders_edge::claim_expedition_slot(game_state, player, slot)?
        }
        Event::SetBreakthrough {
            player,
            breakthrough,
        } => thunders_edge::set_breakthrough(game_state, player, breakthrough)?,
        Event::CommitToThundersEdgeExpedition { player, slot } => {
            thunders_edge::commit_to_expedition(game_state, player, slot)?
        }
        Event::CompleteThundersEdgeExpedition {
            player,
            placed_in,
            won_by,
        } => thunders_edge::complete_expedition(game_state, player, placed_in, won_by)?,
        Event::SetThundersEdgeCompletion { new_completion } => {
            thunders_edge::set_completion(game_state, new_completion)?
        }
        Event::SetFractureInPlay { in_play } => {
            game_state.assert_expansion(&Expansion::ThundersEdge)?;
            ensure!(
                game_state.fracture_in_play != in_play,
                "No change provided in event"
            );

            game_state.fracture_in_play = in_play;
        }
        Event::SetStellarAtomicsUsed { player, used } => {
            galactic_events::set_stellar_atomics_used(game_state, player, used)?
        }
        Event::SetZealousOrthodoxyOwner { player } => {
            galactic_events::set_zealous_orthodoxy_owner(game_state, player)?
        }
        Event::SetTotalWarPoints { player, points } => {
            galactic_events::set_total_war_points(game_state, player, points)?
        }
        Event::BuildMonument { player, planet } => {
            galactic_events::build_monument(game_state, player, planet)?
        }
        Event::DestroyMonument { planet } => galactic_events::destroy_monument(game_state, planet)?,
        Event::SetMonumentCommodities {
            planet,
            commodities,
        } => galactic_events::set_monument_commodities(game_state, planet, commodities)?,
        Event::RollForTheFracture {
            player,
            entered_play,
        } => thunders_edge::roll_for_the_fracture(game_state, player, entered_play)?,
        Event::FractureSpawned { player } => thunders_edge::fracture_spawned(game_state, player)?,
        Event::GainMinisterOfAntiquesRelic { player, relic } => {
            game_state.assert_phase(Phase::AgendaEffect)?;
            ensure!(
                matches!(
                    game_state.agenda.as_ref().and_then(|agenda| agenda.minister_of_antiques.as_ref()),
                    Some(MinisterOfAntiquesProgress::GainRelic { player: p }) if p == &player
                ),
                "{player} has no Minister of Antiques relic to gain"
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

            let next = (relic == Some(Relic::BookOfLatvinia)).then(|| {
                MinisterOfAntiquesProgress::BookOfLatviniaResearch {
                    player: player.clone(),
                }
            });
            if next.is_none() {
                game_state.phase = Phase::Agenda;
            }
            game_state
                .agenda
                .as_mut()
                .context("agenda state not initialized, this is a bug.")?
                .minister_of_antiques = next;
            if let Some(relic) = relic {
                game_state.set_relic_owner(relic, Some(player))?;
            }
        }
        Event::ResolveRapidMobilization {
            player,
            planets,
            technologies,
        } => {
            galactic_events::resolve_rapid_mobilization(game_state, player, planets, technologies)?
        }
    }

    // TODO: maybe not recalculate this all the time?
    game_state.update_available_leaders();
    let monument_points = game_state.monument_points();
    game_state
        .score
        .update_player_points(&game_state.players, &monument_points);

    Ok(())
}

/// True if the final expedition slot is claimed, but Thunder's Edge hasn't been placed yet.
fn is_awaiting_thunders_edge_placement(game_state: &GameState) -> bool {
    (game_state.te_expedition.as_ref()).is_some_and(|te| te.is_awaiting_placement())
}

/// Perform the tracked effects of the primary ability of `card` for `player`.
fn perform_strategic_primary(
    game_state: &mut GameState,
    player: &PlayerId,
    card: StrategyCard,
    action: StrategicPrimaryAction,
) -> Result<StrategicPrimaryProgress> {
    game_state.assert_action_expansion(&action)?;

    match (card, action) {
        (
            StrategyCard::Technology,
            StrategicPrimaryAction::Technology {
                first_tech,
                second_tech,
            },
        ) => {
            ensure!(
                second_tech.is_none() || first_tech.is_some(),
                "Cannot take a second tech without a first tech"
            );

            if let Some(t) = first_tech.clone() {
                game_state.research_tech(player, t)?;
            }
            if let Some(t) = second_tech.clone() {
                game_state.research_tech(player, t)?;
            }

            Ok(StrategicPrimaryProgress::Technology {
                first_tech,
                second_tech,
            })
        }
        (StrategyCard::Politics, StrategicPrimaryAction::Politics { new_speaker }) => {
            game_state.speaker = Some(new_speaker.clone());

            Ok(StrategicPrimaryProgress::Politics { new_speaker })
        }
        (StrategyCard::Imperial, StrategicPrimaryAction::Imperial { score_objective }) => {
            if let Some(objective) = score_objective {
                let Some(players) = game_state.score.revealed_objectives.get_mut(&objective) else {
                    bail!("objective {objective:?} has not been revealed");
                };

                if !players.insert(player.clone()) {
                    bail!("player {player:?} has already scored {objective:?}");
                }
            }

            let controls_mecatol_rex = (game_state.players.get(player))
                .context("Player not found")?
                .planets
                .keys()
                .any(|p| p.is_mecatol_rex());
            if controls_mecatol_rex {
                let imperial_points = game_state.score.imperial.entry(player.clone()).or_default();
                *imperial_points = imperial_points.saturating_add(1);
            }

            Ok(StrategicPrimaryProgress::Imperial {
                objective: score_objective,
            })
        }
        (card, action) => {
            bail!("Mismatch between progress card {card:?} and action {action:?}")
        }
    }
}

/// Perform the tracked effects of a secondary ability for `player`.
fn perform_strategic_secondary(
    game_state: &mut GameState,
    player: &PlayerId,
    action: &StrategicSecondaryAction,
) -> Result<()> {
    game_state.assert_secondary_action_expansion(action)?;

    match action.clone() {
        StrategicSecondaryAction::Technology { tech } => {
            game_state.research_tech(player, tech)?;
        }
        StrategicSecondaryAction::TechnologyJolNar {
            first_tech,
            second_tech,
        } => {
            let faction = (game_state.players.get(player))
                .context("Player not found")?
                .faction;
            ensure!(
                faction == Faction::UniversitiesOfJolNar,
                "Only the universities of Jol-Nar can use the 'Brilliant' ability to perform the primary tech action as a secondary"
            );
            game_state.research_tech(player, first_tech)?;
            if let Some(t) = second_tech {
                game_state.research_tech(player, t)?;
            }
        }
        _ => {}
    }

    Ok(())
}

/// Start a tactical action for the current player.
fn start_tactical_action(game_state: &mut GameState) {
    game_state.phase = Phase::TacticalAction;
    game_state.action_progress = Some(ActionPhaseProgress::Tactical(TacticalProgress::default()));
}

/// Ensure the tactical action being performed, if any, has gained the relics it owes and has
/// researched with a Book of Latvinia gained during it.
fn ensure_tactical_action_finished(game_state: &GameState) -> Result<()> {
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

/// End the current action turn, which isn't possible while Thunder's Edge waits to be placed.
fn end_action_turn(game_state: &mut GameState) -> Result<()> {
    ensure!(
        !is_awaiting_thunders_edge_placement(game_state),
        "Thunder's Edge must be placed before ending the turn"
    );
    game_state.action_progress = None;
    game_state.phase = Phase::Action;
    Ok(())
}

/// Returns false if we should never track time in the provided phase, true otherwise.
fn should_track_time_in(phase: Phase) -> bool {
    match phase {
        Phase::Strategy
        | Phase::Action
        | Phase::StrategicAction
        | Phase::TacticalAction
        | Phase::EndActionTurn
        | Phase::LeaderAction
        | Phase::FrontierCardAction
        | Phase::BreakthroughAction
        | Phase::RelicAction
        | Phase::Fracture
        | Phase::ActionCardAction => true,

        Phase::Relics
        | Phase::Setup
        | Phase::PostSetup
        | Phase::EntropicScar
        | Phase::Status
        | Phase::Agenda
        | Phase::AgendaEffect
        | Phase::Creation => false,
    }
}

fn get_phase_after_status(game_state: &GameState) -> Phase {
    if game_state
        .players
        .values()
        .any(|p| p.relics.contains(&Relic::TheCrownOfEmphidia) && p.controls_tomb_of_emphidia())
    {
        return Phase::Relics;
    }

    if game_state.is_agenda_enabled() {
        return if game_state
            .players
            .values()
            .any(|p| p.relics.contains(&Relic::MawOfWorlds))
        {
            Phase::Relics
        } else {
            Phase::Agenda
        };
    }

    Phase::Strategy
}
