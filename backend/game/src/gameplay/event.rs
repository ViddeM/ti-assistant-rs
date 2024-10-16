use serde::{Deserialize, Serialize};

use crate::data::common::faction::Faction;
use crate::data::components::agenda::{Agenda, AgendaElect};
use crate::data::components::frontier_card::FrontierCard;
use crate::data::components::leaders::Leader;
use crate::data::components::objectives::public::PublicObjective;
use crate::data::components::objectives::{secret::SecretObjective, Objective};
use crate::data::components::planet_attachment::PlanetAttachment;
use crate::data::components::relic::Relic;
use crate::data::components::{
    action_card::ActionCard, planet::Planet, strategy_card::StrategyCard, tech::Technology,
};

use super::game_settings::GameSettings;
use super::{
    game_state::StrategicSecondaryProgress,
    player::{NewPlayer, PlayerId},
};

/// An event in the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /* -- CREATION PHASE EVENTS -- */
    /// Set the game settings.
    SetSettings {
        /// Settings for the game.
        settings: GameSettings,
    },

    /// Add a new player to the game.
    AddPlayer {
        /// The new player that joined the game.
        player: NewPlayer,
    },

    /// Creation phase is done.
    CreationDone,

    /* -- SETUP PHASE EVENTS -- */
    /// Faction specific setup for council keleres ability 'The Tribunii' (note that this does not include tech).
    SetupTheTribunii {
        /// The player who selects the system.
        player: PlayerId,
        /// The faction to replicate.
        faction: Faction,
    },

    /// Faction-specific technology selection for a player.
    SetupPlayerTechs {
        /// The player who selects the technologies.
        player: PlayerId,
        /// The selected technologies.
        technologies: Vec<Technology>,
    },

    /// Set the starting speaker.
    SetupSpeaker {
        /// The player who shall be the new speaker.
        player: PlayerId,
    },

    /// Reveal the two starting objectives.
    #[serde(rename_all = "camelCase")]
    RevealInitialObjectives {
        /// The first objective.
        first_objective: Objective,
        /// The second objective.
        second_objective: Objective,
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

    /// Attach an attachment to a newly taken planet.
    TacticalActionAttachPlanetAttachment {
        /// Which player takes the attachment.
        player: PlayerId,
        /// The planet that will get the attachment.
        planet: Planet,
        /// The attachment to be attached.
        attachment: PlanetAttachment,
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
        data: Option<ActionCardAction>,
    },

    /// Bergin playing a leader.
    LeaderActionBegin {
        /// The player that is taking the action.
        player: PlayerId,

        /// The leader that is being played.
        leader: Leader,
    },

    /// Finish a leader action.
    LeaderActionCommit {
        /// The player that is taking the action.
        player: PlayerId,

        /// The leader that is being played.
        leader: Leader,
    },

    /// Begin playing a frontier card.
    FrontierCardActionBegin {
        /// The player who plays the card.
        player: PlayerId,
        /// The card that is being played.
        card: FrontierCard,
    },

    /// Finish a frontier card action.
    FrontierCardActionCommit {
        /// The player who takes the action.
        player: PlayerId,
        /// Additional information required to carry out the action.
        data: Option<FrontierCardAction>,
    },

    /// Gain a relic
    GainRelicAction {
        /// The player who gains the relic.
        player: PlayerId,
        /// The relic to be gained.
        relic: Relic,
    },

    /// Begin playing a relic action.
    RelicActionBegin {
        /// The player who plays the relic.
        player: PlayerId,
        /// The relic being played.
        relic: Relic,
    },

    /// Finish a relic action.
    RelicActionCommit {
        /// The player who took the action.
        player: PlayerId,
        /// Additional information required to carry out the action.
        data: Option<RelicAction>,
    },

    /// End turn
    EndTurn {
        /// The player that ends their turn.
        player: PlayerId,
    },

    /// Take another turn
    TakeAnotherTurn {
        /// The player that will take another turn.
        player: PlayerId,
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
        objective: Option<Objective>,
    },

    /// Score a secret objective.
    ScoreSecretObjective {
        /// The player that scores the objective.
        player: PlayerId,
        /// The objective being scored.
        objective: Option<SecretObjective>,
    },

    /// Reveal a new public objective.
    RevealPublicObjective {
        /// The objective that was revealed.
        objective: PublicObjective,
    },

    /// Complete the status phase.
    CompleteStatusPhase,

    /// A player plays the Crown of Emphidia card.
    PlayCrownOfEmphidia {
        /// The player who plays the Crown of Emphidia card.
        player: PlayerId,
    },

    /// A player plays the Maw of Worlds relic.
    PlayMawOfWorlds {
        /// The player who plays Maw of Worlds.
        player: PlayerId,
        /// The tech they decided to take.
        tech: Technology,
    },

    /// Complete the Maw of Worlds phase, moving on to the Agenda phase.
    CompleteRelicsPhase,

    /* -- AGENDA PHASE EVENTS -- */
    /// Reveal a new agenda.
    RevealAgenda {
        /// The agenda being revealed.
        agenda: Agenda,
    },

    /// Discard the current agenda without voting.
    ///
    /// This does not increment agenda round number, so another agenda must be revealed using
    /// `RevealAgenda`.
    VetoAgenda,

    /// Cast votes for an outcome of the current agenda.
    CastAgendaVote {
        /// The player that is casting the vote.
        player: PlayerId,

        /// The outcome the player is voting for. `None` means the player is abstaining.
        outcome: Option<AgendaElect>,

        /// Number of votes cast, should be omitted in case player abstains.
        #[serde(default)]
        votes: u16,
    },

    /// Resolve agenda with the selected outcome.
    ResolveAgenda {
        /// The outcome to resolve. `None` means the agenda is discarded without an outcome.
        outcome: Option<AgendaElect>,
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

    /// Score a public objective outside of the status phase.
    ScoreExtraPublicObjective {
        /// The player that scores the objective.
        player: PlayerId,
        /// The objective that is scored
        objective: Objective,
    },

    /// Score a secret objective outside of the status phase.
    ScoreExtraSecretObjective {
        /// The player that scores the secret objective.
        player: PlayerId,
        /// The objective that is scored.
        objective: SecretObjective,
    },

    /// Reveal a new public objective outside of the status phase.
    RevealExtraPublicObjective {
        /// The objective that was revealed.
        objective: PublicObjective,
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

    /// Give / remove shard of the throne to/from a player.
    SetShardForTheThroneOwner {
        /// The player that should receive it, or None if no one should have it.
        player: Option<PlayerId>,
    },

    /// Give / remove the Crown of Emphidia to/from a player.
    SetCrownOfEmphidiaOwner {
        /// The player that should receive it, or None if no one should have it.
        player: Option<PlayerId>,
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

    /// Remove a law from play.
    RepealLaw {
        /// Remove a law from play.
        law: Agenda,
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

    /// Add planet attachment to planet.
    AddPlanetAttachment {
        /// The player who owns the planet.
        player: PlayerId,
        /// The planet that should receive the attachment.
        planet: Planet,
        /// The attachment that is added.
        attachment: PlanetAttachment,
    },

    /// Remove a planet attachment from a planet.
    RemovePlanetAttachment {
        /// The player who owns the planet.
        player: PlayerId,
        /// The planet that should lose the attachment.
        planet: Planet,
        /// The attachment that is to be removed.
        attachment: PlanetAttachment,
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
pub enum ActionCardAction {
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

/// Returns weather the [ActionCardAction] is for the provided [ActionCard].
pub fn action_matches_action_card(action: &Option<ActionCardAction>, card: &ActionCard) -> bool {
    match card {
        ActionCard::FocusedResearch => {
            matches!(action, Some(ActionCardAction::FocusedResearch { .. }))
        }
        ActionCard::DivertFunding => {
            matches!(action, Some(ActionCardAction::DivertFunding { .. }))
        }
        ActionCard::Plagiarize => {
            matches!(action, Some(ActionCardAction::Plagiarize { .. }))
        }
        _ => action.is_none(),
    }
}

/// The actions taken for specific frontier cards.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum FrontierCardAction {
    EnigmaticDevice {
        /// The tech to gain.
        tech: Technology,
    },
}

/// Returns weather the [FrontierCardAction] is for the provided [FrontierCard].
pub fn action_matches_frontier_card(
    action: &Option<FrontierCardAction>,
    card: &FrontierCard,
) -> bool {
    match card {
        FrontierCard::EnigmaticDevice => {
            matches!(action, Some(FrontierCardAction::EnigmaticDevice { .. }))
        }
        _ => action.is_none(),
    }
}

/// The actions taken for specific frontier cards.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum RelicAction {
    StellarConverter { planet: Planet },
    NanoForge { planet: Planet },
}

/// Returns weather the [RelicAction] is for the provided [Relic].
pub fn action_matches_relic(action: &Option<RelicAction>, relic: &Relic) -> bool {
    match relic {
        Relic::StellarConverter => matches!(action, Some(RelicAction::StellarConverter { .. })),
        Relic::NanoForge => matches!(action, Some(RelicAction::NanoForge { .. })),
        _ => action.is_none(),
    }
}
