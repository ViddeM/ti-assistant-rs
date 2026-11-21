use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    sync::Arc,
};

use anyhow::{bail, ensure};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    common::{color::Color, faction::Faction, game_settings::Expansions},
    components::{
        breakthrough::Breakthrough,
        ocean::Ocean,
        planet::Planet,
        planet_attachment::PlanetAttachment,
        relic::Relic,
        tech::{TechCategory, TechOrigin, Technology},
    },
    state::faction_state::{DeepwroughtState, FactionState, FirmamentSide, FirmamentState},
};

/// A new player that is currently being created.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    /// The faction whose leaders the player holds instead of their own ('Cultural Exchange
    /// Program').
    pub leaders_from: Option<Faction>,
    /// The faction whose home system the player was dealt as a minor faction system ('Minor
    /// Factions').
    pub minor_faction: Option<Faction>,
    /// The faction whose home system the player took as their own ('The Tribunii').
    pub home_system_from: Option<Faction>,
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
            Faction::DeepwroughtScholarate => {
                FactionState::Deepwrought(DeepwroughtState::default())
            }
            _ => FactionState::None,
        };
        let breakthrough = match self.faction {
            Faction::CrimsonRebellion => Some(self.faction.breakthrough()),
            _ => None,
        };
        Player {
            name: self.name,
            faction: self.faction,
            color: self.color,
            planets,
            technologies: techs,
            relics: BTreeSet::new(),
            breakthrough,
            faction_state: Arc::new(faction_state),
            leaders_from: None,
            minor_faction: None,
            home_system_from: None,
        }
    }
}

impl Player {
    /// Whether the player controls the Tomb of Emphidia, needed to play the Crown of Emphidia.
    pub fn controls_tomb_of_emphidia(&self) -> bool {
        self.planets
            .values()
            .any(|attachments| attachments.contains(&PlanetAttachment::TombOfEmphidia))
    }

    /// The faction whose leaders the player holds, see [Player::leaders_from].
    pub fn leader_faction(&self) -> Faction {
        self.leaders_from.unwrap_or(self.faction)
    }

    /// The faction whose home system is the player's home system, see
    /// [Player::home_system_from].
    pub fn home_system_faction(&self) -> Faction {
        self.home_system_from.unwrap_or(self.faction)
    }

    /// Which side of the Firmament / The Obsidian the player is playing, if they play that faction.
    pub fn firmament_side(&self) -> Option<FirmamentSide> {
        match &*self.faction_state {
            FactionState::Firmament(state) => Some(state.side),
            FactionState::None | FactionState::Deepwrought(_) => None,
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

    /// The ocean cards in play for The Deepwrought Scholarate, by the planet they are for.
    pub fn ocean_cards(&self) -> BTreeMap<Planet, Ocean> {
        match &*self.faction_state {
            FactionState::Deepwrought(state) => state.oceans.clone(),
            FactionState::None | FactionState::Firmament(_) => BTreeMap::new(),
        }
    }

    /// Start coexisting on `planet` (Oceanbound), gaining `ocean` if one is left. `ocean` must be
    /// `None` only when all ocean cards are already in play.
    pub fn begin_deepwrought_coexistence(
        &mut self,
        planet: Planet,
        ocean: Option<Ocean>,
    ) -> anyhow::Result<()> {
        let FactionState::Deepwrought(state) = &*self.faction_state else {
            bail!("Only The Deepwrought Scholarate can coexist");
        };
        ensure!(
            !state.coexisting.contains(&planet),
            "Already coexisting on {planet:?}"
        );
        let in_play: BTreeSet<Ocean> = state.oceans.values().copied().collect();
        match ocean {
            Some(ocean) => ensure!(
                !in_play.contains(&ocean),
                "{} is already in play",
                ocean.name()
            ),
            None => ensure!(
                in_play.len() == Ocean::iter().count(),
                "An ocean card must be gained while one is left"
            ),
        }

        let FactionState::Deepwrought(state) = Arc::make_mut(&mut self.faction_state) else {
            bail!("Only The Deepwrought Scholarate can coexist");
        };
        state.coexisting.insert(planet.clone());
        if let Some(ocean) = ocean {
            state.oceans.insert(planet, ocean);
        }
        Ok(())
    }

    /// Stop coexisting on `planet`, discarding the ocean card that was gained for it.
    pub fn end_deepwrought_coexistence(&mut self, planet: &Planet) -> anyhow::Result<()> {
        let FactionState::Deepwrought(state) = &*self.faction_state else {
            bail!("Only The Deepwrought Scholarate can coexist");
        };
        ensure!(
            state.coexisting.contains(planet),
            "Not coexisting on {planet:?}"
        );

        let FactionState::Deepwrought(state) = Arc::make_mut(&mut self.faction_state) else {
            bail!("Only The Deepwrought Scholarate can coexist");
        };
        state.coexisting.remove(planet);
        state.oceans.remove(planet);
        Ok(())
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
        self.assert_can_research(&tech)?;
        self.take_tech(tech)
    }

    /// Asserts that the player is allowed to research `tech`: Nekro Virus can't research, and
    /// faction technologies can only be researched by their own faction.
    pub fn assert_can_research(&self, tech: &Technology) -> anyhow::Result<()> {
        ensure!(
            self.faction != Faction::NekroVirus,
            "Nekro Virus cannot research techs"
        );
        ensure!(
            match tech.info().origin {
                TechOrigin::Base => true,
                TechOrigin::Faction(faction) => faction == self.faction,
            },
            "{tech:?} is another faction's technology"
        );
        Ok(())
    }

    /// Returns true if the player currently has the technology.
    pub fn has_tech(&self, tech: &Technology) -> bool {
        self.technologies.contains(tech)
    }

    /// Returns true if the player has any version of X-89 Bacterial Weapon.
    pub fn has_x89_bacterial_weapon(&self) -> bool {
        [
            Technology::X89BacterialWeapon,
            Technology::X89BacterialWeaponOmega,
            Technology::X89BacterialWeaponOmegaOmega,
        ]
        .iter()
        .any(|tech| self.has_tech(tech))
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
