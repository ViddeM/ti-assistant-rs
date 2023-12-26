pub mod public;
pub mod secret;

use serde::{Deserialize, Serialize};

use self::{public::PublicObjective, secret::SecretObjective};

use super::phase::Phase;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(untagged)] // crimes
pub enum Objective {
    Public(PublicObjective),
    Secret(SecretObjective),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveInfo {
    /// Name of the objective.
    pub name: String,

    /// Requirements to achieve the objective.
    pub condition: String,

    //// Is this a stage I, II, or secret objective?
    pub kind: ObjectiveKind,

    //// How many points are given by this objective
    pub points: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveKind {
    StageI,
    StageII,
    #[serde(rename_all = "camelCase")]
    Secret {
        phase: Phase,
    },
}

impl Objective {
    pub fn get_objective_info(&self) -> ObjectiveInfo {
        match self {
            Objective::Public(o) => o.get_objective_info(),
            Objective::Secret(o) => o.get_objective_info(),
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
