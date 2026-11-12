use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{
    common::player_id::PlayerId,
    components::{galactic_event::GalacticEvent, planet::Planet, tech::Technology},
};

/// The galactic events in play and the state they need.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GalacticEventsState {
    /// The galactic events in play.
    pub in_play: BTreeSet<GalacticEvent>,

    /// The players who still have to resolve Rapid Mobilization in the post-setup phase.
    pub rapid_mobilization: BTreeSet<PlayerId>,

    /// What each player picked when resolving Rapid Mobilization, for display purposes.
    pub rapid_mobilization_resolved: BTreeMap<PlayerId, RapidMobilizationResolution>,

    /// The players who have discarded their Stellar Atomics token. It can't be derived, since the
    /// event history isn't in [GameState](super::game_state::GameState).
    pub stellar_atomics_used: BTreeSet<PlayerId>,

    /// The planets with a monument ('Monuments to the Ages'), and the number of status phases
    /// that had started when it had no commodities, see
    /// [GameState::monument_commodities](super::game_state::GameState::monument_commodities).
    /// That is when it was built, unless its commodities have been set; it is negative if they
    /// have been set above the number of status phases so far.
    pub monuments: BTreeMap<Planet, i64>,
}

/// The planets and technologies a player gained from Rapid Mobilization.
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RapidMobilizationResolution {
    pub planets: Vec<Planet>,
    pub technologies: Vec<Technology>,
}
