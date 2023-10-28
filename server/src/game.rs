use eyre::{bail, ensure, eyre};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{data::components::strategy_card::StrategyCard, phases::Phase, player::Player};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    players: Vec<Player>,
    current: GameState,
    history: Vec<Event>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            players,
            current: Default::default(),
            history: Default::default(),
        }
    }

    /// Apply an event and update the game state.
    ///
    /// If the event is not valid for the current state it is rejected.
    pub fn apply(&mut self, event: Event) {
        println!("{event:?}");
        if let Err(e) = self.current.apply(event.clone()) {
            println!("event not valid for current state");
            println!("{e}");
            return;
        }

        println!("{:#?}", self.current);
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
struct GameState {
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
    pub strategic_action: Option<StrategyCardProgress>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StrategyCardProgress {
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
        player: Player,
    },
    StartGame,

    /* -- STRATEGY PHASE EVENTS -- */
    TakeStrategyCard {
        player: PlayerId,
        card: StrategyCard,
    },
    CompleteStrategyPhase,

    /* -- ACTION PHASE EVENTS -- */
    TacticalAction {
        player: PlayerId,
    },

    StrategicActionBegin {
        player: PlayerId,
        card: StrategyCard,
    },

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
                self.players.insert(id, player);
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
            Event::TacticalAction { player } => {
                self.assert_phase(Phase::Action)?;
                self.assert_player_turn(&player)?;
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
                    self.strategic_action.is_none(),
                    "we are already performing a strategic action",
                );
                self.phase = Phase::StrategicAction;
                self.strategic_action = Some(StrategyCardProgress {
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

                let Some(strategic_action) = &mut self.strategic_action else {
                    bail!("no strategic action in progress");
                };
                strategic_action.other_players.insert(player, did_secondary);
            }
            Event::StrategicActionCommit => {
                self.assert_phase(Phase::StrategicAction)?;
                ensure!(
                    self.strategic_action.is_some(),
                    "not currently performing a strategic action",
                );
                self.phase = Phase::Action;
                self.strategic_action = None;
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
}

pub type GameError = eyre::Report;
