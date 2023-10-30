use serde::{Deserialize, Serialize};

use crate::data::{
    common::{color::Color, faction::Faction},
    components::planet::Planet,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub faction: Faction,
    pub color: Color,
    pub planets: Vec<Planet>,
}

impl Player {
    pub fn new(name: String, color: Color, faction: Faction) -> Self {
        let planets = faction.get_starting_planets();
        Self {
            name,
            faction,
            color,
            planets,
        }
    }
}
