use serde::{Deserialize, Serialize};

/// A phase of the game (including some that exist only for technical reasons).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Game creation, selecting players & game rules
    #[default]
    Creation,

    /// Game setup, perform faction specific setup.
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
