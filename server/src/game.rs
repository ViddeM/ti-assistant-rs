use eyre::{bail, ensure, eyre};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    data::components::{planet::Planet, strategy_card::StrategyCard, system::System},
    phases::Phase,
    player::{NewPlayer, Player},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub players: Vec<Player>,
    pub current: GameState,
    pub history: Vec<Event>,
}

impl Game {
    /// Apply an event and update the game state.
    ///
    /// If the event is not valid for the current state it is rejected.
    pub fn apply(&mut self, event: Event) {
        log::debug!("{event:?}");
        if let Err(e) = self.current.apply(event.clone()) {
            log::warn!("event not valid for current state");
            log::warn!("{e}");
            return;
        }

        log::info!("{:#?}", self.current);
        self.history.push(event);
    }

    /// Undo the last event
    pub fn undo(&mut self) {
        self.history.pop();

        self.current = Default::default();
        for event in &self.history {
            self.current
                .apply(event.clone())
                .expect("wait... this worked before??");
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    /// The current phase of the game.
    pub phase: Phase,

    /// Which players are in the game.
    pub players: HashMap<PlayerId, Player>,

    /// The order that players are sitting around the table, starting with the speaker.
    pub table_order: Vec<PlayerId>,

    /// The current turn order, either based on table order or strategy cards (initiative).
    pub turn_order: Vec<PlayerId>,

    /// Which players hold which strategy cards.
    pub strategy_card_holders: HashMap<StrategyCard, PlayerId>,

    /// The current player, if any.
    pub current_player: Option<PlayerId>,

    /// Which strategy cards have been spent this action phase.
    pub spent_strategy_cards: HashSet<StrategyCard>,

    /// Which players have passed this phase.
    pub passed_players: HashSet<PlayerId>,

    /// Tracks progress of a strategy card action.
    pub action_progress: Option<ActionPhaseProgress>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionPhaseProgress {
    #[serde(rename_all = "camelCase")]
    StrategyCard {
        card: StrategyCard,
        other_players: HashMap<PlayerId, bool>,
    },
    #[serde(rename_all = "camelCase")]
    Tactical {
        activated_system: Option<System>, // TODO: Maybe in the future we should track systems for all tactical actions (Could use some cool interactive map :eyes:)
    },
}

impl ActionPhaseProgress {
    fn is_strategy_card(&self) -> bool {
        matches!(self, ActionPhaseProgress::StrategyCard { .. })
    }

    fn is_tactical(&self) -> bool {
        matches!(self, ActionPhaseProgress::Tactical { .. })
    }

    /// Returns true if the provided system is the currently activated system or there isn't an activated system.
    fn system_is_activated_or_none(&self, system: &System) -> bool {
        match self {
            ActionPhaseProgress::Tactical {
                activated_system: None,
            } => true,
            ActionPhaseProgress::Tactical {
                activated_system: Some(s),
            } => s == system,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrategyCardProgress {
    pub card: StrategyCard,
    pub other_players: HashMap<PlayerId, bool>,
}

const MIN_PLAYER_COUNT: usize = 3;
const MAX_PLAYER_COUNT: usize = 8;

// TODO: maybe make this be not a string...
pub type PlayerId = Arc<str>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    /* -- SETUP PHASE EVENTS -- */
    AddPlayer {
        player: NewPlayer,
    },
    StartGame,

    /* -- STRATEGY PHASE EVENTS -- */
    TakeStrategyCard {
        player: PlayerId,
        card: StrategyCard,
    },
    CompleteStrategyPhase,

    /* -- ACTION PHASE EVENTS -- */
    TacticalActionBegin {
        player: PlayerId,
    },

    TacticalActionTakePlanet {
        player: PlayerId,
        planet: Planet,
    },

    TacticalActionCommit {
        player: PlayerId,
    },

    StrategicActionBegin {
        player: PlayerId,
        card: StrategyCard,
    },

    #[serde(rename_all = "camelCase")]
    StrategicActionSecondary {
        player: PlayerId,
        did_secondary: bool,
    },

    StrategicActionCommit,

    ComponentAction {
        player: PlayerId,
        component: (), // TODO
    },

    PassAction {
        player: PlayerId,
    },

    /* -- STATUS PHASE EVENTS -- */
    // TODO: Score objectives & Reveal objectives
    CompleteStatusPhase,
    // TODO: Agenda phase
}

impl GameState {
    pub fn apply(&mut self, event: Event) -> Result<(), GameError> {
        match event {
            Event::AddPlayer { player } => {
                self.assert_phase(Phase::Setup)?;
                ensure!(
                    self.players.len() <= MAX_PLAYER_COUNT,
                    "can't have more than {MAX_PLAYER_COUNT} players"
                );
                let id: PlayerId = player.name.clone().into();
                self.table_order.push(id.clone());
                self.players.insert(id, player.into());
            }
            Event::StartGame => {
                self.assert_phase(Phase::Setup)?;
                ensure!(
                    self.players.len() >= MIN_PLAYER_COUNT,
                    "can't have less than {MIN_PLAYER_COUNT} players"
                );

                // set turn order for strategy phase
                self.turn_order = self.table_order.clone();
                self.current_player = self.turn_order.first().cloned();
                self.phase = Phase::Strategy;
            }
            Event::TakeStrategyCard { player, card } => {
                self.assert_phase(Phase::Strategy)?;
                ensure!(
                    !self.strategy_card_holders.contains_key(&card),
                    "strategy card can't be picked twice"
                );
                self.strategy_card_holders.insert(card, player);
                self.advance_turn()?;
            }
            Event::CompleteStrategyPhase => {
                let how_many_card_must_pick = match self.players.len() {
                    3 => 6,
                    4 => 8,
                    n => n,
                };
                if self.strategy_card_holders.len() != how_many_card_must_pick {
                    eyre::bail!("can't complete strategy phase, all players have not selected strategy cards");
                }
                self.calculate_action_turn_order()?;
                self.phase = Phase::Action;
            }
            Event::TacticalActionBegin { player } => {
                self.assert_phase(Phase::Action)?;
                self.assert_player_turn(&player)?;
                self.phase = Phase::TacticalAction;
                self.action_progress = Some(ActionPhaseProgress::Tactical {
                    activated_system: None,
                });
            }
            Event::TacticalActionTakePlanet { player, planet } => {
                self.assert_phase(Phase::TacticalAction)?;
                self.assert_player_turn(&player)?;
                ensure!(
                    self.action_progress.is_some(),
                    "Must have action progress to perform take planet action."
                );
                ensure!(
                    self.action_progress.as_ref().unwrap().is_tactical(),
                    "Invalid state: Expected to be in tactical action"
                );

                let planet_system = System::for_planet(&planet)?;
                ensure!(
                    self.action_progress
                        .as_ref()
                        .unwrap()
                        .system_is_activated_or_none(&planet_system),
                    "Trying to take planet in a system that isn't the currently activated system!"
                );

                self.action_progress = Some(ActionPhaseProgress::Tactical {
                    activated_system: Some(planet_system),
                });

                // In case someone else currently owns the planet, remove it from them.
                self.players
                    .values_mut()
                    .for_each(|p| p.remove_planet(&planet));

                let current_player = self.get_current_player()?;
                current_player.planets.insert(planet);
            }
            Event::TacticalActionCommit { player } => {
                self.assert_phase(Phase::Action)?;
                self.assert_player_turn(&player)?;
                self.action_progress = None;
                self.advance_turn()?;
            }
            Event::StrategicActionBegin { player, card } => {
                self.assert_phase(Phase::Action)?;
                self.assert_player_turn(&player)?;
                ensure!(
                    self.strategy_card_holders.get(&card) == Some(&player),
                    "current player must hold the strategy card",
                );
                ensure!(
                    !self.spent_strategy_cards.contains(&card),
                    "strategy card can't already be used",
                );
                ensure!(
                    self.action_progress.is_none(),
                    "we are already performing an action phase action",
                );
                self.phase = Phase::StrategicAction;
                self.action_progress = Some(ActionPhaseProgress::StrategyCard {
                    card,
                    other_players: Default::default(),
                });
                self.spent_strategy_cards.insert(card);
            }
            Event::StrategicActionSecondary {
                player,
                did_secondary,
            } => {
                self.assert_phase(Phase::StrategicAction)?;
                let current_player = self.current_player()?;
                ensure!(
                    &player != current_player,
                    "current player can't perform the secondary on a strategy card",
                );

                let Some(action_progress) = &mut self.action_progress else {
                    bail!("no strategic action in progress");
                };

                match action_progress {
                    ActionPhaseProgress::Tactical { .. } => {
                        bail!("cannot perform strategic actions during a tactical action")
                    }
                    ActionPhaseProgress::StrategyCard { other_players, .. } => {
                        other_players.insert(player, did_secondary)
                    }
                };
            }
            Event::StrategicActionCommit => {
                self.assert_phase(Phase::StrategicAction)?;
                ensure!(
                    self.action_progress.is_some(),
                    "not currently performing an action",
                );
                ensure!(
                    self.action_progress.as_ref().unwrap().is_strategy_card(),
                    "not currently performing a strategic action"
                );
                self.phase = Phase::Action;
                self.action_progress = None;
                self.advance_turn()?;
            }
            Event::ComponentAction {
                player,
                component: _,
            } => {
                self.assert_phase(Phase::Action)?;
                self.assert_player_turn(&player)?;
                self.advance_turn()?;
            }
            Event::PassAction { player } => {
                self.assert_player_turn(&player)?;

                let has_used_strategy_cards = self
                    .strategy_card_holders
                    .iter()
                    .filter(|(card, _)| !self.spent_strategy_cards.contains(card))
                    .all(|(_, holder)| *holder != player);

                ensure!(
                    has_used_strategy_cards,
                    "player must use all strategy cards before passing"
                );

                self.passed_players.insert(player);
                self.advance_turn()?;
            }
            Event::CompleteStatusPhase => {
                // TODO: Require objectives scored & revealed

                // TODO: Agenda phase

                // Reset state
                // TODO: Set current player to speaker
                self.phase = Phase::Strategy;
                self.turn_order = self.table_order.clone();
                self.strategy_card_holders = HashMap::new();
                self.passed_players = HashSet::new();
                self.spent_strategy_cards = HashSet::new();
            }
        }
        Ok(())
    }

    pub fn calculate_action_turn_order(&mut self) -> Result<(), GameError> {
        self.turn_order = self.table_order.clone();

        // sort players by the smallest number of the strategy cards they hold (initiative order)
        self.turn_order.sort_by_key(|player| {
            self.strategy_card_holders
                .iter()
                .filter(|&(_, holder)| holder == player)
                .map(|(card, _)| card.card_number())
                .min()
        });

        Ok(())
    }

    /// Advance to the next players turn, if all players have passed, advance to the next phase.
    pub fn advance_turn(&mut self) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        let next_player = self.next_player_after(current_player)?;

        // everyone has passed, i guess. what do?
        if next_player.is_none() {
            match self.phase {
                Phase::Action => {
                    self.calculate_action_turn_order()?;
                    self.passed_players.clear();
                    self.phase = Phase::Status;
                }
                _ => bail!("wtf"),
            }
        } else {
            self.current_player = next_player;
        }

        Ok(())
    }

    pub fn next_player_after(&self, after: &PlayerId) -> Result<Option<PlayerId>, GameError> {
        let next_player = self
            .turn_order
            .iter()
            .skip_while(|&player| player != after)
            .skip(1)
            .chain(self.turn_order.iter())
            .find(|&player| !self.passed_players.contains(player))
            .cloned();

        Ok(next_player)
    }

    pub fn current_player(&self) -> Result<&PlayerId, GameError> {
        self.current_player
            .as_ref()
            .ok_or_else(|| eyre!("no active player"))
    }

    pub fn assert_player_turn(&self, player: &PlayerId) -> Result<(), GameError> {
        let current_player = self.current_player()?;
        if current_player != player {
            bail!("wrong players turn, expected {player:?}, got {current_player:?}");
        }

        Ok(())
    }

    pub fn assert_phase(&self, phase: Phase) -> Result<(), GameError> {
        if self.phase != phase {
            bail!(
                "invalid game state, expected {phase:?}, was {:?}",
                self.phase
            );
        }
        Ok(())
    }

    pub fn get_current_player(&mut self) -> Result<&mut Player, GameError> {
        let current_player_id = match self.current_player.as_ref() {
            Some(p) => p,
            None => bail!("invalid game state, expected there to be a player"),
        };

        let current_player = match self.players.get_mut(current_player_id) {
            Some(p) => p,
            None => bail!("invalid game state, expected current player (id: {current_player_id:?}) to be in the players map")
        };

        Ok(current_player)
    }
}

pub type GameError = eyre::Report;
