use serde::{Deserialize, Serialize};

pub mod action;
pub mod agenda;
pub mod status;
pub mod strategy;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Game setup, selecting player.
    #[default]
    Setup,

    /// The Strategy phase.
    Strategy,

    /// The Action phase.
    Action,

    /// Performing a strategic action
    StrategicAction,

    /// The Status phase.
    Status,

    /// The Agenda phase.
    Agenda,
}
