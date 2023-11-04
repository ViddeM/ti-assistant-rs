use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::data::{
    common::{color::Color, faction::Faction},
    components::planet::Planet,
};

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
}

impl Player {
    pub fn remove_planet(&mut self, planet: &Planet) {
        self.planets.remove(planet);
    }
}

impl From<NewPlayer> for Player {
    fn from(new: NewPlayer) -> Self {
        let planets = new.faction.get_starting_planets();
        Player {
            name: new.name,
            faction: new.faction,
            color: new.color,
            planets: planets,
        }
    }
}
