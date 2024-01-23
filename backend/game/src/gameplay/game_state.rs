use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use chrono::{DateTime, Utc};
use eyre::{bail, eyre, Context};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    data::{
        common::{expansions::Expansion, faction::Faction},
        components::{
            action_card::ActionCard,
            agenda::{Agenda, AgendaElect},
            frontier_card::FrontierCard,
            leaders::Leader,
            objectives::Objective,
            phase::Phase,
            planet::Planet,
            planet_attachment::PlanetAttachment,
            relic::Relic,
            strategy_card::StrategyCard,
            system::SystemId,
            tech::Technology,
        },
    },
    enum_map::EnumMap,
};

use super::{
    agenda::{AgendaRecord, AgendaState},
    error::GameError,
    event::{StrategicPrimaryAction, StrategicSecondaryAction},
    game_settings::GameSettings,
    player::{Player, PlayerId},
    score::Score,
    status::StatusPhaseState,
};

/// A snapshot of the game state.
#[derive(Clone, Default, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    /// The current round number of the game.
    pub round: u32,

    /// The settings for the game.
    pub game_settings: GameSettings,

    /// The current phase of the game.
    pub phase: Phase,

    /// Which players are in the game.
    pub players: HashMap<PlayerId, Player>,

    /// The current speaker, if any.
    pub speaker: Option<PlayerId>,

    /// The order that players are sitting around the table, starting with the speaker.
    pub table_order: Vec<PlayerId>,

    /// The current turn order, either based on table order or strategy cards (initiative).
    pub turn_order: Vec<PlayerId>,

    /// Which players hold which strategy cards.
    pub strategy_card_holders: EnumMap<StrategyCard, PlayerId>,

    /// The current player, if any.
    pub current_player: Option<PlayerId>,

    /// Which strategy cards have been spent this action phase.
    pub spent_strategy_cards: HashSet<StrategyCard>,

    /// Which players have passed this phase.
    pub passed_players: HashSet<PlayerId>,

    /// Tracks progress of the current action (if any) that is being taken.
    pub action_progress: Option<ActionPhaseProgress>,

    /// All things that concern scoring for the game.
    pub score: Score,

    /// State for agenda phase.
    pub agenda: Option<AgendaState>,

    /// List of past things voted on in the agenda phase.
    pub agenda_vote_history: Vec<AgendaRecord>,

    /// Laws in play.
    pub laws: EnumMap<Agenda, AgendaElect>,

    /// State for the status phase.
    pub status_phase_state: Option<StatusPhaseState>,

    /// Weather or not time should be tracked.
    pub time_tracking_paused: bool,

    /// Time taken by each player to complete their rounds during the action phase.
    ///
    /// This does not include the time taken for the current round, that will be calculated and
    /// included when the current player ends their turn.
    #[ts(type = "{ [playerId: string]: { secs: number, nanos: number } }")]
    pub players_play_time: HashMap<PlayerId, Duration>,

    /// When the current player started their turn.
    #[ts(type = "string | null")]
    pub current_turn_start_time: Option<DateTime<Utc>>,
}

/// The current progress of an action-phase action.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(tag = "t")]
#[ts(export)]
pub enum ActionPhaseProgress {
    /// The progress of a strategy card.
    Strategic(StrategicProgress),
    /// The progress of a tactical action.
    Tactical(TacticalProgress),
    /// The progress of an action card.
    ActionCard(ActionCardProgress),
    /// The progress of a leader action.
    Leader(LeaderProgress),
    /// The progress of a frontier card.
    FrontierCard(FrontierCardProgress),
    /// The progress of a relic action.
    Relic(RelicProgress),
}

impl ActionPhaseProgress {
    /// Weather the progress is for a strategy card.
    pub fn is_strategy_card(&self) -> bool {
        matches!(self, ActionPhaseProgress::Strategic(_))
    }

    /// Weather the progress is for a tactical action.
    pub fn is_tactical(&self) -> bool {
        matches!(self, ActionPhaseProgress::Tactical(_))
    }
}

/// Progress of a strategy card.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StrategicProgress {
    /// The strategy card being played.
    pub card: StrategyCard,
    /// What, if any, progress has been made for the primary part of the strategy card.
    pub primary: Option<StrategicPrimaryProgress>,
    /// What secondary actions other players have taken.
    pub other_players: HashMap<PlayerId, StrategicSecondaryProgress>,
}

/// The progress of the primary section of a strategy card.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum StrategicPrimaryProgress {
    /// Primary progress for the technology strategy card.
    Technology {
        /// What main technology was taken.
        tech: Option<Technology>,
        /// What, if any, extra tech was taken (and paid for).
        extra: Option<Technology>,
    },
    /// Primary progress for the politics strategy card.
    #[serde(rename_all = "camelCase")]
    Politics {
        /// Who the new speaker should be.
        new_speaker: PlayerId,
    },
    /// Primary progress for the imperial strategy card.
    Imperial {
        /// What objective, if any, was scored.
        objective: Option<Objective>,
    },
}

impl StrategicPrimaryProgress {
    /// Returns the default value for the provided strategy card and faction.
    pub fn default_for_card_and_faction(
        card: StrategyCard,
        faction: Faction,
    ) -> Option<StrategicPrimaryProgress> {
        if card == StrategyCard::Technology && faction == Faction::NekroVirus {
            return Some(StrategicPrimaryProgress::Technology {
                tech: None,
                extra: None,
            });
        }

        None
    }
}

/// The progress of the secondary portion of a strategy card.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[allow(missing_docs)]
pub enum StrategicSecondaryProgress {
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology {
        /// What tech was taken.
        tech: Technology,
    },
    Imperial,
    Skipped,
}

/// Progress during a tactical action.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct TacticalProgress {
    /// What system was activated, if any.
    pub activated_system: Option<SystemId>, // TODO: Maybe in the future we should track systems for all tactical actions (Could use some cool interactive map :eyes:)
    /// Which planets have been taken this far and the player who owned them previously (if any).
    pub taken_planets: EnumMap<Planet, Option<PlayerId>>,
    /// What planet attachments have been selected for the taken planets.
    /// NOTE: Does not include attachments kept when taken from another player.
    pub planet_attachments: EnumMap<Planet, PlanetAttachment>,
}

/// The progress of an action card being played.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ActionCardProgress {
    /// Which card is being played.
    pub card: ActionCard,
}

/// The progress of a leader action being played.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
#[ts(export)]
#[allow(missing_docs)]
pub enum LeaderProgress {
    /// This leader needs no special handling.
    Nothing { leader: Leader },
}

/// The progress of a frontier card being played.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct FrontierCardProgress {
    /// Which card is being played.
    pub card: FrontierCard,
}

/// The progress of a frontier card being played.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RelicProgress {
    /// The relic whose action is being taken.
    pub relic: Relic,
}

impl GameState {
    /// Update the turn order according to initiative order.
    pub fn calculate_action_turn_order(&mut self) -> Result<(), GameError> {
        self.turn_order = self.table_order.clone();

        // sort players by the smallest number of the strategy cards they hold (initiative order)
        self.turn_order.sort_by_key(|player| {
            self.strategy_card_holders
                .iter()
                .filter(|&(_, holder)| holder == player)
                .map(|(card, _)| card.card_number())
                .min()
        });

        Ok(())
    }

    /// Update the turn order according to table-order, starting with the speaker.
    pub fn calculate_turn_order_from_speaker(&mut self) -> Result<(), GameError> {
        let speaker = self.speaker()?;
        let speaker_index = self
            .table_order
            .iter()
            .position(|p| p == speaker)
            .ok_or(eyre!("No speaker index?"))?;

        let mut tmp = self.table_order.clone();
        tmp.rotate_left(speaker_index);
        self.turn_order = tmp;

        Ok(())
    }

    /// If we're tracking turn time, save the result for the current player and return true.
    /// Otherwise return false.
    pub fn commit_turn_time(&mut self, timestamp: DateTime<Utc>) -> Result<bool, GameError> {
        let current_turn_start_time = self.current_turn_start_time.take();

        if let Some(turn_start_time) = current_turn_start_time {
            let current_player = self.current_player()?;
            let turn_time_elapsed = timestamp - turn_start_time;
            let turn_time_elapsed = turn_time_elapsed
                .to_std()
                .wrap_err("turn time out of range")?;
            *self.players_play_time.entry(current_player).or_default() += turn_time_elapsed;
        }

        Ok(current_turn_start_time.is_some())
    }

    /// Advance to the next players turn, if all players have passed, advance to the next phase.
    pub fn advance_turn(&mut self, timestamp: DateTime<Utc>) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        let next_player = self.next_player_after(&current_player)?;

        if self.commit_turn_time(timestamp)? {
            self.current_turn_start_time = Some(timestamp);
        }

        // everyone has passed, i guess. what do?
        if next_player.is_none() {
            match self.phase {
                Phase::Action => {
                    self.calculate_action_turn_order()?;
                    self.passed_players.clear();
                    self.phase = Phase::Status;
                    self.current_turn_start_time = None;

                    self.status_phase_state = Some(StatusPhaseState::new(
                        self.expected_objectives_before_stage_two(),
                    ))
                }
                _ => bail!("wtf"),
            }
        } else {
            self.current_player = next_player;
        }

        Ok(())
    }

    /// Set the phase to the provided `phase` and update turn order accordingly.
    pub fn change_phase(
        &mut self,
        phase: Phase,
        timestamp: DateTime<Utc>,
    ) -> Result<(), GameError> {
        self.phase = phase;
        match phase {
            Phase::Strategy => {
                self.calculate_turn_order_from_speaker()?;
                self.round = self.round.saturating_add(1);
            }
            Phase::Action => {
                self.calculate_action_turn_order()?;
            }
            Phase::Status => {
                self.calculate_action_turn_order()?;
            }
            Phase::Agenda => {
                self.calculate_turn_order_from_speaker()?;
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
    pub fn next_player_after(&self, after: &PlayerId) -> Result<Option<PlayerId>, GameError> {
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
    pub fn current_player(&self) -> Result<PlayerId, GameError> {
        self.current_player.clone().ok_or(eyre!("no active player"))
    }

    /// Returns the current speaker.
    pub fn speaker(&self) -> Result<&PlayerId, GameError> {
        self.speaker.as_ref().ok_or(eyre!("No speaker"))
    }

    /// Asserts that the provided player is the currently active player.
    pub fn assert_player_turn(&self, player: &PlayerId) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        if &current_player != player {
            bail!("wrong players turn, expected {player:?}, got {current_player:?}");
        }

        Ok(())
    }

    /// Assert that the provided phase is the current phase.
    pub fn assert_phase(&self, phase: Phase) -> Result<(), GameError> {
        if self.phase != phase {
            bail!(
                "invalid game state, expected {phase:?}, was {:?}",
                self.phase
            );
        }
        Ok(())
    }

    /// Asserts that the provided expansion is enabled.
    pub fn assert_expansion(&self, expansion: &Expansion) -> Result<(), GameError> {
        if !self.game_settings.expansions.is_enabled(expansion) {
            bail!("Expansion is not enabled {expansion:?}");
        }
        Ok(())
    }

    /// Asserts that the configured expansions is valid for the provided action.
    pub fn assert_action_expansion(
        &self,
        action: &StrategicPrimaryAction,
    ) -> Result<(), GameError> {
        match action {
            StrategicPrimaryAction::Technology { tech, extra } => {
                self.assert_expansion(&tech.info().expansion)?;
                if let Some(t) = extra {
                    self.assert_expansion(&t.info().expansion)?;
                }
            }
            StrategicPrimaryAction::Imperial { score_objective } => {
                if let Some(obj) = score_objective {
                    self.assert_expansion(&obj.info().expansion)?;
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
    ) -> Result<(), GameError> {
        match action {
            StrategicSecondaryAction::Technology { tech } => {
                self.assert_expansion(&tech.info().expansion)?;
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
    pub fn get_current_player(&mut self) -> Result<&mut Player, GameError> {
        let current_player_id = match self.current_player.as_ref() {
            Some(p) => p,
            None => bail!("invalid game state, expected there to be a player"),
        };

        let current_player = match self.players.get_mut(current_player_id) {
            Some(p) => p,
            None => bail!("invalid game state, expected current player (id: {current_player_id:?}) to be in the players map")
        };

        Ok(current_player)
    }

    /// Returns the number of cards that is expected to have been revealed before we can start revealing stage II cards.
    pub fn expected_objectives_before_stage_two(&self) -> usize {
        let extras = self
            .laws
            .keys()
            .filter(|l| l == &&Agenda::IncentiveProgram || l == &&Agenda::ClassifiedDocumentLeaks)
            .count();

        5 + extras
    }

    /// Returns true if the player has performed any required initialization for their faction.
    pub fn player_initialization_finished(&self, player_id: &PlayerId) -> Result<bool, GameError> {
        let Some(player) = self.players.get(player_id) else {
            bail!("player does not exist (this is a bug)");
        };

        Ok(match player.faction {
            Faction::Winnu => player.technologies.len() == 1,
            Faction::ArgentFlight => player.technologies.len() == 2,
            Faction::CouncilKeleres => player.technologies.len() == 2 && !player.planets.is_empty(),
            _ => true,
        })
    }

    /// The max number of players allowed for this game.
    pub fn max_players(&self) -> usize {
        self.game_settings.expansions.max_number_of_players()
    }
}
