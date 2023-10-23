use crate::data::{common::faction::Faction, components::planet::Planet};

pub struct Player {
    name: String,
    faction: Faction,
    planets: Vec<Planet>,
}

impl Player {
    pub fn new(name: String, faction: Faction) -> Self {
        let planets = faction.get_starting_planets();
        Self {
            name,
            faction,
            planets,
        }
    }
}
