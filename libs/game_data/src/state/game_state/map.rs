use std::collections::BTreeSet;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    common::{expansions::Expansion, faction::Faction, map::HexMap, player_id::PlayerId},
    components::{
        planet::{Planet, PlanetTrait},
        relic::Relic,
        system::{AnomalyType, SystemId},
    },
    state::faction_state::FirmamentSide,
};

use super::GameState;

/// Where a planet is on the board, see [GameState::planet_location].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanetLocation {
    /// The planet is part of a pre-defined system, see
    /// [crate::components::system::System::for_planet].
    SystemTile,
    /// The planet is a token on the board, in the given system if that is tracked.
    Token(Option<SystemId>),
    /// The planet hasn't been spawned (Mirage, Avernus, Thunder's Edge), only exists as a card
    /// (Custodia Vigilia), is on the hidden side of the Firmament / The Obsidian home system tile,
    /// is in the Fracture before it's in play or has been destroyed.
    NotOnBoard,
}

/// Information relevant to things that has happened on the gameboard.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MapData {
    /// Map information relevant to games imported from milty draft.
    pub milty_information: Option<MiltyInformation>,
    /// Which planets (if any) have been destroyed or purged.
    pub destroyed_planets: Vec<Planet>,
    /// Which planets have been given to a player since the game started, see
    /// [GameState::has_been_owned].
    pub ever_owned_planets: BTreeSet<Planet>,
}

/// Map information only relevant / obtainable for games imported from milty draft.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MiltyInformation {
    /// The hex map for the game.
    pub hex_map: HexMap,
    /// The system that contains mirage (if any).
    pub mirage_system: Option<SystemId>,
}

impl GameState {
    /// Where the planet is on the board.
    ///
    /// Planets without a system tile are only on the board once they have been placed. Where
    /// Mirage and Avernus are isn't tracked, Thunder's Edge is where the expedition placed it and
    /// Custodia Vigilia only exists as a card. Only the planets on the shown side of the
    /// Firmament / The Obsidian home system tile are on the board. The Fracture planets are only
    /// on the board once the Fracture is in play. A planet that has been destroyed isn't on the
    /// board.
    pub fn planet_location(&self, planet: &Planet) -> PlanetLocation {
        match planet {
            _ if self.map_data.destroyed_planets.contains(planet) => PlanetLocation::NotOnBoard,
            Planet::Cocytus | Planet::Lethe | Planet::Phlegethon | Planet::Styx
                if !self.fracture_in_play =>
            {
                PlanetLocation::NotOnBoard
            }
            // Only one side of the Firmament / The Obsidian home system tile (96A / 96B) is on
            // the board.
            Planet::Cronos | Planet::Tallin if self.obsidian_in_play() => {
                PlanetLocation::NotOnBoard
            }
            Planet::CronosHollow | Planet::TallinHollow if !self.obsidian_in_play() => {
                PlanetLocation::NotOnBoard
            }
            _ if planet.has_system_tile() => PlanetLocation::SystemTile,
            Planet::ThundersEdge => self
                .te_expedition
                .as_ref()
                .and_then(|te| te.completed.as_ref())
                .map(|completion| PlanetLocation::Token(Some(completion.placed_in.clone())))
                .unwrap_or(PlanetLocation::NotOnBoard),
            Planet::Mirage | Planet::Avernus
                if self
                    .players
                    .values()
                    .any(|p| p.planets.contains_key(planet)) =>
            {
                PlanetLocation::Token(None)
            }
            _ => PlanetLocation::NotOnBoard,
        }
    }

    /// The minor factions dealt to the players ('Minor Factions').
    pub fn minor_factions(&self) -> BTreeSet<Faction> {
        (self.players.values())
            .filter_map(|player| player.minor_faction)
            .collect()
    }

    /// The minor faction whose home system holds `planet`, if a player was dealt that faction
    /// ('Minor Factions').
    pub fn minor_faction_of(&self, planet: &Planet) -> Option<Faction> {
        (self.players.values())
            .filter_map(|player| player.minor_faction)
            .find(|faction| planet.is_in_home_system_of(*faction))
    }

    /// Whether `player` can be dealt `faction` as their minor faction: a faction that isn't being
    /// played and isn't another player's minor faction, with a home system that no player
    /// controls a planet of.
    pub fn is_minor_faction_available(&self, player: &PlayerId, faction: Faction) -> bool {
        let home_planets = faction.get_starting_planets();
        !home_planets.is_empty()
            && self.players.iter().all(|(id, other)| {
                other.faction != faction
                    && (id == player || other.minor_faction != Some(faction))
                    && home_planets
                        .iter()
                        .all(|planet| !other.planets.contains_key(planet))
            })
    }

    /// Whether `planet` is a home planet. Minor faction systems don't count as home systems.
    pub fn is_home_planet(&self, planet: &Planet) -> bool {
        planet.is_in_home_system() && self.minor_faction_of(planet).is_none()
    }

    /// The traits of `planet`, including those added by its attachments. Planets in minor faction
    /// systems have all three traits.
    pub fn planet_traits(&self, planet: &Planet) -> BTreeSet<PlanetTrait> {
        if self.minor_faction_of(planet).is_some() {
            return BTreeSet::from([
                PlanetTrait::Cultural,
                PlanetTrait::Hazardous,
                PlanetTrait::Industrial,
            ]);
        }

        let attachments = (self.players.values()).filter_map(|player| player.planets.get(planet));
        planet
            .info()
            .planet_traits
            .into_iter()
            .chain(
                attachments
                    .flatten()
                    .flat_map(|attachment| attachment.info().added_planet_traits),
            )
            .collect()
    }

    /// The minor faction whose alliance card `player` takes for controlling every planet in its
    /// system, if `planet` is in that system.
    pub fn minor_faction_alliance(&self, player: &PlayerId, planet: &Planet) -> Option<Faction> {
        let faction = self.minor_faction_of(planet)?;
        let player = self.players.get(player)?;
        (faction.get_starting_planets())
            .iter()
            .all(|planet| player.planets.contains_key(planet))
            .then_some(faction)
    }

    /// Give `planet` and its attachments to `owner`, or to nobody, returning the previous owner.
    ///
    /// Shard of the Throne follows a legendary planet, or a planet in its holder's home system, to
    /// its new owner, and the first player to control Mecatol Rex gains the custodians token.
    pub fn set_planet_owner(
        &mut self,
        planet: Planet,
        owner: Option<PlayerId>,
    ) -> anyhow::Result<Option<PlayerId>> {
        let (previous_owner, attachments) = self
            .players
            .iter_mut()
            .find_map(|(id, player)| {
                (player.planets.remove(&planet)).map(|attachments| (Some(id.clone()), attachments))
            })
            .unwrap_or_default();

        if previous_owner.is_some() || owner.is_some() {
            self.map_data.ever_owned_planets.insert(planet.clone());
        }

        let Some(owner) = owner else {
            return Ok(previous_owner);
        };

        let legendary =
            planet.info().is_legendary || attachments.iter().any(|a| a.info().set_legendary);
        let moves_shard = (previous_owner.as_ref())
            .and_then(|previous| self.players.get_mut(previous))
            .is_some_and(|previous| {
                (legendary || planet.is_in_home_system_of(previous.home_system_faction()))
                    && previous.relics.remove(&Relic::ShardOfTheThrone)
            });

        let player = self
            .players
            .get_mut(&owner)
            .context("Player does not exist")?;
        if moves_shard {
            player.relics.insert(Relic::ShardOfTheThrone);
            self.score.shard_of_the_throne = Some(owner.clone());
        }
        if planet.is_mecatol_rex() && self.score.custodians.is_none() {
            self.score.custodians = Some(owner.clone());
        }
        player.planets.insert(planet, attachments);

        Ok(previous_owner)
    }

    /// Whether `planet` is or has been owned by a player, which also counts planets owned from the
    /// start of the game.
    pub fn has_been_owned(&self, planet: &Planet) -> bool {
        self.map_data.ever_owned_planets.contains(planet)
            || self
                .players
                .values()
                .any(|p| p.planets.contains_key(planet))
    }

    /// Destroy or purge `planet`: it's taken from its owner along with its attachments and added
    /// to [MapData::destroyed_planets].
    pub fn destroy_planet(&mut self, planet: Planet) {
        for player in self.players.values_mut() {
            player.remove_planet(&planet);
        }
        self.galactic_events.monuments.remove(&planet);
        self.map_data.destroyed_planets.push(planet);
    }

    /// Whether the Firmament has flipped to The Obsidian.
    pub fn obsidian_in_play(&self) -> bool {
        self.players
            .values()
            .any(|p| p.firmament_side() == Some(FirmamentSide::Obsidian))
    }

    /// Returns true if an entropic scar can be in play: Thunder's Edge is enabled and the map, if
    /// it is known, has an entropic scar.
    pub fn entropic_scar_in_play(&self) -> bool {
        if !self
            .game_settings
            .expansions
            .is_enabled(&Expansion::ThundersEdge)
        {
            return false;
        }

        self.map_data
            .milty_information
            .as_ref()
            .is_none_or(|milty| milty.hex_map.has_anomaly(&AnomalyType::EntropicScar))
    }
}
