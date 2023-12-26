use serde::{Deserialize, Serialize};

use crate::data::components::agenda::{Agenda, AgendaElect};
use crate::data::components::objectives::public::PublicObjective;
use crate::data::components::objectives::{secret::SecretObjective, Objective};
use crate::data::components::{
    action_card::ActionCard, planet::Planet, strategy_card::StrategyCard, tech::Technology,
};

use super::{
    game_state::StrategicSecondaryProgress,
    player::{NewPlayer, PlayerId},
};

/// An event in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /* -- SETUP PHASE EVENTS -- */
    /// Add a new player to the setup game.
    AddPlayer {
        /// The new player that joined the game.
        player: NewPlayer,
    },
    /// Start the game.
    StartGame,

    /* -- STRATEGY PHASE EVENTS -- */
    /// A player takes a strategy card.
    TakeStrategyCard {
        /// Which player takes the card.
        player: PlayerId,
        /// The card taken.
        card: StrategyCard,
    },
    /// Finish the strategy phase.
    CompleteStrategyPhase,

    /* -- ACTION PHASE EVENTS -- */
    /// Begin a tactical action.
    TacticalActionBegin {
        /// What player is taking the action.
        player: PlayerId,
    },

    /// Take a planet during a tactical action.
    TacticalActionTakePlanet {
        /// Which player takes the planet.
        player: PlayerId,
        /// Which planet is taken.
        planet: Planet,
    },

    /// End a tactical action.
    TacticalActionCommit {
        /// The player that has taken the action.
        player: PlayerId,
    },

    /// Begin playing a strategy card.
    StrategicActionBegin {
        /// The player that plays the card.
        player: PlayerId,
        /// The card being played.
        card: StrategyCard,
    },

    /// Perform the primary part of a strategy card.
    #[serde(rename_all = "camelCase")]
    StrategicActionPrimary {
        /// What player perform the primary action.
        player: PlayerId,
        /// What action they perform.
        action: StrategicPrimaryAction,
    },

    /// Perform the secondary part of a strategy card for a specific player.
    #[serde(rename_all = "camelCase")]
    StrategicActionSecondary {
        /// The player that performs the action.
        player: PlayerId,
        /// What action is being taken.
        action: StrategicSecondaryAction,
    },

    /// Finish a strategic action.
    StrategicActionCommit,

    /// Begin playing an action card.
    ActionCardActionBegin {
        /// Which player is playing the card.
        player: PlayerId,
        /// Which card is being played.
        card: ActionCard,
    },

    /// Finish an action card action.
    ActionCardActionCommit {
        /// What player played the data.
        player: PlayerId,
        /// Additional information about what occurred when they played the card.
        data: Option<ActionCardInfo>,
    },

    /// A player passes their turn.
    PassAction {
        /// The player that passed.
        player: PlayerId,
    },

    /* -- STATUS PHASE EVENTS -- */
    /// Score a public objective.
    ScorePublicObjective {
        /// The player that scores the objective.
        player: PlayerId,
        /// Which objective is being scored.
        objective: Objective,
    },

    /// Score a secret objective.
    ScoreSecretObjective {
        /// The player that scores the objective.
        player: PlayerId,
        /// The objective being scored.
        objective: SecretObjective,
    },

    /// Reveal a new public objective.
    RevealPublicObjective {
        /// The objective that was revealed.
        objective: PublicObjective,
    },

    // TODO: Score objectives
    /// Complete the status phase.
    CompleteStatusPhase,

    /* -- ACTION PHASE EVENTS -- */
    /// Reveal a new agenda.
    RevealAgenda {
        /// The agenda being revealed.
        agenda: Agenda,
    },

    /// Resolve an agenda.
    ResolveAgenda {
        /// The outcome that came from the agenda.
        outcome: AgendaElect,
    },

    /// End the agenda phase.
    CompleteAgendaPhase,

    /* -- ANY PHASE EVENTS -- */
    /// Give `giver` players Support for the Throne to `receiver`.
    GiveSupportForTheThrone {
        /// The player that gives away their support for the throne card.
        giver: PlayerId,
        /// The player that received the support for the throne card.
        receiver: PlayerId,
    },

    /// Set the current custodians, usually set to the first player to take mecatol rex.
    SetCustodians {
        /// The new custodians, removes custodians from anyone that has it if None.
        player: Option<PlayerId>,
    },

    /// Increment the `extra_points` value for a player with the given value.
    AddExtraPoints {
        /// The player who shall receive the extra points.
        player: PlayerId,
        /// How many points the player should gain (or lose if negative).
        value: i8,
    },

    /// Increment the number of points received from the imperial strategy card for a player with the given value.
    AddImperial {
        /// The player that will receive the extra imperial points.
        player: PlayerId,
        /// How many imperial points the player should gain (or lose if negative).
        value: i8,
    },

    /// Unscore a revealed objective.
    UnscoreObjective {
        /// The player that will lose the objective.
        player: PlayerId,
        /// The objective that shall be 'un-scored'.
        objective: Objective,
    },

    /// Unscore a secret objective.
    UnscoreSecretObjective {
        /// The player that will lose the objective.
        player: PlayerId,
        /// The objective that shall be 'un-scored'.
        objective: SecretObjective,
    },

    /// Add the tech to the players list of techs.
    AddTechToPlayer {
        /// The player to receive the technology.
        player: PlayerId,
        /// The technology to be gained.
        tech: Technology,
    },

    /// Remove the tech from the players list of techs.
    RemoveTechFromPlayer {
        /// The player who will lose the technology.
        player: PlayerId,
        /// The player to be lost.
        tech: Technology,
    },

    /// Pause/unpause time-tracking.
    TrackTime {
        /// Weather time-tracking should be paused or not.
        paused: bool,
    },

    /// Give the planet to a specific player, removing it from the current player that owns it (if any).
    /// If `playerId` is None, instead removes the planet from any player that owns it without giving it to anyone else.
    SetPlanetOwner {
        /// The player who will gain control of the planet, if any.
        player: Option<PlayerId>,
        /// The planet to change hands.
        planet: Planet,
    },
}

/// Primary action taken during a strategy card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategicPrimaryAction {
    /// The primary action for the technology card.
    Technology {
        /// What tech shall be taken.
        tech: Technology,
        /// What extra tech shall be taken (if any).
        extra: Option<Technology>,
    },

    /// The primary action for the politics card.
    #[serde(rename_all = "camelCase")]
    Politics {
        /// Who the new speaker should be.
        new_speaker: PlayerId,
    },

    /// The primary action for the imperial card.
    #[serde(rename_all = "camelCase")]
    Imperial {
        /// The objective that should be scored, if any.
        score_objective: Option<Objective>,
    },
}

/// The actions taken for the secondary part of a strategy card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum StrategicSecondaryAction {
    Skip,
    Leadership,
    Diplomacy,
    Politics,
    Construction,
    Trade,
    Warfare,
    Technology {
        /// The tech that was taken.
        tech: Technology,
    },
    Imperial,
}

impl StrategicSecondaryAction {
    /// Weather the action is the provided strategy card or not.
    pub fn is_for_card(&self, card: StrategyCard) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, card) {
            (StrategicSecondaryAction::Skip, _) => true,
            (StrategicSecondaryAction::Leadership, StrategyCard::Leadership) => true,
            (StrategicSecondaryAction::Diplomacy, StrategyCard::Diplomacy) => true,
            (StrategicSecondaryAction::Politics, StrategyCard::Politics) => true,
            (StrategicSecondaryAction::Construction, StrategyCard::Construction) => true,
            (StrategicSecondaryAction::Trade, StrategyCard::Trade) => true,
            (StrategicSecondaryAction::Warfare, StrategyCard::Warfare) => true,
            (StrategicSecondaryAction::Technology { .. }, StrategyCard::Technology) => true,
            (StrategicSecondaryAction::Imperial, StrategyCard::Imperial) => true,
            _ => false,
        }
    }

    /// Weather the player skipped taking their secondary action.
    pub fn skipped(&self) -> bool {
        matches!(self, StrategicSecondaryAction::Skip)
    }
}

impl From<StrategicSecondaryAction> for StrategicSecondaryProgress {
    fn from(value: StrategicSecondaryAction) -> Self {
        match value {
            StrategicSecondaryAction::Skip => Self::Skipped,
            StrategicSecondaryAction::Leadership => Self::Leadership,
            StrategicSecondaryAction::Diplomacy => Self::Diplomacy,
            StrategicSecondaryAction::Politics => Self::Politics,
            StrategicSecondaryAction::Construction => Self::Construction,
            StrategicSecondaryAction::Trade => Self::Trade,
            StrategicSecondaryAction::Warfare => Self::Warfare,
            StrategicSecondaryAction::Technology { tech } => Self::Technology { tech },
            StrategicSecondaryAction::Imperial => Self::Imperial,
        }
    }
}

/// The actions taken for specific action cards.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ActionCardInfo {
    FocusedResearch {
        /// The technology that was taken.
        tech: Technology,
    },
    #[serde(rename_all = "camelCase")]
    DivertFunding {
        /// What tech should be replaced.
        remove_tech: Technology,
        /// The tech to replace it with.
        take_tech: Technology,
    },
    Plagiarize {
        /// The tech to gain.
        tech: Technology,
    },
}

/// Returns weather the [ActionCardInfo] is for the provided [ActionCard].
pub fn action_matches_action_card(action: &Option<ActionCardInfo>, card: &ActionCard) -> bool {
    match card {
        ActionCard::FocusedResearch => {
            matches!(action, Some(ActionCardInfo::FocusedResearch { .. }))
        }
        ActionCard::DivertFunding => {
            matches!(action, Some(ActionCardInfo::DivertFunding { .. }))
        }
        ActionCard::Plagiarize => {
            matches!(action, Some(ActionCardInfo::Plagiarize { .. }))
        }
        _ => action.is_none(),
    }
}