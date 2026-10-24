//! Helpers for driving a [TestGame] through the phases of a round.
//!
//! The action phase helpers hand out small handles ([Turn], [Tactical], [Strategic]) that keep
//! track of whose turn it is, so a scenario reads like the game:
//!
//! ```ignore
//! g.turn("Alice").tactical().take(Planet::Centauri).commit().end_turn();
//! ```

use std::ops::{Deref, DerefMut};

use ti_helper_game_data::{
    actions::{
        event::{ActionCardAction, Event, FrontierCardAction, RelicAction},
        strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
    },
    common::{faction::Faction, player_id::PlayerId},
    components::{
        action_card::ActionCard,
        agenda::{Agenda, AgendaElect},
        frontier_card::FrontierCard,
        leaders::Leader,
        objectives::{Objective, public::PublicObjective, secret::SecretObjective},
        phase::Phase,
        planet::Planet,
        planet_attachment::PlanetAttachment,
        relic::Relic,
        strategy_card::StrategyCard,
    },
    state::{expedition::TEExpeditionSlot, game_state::ActionPhaseProgress},
};

use crate::game::{TestGame, p};

impl TestGame {
    /// Pick strategy cards in order, then complete the strategy phase.
    ///
    /// Each pick is checked to be made by the player whose turn it is.
    #[track_caller]
    pub fn strategy_phase(&mut self, picks: &[(&str, StrategyCard)]) -> &mut Self {
        for &(player, card) in picks {
            self.pick_strategy_card(player, card);
        }
        self.apply(Event::CompleteStrategyPhase)
    }

    /// Pick a single strategy card, checking that it is the players turn to pick.
    #[track_caller]
    pub fn pick_strategy_card(&mut self, player: &str, card: StrategyCard) -> &mut Self {
        assert_eq!(self.phase(), Phase::Strategy, "not in the strategy phase");
        assert_eq!(
            &*self.current_player(),
            player,
            "{player} picked {card:?} out of turn"
        );
        self.apply(Event::TakeStrategyCard {
            player: p(player),
            card,
        })
    }

    /// Start the turn of `player`, asserting that it is their turn.
    #[track_caller]
    pub fn turn(&mut self, player: &str) -> Turn<'_> {
        assert!(
            matches!(self.phase(), Phase::Action | Phase::EndActionTurn),
            "expected the action phase\n{}",
            self.describe()
        );
        assert_eq!(
            &*self.current_player(),
            player,
            "expected it to be {player}'s turn\n{}",
            self.describe()
        );
        Turn {
            player: p(player),
            game: self,
        }
    }

    /// Start the turn of whoever is the current player.
    #[track_caller]
    pub fn current(&mut self) -> Turn<'_> {
        let player = self.current_player();
        self.turn(&player)
    }

    /// Let every remaining player pass, ending the action phase.
    #[track_caller]
    pub fn pass_all(&mut self) -> &mut Self {
        while self.phase() == Phase::Action {
            self.current().pass();
        }
        self
    }

    /// Finish the action phase: on their turn, each player uses their remaining strategy cards
    /// (in initiative order, choosing no effects) and then passes.
    ///
    /// "No effects" means no techs for Technology, keeping the speaker for Politics and no
    /// objective for Imperial.
    #[track_caller]
    pub fn play_out_action_phase(&mut self) -> &mut Self {
        while self.phase() == Phase::Action {
            let player = self.current_player();
            let state = self.state();
            let card = state
                .strategy_card_holders
                .iter()
                .filter(|(card, holder)| {
                    **holder == player && !state.spent_strategy_cards.contains(*card)
                })
                .map(|(card, _)| *card)
                .min_by_key(|card| card.card_number());
            match card {
                Some(card) => {
                    self.turn(&player).play_card(card);
                }
                None => {
                    self.turn(&player).pass();
                }
            }
        }
        self
    }

    /// Start scoring the status phase, see [StatusPhase].
    #[track_caller]
    pub fn status_phase(&mut self) -> StatusPhase<'_> {
        assert_eq!(
            self.phase(),
            Phase::Status,
            "expected the status phase\n{}",
            self.describe()
        );
        StatusPhase { game: self }
    }

    /// Reveal an agenda and start voting on it, see [AgendaVote].
    #[track_caller]
    pub fn agenda(&mut self, agenda: Agenda) -> AgendaVote<'_> {
        self.apply(Event::RevealAgenda { agenda });
        AgendaVote { game: self }
    }

    /// Complete the agenda phase, entering the strategy phase of the next round.
    #[track_caller]
    pub fn complete_agenda_phase(&mut self) -> &mut Self {
        self.apply(Event::CompleteAgendaPhase)
    }
}

/// Implements `Deref` to the game plus chaining versions of `apply` and `expect_rejected`.
macro_rules! handle {
    ($handle:ident) => {
        impl<'g> Deref for $handle<'g> {
            type Target = TestGame;
            fn deref(&self) -> &TestGame {
                self.game
            }
        }

        impl<'g> DerefMut for $handle<'g> {
            fn deref_mut(&mut self) -> &mut TestGame {
                self.game
            }
        }

        impl<'g> $handle<'g> {
            /// Apply a raw event in the middle of the chain.
            #[track_caller]
            pub fn apply(self, event: Event) -> Self {
                self.game.apply(event);
                self
            }

            /// Apply an event that must be rejected, see [TestGame::expect_rejected].
            #[track_caller]
            pub fn expect_rejected(self, event: Event, contains: &str) -> Self {
                self.game.expect_rejected(event, contains);
                self
            }
        }
    };
}

/// A players turn in the action phase.
pub struct Turn<'g> {
    game: &'g mut TestGame,
    player: PlayerId,
}

handle!(Turn);

impl<'g> Turn<'g> {
    fn id(&self) -> PlayerId {
        self.player.clone()
    }

    /// Begin a tactical action.
    #[track_caller]
    pub fn tactical(self) -> Tactical<'g> {
        self.game
            .apply(Event::TacticalActionBegin { player: self.id() });
        Tactical { turn: self }
    }

    /// Begin playing a strategy card.
    #[track_caller]
    pub fn strategic(self, card: StrategyCard) -> Strategic<'g> {
        self.game.apply(Event::StrategicActionBegin {
            player: self.id(),
            card,
        });
        Strategic { turn: self }
    }

    /// Use a strategy card choosing no effects (see [TestGame::play_out_action_phase]), without
    /// any secondaries, and end the turn.
    #[track_caller]
    pub fn play_card(self, card: StrategyCard) -> &'g mut TestGame {
        let strategic = self.strategic(card);
        let Some(ActionPhaseProgress::Strategic(progress)) = &strategic.state().action_progress
        else {
            panic!("no strategic action in progress");
        };
        let primary = match (card, progress.primary.is_some()) {
            (_, true) => None,
            (StrategyCard::Technology, false) => Some(StrategicPrimaryAction::Technology {
                first_tech: None,
                second_tech: None,
            }),
            (StrategyCard::Politics, false) => Some(StrategicPrimaryAction::Politics {
                new_speaker: strategic.state().speaker.clone().expect("no speaker"),
            }),
            (StrategyCard::Imperial, false) => Some(StrategicPrimaryAction::Imperial {
                score_objective: None,
            }),
            _ => None,
        };
        let strategic = match primary {
            Some(action) => strategic.primary(action),
            None => strategic,
        };
        strategic.commit().end_turn()
    }

    /// Play an action card as an action.
    #[track_caller]
    pub fn action_card(self, card: ActionCard, data: Option<ActionCardAction>) -> Self {
        let player = self.id();
        self.game.apply(Event::ActionCardActionBegin {
            player: player.clone(),
            card,
        });
        self.game
            .apply(Event::ActionCardActionCommit { player, data });
        self
    }

    /// Play a leader as an action.
    #[track_caller]
    pub fn leader(self, leader: Leader) -> Self {
        self.game.apply(Event::LeaderActionBegin {
            player: self.id(),
            leader,
        });
        self
    }

    /// Play a frontier card as an action.
    #[track_caller]
    pub fn frontier(self, card: FrontierCard, data: Option<FrontierCardAction>) -> Self {
        let player = self.id();
        self.game.apply(Event::FrontierCardActionBegin {
            player: player.clone(),
            card,
        });
        self.game
            .apply(Event::FrontierCardActionCommit { player, data });
        self
    }

    /// Gain a relic as an action.
    #[track_caller]
    pub fn gain_relic(self, relic: Relic) -> Self {
        self.game.apply(Event::GainRelicAction {
            player: self.id(),
            relic,
        });
        self
    }

    /// Play a relic as an action.
    #[track_caller]
    pub fn relic(self, relic: Relic, data: Option<RelicAction>) -> Self {
        let player = self.id();
        self.game.apply(Event::RelicActionBegin {
            player: player.clone(),
            relic,
        });
        self.game.apply(Event::RelicActionCommit { player, data });
        self
    }

    /// Take another action after finishing one.
    #[track_caller]
    pub fn take_another_turn(self) -> Self {
        self.game
            .apply(Event::TakeAnotherTurn { player: self.id() });
        self
    }

    /// End the turn after finishing an action.
    #[track_caller]
    pub fn end_turn(self) -> &'g mut TestGame {
        self.game.apply(Event::EndTurn { player: self.id() })
    }

    /// Pass for the rest of the action phase, declining to commit to the Thunder's Edge expedition.
    #[track_caller]
    pub fn pass(self) -> &'g mut TestGame {
        let player = self.id();
        self.game.apply(Event::PassAction {
            player: player.clone(),
        });
        if self.game.phase() == Phase::EndActionTurn {
            self.game.apply(Event::EndTurn { player });
        }
        self.game
    }

    /// Pass for the rest of the action phase, staying in the end of turn step.
    #[track_caller]
    pub fn pass_and_stay(self) -> Self {
        self.game.apply(Event::PassAction { player: self.id() });
        self
    }

    /// Commit to the Thunder's Edge expedition at the end of the turn.
    #[track_caller]
    pub fn commit_to_expedition(self, slot: TEExpeditionSlot) -> Self {
        self.game.apply(Event::CommitToThundersEdgeExpedition {
            player: self.id(),
            slot,
        });
        self
    }
}

/// A tactical action in progress.
pub struct Tactical<'g> {
    turn: Turn<'g>,
}

impl<'g> Deref for Tactical<'g> {
    type Target = TestGame;
    fn deref(&self) -> &TestGame {
        self.turn.game
    }
}

impl<'g> Tactical<'g> {
    /// Take control of a planet.
    #[track_caller]
    pub fn take(self, planet: Planet) -> Self {
        let player = self.turn.id();
        self.turn
            .game
            .apply(Event::TacticalActionTakePlanet { player, planet });
        self
    }

    /// Explore a planet taken this action, attaching the attachment to it.
    #[track_caller]
    pub fn attach(self, planet: Planet, attachment: PlanetAttachment) -> Self {
        let player = self.turn.id();
        self.turn
            .game
            .apply(Event::TacticalActionAttachPlanetAttachment {
                player,
                planet,
                attachment,
            });
        self
    }

    /// Apply an event that must be rejected, see [TestGame::expect_rejected].
    #[track_caller]
    pub fn expect_rejected(self, event: Event, contains: &str) -> Self {
        self.turn.game.expect_rejected(event, contains);
        self
    }

    /// Finish the tactical action.
    #[track_caller]
    pub fn commit(self) -> Turn<'g> {
        let player = self.turn.id();
        self.turn.game.apply(Event::TacticalActionCommit { player });
        self.turn
    }
}

/// A strategic action in progress.
pub struct Strategic<'g> {
    turn: Turn<'g>,
}

impl<'g> Deref for Strategic<'g> {
    type Target = TestGame;
    fn deref(&self) -> &TestGame {
        self.turn.game
    }
}

impl<'g> Strategic<'g> {
    /// Perform the primary action of the card.
    #[track_caller]
    pub fn primary(self, action: StrategicPrimaryAction) -> Self {
        let player = self.turn.id();
        self.turn
            .game
            .apply(Event::StrategicActionPrimary { player, action });
        self
    }

    /// Perform (or skip) the secondary action for another player.
    #[track_caller]
    pub fn secondary(self, player: &str, action: StrategicSecondaryAction) -> Self {
        self.turn.game.apply(Event::StrategicActionSecondary {
            player: p(player),
            action,
        });
        self
    }

    /// Skip the secondary for every other player that hasn't made a decision yet.
    #[track_caller]
    pub fn skip_rest(self) -> Self {
        let state = self.turn.game.state();
        let Some(ActionPhaseProgress::Strategic(progress)) = &state.action_progress else {
            panic!("no strategic action in progress");
        };
        let remaining: Vec<PlayerId> = state
            .turn_order
            .iter()
            .filter(|player| **player != self.turn.player)
            .filter(|player| !progress.other_players.contains_key(*player))
            .cloned()
            .collect();
        for player in remaining {
            self.turn.game.apply(Event::StrategicActionSecondary {
                player,
                action: StrategicSecondaryAction::Skip,
            });
        }
        self
    }

    /// Apply an event that must be rejected, see [TestGame::expect_rejected].
    #[track_caller]
    pub fn expect_rejected(self, event: Event, contains: &str) -> Self {
        self.turn.game.expect_rejected(event, contains);
        self
    }

    /// Finish the strategic action.
    #[track_caller]
    pub fn commit(self) -> Turn<'g> {
        self.turn.game.apply(Event::StrategicActionCommit);
        self.turn
    }
}

/// Scoring and revealing objectives in the status phase.
///
/// Scoring decisions are applied immediately. [StatusPhase::reveal] fills in "nothing scored"
/// for every player that hasn't made a decision before revealing the objective.
pub struct StatusPhase<'g> {
    game: &'g mut TestGame,
}

handle!(StatusPhase);

impl<'g> StatusPhase<'g> {
    /// Score a public objective for the player.
    #[track_caller]
    pub fn public(self, player: &str, objective: impl Into<Objective>) -> Self {
        self.game.apply(Event::ScorePublicObjective {
            player: p(player),
            objective: Some(objective.into()),
        });
        self
    }

    /// Score a secret objective for the player.
    #[track_caller]
    pub fn secret(self, player: &str, objective: SecretObjective) -> Self {
        self.game.apply(Event::ScoreSecretObjective {
            player: p(player),
            objective: Some(objective),
        });
        self
    }

    /// Record "nothing scored" for everyone without a decision, then reveal the objective.
    #[track_caller]
    pub fn reveal(self, objective: PublicObjective) -> Self {
        let state = self.game.state();
        let status = state
            .status_phase_state
            .as_ref()
            .expect("no status phase state");
        let undecided_public: Vec<PlayerId> = state
            .turn_order
            .iter()
            .filter(|player| !status.scored_public_objectives.contains_key(*player))
            .cloned()
            .collect();
        let undecided_secret: Vec<PlayerId> = state
            .turn_order
            .iter()
            .filter(|player| !status.scored_secret_objectives.contains_key(*player))
            .cloned()
            .collect();

        for player in undecided_public {
            self.game.apply(Event::ScorePublicObjective {
                player,
                objective: None,
            });
        }
        for player in undecided_secret {
            self.game.apply(Event::ScoreSecretObjective {
                player,
                objective: None,
            });
        }
        self.game.apply(Event::RevealPublicObjective { objective });
        self
    }

    /// Complete the status phase.
    #[track_caller]
    pub fn complete(self) -> &'g mut TestGame {
        self.game.apply(Event::CompleteStatusPhase)
    }
}

/// Voting on a revealed agenda.
pub struct AgendaVote<'g> {
    game: &'g mut TestGame,
}

handle!(AgendaVote);

impl<'g> AgendaVote<'g> {
    /// Cast votes for an outcome.
    #[track_caller]
    pub fn vote(self, player: &str, outcome: AgendaElect, votes: u16) -> Self {
        self.game.apply(Event::CastAgendaVote {
            player: p(player),
            outcome: Some(outcome),
            votes,
        });
        self
    }

    /// Abstain from voting.
    #[track_caller]
    pub fn abstain(self, player: &str) -> Self {
        self.game.apply(Event::CastAgendaVote {
            player: p(player),
            outcome: None,
            votes: 0,
        });
        self
    }

    /// Every player who hasn't voted yet (except Nekro, who can't) casts `votes` for `outcome`.
    #[track_caller]
    pub fn vote_all(self, outcome: AgendaElect, votes: u16) -> Self {
        let state = self.game.state();
        let voted = &state
            .agenda
            .as_ref()
            .and_then(|a| a.vote.as_ref())
            .expect("no agenda is revealed")
            .player_votes;
        let remaining: Vec<PlayerId> = state
            .turn_order
            .iter()
            .filter(|player| !voted.contains_key(*player))
            .filter(|player| state.players[*player].faction != Faction::NekroVirus)
            .cloned()
            .collect();
        for player in remaining {
            self.game.apply(Event::CastAgendaVote {
                player,
                outcome: Some(outcome.clone()),
                votes,
            });
        }
        self
    }

    /// Resolve the agenda with the provided outcome, `None` discards it.
    #[track_caller]
    pub fn resolve(self, outcome: Option<AgendaElect>) -> &'g mut TestGame {
        self.game.apply(Event::ResolveAgenda { outcome })
    }

    /// Resolve the agenda with the outcome that has the most votes.
    #[track_caller]
    pub fn resolve_expected(self) -> &'g mut TestGame {
        let outcome = self
            .game
            .state()
            .agenda
            .as_ref()
            .and_then(|a| a.vote.as_ref())
            .and_then(|v| v.expected_outcome.clone());
        assert!(outcome.is_some(), "the vote has no expected outcome");
        self.resolve(outcome)
    }

    /// Veto the agenda, discarding it without using up an agenda round.
    #[track_caller]
    pub fn veto(self) -> &'g mut TestGame {
        self.game.apply(Event::VetoAgenda)
    }
}
