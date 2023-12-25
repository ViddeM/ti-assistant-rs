use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use chrono::{DateTime, Utc};
use eyre::{bail, eyre, Context};
use serde::{Deserialize, Serialize};

use crate::data::components::{
    action_card::ActionCard, phase::Phase, planet::Planet, strategy_card::StrategyCard,
    system::SystemId, tech::Technology,
};

use super::{
    error::GameError,
    player::{Player, PlayerId},
    score::Score,
};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
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
    pub strategy_card_holders: HashMap<StrategyCard, PlayerId>,

    /// The current player, if any.
    pub current_player: Option<PlayerId>,

    /// Which strategy cards have been spent this action phase.
    pub spent_strategy_cards: HashSet<StrategyCard>,

    /// Which players have passed this phase.
    pub passed_players: HashSet<PlayerId>,

    /// Tracks progress of a strategy card action.
    pub action_progress: Option<ActionPhaseProgress>,

    pub score: Score,

    pub time_tracking_paused: bool,

    /// Time taken by each player to complete their rounds during the action phase.
    ///
    /// This does not include the time taken for the current round, that will be calculated and
    /// included when the current player ends their turn.
    pub players_play_time: HashMap<PlayerId, Duration>,

    pub current_turn_start_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionPhaseProgress {
    Strategic(StrategicProgress),
    Tactical(TacticalProgress),
    ActionCard(ActionCardProgress),
}

impl ActionPhaseProgress {
    pub fn is_strategy_card(&self) -> bool {
        matches!(self, ActionPhaseProgress::Strategic(_))
    }

    pub fn is_tactical(&self) -> bool {
        matches!(self, ActionPhaseProgress::Tactical(_))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategicProgress {
    pub card: StrategyCard,
    pub primary: Option<StrategicPrimaryProgress>,
    pub other_players: HashMap<PlayerId, StrategicSecondaryProgress>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StrategicPrimaryProgress {
    Technology {
        tech: Technology,
        extra: Option<Technology>,
    },
    #[serde(rename_all = "camelCase")]
    Politics { new_speaker: PlayerId },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StrategicSecondaryProgress {
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology { tech: Technology },
    Imperial,
    Skipped,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TacticalProgress {
    pub activated_system: Option<SystemId>, // TODO: Maybe in the future we should track systems for all tactical actions (Could use some cool interactive map :eyes:)
    pub taken_planets: Vec<Planet>,
}

impl TacticalProgress {
    /// Returns true if the provided system is the currently activated system or there isn't an activated system.
    fn system_is_activated_or_none(&self, system_id: &SystemId) -> bool {
        match self.activated_system.as_ref() {
            Some(id) => id == system_id,
            None => true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyCardProgress {
    pub card: StrategyCard,
    pub other_players: HashMap<PlayerId, bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionCardProgress {
    pub card: ActionCard,
}

impl GameState {
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
                }
                _ => bail!("wtf"),
            }
        } else {
            self.current_player = next_player;
        }

        Ok(())
    }

    pub fn change_phase(
        &mut self,
        phase: Phase,
        timestamp: DateTime<Utc>,
    ) -> Result<(), GameError> {
        self.phase = phase;
        match phase {
            Phase::Strategy => {
                self.calculate_turn_order_from_speaker()?;
            }
            Phase::Action => {
                self.calculate_action_turn_order()?;
            }
            Phase::Status => {
                self.calculate_action_turn_order()?;
            }
            Phase::Agenda => {
                self.calculate_turn_order_from_speaker()?;
            }
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

    pub fn current_player(&self) -> Result<PlayerId, GameError> {
        self.current_player.clone().ok_or(eyre!("no active player"))
    }

    pub fn speaker(&self) -> Result<&PlayerId, GameError> {
        self.speaker.as_ref().ok_or(eyre!("No speaker"))
    }

    pub fn assert_player_turn(&self, player: &PlayerId) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        if &current_player != player {
            bail!("wrong players turn, expected {player:?}, got {current_player:?}");
        }

        Ok(())
    }

    pub fn assert_phase(&self, phase: Phase) -> Result<(), GameError> {
        if self.phase != phase {
            bail!(
                "invalid game state, expected {phase:?}, was {:?}",
                self.phase
            );
        }
        Ok(())
    }

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
}
