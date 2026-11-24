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
    /// Which technologies the player has, changed through [Player::gain_tech] and
    /// [Player::remove_tech] so that the faction effects of technologies stay in sync.
    technologies: BTreeSet<Technology>,
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
        let mut planets: HashMap<Planet, BTreeSet<PlanetAttachment>> = self
            .faction
            .get_starting_planets()
            .into_iter()
            .map(|p| (p, BTreeSet::new()))
            .collect();
        if self.faction == Faction::LastBastion {
            planets.insert(
                Planet::Ordinian,
                BTreeSet::from([PlanetAttachment::HeliosV1]),
            );
        }

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

    /// The number of ocean cards The Deepwrought Scholarate has in play, or 0 for other factions.
    pub fn ocean_cards(&self) -> u8 {
        match &*self.faction_state {
            FactionState::Deepwrought(state) => state.ocean_cards,
            FactionState::None | FactionState::Firmament(_) => 0,
        }
    }

    /// Set the number of ocean cards The Deepwrought Scholarate has in play.
    pub fn set_ocean_cards(&mut self, count: u8) -> anyhow::Result<()> {
        let FactionState::Deepwrought(state) = Arc::make_mut(&mut self.faction_state) else {
            bail!("Only The Deepwrought Scholarate has ocean cards");
        };
        state.ocean_cards = count;
        Ok(())
    }

    /// Keep the player's Helios attachments in line with the Helios V2 technology: they're all
    /// V2 while the player has the technology, and back to V1 when they don't.
    fn sync_helios_with_tech(&mut self) {
        let (from, to) = if self.has_tech(&Technology::HeliosV2) {
            (PlanetAttachment::HeliosV1, PlanetAttachment::HeliosV2)
        } else {
            (PlanetAttachment::HeliosV2, PlanetAttachment::HeliosV1)
        };
        for attachments in self.planets.values_mut() {
            if attachments.remove(&from) {
                attachments.insert(to.clone());
            }
        }
    }

    /// Remove a planet from the players planet list.
    pub fn remove_planet(&mut self, planet: &Planet) {
        self.planets.remove(planet);
    }

    /// The technologies the player has.
    pub fn technologies(&self) -> &BTreeSet<Technology> {
        &self.technologies
    }

    /// Gain a technology, failing if the player already has it.
    pub(crate) fn gain_tech(&mut self, tech: Technology) -> anyhow::Result<()> {
        ensure!(
            !self.has_tech(&tech),
            "Player {self:?} already has tech {tech:?}"
        );
        self.technologies.insert(tech);
        self.sync_helios_with_tech();
        Ok(())
    }

    /// Gain a technology if the player doesn't already have it. Returns whether it was gained.
    pub(crate) fn grant_tech(&mut self, tech: Technology) -> bool {
        let gained = self.technologies.insert(tech);
        self.sync_helios_with_tech();
        gained
    }

    /// Remove a technology. Returns whether the player had it.
    pub fn remove_tech(&mut self, tech: &Technology) -> bool {
        let removed = self.technologies.remove(tech);
        self.sync_helios_with_tech();
        removed
    }

    /// Research a technology (for actions stating 'gain', use [`Player::gain_tech()`] instead),
    /// performing necessary checks for that action.
    pub(crate) fn research_tech(&mut self, tech: Technology) -> anyhow::Result<()> {
        self.assert_can_research(&tech)?;
        self.gain_tech(tech)
    }

    /// Whether the player can research technologies at all: Nekro Virus can't.
    pub fn can_research(&self) -> bool {
        self.faction != Faction::NekroVirus
    }

    /// Asserts that the player is allowed to research `tech`: Nekro Virus can't research, and
    /// faction technologies can only be researched by their own faction.
    pub fn assert_can_research(&self, tech: &Technology) -> anyhow::Result<()> {
        ensure!(self.can_research(), "Nekro Virus cannot research techs");
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
