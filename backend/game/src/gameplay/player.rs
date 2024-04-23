use std::{collections::HashSet, sync::Arc};

use eyre::ensure;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    data::{
        common::{color::Color, faction::Faction},
        components::{
            planet::Planet, planet_attachment::PlanetAttachment, relic::Relic, tech::Technology,
        },
    },
    enum_map::EnumMap,
};

use super::{error::GameError, game_settings::Expansions};

// TODO: maybe make this be not a string...
/// A (per-game) unique ID for a player.
pub type PlayerId = Arc<str>;

/// A new player that is currently being created.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct NewPlayer {
    /// The name of the player.
    pub name: String,
    /// Which faction the player is playing.
    pub faction: Faction,
    /// Which color the player has.
    pub color: Color,
}

/// A player in a running game.
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Player {
    /// The name of the player.
    pub name: String,
    /// Which faction the player is playing.
    pub faction: Faction,
    /// Which color the player has.
    pub color: Color,
    /// Which planets the player controls and their attachments.
    pub planets: EnumMap<Planet, HashSet<PlanetAttachment>>,
    /// Which technologies the player has.
    pub technologies: HashSet<Technology>,
    /// Which relics the player currently owns.
    pub relics: HashSet<Relic>,
}

impl NewPlayer {
    /// Create a [Player] with the corrent starting techs and planets for [NewPlayer::faction].
    pub fn setup(self, expansions: &Expansions) -> Player {
        let planets = self
            .faction
            .get_starting_planets()
            .into_iter()
            .map(|p| (p, HashSet::new()))
            .collect();

        let techs = self.faction.get_starting_techs(expansions);
        Player {
            name: self.name,
            faction: self.faction,
            color: self.color,
            planets,
            technologies: techs,
            relics: HashSet::new(),
        }
    }
}

impl Player {
    /// Remove a planet from the players planet list.
    pub fn remove_planet(&mut self, planet: &Planet) {
        self.planets.remove(planet);
    }

    /// Add a technology to the players technologie list.
    pub fn take_tech(&mut self, tech: Technology) -> Result<(), GameError> {
        ensure!(
            !self.has_tech(&tech),
            "Player {self:?} already has tech {tech:?}"
        );
        self.technologies.insert(tech);
        Ok(())
    }

    /// Research a technology (for actions stating 'gain', use [`take_tech()`] instead), performing necessary checks for that action.
    pub fn research_tech(&mut self, tech: Technology) -> Result<(), GameError> {
        ensure!(
            self.faction != Faction::NekroVirus,
            "Nekro Virus cannot research techs"
        );
        self.take_tech(tech)
    }

    /// Returns true if the player currently has the technology.
    pub fn has_tech(&self, tech: &Technology) -> bool {
        self.technologies.contains(tech)
    }
}

/*
impl From<NewPlayer> for Player {
    fn from(new: NewPlayer) -> Self {
    }
}
*/
