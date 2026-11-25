use std::collections::BTreeSet;

use anyhow::{Context, Result, bail, ensure};
use chrono::{DateTime, Utc};

use super::error::GameError;

use ti_helper_game_data::{
    actions::{
        event::{BreakthroughAction, Event, FrontierCardAction, action_matches_frontier_card},
        strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
    },
    common::{component::Component, expansions::Expansion, faction::Faction, player_id::PlayerId},
    components::{
        action_card::ActionCard,
        agenda::{Agenda, AgendaElect, AgendaElectKind, AgendaKind},
        breakthrough::{Breakthrough, BreakthroughKind},
        frontier_card::FrontierCardType,
        galactic_event::GalacticEvent,
        leaders::LeaderAbilityKind,
        objectives::{Objective, ObjectiveKind},
        phase::Phase,
        planet::{OrbitalBodyType, Planet, PlanetTrait},
        relic::Relic,
        strategy_card::StrategyCard,
        system::System,
        tech::{TechOrigin, TechType},
    },
    state::{
        agenda::{AgendaRound, MinisterOfAntiquesProgress, Vote, VoteState},
        coexistence::Coexistence,
        expedition::{ExpeditionClaim, TECompletion, TEExpedition},
        faction_state::DeepwroughtState,
        game_state::{
            ActionPhaseProgress, AgendaOverrideState, BreakthroughProgress, FractureProgress,
            FrontierCardProgress, GameState, LeaderProgress, PlanetLocation,
            StrategicPrimaryProgress, StrategicProgress, TacticalProgress, TakenFrom,
        },
    },
};

mod action_cards;
mod creation;
mod galactic_events;
mod map;
mod relics;
mod setup;

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
            game_state.assert_phase(Phase::Strategy)?;
            ensure!(
                !game_state.strategy_card_holders.contains_key(&card),
                "strategy card can't be picked twice"
            );
            game_state.strategy_card_holders.insert(card, player);
            game_state.advance_turn(timestamp)?;
        }
        Event::PlayGiftOfPrescience { player } => {
            game_state.assert_phase(Phase::Strategy)?;
            ensure!(
                game_state.players.contains_key(&player),
                "unknown player: {player:?}",
            );
            game_state.naalu_telepathy = Some(player);
        }
        Event::CompleteStrategyPhase => {
            let how_many_card_must_pick = match game_state.players.len() {
                3 => 6,
                4 => 8,
                n => n,
            };
            if game_state.strategy_card_holders.len() != how_many_card_must_pick {
                anyhow::bail!(
                    "can't complete strategy phase, all players have not selected strategy cards"
                );
            }
            game_state.change_phase(Phase::Action, timestamp)?;
        }
        Event::TacticalActionBegin { player } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            start_tactical_action(game_state);
        }
        Event::TacticalActionTakePlanet { player, planet } => {
            game_state.assert_player_turn(&player)?;
            let planet_system = match game_state.planet_location(&planet) {
                PlanetLocation::SystemTile => Some(System::for_planet(&planet)?.id),
                PlanetLocation::Token(system) => system,
                PlanetLocation::NotOnBoard => bail!("{} isn't on the board", planet.info().name),
            };
            game_state.assert_enabled(&planet)?;

            let has_been_owned = game_state.has_been_owned(&planet);
            let previous_owner =
                game_state.set_planet_owner(planet.clone(), Some(player.clone()))?;

            let taken_from = match previous_owner {
                Some(previous_owner) => TakenFrom::Player(previous_owner),
                None if has_been_owned => TakenFrom::Unowned,
                None => TakenFrom::NeverOwned,
            };
            let tactical = game_state.tactical_progress_mut()?;
            tactical.activated_system = planet_system;
            tactical.taken_planets.insert(planet.clone(), taken_from);

            if planet == Planet::ThundersEdge {
                game_state.gain_breakthrough(&player)?;
            }
        }
        Event::TacticalActionAttachPlanetAttachment {
            player,
            planet,
            attachment,
        } => {
            game_state.assert_player_turn(&player)?;
            let planet_traits = game_state.planet_traits(&planet);
            ensure!(
                planet.info().body_type != OrbitalBodyType::SpaceStation,
                "Space stations can't have attachments"
            );

            {
                let tactical = game_state.tactical_progress_mut()?;

                match tactical.taken_planets.get(&planet) {
                    None => bail!("Can only attach attachment to planet taken this turn"),
                    Some(TakenFrom::Player(_)) => {
                        bail!("Cannot explore planet that was taken from another player")
                    }
                    Some(TakenFrom::NeverOwned | TakenFrom::Unowned) => {}
                }

                ensure!(
                    !tactical.planet_attachments.contains_key(&planet),
                    "Planet has already been explored this turn!"
                );

                let attachment_info = attachment.info();
                let Some(attachment_planet_trait) = &attachment_info.planet_trait else {
                    // TODO: Somewhat of an exploit of the fact that currently,
                    //       the attachments that have planet trait requirements match 100%
                    //       with the ones that are available during exploration.
                    bail!("Only attachments with planet_traits can be used during exploration")
                };
                ensure!(
                    planet_traits.contains(attachment_planet_trait),
                    "Planet does not meet the trait requirements for the attachment"
                );
            }

            let current_player = game_state.get_current_player()?;
            let Some(planet_attachments) = current_player.planets.get_mut(&planet) else {
                bail!("Player doesn't have planet that they just took, this is a bug!");
            };
            let actual_attachment = attachment.match_planet(&planet.info());
            planet_attachments.insert(actual_attachment.clone());

            game_state
                .tactical_progress_mut()?
                .planet_attachments
                .insert(planet, actual_attachment);
        }
        Event::TacticalActionResearchTechnology {
            player,
            planet,
            tech,
        } => {
            game_state.assert_player_turn(&player)?;
            game_state.assert_galactic_event(&GalacticEvent::DangerousWilds)?;
            let planet_traits = game_state.planet_traits(&planet);

            {
                let tactical = game_state.tactical_progress_mut()?;

                match tactical.taken_planets.get(&planet) {
                    None => bail!("Can only research for a planet taken this turn"),
                    Some(TakenFrom::Player(_)) => bail!(
                        "{} was taken from another player, not the planet deck",
                        planet.info().name
                    ),
                    Some(TakenFrom::Unowned) => bail!(
                        "{} has been owned before, it wasn't taken from the planet deck",
                        planet.info().name
                    ),
                    Some(TakenFrom::NeverOwned) => {}
                }

                ensure!(
                    planet_traits.contains(&PlanetTrait::Hazardous),
                    "{} isn't hazardous",
                    planet.info().name
                );

                ensure!(
                    !tactical.researched_techs.contains_key(&planet),
                    "Player has already researched a technology for {}",
                    planet.info().name
                );
            }

            game_state.assert_enabled(&tech)?;
            game_state.research_tech(&player, tech.clone())?;

            game_state
                .tactical_progress_mut()?
                .researched_techs
                .insert(planet, tech);
        }
        Event::TacticalActionGainRelic {
            player,
            planet,
            relic,
        } => {
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.tactical_progress_mut()?.owes_relic(&planet),
                "{player} doesn't gain a relic for {}",
                planet.info().name
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

            game_state
                .tactical_progress_mut()?
                .gained_relics
                .insert(planet, relic.clone());
            if let Some(relic) = relic {
                game_state.set_relic_owner(relic, Some(player))?;
            }
        }
        Event::TacticalActionCommit { player } => {
            game_state.assert_phase(Phase::TacticalAction)?;
            game_state.assert_player_turn(&player)?;
            ensure_tactical_action_finished(game_state)?;
            game_state.action_progress = None;
            game_state.phase = Phase::EndActionTurn;
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

            let player_faction = game_state
                .players
                .get(&player)
                .context("Expected player to exist")?
                .faction;

            game_state.action_progress = Some(ActionPhaseProgress::Strategic(StrategicProgress {
                card,
                primary: StrategicPrimaryProgress::default_for(
                    card.version(&game_state.game_settings.expansions),
                    player_faction,
                ),
                other_players: Default::default(),
            }));
            game_state.spent_strategy_cards.insert(card);
        }
        Event::StrategicActionPrimary { player, action } => {
            game_state.assert_phase(Phase::StrategicAction)?;
            game_state.assert_player_turn(&player)?;
            let Some(progress) = &game_state.action_progress else {
                bail!("Invalid game state: expected action_progress not to be None");
            };
            let ActionPhaseProgress::Strategic(progress) = progress else {
                bail!("Invalid action phase progress '{:?}'", progress);
            };

            ensure!(
                progress.primary.is_none(),
                "Primary action has already been performed"
            );
            let card = progress.card;

            let primary = perform_strategic_primary(game_state, &player, card, action)?;

            let Some(ActionPhaseProgress::Strategic(progress)) = &mut game_state.action_progress
            else {
                bail!("not currently performing a strategic action");
            };
            progress.primary = Some(primary);
        }
        Event::StrategicActionSecondary { player, action } => {
            game_state.assert_phase(Phase::StrategicAction)?;
            let current_player = game_state.current_player()?;
            ensure!(
                player != current_player,
                "current player can't perform the secondary on a strategy card",
            );

            let Some(action_progress) = &game_state.action_progress else {
                bail!("no strategic action in progress");
            };
            let ActionPhaseProgress::Strategic(progress) = action_progress else {
                bail!("cannot perform strategic actions during non-strategy actions");
            };
            ensure!(
                action.is_for_card(progress.card),
                "Mismatch between strategic progress {progress:?} and action {action:?}"
            );

            perform_strategic_secondary(game_state, &player, &action)?;

            let Some(ActionPhaseProgress::Strategic(progress)) = &mut game_state.action_progress
            else {
                bail!("no strategic action in progress");
            };
            progress.other_players.insert(player, action.into());
        }
        Event::StrategicActionCommit => {
            game_state.assert_phase(Phase::StrategicAction)?;

            let Some(ActionPhaseProgress::Strategic(progress)) = &game_state.action_progress else {
                bail!("not currently performing a strategic action");
            };

            if progress.card.has_tracked_primary() {
                ensure!(
                    progress.primary.is_some(),
                    "has not performed strategy card primary"
                );
            }
            ensure_tactical_action_finished(game_state)?;

            game_state.action_progress = None;
            game_state.phase = Phase::EndActionTurn;
        }
        Event::ActionCardActionBegin { player, card } => {
            action_cards::begin(game_state, player, card)?
        }
        Event::ActionCardActionCommit { player, data } => {
            action_cards::commit(game_state, player, data)?
        }
        Event::LeaderActionBegin { player, leader } => {
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
            game_state.phase = Phase::EndActionTurn
        }
        Event::LeaderActionCommit { player, leader } => {
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
        }
        Event::PuppetsOfTheBlade { player } => {
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
        }
        Event::PlayGalacticEventAction { player, action } => {
            galactic_events::play_action(game_state, player, action)?
        }
        Event::BreakthroughActionBegin { player } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.action_progress.is_none(),
                "Invalid game state, action_progress not None during Action Phase"
            );
            ensure!(
                (game_state.players.get(&player))
                    .and_then(|p| p.breakthrough.as_ref())
                    .is_some_and(|b| b.info().kind == BreakthroughKind::Action),
                "{player} doesn't have a breakthrough with an action"
            );

            game_state.action_progress = Some(ActionPhaseProgress::Breakthrough(
                BreakthroughProgress::default(),
            ));
            game_state.phase = Phase::BreakthroughAction;
        }
        Event::VisionariaSelectDecision { player, tech } => {
            game_state.assert_phase(Phase::BreakthroughAction)?;
            let deepwrought = game_state.current_player()?;
            ensure!(
                (game_state.players.get(&deepwrought))
                    .is_some_and(|p| p.breakthrough == Some(Breakthrough::VisionariaSelect)),
                "{deepwrought} isn't performing Visionaria Select"
            );
            ensure!(
                player != deepwrought,
                "{player} can't research with their own Visionaria Select"
            );
            let researcher = (game_state.players.get(&player)).context("Player not found")?;
            if let Some(tech) = &tech {
                game_state.assert_enabled(tech)?;
                let info = tech.info();
                ensure!(
                    info.origin == TechOrigin::Base,
                    "{} is a faction technology",
                    info.name
                );
                ensure!(
                    info.tech_type != TechType::UnitUpgrade,
                    "{} is a unit upgrade",
                    info.name
                );
                researcher.assert_can_research(tech)?;
                ensure!(
                    !researcher.has_tech(tech),
                    "{player} already has tech {}",
                    info.name
                );
            }

            let Some(ActionPhaseProgress::Breakthrough(progress)) = &mut game_state.action_progress
            else {
                bail!("Not currently performing a breakthrough action");
            };
            ensure!(
                !progress.visionaria_select.contains_key(&player),
                "{player} has already decided"
            );
            progress.visionaria_select.insert(player, tech);
        }
        Event::BreakthroughActionCommit { player, action } => {
            game_state.assert_phase(Phase::BreakthroughAction)?;
            game_state.assert_player_turn(&player)?;
            let Some(ActionPhaseProgress::Breakthrough(progress)) =
                game_state.action_progress.clone()
            else {
                bail!("Not currently performing a breakthrough action");
            };
            let breakthrough = action.breakthrough();
            ensure!(
                game_state
                    .players
                    .get(&player)
                    .is_some_and(|p| p.breakthrough.as_ref() == Some(&breakthrough)),
                "{player} doesn't have {}",
                breakthrough.info().name
            );
            game_state.action_progress = None;

            match action {
                BreakthroughAction::Psychospore
                | BreakthroughAction::DeorbitBarrage
                | BreakthroughAction::ResonanceGenerator => {}
                BreakthroughAction::VaultsOfTheHeir { purged_tech, relic } => {
                    ensure!(
                        game_state
                            .players
                            .get(&player)
                            .is_some_and(|p| p.has_tech(&purged_tech)),
                        "{player} doesn't own {}",
                        purged_tech.info().name
                    );
                    game_state.assert_enabled(&relic)?;
                    ensure!(
                        !game_state
                            .players
                            .values()
                            .any(|p| p.relics.contains(&relic)),
                        "Relic {relic:?} is already owned by another player."
                    );

                    game_state
                        .players
                        .get_mut(&player)
                        .context("Player not found")?
                        .remove_tech(&purged_tech);
                    game_state.gain_relic_as_action(player.clone(), relic)?;
                }
                BreakthroughAction::VisionariaSelect => {
                    if let Some(undecided) = (game_state.table_order.iter()).find(|p| {
                        **p != player
                            && game_state.players.get(*p).is_some_and(|p| p.can_research())
                            && !progress.visionaria_select.contains_key(*p)
                    }) {
                        bail!("Waiting for {undecided} to decide");
                    }

                    let deepwrought =
                        (game_state.players.get(&player)).context("Player not found")?;
                    let gains = progress.visionaria_select_gains(deepwrought);
                    for (researcher, tech) in progress.visionaria_select {
                        if let Some(tech) = tech {
                            game_state.research_tech(&researcher, tech)?;
                        }
                    }
                    for tech in gains {
                        game_state.gain_tech(&player, tech)?;
                    }
                }
            }

            if game_state.action_progress.is_none() {
                game_state.phase = Phase::EndActionTurn;
            }
        }
        Event::PlayTechAction { player, tech } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.action_progress.is_none(),
                "Invalid game state, action_progress not None during Action Phase"
            );
            ensure!(
                tech.is_single_button_action(),
                "{} doesn't have a single button ACTION",
                tech.info().name
            );
            ensure!(
                game_state
                    .players
                    .get(&player)
                    .is_some_and(|p| p.has_tech(&tech)),
                "{player} doesn't have {}",
                tech.info().name
            );

            game_state.phase = Phase::EndActionTurn;
        }
        Event::CoexistenceBegin { player, planet } => {
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
        }
        Event::CoexistenceEnd { player, planet } => {
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
        }
        Event::FrontierCardActionBegin { player, card } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            game_state.assert_enabled(&card)?;
            ensure!(
                card.info().frontier_type == FrontierCardType::Action,
                "Only able to play frontier card actions during action phase"
            );
            ensure!(
                game_state.action_progress.is_none(),
                "Invalid game state, action_progress not None during Action Phase"
            );

            game_state.action_progress =
                Some(ActionPhaseProgress::FrontierCard(FrontierCardProgress {
                    card,
                }));
            game_state.phase = Phase::FrontierCardAction;
        }
        Event::FrontierCardActionCommit { player, data } => {
            game_state.assert_phase(Phase::FrontierCardAction)?;
            game_state.assert_player_turn(&player)?;

            let Some(ActionPhaseProgress::FrontierCard(progress)) =
                game_state.action_progress.as_ref()
            else {
                bail!(
                    "Invalid state, expected frontier card progress, got {:?}",
                    game_state.action_progress
                );
            };

            let current_player_faction = game_state
                .players
                .iter()
                .find(|&(id, _)| &player == id)
                .context("Current player to exist")?
                .1
                .faction;
            ensure!(
                action_matches_frontier_card(
                    &data,
                    &progress.card,
                    &current_player_faction,
                    game_state.map_data.milty_information.is_some()
                ),
                "Data provided doesn't match the card being played."
            );

            if let Some(progress) = data {
                match progress {
                    FrontierCardAction::EnigmaticDevice { tech } => {
                        game_state.assert_enabled(&tech)?;
                        game_state.research_tech(&player, tech)?;
                    }
                    FrontierCardAction::Mirage { system, attachment } => {
                        ensure!(
                            !game_state
                                .players
                                .values()
                                .flat_map(|p| p.planets.keys())
                                .any(|p| p == &Planet::Mirage),
                            "A player already owns mirage and can only be takn using a tactical action"
                        );
                        ensure!(
                            game_state
                                .map_data
                                .milty_information
                                .as_ref()
                                .map(|data| data.mirage_system.is_none())
                                .unwrap_or(true),
                            "Mirage system is already set?"
                        );
                        let player_id = game_state
                            .current_player
                            .clone()
                            .context("No current player")?;
                        game_state.set_planet_owner(Planet::Mirage, Some(player_id))?;
                        if let Some(attachment) = attachment {
                            game_state
                                .get_current_player()?
                                .planets
                                .entry(Planet::Mirage)
                                .or_default()
                                .insert(attachment);
                        }

                        if let Some(milty_info) = game_state.map_data.milty_information.as_mut() {
                            milty_info.mirage_system = Some(
                                system.context("System to be chosen when milty info is enabled")?,
                            );
                        }
                    }
                }
            }

            game_state.action_progress = None;
            game_state.phase = Phase::EndActionTurn;
        }
        Event::GainRelicAction { player, relic } => {
            relics::gain_as_action(game_state, player, relic)?
        }
        Event::RelicActionBegin { player, relic } => relics::begin(game_state, player, relic)?,
        Event::RelicActionCommit { player, data } => relics::commit(game_state, player, data)?,
        Event::TakeAnotherTurn { player } => {
            game_state.assert_phase(Phase::EndActionTurn)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                !game_state.passed_players.contains(&player),
                "A player that has passed cannot take another turn"
            );

            ensure!(
                !is_awaiting_thunders_edge_placement(game_state),
                "Thunder's Edge must be placed before taking another turn"
            );
            game_state.action_progress = None;
            game_state.turn_number += 1;

            game_state.phase = Phase::Action;
        }
        Event::EndTurn { player } => {
            game_state.assert_phase(Phase::EndActionTurn)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                game_state.action_progress.is_none(),
                "Cannot end turn while an action is in progress"
            );

            end_action_turn(game_state)?;
            game_state.advance_turn(timestamp)?;
        }
        Event::PlayCrisis { player } => action_cards::play_crisis(game_state, player, timestamp)?,
        Event::PlayPuppetsOnAString { player } => {
            action_cards::play_puppets_on_a_string(game_state, player, timestamp)?
        }
        Event::PassAction { player } => {
            game_state.assert_phase(Phase::Action)?;
            game_state.assert_player_turn(&player)?;
            ensure!(
                !game_state.passed_players.contains(&player),
                "{player} has already passed"
            );

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

            // A passing player may still commit to the expedition at the end of their turn.
            let can_commit = (game_state.te_expedition.as_ref()).is_some_and(|te| !te.is_full());
            let has_end_of_turn_cards = [ActionCard::Crisis, ActionCard::PuppetsOnAString]
                .iter()
                .any(|card| card.is_enabled_in(&game_state.game_settings.expansions));
            if can_commit || has_end_of_turn_cards {
                game_state.phase = Phase::EndActionTurn;
            } else {
                game_state.advance_turn(timestamp)?;
            }
        }
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
        }
        Event::SetBreakthrough {
            player,
            breakthrough,
        } => {
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
        }
        Event::CommitToThundersEdgeExpedition { player, slot } => {
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
        }
        Event::CompleteThundersEdgeExpedition {
            player,
            placed_in,
            won_by,
        } => {
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
        }
        Event::SetThundersEdgeCompletion { new_completion } => {
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

            let changed_winner = (te.completed.as_ref()).map(|c| &c.won_by)
                != (new_completion.as_ref()).map(|c| &c.won_by);
            te.completed = new_completion.clone();
            if changed_winner {
                game_state
                    .set_planet_owner(Planet::ThundersEdge, new_completion.map(|c| c.won_by))?;
            }
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
        } => {
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
        }
        Event::FractureSpawned { player } => {
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
        }
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
