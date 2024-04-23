/// Public objectives.
pub mod public;
/// Secret objectives.
pub mod secret;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::data::common::expansions::Expansion;

use self::{public::PublicObjective, secret::SecretObjective};

use super::phase::Phase;

/// An objective in the game.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, TS)]
#[serde(untagged)] // crimes
#[ts(export)]
pub enum Objective {
    /// A public objective.
    Public(PublicObjective),
    /// A secret objective.
    Secret(SecretObjective),
}

/// All relevant information about an objective.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ObjectiveInfo {
    /// Name of the objective.
    pub name: String,

    /// Requirements to achieve the objective.
    pub condition: String,

    /// Is this a stage I, II, or secret objective?
    pub kind: ObjectiveKind,

    /// How many points are given by this objective.
    pub points: i8,

    /// Which expansion this objective came from.
    pub expansion: Expansion,
}

/// What type of objective this is.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum ObjectiveKind {
    /// A stage I objective.
    StageI,
    /// A stage II objective.
    StageII,
    /// A secret objective.
    #[serde(rename_all = "camelCase")]
    Secret {
        /// The [Phase] that this secret can be played in.
        phase: Phase,
    },
}

impl Objective {
    /// Get the [ObjectiveInfo] for this objective.
    pub fn info(&self) -> ObjectiveInfo {
        match self {
            Objective::Public(o) => o.info(),
            Objective::Secret(o) => o.info(),
        }
    }
}

impl From<PublicObjective> for Objective {
    fn from(public: PublicObjective) -> Self {
        Objective::Public(public)
    }
}

impl From<SecretObjective> for Objective {
    fn from(secret: SecretObjective) -> Self {
        Objective::Secret(secret)
    }
}
