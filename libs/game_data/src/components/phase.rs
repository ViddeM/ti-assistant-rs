use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// A phase of the game (including some that exist only for technical reasons).
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    /// Game creation, selecting players & game rules
    #[default]
    Creation,

    /// Game setup, perform faction specific setup.
    Setup,

    /// After setup and before the first strategy phase, when an effect resolves after setup
    /// (Rapid Mobilization).
    PostSetup,

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

    /// Before the status phase when an entropic scar is in play: players with ships in an
    /// entropic scar may gain a faction technology.
    EntropicScar,

    /// The Status phase.
    Status,

    /// Optional phase to allow a player to play the Maw of Worlds and Crown of Emphidia cards.
    Relics,

    /// The Agenda phase.
    Agenda,
}

impl Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Phase::Creation => "Creation",
                Phase::Setup => "Setup",
                Phase::PostSetup => "Post-Setup",
                Phase::Strategy => "Strategy",
                Phase::Action => "Action",
                Phase::StrategicAction => "Strategic Action",
                Phase::TacticalAction => "Tactical Action",
                Phase::ActionCardAction => "Action Card Action",
                Phase::LeaderAction => "Leader Action",
                Phase::FrontierCardAction => "Frontier Card Action",
                Phase::RelicAction => "Relic Action",
                Phase::EndActionTurn => "End Action Turn",
                Phase::EntropicScar => "Entropic Scar",
                Phase::Status => "Status",
                Phase::Relics => "Relics",
                Phase::Agenda => "Agenda",
            }
        )
    }
}
