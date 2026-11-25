use std::collections::VecDeque;

use anyhow::ensure;
use serde::{Deserialize, Serialize};

use crate::{common::player_id::PlayerId, components::phase::Phase};

use super::{GameState, action_progress::ActionPhaseProgress};

/// The state for the fracture progress
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FractureProgressState {
    pub progress: FractureProgress,
    /// The phase to return to after fracture rolling / spawning is complete.
    pub phase: Phase,
    /// The action that was in progress when the roll started, resumed once the roll is complete.
    pub interrupted: Option<Box<ActionPhaseProgress>>,
}

/// The progress of fracture spawning & info.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FractureProgress {
    /// When someone is rolling for the fracture to spawn.
    Roll(FractureRollState),
    /// When the fracture has spawned.
    Spawned { player: PlayerId },
}

/// The fracture roll state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FractureRollState {
    /// The player who is rolling.
    player: PlayerId,
    /// Other players that are waiting to roll at the same time.
    queued: VecDeque<PlayerId>,
}

impl FractureRollState {
    /// Add a player to the roll queue.
    pub fn add_player(&mut self, new_player: PlayerId) -> anyhow::Result<()> {
        ensure!(
            self.player != new_player,
            "Player {new_player} is already rolling for the fracture"
        );
        ensure!(
            !self.queued.contains(&new_player),
            "Player {new_player} is already in the queue to roll for the fracture"
        );
        self.queued.push_back(new_player);

        Ok(())
    }

    /// Let the next player roll for the fracture.
    /// Returns the new player or None if there are none more that are waiting to roll.
    pub fn next_player(&mut self) -> Option<PlayerId> {
        let next = self.queued.pop_front()?;

        self.player = next.clone();

        Some(next)
    }

    /// Create a new fracture roll state with the provided player as first roller.
    pub fn new(player: PlayerId) -> Self {
        Self {
            player,
            queued: VecDeque::new(),
        }
    }

    /// Returns the current player rolling for the fracture.
    pub fn current_player(&self) -> &PlayerId {
        &self.player
    }
}

impl GameState {
    /// Start `player`'s roll for the Fracture, unless it's already in play, or queue them behind
    /// the roll in progress. The game returns to the phase and action it is in now once the rolls
    /// are done (see [GameState::end_fracture]), so call this after the event's own phase and
    /// action changes.
    pub fn roll_for_the_fracture(&mut self, player: &PlayerId) -> anyhow::Result<()> {
        if self.fracture_in_play || matches!(self.phase, Phase::Creation | Phase::Setup) {
            // Setup breakthroughs roll when the game starts, see `Event::StartGame`.
            return Ok(());
        }

        if let Some(ActionPhaseProgress::Fracture(FractureProgressState {
            progress: FractureProgress::Roll(roll_state),
            ..
        })) = &mut self.action_progress
        {
            return roll_state.add_player(player.clone());
        }

        let interrupted = self.action_progress.take().map(Box::new);
        self.action_progress = Some(ActionPhaseProgress::Fracture(FractureProgressState {
            progress: FractureProgress::Roll(FractureRollState::new(player.clone())),
            phase: self.phase,
            interrupted,
        }));
        self.phase = Phase::Fracture;
        Ok(())
    }

    /// Return to the phase and action that the Fracture roll interrupted.
    pub fn end_fracture(&mut self, state: FractureProgressState) {
        self.phase = state.phase;
        self.action_progress = state.interrupted.map(|action| *action);
    }
}
