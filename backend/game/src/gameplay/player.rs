use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use eyre::ensure;
use serde::{Deserialize, Serialize};

use crate::data::{
    common::{color::Color, faction::Faction},
    components::{planet::Planet, planet_attachment::PlanetAttachment, tech::Technology},
};

use super::error::GameError;

// TODO: maybe make this be not a string...
/// A (per-game) unique ID for a player.
pub type PlayerId = Arc<str>;

/// A new player that is currently being created.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPlayer {
    /// The name of the player.
    pub name: String,
    /// Which faction the player is playing.
    pub faction: Faction,
    /// Which color the player has.
    pub color: Color,
}

/// A player in a running game.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// The name of the player.
    pub name: String,
    /// Which faction the player is playing.
    pub faction: Faction,
    /// Which color the player has.
    pub color: Color,
    /// Which planets the player controls and their attachments.
    pub planets: HashMap<Planet, HashSet<PlanetAttachment>>,
    /// Which technologies the player has.
    pub technologies: HashSet<Technology>,
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

    /// Returns true if the player currently has the technology.
    pub fn has_tech(&self, tech: &Technology) -> bool {
        self.technologies.contains(tech)
    }
}

impl From<NewPlayer> for Player {
    fn from(new: NewPlayer) -> Self {
        let planets = new
            .faction
            .get_starting_planets()
            .into_iter()
            .map(|p| (p, HashSet::new()))
            .collect();
        let techs = new.faction.get_starting_techs();
        Player {
            name: new.name,
            faction: new.faction,
            color: new.color,
            planets,
            technologies: techs,
        }
    }
}
