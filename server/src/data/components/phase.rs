use serde::{Deserialize, Serialize};

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

    /// Performing a tactical action
    TacticalAction,

    /// Performing an action card action
    ActionCardAction,

    /// The Status phase.
    Status,

    /// The Agenda phase.
    Agenda,
}
