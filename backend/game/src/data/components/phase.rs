use serde::{Deserialize, Serialize};

/// A phase of the game (including some that exist only for technical reasons).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
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

    /// Performing a leader action
    LeaderAction,

    /// Performing a frontier card action
    FrontierCardAction,

    /// Performing a relic action
    RelicAction,

    /// Player has finished one action turn, check if they're gonna take another one.
    EndActionTurn,

    /// The Status phase.
    Status,

    /// Optional phase to allow a player to play the Maw of Worlds and Crown of Emphidia cards.
    Relics,

    /// The Agenda phase.
    Agenda,
}
