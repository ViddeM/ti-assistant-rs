use anyhow::{Context, ensure};

use crate::{
    common::player_id::PlayerId,
    components::{
        breakthrough::Breakthrough, phase::Phase, planet::Planet, relic::Relic, tech::Technology,
    },
};

use super::{
    GameState,
    action_progress::{ActionPhaseProgress, RelicProgress, RelicStep},
};

impl GameState {
    /// Give `relic` to `owner`, or to nobody, taking it from whoever had it.
    ///
    /// Shard of the Throne's point follows the relic, and gaining The Quantumcore unlocks the new
    /// owner's breakthrough.
    pub fn set_relic_owner(&mut self, relic: Relic, owner: Option<PlayerId>) -> anyhow::Result<()> {
        if let Some(p) = owner.as_ref() {
            ensure!(self.players.contains_key(p), "Player doesn't exist '{p}'");
        }

        for (id, player) in self.players.iter_mut() {
            if owner.as_ref() == Some(id) {
                player.relics.insert(relic.clone());
            } else {
                player.relics.remove(&relic);
            }
        }

        match (&relic, owner) {
            (Relic::ShardOfTheThrone, owner) => self.score.shard_of_the_throne = owner,
            (Relic::TheQuantumcore, Some(owner)) => self.gain_breakthrough(&owner)?,
            _ => {}
        }

        Ok(())
    }

    /// Give `relic` to `player` as (part of) their action, after which their turn ends, unless
    /// it's the Book of Latvinia: they research with it first.
    pub fn gain_relic_as_action(&mut self, player: PlayerId, relic: Relic) -> anyhow::Result<()> {
        if relic == Relic::BookOfLatvinia {
            self.action_progress = Some(ActionPhaseProgress::Relic(RelicProgress {
                relic: relic.clone(),
                step: RelicStep::Gain,
            }));
            self.phase = Phase::RelicAction;
        } else {
            self.phase = Phase::EndActionTurn;
        }
        self.set_relic_owner(relic, Some(player))
    }

    /// Unlock the player's breakthrough and apply its on-gain effects, unless it's already
    /// unlocked.
    pub fn gain_breakthrough(&mut self, player: &PlayerId) -> anyhow::Result<()> {
        let player_state = self.players.get_mut(player).context("Player not found")?;
        if player_state.breakthrough.is_some() {
            return Ok(());
        }

        let breakthrough = player_state.faction_breakthrough();
        player_state.breakthrough = Some(breakthrough.clone());
        self.apply_breakthrough_gained(player, &breakthrough)
    }

    /// Apply the effects of `player` gaining `breakthrough`.
    pub fn apply_breakthrough_gained(
        &mut self,
        player: &PlayerId,
        breakthrough: &Breakthrough,
    ) -> anyhow::Result<()> {
        match breakthrough {
            Breakthrough::IIHQModernization => {
                self.set_planet_owner(Planet::CustodiaVigiliaOmega, Some(player.clone()))?;
            }
            Breakthrough::StellarGenesis => {
                self.set_planet_owner(Planet::Avernus, Some(player.clone()))?;
            }
            Breakthrough::AlRaithIxIanovar => {
                self.fracture_in_play = true;
            }
            _ => {}
        }
        self.roll_for_the_fracture(player)
    }

    /// Give `player` `tech` and apply its on-gain effects, see [Player::gain_tech].
    ///
    /// [Player::gain_tech]: crate::state::player::Player::gain_tech
    pub fn gain_tech(&mut self, player: &PlayerId, tech: Technology) -> anyhow::Result<()> {
        (self.players.get_mut(player))
            .context("Player not found")?
            .gain_tech(tech.clone())?;
        self.apply_tech_gained(player, &tech)
    }

    /// Give `player` `tech` unless they already have it, applying its on-gain effects if they
    /// didn't, see [Player::grant_tech].
    ///
    /// [Player::grant_tech]: crate::state::player::Player::grant_tech
    pub fn grant_tech(&mut self, player: &PlayerId, tech: Technology) -> anyhow::Result<()> {
        let gained = (self.players.get_mut(player))
            .context("Player not found")?
            .grant_tech(tech.clone());
        if gained {
            self.apply_tech_gained(player, &tech)?;
        }
        Ok(())
    }

    /// Have `player` research `tech` and apply its on-gain effects, see [Player::research_tech].
    ///
    /// [Player::research_tech]: crate::state::player::Player::research_tech
    pub fn research_tech(&mut self, player: &PlayerId, tech: Technology) -> anyhow::Result<()> {
        (self.players.get_mut(player))
            .context("Player not found")?
            .research_tech(tech.clone())?;
        self.apply_tech_gained(player, &tech)
    }

    /// Apply the effects of `player` gaining `tech`.
    fn apply_tech_gained(&mut self, player: &PlayerId, tech: &Technology) -> anyhow::Result<()> {
        if *tech == Technology::IIHQModernization {
            self.set_planet_owner(Planet::CustodiaVigilia, Some(player.clone()))?;
        }
        Ok(())
    }
}
