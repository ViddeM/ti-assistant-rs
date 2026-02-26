/// Public objectives.
pub mod public;
/// Secret objectives.
pub mod secret;

use std::{cmp::Ordering, fmt::Display, str::FromStr};

use crate::common::expansions::Expansion;
use serde::{Deserialize, Serialize};

use self::{public::PublicObjective, secret::SecretObjective};

use super::phase::Phase;

/// An objective in the game.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(untagged)] // crimes
pub enum Objective {
    /// A public objective.
    Public(PublicObjective),
    /// A secret objective.
    Secret(SecretObjective),
}

impl Display for Objective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Objective::Public(public_objective) => public_objective.fmt(f),
            Objective::Secret(secret_objective) => secret_objective.fmt(f),
        }
    }
}

// NOTE: Assumes that there are no overlapping names between public & secret objectives!
impl FromStr for Objective {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(obj) = PublicObjective::from_str(s) {
            return Ok(Objective::Public(obj));
        }

        if let Ok(obj) = SecretObjective::from_str(s) {
            return Ok(Objective::Secret(obj));
        }

        anyhow::bail!("Unknown objetive {s}");
    }
}

/// All relevant information about an objective.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Display for ObjectiveKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ObjectiveKind::StageI => "Objective, Stage I",
                ObjectiveKind::StageII => "Objective, Stage II",
                ObjectiveKind::Secret { .. } => "Secret Objective",
            }
        )
    }
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

impl PartialOrd for Objective {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Objective {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Objective::Public(a), Objective::Public(b)) => a.cmp(b),
            (Objective::Public(_), Objective::Secret(_)) => Ordering::Greater,
            (Objective::Secret(_), Objective::Public(_)) => Ordering::Less,
            (Objective::Secret(a), Objective::Secret(b)) => a.cmp(b),
        }
    }
}
