use std::{
    collections::{BTreeSet, HashMap},
    sync::Arc,
};

use anyhow::{bail, ensure};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    common::{color::Color, faction::Faction, game_settings::Expansions},
    components::{
        breakthrough::Breakthrough,
        planet::Planet,
        planet_attachment::PlanetAttachment,
        relic::Relic,
        tech::{TechCategory, Technology},
    },
    state::faction_state::{FactionState, FirmamentSide, FirmamentState},
};

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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// The name of the player.
    pub name: String,
    /// Which faction the player is playing.
    pub faction: Faction,
    /// Which color the player has.
    pub color: Color,
    /// Which planets the player controls and their attachments.
    pub planets: HashMap<Planet, BTreeSet<PlanetAttachment>>,
    /// Which technologies the player has.
    pub technologies: BTreeSet<Technology>,
    /// Which relics the player currently owns.
    pub relics: BTreeSet<Relic>,
    /// Which breakthrough the player currently has, or none if it is not yet unlocked.
    pub breakthrough: Option<Breakthrough>,
    /// State that is specific to the players faction, if the faction has any.
    pub faction_state: Arc<FactionState>,
}

impl NewPlayer {
    /// Create a [Player] with the corrent starting techs and planets for [NewPlayer::faction].
    pub fn setup(self, expansions: &Expansions) -> Player {
        let planets = self
            .faction
            .get_starting_planets()
            .into_iter()
            .map(|p| (p, BTreeSet::new()))
            .collect();

        let techs = self.faction.get_starting_techs(expansions);
        let faction_state = match self.faction {
            Faction::FirmamentObsidian => FactionState::Firmament(FirmamentState::default()),
            _ => FactionState::None,
        };
        Player {
            name: self.name,
            faction: self.faction,
            color: self.color,
            planets,
            technologies: techs,
            relics: BTreeSet::new(),
            breakthrough: None,
            faction_state: Arc::new(faction_state),
        }
    }
}

impl Player {
    /// Which side of the Firmament / The Obsidian the player is playing, if they play that faction.
    pub fn firmament_side(&self) -> Option<FirmamentSide> {
        match &*self.faction_state {
            FactionState::Firmament(state) => Some(state.side),
            FactionState::None => None,
        }
    }

    /// The pretty name of the players faction, taking the side of two-sided factions into account.
    pub fn faction_name(&self) -> String {
        match self.firmament_side() {
            Some(FirmamentSide::Firmament) => "The Firmament".to_string(),
            Some(FirmamentSide::Obsidian) => "The Obsidian".to_string(),
            None => self.faction.name(),
        }
    }

    /// The breakthrough the player gets when unlocking their breakthrough.
    pub fn faction_breakthrough(&self) -> Breakthrough {
        match self.firmament_side() {
            Some(FirmamentSide::Obsidian) => Breakthrough::TheReaping,
            _ => self.faction.breakthrough(),
        }
    }

    /// Unlock the player's breakthrough, unless it's already unlocked.
    pub fn gain_breakthrough(&mut self) {
        if self.breakthrough.is_none() {
            self.breakthrough = Some(self.faction_breakthrough());
        }
    }

    /// Flip the player from The Firmament to The Obsidian ('Puppets of the Blade').
    ///
    /// This flips the faction specific components of the player, i.e. their breakthrough.
    /// Planets in the Firmament home system are flipped for all players using
    /// [`Player::flip_home_system_planets()`].
    pub fn become_obsidian(&mut self) -> anyhow::Result<()> {
        let FactionState::Firmament(state) = Arc::make_mut(&mut self.faction_state) else {
            bail!("Only The Firmament can become The Obsidian");
        };

        ensure!(
            state.side == FirmamentSide::Firmament,
            "Already playing as The Obsidian"
        );

        state.side = FirmamentSide::Obsidian;
        if self.breakthrough == Some(Breakthrough::TheSowing) {
            self.breakthrough = Some(Breakthrough::TheReaping);
        }
        Ok(())
    }

    /// Flip any planets the player controls in the Firmament / Obsidian home system.
    pub fn flip_home_system_planets(&mut self) {
        for (firmament, obsidian) in [
            (Planet::Cronos, Planet::CronosHollow),
            (Planet::Tallin, Planet::TallinHollow),
        ] {
            if let Some(attachments) = self.planets.remove(&firmament) {
                self.planets.insert(obsidian, attachments);
            }
        }
    }

    /// Remove a planet from the players planet list.
    pub fn remove_planet(&mut self, planet: &Planet) {
        self.planets.remove(planet);
    }

    /// Add a technology to the players technologie list.
    pub fn take_tech(&mut self, tech: Technology) -> anyhow::Result<()> {
        ensure!(
            !self.has_tech(&tech),
            "Player {self:?} already has tech {tech:?}"
        );
        self.technologies.insert(tech);
        Ok(())
    }

    /// Research a technology (for actions stating 'gain', use [`take_tech()`] instead), performing necessary checks for that action.
    pub fn research_tech(&mut self, tech: Technology) -> anyhow::Result<()> {
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

    /// The technology specialties of the planets the player controls, including those gained from attachments.
    pub fn controlled_tech_specialties(&self) -> BTreeSet<TechCategory> {
        self.planets
            .iter()
            .flat_map(|(planet, attachments)| {
                planet
                    .info()
                    .tech_specialities
                    .into_iter()
                    .chain(attachments.iter().filter_map(|a| a.info().tech_specialty))
            })
            .collect()
    }

    /// Returns true if the player controls planets with all 4 types of technology specialties.
    pub fn controls_all_tech_specialties(&self) -> bool {
        let specialties = self.controlled_tech_specialties();
        TechCategory::iter().all(|c| specialties.contains(&c))
    }
}
