use std::collections::HashMap;

use anyhow::ensure;

use crate::{
    common::player_id::PlayerId,
    components::{galactic_event::GalacticEvent, planet::Planet},
};

use super::{GameState, map::PlanetLocation};

impl GameState {
    /// Asserts that the provided galactic event is in play.
    pub fn assert_galactic_event(&self, event: &GalacticEvent) -> anyhow::Result<()> {
        ensure!(
            self.galactic_events.in_play.contains(event),
            "{} isn't in play",
            event.info().name
        );
        Ok(())
    }

    /// Whether `planet` can be taken with Rapid Mobilization: a planet on a system tile on the
    /// board, outside the home systems, that no player controls.
    pub fn is_rapid_mobilization_planet(&self, planet: &Planet) -> bool {
        self.planet_location(planet) == PlanetLocation::SystemTile
            && !self.is_home_planet(planet)
            && !self
                .players
                .values()
                .any(|p| p.planets.contains_key(planet))
    }

    /// The commodities beneath the monument on `planet`, if it has one: one per status phase
    /// started since it had none.
    pub fn monument_commodities(&self, planet: &Planet) -> Option<u32> {
        let empty_at = self.galactic_events.monuments.get(planet)?;
        Some(u32::try_from(i64::from(self.status_phases_started()) - empty_at).unwrap_or(0))
    }

    /// The victory points of the players who control a planet with a monument: 1 per 3
    /// commodities beneath it.
    pub fn monument_points(&self) -> HashMap<PlayerId, i8> {
        let mut points = HashMap::new();
        for planet in self.galactic_events.monuments.keys() {
            let monument_points = self.monument_commodities(planet).unwrap_or(0) / 3;
            let owner = (self.players.iter())
                .find(|(_, player)| player.planets.contains_key(planet))
                .filter(|_| monument_points > 0);
            if let Some((owner, _)) = owner {
                *points.entry(owner.clone()).or_insert(0) +=
                    i8::try_from(monument_points).unwrap_or(i8::MAX);
            }
        }
        points
    }

    /// Whether the game has a post-setup phase between setup and the first strategy phase: when
    /// Rapid Mobilization is in play.
    pub fn has_post_setup_phase(&self) -> bool {
        self.galactic_events
            .in_play
            .contains(&GalacticEvent::RapidMobilization)
    }

    /// Zealous Orthodoxy: the first of `candidates` with 2 scored secret objectives gains its point,
    /// if the event is in play and nobody has gained it yet.
    pub fn gain_zealous_orthodoxy(&mut self, candidates: &[PlayerId]) {
        if !self
            .galactic_events
            .in_play
            .contains(&GalacticEvent::ZealousOrthodoxy)
            || self.score.galactic_events.zealous_orthodoxy.is_some()
        {
            return;
        }

        self.score.galactic_events.zealous_orthodoxy = candidates
            .iter()
            .find(|player| self.score.scored_secret_objectives_count(player) >= 2)
            .cloned();
    }
}
