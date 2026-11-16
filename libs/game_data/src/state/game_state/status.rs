use crate::components::{galactic_event::GalacticEvent, phase::Phase};

use super::GameState;

/// The first round whose status phase reveals a stage II objective. Setup reveals two of the
/// five stage I objectives and the status phases of rounds 1 to 3 reveal one each.
const FIRST_STAGE_TWO_ROUND: u32 = 4;

impl GameState {
    /// The number of status phases that have started: one per earlier round, plus the current
    /// round's once it has begun.
    pub fn status_phases_started(&self) -> u32 {
        let this_round = matches!(
            self.phase,
            Phase::EntropicScar | Phase::Status | Phase::Relics | Phase::Agenda
        );
        self.round.saturating_sub(1) + u32::from(this_round)
    }

    /// Returns true if this round's status phase reveals a stage II objective instead of a stage I
    /// objective. Extra objectives revealed outside of the status phase don't change it.
    pub fn status_phase_reveals_stage_two(&self) -> bool {
        self.round >= FIRST_STAGE_TWO_ROUND
    }

    /// Returns true if the status phase reveals a public objective, which it doesn't under
    /// Civilized Society since every public objective is revealed at setup.
    pub fn status_phase_reveals_objective(&self) -> bool {
        !self
            .galactic_events
            .in_play
            .contains(&GalacticEvent::CivilizedSociety)
    }

    /// Returns true if a player can score any number of public objectives in the status phase,
    /// which they can under Civilized Society.
    pub fn status_phase_scores_any_number_of_public_objectives(&self) -> bool {
        self.galactic_events
            .in_play
            .contains(&GalacticEvent::CivilizedSociety)
    }
}
