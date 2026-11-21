use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Debug,
    time::Duration,
};

use anyhow::{Context, anyhow, bail};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    actions::{
        event::Event,
        strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
    },
    common::{
        component::Component, expansions::Expansion, faction::Faction, game_settings::GameSettings,
        player_id::PlayerId,
    },
    components::{
        agenda::{Agenda, AgendaElect},
        galactic_event::GalacticEvent,
        leaders::{Leader, LeaderAbilityKind},
        phase::Phase,
        strategy_card::StrategyCard,
    },
    state::{coexistence::Coexistence, expedition::TEExpedition},
};

use super::{
    agenda::{AgendaRecord, AgendaState},
    galactic_events::GalacticEventsState,
    player::Player,
    score::Score,
    status::StatusPhaseState,
};

mod action_progress;
mod agenda;
mod fracture;
mod gains;
mod galactic_events;
mod map;
mod status;

pub use action_progress::{
    ActionCardProgress, ActionPhaseProgress, FrontierCardProgress, LeaderProgress, RelicProgress,
    RelicStep, StrategicPrimaryProgress, StrategicProgress, TacticalProgress, TakenFrom,
};
pub use agenda::{AgendaOverrideState, VoteRestriction};
pub use fracture::{FractureProgress, FractureProgressState, FractureRollState};
pub use map::{MapData, MiltyInformation, PlanetLocation};

/// A snapshot of the game state.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    /// The current round number of the game.
    pub round: u32,

    /// The settings for the game.
    pub game_settings: GameSettings,

    /// The galactic events in play and their state.
    pub galactic_events: GalacticEventsState,

    /// The current phase of the game.
    pub phase: Phase,

    /// Information about the map, only available on games imported from milty.
    pub map_data: MapData,

    /// Which players are in the game.
    pub players: BTreeMap<PlayerId, Player>,

    /// The current speaker, if any.
    pub speaker: Option<PlayerId>,

    /// The order that players are sitting around the table, starting with the speaker.
    pub table_order: Vec<PlayerId>,

    /// The current turn order, either based on table order or strategy cards (initiative).
    pub turn_order: Vec<PlayerId>,

    /// Which players hold which strategy cards.
    pub strategy_card_holders: HashMap<StrategyCard, PlayerId>,

    /// The current player, if any.
    pub current_player: Option<PlayerId>,

    /// Counts every action turn in the game, so that actions can be tied to a single turn.
    pub turn_number: u32,

    /// Players that must roll for the Fracture once the current event has been applied.
    #[serde(skip)]
    pub pending_fracture_rolls: Vec<PlayerId>,

    /// Which strategy cards have been spent this action phase.
    pub spent_strategy_cards: BTreeSet<StrategyCard>,

    /// Which players have passed this phase.
    pub passed_players: BTreeSet<PlayerId>,

    /// The player whose turn starts once the current (passed) player has performed their action
    /// from "Puppets on a String". It's `None` if the card was played after the last player
    /// passed, in which case the action phase ends after that action.
    pub interrupted_turn: Option<PlayerId>,

    /// Tracks progress of the current action (if any) that is being taken.
    pub action_progress: Option<ActionPhaseProgress>,

    /// All things that concern scoring for the game.
    pub score: Score,

    /// State for agenda phase.
    pub agenda: Option<AgendaState>,

    /// List of past things voted on in the agenda phase.
    pub agenda_vote_history: Vec<AgendaRecord>,

    /// Laws in play.
    pub laws: HashMap<Agenda, AgendaElect>,

    /// State required for the agenda 'admin view'.
    pub agenda_override_state: Option<AgendaOverrideState>,

    /// State for the status phase.
    pub status_phase_state: Option<StatusPhaseState>,

    /// Leaders available for play for each player.
    pub available_leaders: HashMap<PlayerId, Vec<Leader>>,

    /// Weather or not time should be tracked.
    pub time_tracking_paused: bool,

    /// Time taken by each player to complete their rounds during the action phase.
    ///
    /// This does not include the time taken for the current round, that will be calculated and
    /// included when the current player ends their turn.
    pub players_play_time: HashMap<PlayerId, Duration>,

    /// When the current player started their turn.
    pub current_turn_start_time: Option<DateTime<Utc>>,

    /// The player (if any) currently using [Faction::NaaluCollective]s faction ability:
    /// "Telepathic", or the Naalu promisary note "Gift of Prescience".
    ///
    /// This player has initiative 0 in the action and status phase.
    pub naalu_telepathy: Option<PlayerId>,

    /// The current expedition for Thunder's edge or None if thunder's edge is not in play.
    pub te_expedition: Option<TEExpedition>,

    /// Whether the Fracture has entered play. Until then its planets (Cocytus, Lethe, Phlegethon
    /// and Styx) aren't on the board.
    pub fracture_in_play: bool,

    /// The planets players' units coexist on, see [Coexistence]. Only used with Thunder's Edge.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub coexistences: BTreeSet<Coexistence>,

    /// The last applied event and when it happened.
    pub last_event: Option<(Event, DateTime<Utc>)>,
}

impl GameState {
    /// Update the turn order according to initiative order.
    pub fn calculate_action_turn_order(&mut self) -> anyhow::Result<()> {
        self.turn_order = self.table_order.clone();

        let mut player_nums = HashMap::new();
        for player in self.players.keys() {
            if self.naalu_telepathy.as_ref() == Some(player) {
                player_nums.insert(player, 0);
                continue;
            }

            let strat_cards = self
                .strategy_card_holders
                .iter()
                .filter(|(_, p)| player.eq(p))
                .map(|(card, _)| card.card_number())
                .min()
                .with_context(|| format!("Player {player:?} does not hold a strategy card"))?;

            player_nums.insert(player, strat_cards);
        }

        let mut result = Ok(());
        self.turn_order.sort_by_key(|player| {
            let Some(num) = player_nums.get(player) else {
                result = Err(anyhow!("Player {player:?} does not have a strategy card."));
                return 99;
            };

            *num
        });

        result
    }

    /// Update the turn order according to table-order, starting with the speaker.
    pub fn calculate_strategy_turn_order(&mut self) -> anyhow::Result<()> {
        let speaker = self.speaker()?;
        let speaker_index = self
            .table_order
            .iter()
            .position(|p| p == speaker)
            .ok_or(anyhow!("No speaker index?"))?;

        self.turn_order = self.table_order.clone();
        self.turn_order.rotate_left(speaker_index); // speaker goes first

        Ok(())
    }

    /// If we're tracking turn time, save the result for the current player and return true.
    /// Otherwise return false.
    pub fn commit_turn_time(&mut self, timestamp: DateTime<Utc>) -> anyhow::Result<bool> {
        let current_turn_start_time = self.current_turn_start_time.take();

        if let Some(turn_start_time) = current_turn_start_time {
            let current_player = self.current_player()?;
            let turn_time_elapsed = timestamp - turn_start_time;
            let turn_time_elapsed = turn_time_elapsed
                .to_std()
                .context("turn time out of range")?;
            *self.players_play_time.entry(current_player).or_default() += turn_time_elapsed;
        }

        Ok(current_turn_start_time.is_some())
    }

    /// Advance to the next players turn, if all players have passed, advance to the next phase.
    pub fn advance_turn(&mut self, timestamp: DateTime<Utc>) -> anyhow::Result<()> {
        self.turn_number += 1;
        let current_player = self.current_player()?;
        let next_player = self.next_player_after(&current_player)?;

        if self.commit_turn_time(timestamp)? {
            self.current_turn_start_time = Some(timestamp);
        }

        if let Some(player) = self.interrupted_turn.take() {
            self.current_player = Some(player);
            return Ok(());
        }

        // everyone has passed, i guess. what do?
        if next_player.is_none() {
            match self.phase {
                Phase::Action => {
                    self.calculate_action_turn_order()?;
                    self.passed_players.clear();
                    self.phase = if self.entropic_scar_in_play() {
                        Phase::EntropicScar
                    } else {
                        Phase::Status
                    };
                    self.current_turn_start_time = None;

                    self.status_phase_state = Some(StatusPhaseState::new())
                }
                _ => bail!("wtf"),
            }
        } else {
            self.current_player = next_player;
        }

        Ok(())
    }

    /// Set the phase to the provided `phase` and update turn order accordingly.
    pub fn change_phase(&mut self, phase: Phase, timestamp: DateTime<Utc>) -> anyhow::Result<()> {
        self.phase = phase;
        match phase {
            Phase::Strategy => {
                self.calculate_strategy_turn_order()?;
                self.round = self.round.saturating_add(1);
                self.naalu_telepathy = (self.players.iter())
                    .find(|(_player_id, player)| player.faction == Faction::NaaluCollective)
                    .map(|(player_id, _player)| player_id.clone());
            }
            Phase::Action => {
                self.calculate_action_turn_order()?;
            }
            Phase::Status => {
                self.calculate_action_turn_order()?;
            }
            Phase::Agenda => {
                self.calculate_agenda_turn_order()?;
                self.agenda = Some(AgendaState::default());
            }
            Phase::Relics => { /* Nein */ }
            _ => bail!(
                "reset turn order called in unexpected phase: {:?}",
                self.phase
            ),
        }

        self.commit_turn_time(timestamp)?;
        if !self.time_tracking_paused {
            self.current_turn_start_time = Some(timestamp);
        }

        self.current_player = self.turn_order.first().cloned();

        Ok(())
    }

    /// Returns the player after the provided player.
    pub fn next_player_after(&self, after: &PlayerId) -> anyhow::Result<Option<PlayerId>> {
        let next_player = self
            .turn_order
            .iter()
            .skip_while(|&player| player != after)
            .skip(1)
            .chain(self.turn_order.iter())
            .find(|&player| !self.passed_players.contains(player))
            .cloned();

        Ok(next_player)
    }

    /// Returns the current players [PlyerId].
    pub fn current_player(&self) -> anyhow::Result<PlayerId> {
        self.current_player
            .clone()
            .ok_or(anyhow!("no active player"))
    }

    /// Returns the current speaker.
    pub fn speaker(&self) -> anyhow::Result<&PlayerId> {
        self.speaker.as_ref().ok_or(anyhow!("No speaker"))
    }

    /// Returns weather the provided player is the speaker.
    pub fn is_speaker(&self, player: &PlayerId) -> bool {
        self.speaker.as_ref().eq(&Some(player))
    }

    /// Asserts that the provided player is the currently active player.
    pub fn assert_player_turn(&self, player: &PlayerId) -> anyhow::Result<()> {
        let current_player = self.current_player()?;
        if &current_player != player {
            bail!("wrong players turn, current player is {current_player:?} got  {player:?}");
        }

        Ok(())
    }

    /// Assert that the provided phase is the current phase.
    pub fn assert_phase(&self, phase: Phase) -> anyhow::Result<()> {
        if self.phase != phase {
            bail!(
                "invalid game state, expected {phase:?}, was {:?}",
                self.phase
            );
        }
        Ok(())
    }

    /// Asserts that the provided expansion is enabled.
    pub fn assert_expansion(&self, expansion: &Expansion) -> anyhow::Result<()> {
        if !self.game_settings.expansions.is_enabled(expansion) {
            bail!("Expansion is not enabled {expansion:?}");
        }
        Ok(())
    }

    /// Whether `player` has claimed a Thunder's Edge expedition slot this turn.
    pub fn has_committed_to_thunders_edge(&self, player: &PlayerId) -> bool {
        (self.te_expedition.as_ref())
            .is_some_and(|te| te.has_committed(player, self.round, self.turn_number))
    }

    /// Asserts that the provided component is available with the enabled expansions, i.e. that
    /// its expansion is enabled and that it hasn't been removed or replaced by a newer version.
    pub fn assert_enabled<T: Component + Debug>(&self, component: &T) -> anyhow::Result<()> {
        if !component.is_enabled_in(&self.game_settings.expansions) {
            bail!("{component:?} is not available with the enabled expansions");
        }
        Ok(())
    }

    /// Asserts that the configured expansions is valid for the provided action.
    pub fn assert_action_expansion(&self, action: &StrategicPrimaryAction) -> anyhow::Result<()> {
        match action {
            StrategicPrimaryAction::Technology {
                first_tech: tech,
                second_tech: extra,
            } => {
                if let Some(t) = tech {
                    self.assert_enabled(t)?;
                }
                if let Some(t) = extra {
                    self.assert_enabled(t)?;
                }
            }
            StrategicPrimaryAction::Imperial { score_objective } => {
                if let Some(obj) = score_objective {
                    self.assert_enabled(obj)?;
                }
            }
            StrategicPrimaryAction::Politics { .. } => {}
        }
        Ok(())
    }

    /// Asserts that the configured expansions is valid for the provided action.
    pub fn assert_secondary_action_expansion(
        &self,
        action: &StrategicSecondaryAction,
    ) -> anyhow::Result<()> {
        match action {
            StrategicSecondaryAction::Technology { tech } => {
                self.assert_enabled(tech)?;
            }
            StrategicSecondaryAction::TechnologyJolNar {
                first_tech,
                second_tech,
            } => {
                self.assert_enabled(first_tech)?;
                if let Some(t) = second_tech {
                    self.assert_enabled(t)?;
                }
            }
            StrategicSecondaryAction::Skip => {}
            StrategicSecondaryAction::Leadership => {}
            StrategicSecondaryAction::Diplomacy => {}
            StrategicSecondaryAction::Politics => {}
            StrategicSecondaryAction::Construction => {}
            StrategicSecondaryAction::Trade => {}
            StrategicSecondaryAction::Warfare => {}
            StrategicSecondaryAction::Imperial => {}
        }
        Ok(())
    }

    /// Get a mutable reference to the currently active player.
    pub fn get_current_player(&mut self) -> anyhow::Result<&mut Player> {
        let current_player_id = match self.current_player.as_ref() {
            Some(p) => p,
            None => bail!("invalid game state, expected there to be a player"),
        };

        let current_player = match self.players.get_mut(current_player_id) {
            Some(p) => p,
            None => bail!(
                "invalid game state, expected current player (id: {current_player_id:?}) to be in the players map"
            ),
        };

        Ok(current_player)
    }

    /// Returns true if the player has performed any required initialization for their faction.
    pub fn player_initialization_finished(&self, player_id: &PlayerId) -> anyhow::Result<bool> {
        let Some(player) = self.players.get(player_id) else {
            anyhow::bail!("player does not exist (this is a bug)");
        };

        let faction_setup_finished = match player.faction {
            Faction::Winnu => player.technologies().len() == 1,
            Faction::ArgentFlight => player.technologies().len() == 2,
            Faction::CouncilKeleres => {
                player.technologies().len() == 2 && !player.planets.is_empty()
            }
            Faction::CrimsonRebellion => player.technologies().len() == 1,
            Faction::FirmamentObsidian => player.technologies().len() == 1,
            Faction::LastBastion => player.technologies().len() == 1,
            Faction::DeepwroughtScholarate => player.technologies().len() == 2,
            Faction::RalNelConsortium => player.technologies().len() == 1,
            _ => true,
        };
        let cultural_exchange_finished = !self
            .galactic_events
            .in_play
            .contains(&GalacticEvent::CulturalExchangeProgram)
            || player.faction == Faction::FirmamentObsidian
            || player.leaders_from.is_some();
        let minor_faction_finished = !self
            .galactic_events
            .in_play
            .contains(&GalacticEvent::MinorFactions)
            || player.minor_faction.is_some();

        Ok(faction_setup_finished && cultural_exchange_finished && minor_faction_finished)
    }

    /// The max number of players allowed for this game.
    pub fn max_players(&self) -> usize {
        self.game_settings.expansions.max_number_of_players()
    }

    /// Update [GameState::available_leaders].
    pub fn update_available_leaders(&mut self) {
        for (player_id, player) in self.players.iter() {
            let can_play_hero = self.score.scored_objectives_count(player_id) >= 3;

            // Yssaril can play everyones agents
            let yssaril_agent = |leader: &Leader| {
                if player.leader_faction() != Faction::YssarilTribes {
                    return false;
                }

                if !matches!(leader, Leader::Agent(..)) {
                    return false;
                }

                self.players
                    .values()
                    .any(|player| player.leader_faction() == leader.info().faction())
            };

            let available_to_this_player = Leader::iter()
                .filter(|leader| leader.is_enabled_in(&self.game_settings.expansions))
                // TODO: there may be cases where players can use other players factions
                .filter(|leader| {
                    leader.info().faction() == player.leader_faction() || yssaril_agent(leader)
                })
                .filter(|leader| leader.info().ability_kind() == LeaderAbilityKind::Action)
                .filter(|leader| can_play_hero || !matches!(leader, Leader::Hero(..)))
                .collect();

            self.available_leaders
                .insert(player_id.clone(), available_to_this_player);
        }
    }
}
