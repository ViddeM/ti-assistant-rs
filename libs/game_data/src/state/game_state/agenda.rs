use std::collections::BTreeSet;

use anyhow::{anyhow, bail, ensure};
use serde::{Deserialize, Serialize};

use crate::{
    common::{faction::Faction, player_id::PlayerId},
    components::{
        agenda::{Agenda, AgendaElect, AgendaElectKind, AgendaKind, ForOrAgainst},
        objectives::{Objective, secret::SecretObjective},
    },
    state::agenda::{AgendaRecord, VoteState},
};

use super::GameState;

/// Why a player can't vote on agendas, see [GameState::vote_restriction].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoteRestriction {
    /// The player is [Faction::NekroVirus].
    NekroVirus,
    /// The player has discarded their Stellar Atomics token.
    StellarAtomics,
}

impl VoteRestriction {
    /// The message shown to the players.
    pub fn reason(&self) -> &'static str {
        match self {
            VoteRestriction::NekroVirus => "Nekro Virus cannot vote on agendas",
            VoteRestriction::StellarAtomics => "Stellar Atomics token used, cannot vote on agendas",
        }
    }
}

/// Information relevant to things that has happened on the gameboard.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgendaOverrideState {
    /// The vote state of this agenda override.
    pub vote_state: VoteState,
}

impl GameState {
    /// Update the turn order according to table-order, ending with the speaker.
    pub fn calculate_agenda_turn_order(&mut self) -> anyhow::Result<()> {
        let speaker = self.speaker()?;
        let speaker_index = self
            .table_order
            .iter()
            .position(|p| p == speaker)
            .ok_or(anyhow!("No speaker index?"))?;

        self.turn_order = self.table_order.clone();
        self.turn_order.rotate_left(speaker_index + 1); // speaker goes last
        self.turn_order
            // Argent Flight always votes first
            .sort_by_key(|p| self.players[p].faction != Faction::ArgentFlight);

        Ok(())
    }

    /// Whether rounds have an agenda phase, which they do once the custodians token has been
    /// taken from Mecatol Rex.
    pub fn is_agenda_enabled(&self) -> bool {
        self.score.custodians.is_some()
    }

    /// Why the player can't vote on agendas, `None` if they can (or don't exist).
    pub fn vote_restriction(&self, player: &PlayerId) -> Option<VoteRestriction> {
        let player_state = self.players.get(player)?;
        if player_state.faction == Faction::NekroVirus {
            Some(VoteRestriction::NekroVirus)
        } else if self.galactic_events.stellar_atomics_used.contains(player) {
            Some(VoteRestriction::StellarAtomics)
        } else {
            None
        }
    }

    /// Returns true if the objective used to be a public objective but no longer is: a secret
    /// objective made public by Classified Document Leaks, once the law has been discarded. The
    /// players who scored it keep their points.
    pub fn is_former_public_objective(&self, objective: &Objective) -> bool {
        let Objective::Secret(secret) = objective else {
            return false;
        };

        let leak = AgendaElect::SecretObjective(*secret);
        let leaked = self.agenda_vote_history.iter().any(|record| {
            record.vote.agenda == Agenda::ClassifiedDocumentLeaks
                && record.outcome.as_ref() == Some(&leak)
        });
        leaked && self.laws.get(&Agenda::ClassifiedDocumentLeaks) != Some(&leak)
    }

    /// Move a scored secret objective to the public objectives, scored by the player who owned it.
    fn leak_secret_objective(&mut self, secret: SecretObjective) -> anyhow::Result<()> {
        let Some((owner, secrets)) = (self.score.secret_objectives.iter_mut())
            .find(|(_, secrets)| secrets.contains(&secret))
        else {
            bail!("{secret:?} hasn't been scored by any player");
        };

        secrets.remove(&secret);
        let owner = owner.clone();
        self.score
            .revealed_objectives
            .insert(Objective::Secret(secret), BTreeSet::from([owner]));
        Ok(())
    }

    /// Resolve an agenda.
    pub fn resolve_agenda(
        &mut self,
        vote: VoteState,
        outcome: Option<AgendaElect>,
    ) -> anyhow::Result<()> {
        if let Some(outcome) = &outcome {
            ensure!(
                AgendaElectKind::from(outcome) == vote.elect,
                "invalid elect kind, expected {:?}",
                vote.elect
            )
        }

        let agenda_record = AgendaRecord {
            round: self.round,
            vote: vote.clone(),
            outcome: outcome.clone(),
        };

        if let Some(outcome) = outcome.as_ref() {
            if let (Agenda::ClassifiedDocumentLeaks, AgendaElect::SecretObjective(secret)) =
                (&vote.agenda, outcome)
            {
                self.leak_secret_objective(*secret)?;
            }

            if vote.kind == AgendaKind::Law
                && outcome != &AgendaElect::ForOrAgainst(ForOrAgainst::Against)
            {
                self.laws.insert(vote.agenda, outcome.clone());
            }

            self.score.add_agenda_record(&agenda_record);

            // TODO: resolve any other vote effects such as VPs or techs
        } else {
            // Do nothing, i.e. discard agenda without resolving it.
        }

        self.agenda_vote_history.push(agenda_record);
        Ok(())
    }
}
