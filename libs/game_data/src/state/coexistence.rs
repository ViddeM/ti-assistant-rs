use serde::{Deserialize, Serialize};

use crate::{common::player_id::PlayerId, components::planet::Planet};

/// A player's units coexisting on a planet they don't control (Thunder's Edge).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Coexistence {
    /// The planet the units coexist on.
    pub planet: Planet,
    /// The player whose units coexist on the planet.
    pub player: PlayerId,
}
