use anyhow::{Result, bail, ensure};
use chrono::{DateTime, Utc};

use super::error::GameError;

use ti_helper_game_data::{
    actions::event::Event,
    common::expansions::Expansion,
    components::{phase::Phase, relic::Relic},
    state::{expedition::TEExpedition, game_state::GameState},
};

mod action_cards;
mod action_turn;
mod agenda;
mod breakthroughs;
mod creation;
mod frontier;
mod galactic_events;
mod leaders;
mod map;
mod objectives;
mod relics;
mod setup;
mod status;
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
            tactical::start_tactical_action(game_state);
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
            tactical::ensure_tactical_action_finished(game_state)?;
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
            objectives::score_public(game_state, player, objective)?
        }
        Event::ScoreSecretObjective { player, objective } => {
            objectives::score_secret(game_state, player, objective)?
        }
        Event::RevealPublicObjective { objective } => {
            objectives::reveal_public(game_state, objective)?
        }
        Event::CompleteStatusPhase => status::complete_status_phase(game_state, timestamp)?,
        Event::PlayNeuraloop {
            player,
            purged_relic,
            replacement,
        } => status::play_neuraloop(game_state, player, purged_relic, replacement)?,
        Event::GainEntropicScarTech { player, tech } => {
            status::gain_entropic_scar_tech(game_state, player, tech)?
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
        Event::CompleteRelicsPhase => status::complete_relics_phase(game_state, timestamp)?,
        /* Agenda phase events */
        Event::RevealAgenda { agenda } => agenda::reveal_agenda(game_state, agenda)?,
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
        } => agenda::cast_vote(game_state, player, outcome, votes)?,
        Event::ResolveAgenda { outcome } => agenda::resolve_agenda(game_state, outcome)?,
        Event::CompleteAgendaPhase => agenda::complete_agenda_phase(game_state, timestamp)?,
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
            objectives::score_extra_public(game_state, player, objective)?
        }
        Event::ScoreExtraSecretObjective { player, objective } => {
            objectives::score_extra_secret(game_state, player, objective)?
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
            objectives::reveal_extra_public(game_state, objective)?
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
        Event::RepealLaw { law } => agenda::repeal_law(game_state, law)?,
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

        Event::AddAgendaBegin { agenda } => agenda::add_agenda_begin(game_state, agenda)?,

        Event::AddAgendaPlayerVote {
            player,
            outcome,
            votes,
        } => agenda::add_agenda_player_vote(game_state, player, outcome, votes)?,

        Event::AddAgendaCancel => {
            ensure!(
                game_state.agenda_override_state.is_some(),
                "No agenda override is in progress"
            );

            game_state.agenda_override_state = None;
        }

        Event::AddAgendaResolve { elected_outcome } => {
            agenda::add_agenda_resolve(game_state, elected_outcome)?
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
            agenda::gain_minister_of_antiques_relic(game_state, player, relic)?
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
