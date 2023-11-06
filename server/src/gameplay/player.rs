use std::{collections::HashSet, sync::Arc};

use eyre::ensure;
use serde::{Deserialize, Serialize};

use crate::data::{
    common::{color::Color, faction::Faction},
    components::{planet::Planet, tech::Technology},
};

use super::error::GameError;

// TODO: maybe make this be not a string...
pub type PlayerId = Arc<str>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewPlayer {
    pub name: String,
    pub faction: Faction,
    pub color: Color,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub faction: Faction,
    pub color: Color,
    pub planets: HashSet<Planet>,
    pub technologies: HashSet<Technology>,
}

impl Player {
    pub fn remove_planet(&mut self, planet: &Planet) {
        self.planets.remove(planet);
    }

    pub fn take_tech(&mut self, tech: Technology) -> Result<(), GameError> {
        ensure!(
            !self.technologies.contains(&tech),
            "Player {self:?} already has tech {tech:?}"
        );
        self.technologies.insert(tech);
        Ok(())
    }
}

impl From<NewPlayer> for Player {
    fn from(new: NewPlayer) -> Self {
        let planets = new.faction.get_starting_planets();
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
