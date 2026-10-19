use std::{collections::BTreeSet, sync::Arc};

use chrono::{DateTime, TimeDelta, Utc};
use strum::IntoEnumIterator;
use ti_helper_game_data::{
    actions::event::Event,
    common::{
        color::Color,
        faction::Faction,
        game_settings::{Expansions, GameSettings},
        milty_data::MiltyData,
        player_id::PlayerId,
    },
    components::{
        agenda::{AgendaElect, ForOrAgainst},
        objectives::public::PublicObjective,
        phase::Phase,
        tech::{TechOrigin, Technology},
    },
    state::{
        game_state::GameState,
        player::{NewPlayer, Player},
    },
};
use ti_helper_game_logic::gameplay::game::Game;

use crate::{clock::Clock, timeline};

/// Shorthand for creating a [PlayerId].
pub fn p(name: &str) -> PlayerId {
    name.into()
}

/// Shorthand for electing a player in an agenda.
pub fn elect(name: &str) -> AgendaElect {
    AgendaElect::Player(p(name))
}

/// The "For" outcome of an agenda.
pub const FOR: AgendaElect = AgendaElect::ForOrAgainst(ForOrAgainst::For);
/// The "Against" outcome of an agenda.
pub const AGAINST: AgendaElect = AgendaElect::ForOrAgainst(ForOrAgainst::Against);

/// A game driven by a scenario.
///
/// Every event goes through the real [Game::apply_or_err] with timestamps from a fake [Clock].
/// [TestGame::apply] panics with context if an event is rejected, use
/// [TestGame::expect_rejected] for events that should fail.
pub struct TestGame {
    game: Game,
    clock: Clock,
    setup: SetupPlan,
}

/// The parts of the builder needed after the creation phase.
#[derive(Clone)]
struct SetupPlan {
    speaker: Option<String>,
    objectives: (PublicObjective, PublicObjective),
    setup_techs: Vec<(String, Vec<Technology>)>,
    tribunii: Option<Faction>,
}

impl TestGame {
    /// Start building a new game.
    pub fn builder() -> TestGameBuilder {
        TestGameBuilder::default()
    }

    /* -- Raw events -- */

    /// Apply an event, panicking with context if it is rejected.
    #[track_caller]
    pub fn apply(&mut self, event: Event) -> &mut Self {
        if let Err(e) = self.try_apply(event.clone()) {
            panic!(
                "event #{} was rejected: {event:?}\n  error: {e:#}\n{}",
                self.game.history.len(),
                self.describe(),
            );
        }
        self
    }

    /// Apply an event, returning the error if it is rejected.
    pub fn try_apply(&mut self, event: Event) -> anyhow::Result<()> {
        self.game.apply_or_err(event, self.clock.now())?;
        self.clock.tick();
        Ok(())
    }

    /// Apply an event that must be rejected, returning the error.
    ///
    /// Also checks that the rejected event left the game untouched.
    #[track_caller]
    pub fn apply_err(&mut self, event: Event) -> anyhow::Error {
        let before = self.state().clone();
        let history_len = self.game.history.len();
        match self.try_apply(event.clone()) {
            Ok(()) => panic!(
                "expected event to be rejected, but it was accepted: {event:?}\n{}",
                self.describe()
            ),
            Err(e) => {
                assert!(
                    *self.state() == before && self.game.history.len() == history_len,
                    "rejected event {event:?} modified the game"
                );
                e
            }
        }
    }

    /// Apply an event that must be rejected with an error containing `contains`.
    #[track_caller]
    pub fn expect_rejected(&mut self, event: Event, contains: &str) -> &mut Self {
        let error = format!("{:#}", self.apply_err(event.clone()));
        assert!(
            error.contains(contains),
            "event {event:?} was rejected with the wrong error\n  expected it to contain: {contains:?}\n  got: {error:?}"
        );
        self
    }

    /// Move the fake clock forward.
    #[track_caller]
    pub fn advance(&mut self, delta: TimeDelta) -> &mut Self {
        self.clock.advance(delta);
        self
    }

    /// Undo the last event. The clock is not rewound.
    pub fn undo(&mut self) -> &mut Self {
        self.game.undo();
        self
    }

    /* -- Game flow, see `phases.rs` for the per-phase helpers -- */

    /// Finish the creation phase.
    #[track_caller]
    pub fn finish_creation(&mut self) -> &mut Self {
        self.apply(Event::CreationDone)
    }

    /// Perform the faction-specific setup, choose the speaker and reveal the initial objectives.
    ///
    /// Defaults (all overridable through the builder):
    /// - Winnu: Neural Motivator.
    /// - Argent Flight: Neural Motivator and Plasma Scoring.
    /// - Council Keleres: the first of Mentak/Xxcha/Argent not in the game, and the first two
    ///   (by name) base techs owned by the other players.
    /// - Speaker: the first player in table order, unless imported from milty.
    #[track_caller]
    pub fn auto_setup(&mut self) -> &mut Self {
        let setup = self.setup.clone();
        let table_order = self.state().table_order.clone();
        let custom_techs = |player: &str| {
            setup
                .setup_techs
                .iter()
                .find(|(name, _)| name == player)
                .map(|(_, techs)| techs.clone())
        };

        let mut keleres = None;
        for player in &table_order {
            let technologies = match self.state().players[player].faction {
                Faction::Winnu => vec![Technology::NeuralMotivator],
                Faction::ArgentFlight => {
                    vec![Technology::NeuralMotivator, Technology::PlasmaScoring]
                }
                Faction::CouncilKeleres => {
                    keleres = Some(player.clone());
                    continue;
                }
                _ => continue,
            };
            self.apply(Event::SetupPlayerTechs {
                player: player.clone(),
                technologies: custom_techs(player).unwrap_or(technologies),
            });
        }

        // Keleres goes last since they pick their techs from the other players.
        if let Some(player) = keleres {
            let in_game: Vec<Faction> = self.state().players.values().map(|p| p.faction).collect();
            let faction = setup.tribunii.unwrap_or_else(|| {
                [
                    Faction::MentakCoalition,
                    Faction::XxchaKingdom,
                    Faction::ArgentFlight,
                ]
                .into_iter()
                .find(|f| !in_game.contains(f))
                .expect("no faction available for the Tribunii")
            });
            self.apply(Event::SetupTheTribunii {
                player: player.clone(),
                faction,
            });

            let technologies = custom_techs(&player).unwrap_or_else(|| {
                let base_techs: BTreeSet<Technology> = self
                    .state()
                    .players
                    .values()
                    .filter(|p| p.faction != Faction::CouncilKeleres)
                    .flat_map(|p| p.technologies.iter().cloned())
                    .filter(|t| t.info().origin == TechOrigin::Base)
                    .collect();
                assert!(
                    base_techs.len() >= 2,
                    "Keleres needs at least two base techs from the other players"
                );
                base_techs.into_iter().take(2).collect()
            });
            self.apply(Event::SetupPlayerTechs {
                player,
                technologies,
            });
        }

        match setup.speaker {
            Some(speaker) => {
                self.apply(Event::SetupSpeaker {
                    player: p(&speaker),
                });
            }
            None if self.state().map_data.milty_information.is_none() => {
                let player = table_order.first().expect("no players").clone();
                self.apply(Event::SetupSpeaker { player });
            }
            // The milty import already sets the speaker.
            None => {}
        }

        let (first, second) = setup.objectives;
        self.apply(Event::RevealInitialObjectives {
            first_objective: first.into(),
            second_objective: second.into(),
        })
    }

    /// Start the game, entering the strategy phase of round 1.
    #[track_caller]
    pub fn start_game(&mut self) -> &mut Self {
        self.apply(Event::StartGame)
    }

    /* -- Accessors -- */

    /// The current game state.
    pub fn state(&self) -> &GameState {
        &self.game.current
    }

    /// The current phase.
    pub fn phase(&self) -> Phase {
        self.state().phase
    }

    /// The current round.
    pub fn round(&self) -> u32 {
        self.state().round
    }

    /// The player whose turn it is.
    #[track_caller]
    pub fn current_player(&self) -> PlayerId {
        self.state()
            .current_player
            .clone()
            .unwrap_or_else(|| panic!("there is no current player\n{}", self.describe()))
    }

    /// The current speaker.
    pub fn speaker(&self) -> Option<&str> {
        self.state().speaker.as_deref()
    }

    /// The player with the provided name.
    #[track_caller]
    pub fn player(&self, name: &str) -> &Player {
        self.state()
            .players
            .get(name)
            .unwrap_or_else(|| panic!("no player called {name:?}"))
    }

    /// The points of the player with the provided name.
    #[track_caller]
    pub fn points(&self, name: &str) -> i8 {
        self.player(name);
        self.state()
            .score
            .player_points
            .get(name)
            .copied()
            .unwrap_or(0)
    }

    /// The current turn order.
    pub fn turn_order(&self) -> Vec<&str> {
        self.state().turn_order.iter().map(|p| &**p).collect()
    }

    /// All accepted events and their timestamps.
    pub fn history(&self) -> &[(Event, DateTime<Utc>)] {
        &self.game.history
    }

    /// One line per event describing how the game progressed, see [timeline::render].
    pub fn timeline(&self) -> String {
        timeline::render(self.history())
    }

    /// The timestamp the next event will get.
    pub fn now(&self) -> DateTime<Utc> {
        self.clock.now()
    }

    /// A short description of the current state, used in panic messages.
    pub(crate) fn describe(&self) -> String {
        let s = self.state();
        format!(
            "  phase: {:?}, round: {}, current: {:?}, speaker: {:?}\n  turn order: {:?}, passed: {:?}",
            s.phase, s.round, s.current_player, s.speaker, s.turn_order, s.passed_players,
        )
    }

    pub(crate) fn from_history(history: Vec<(Event, DateTime<Utc>)>, current: GameState) -> Game {
        Game {
            players: vec![],
            current: Arc::new(current),
            history,
        }
    }
}

/// Builds a [TestGame] and drives it to the creation, setup or strategy phase.
#[derive(Clone)]
pub struct TestGameBuilder {
    settings: GameSettings,
    players: Vec<(String, Faction, Option<Color>)>,
    milty: Option<MiltyData>,
    clock_step: TimeDelta,
    setup: SetupPlan,
}

impl Default for TestGameBuilder {
    fn default() -> Self {
        Self {
            settings: GameSettings::default(),
            players: vec![],
            milty: None,
            clock_step: TimeDelta::minutes(1),
            setup: SetupPlan {
                speaker: None,
                objectives: (
                    PublicObjective::CornerTheMarket,
                    PublicObjective::DevelopWeaponry,
                ),
                setup_techs: vec![],
                tribunii: None,
            },
        }
    }
}

impl TestGameBuilder {
    /// Enable Prophecy of Kings.
    pub fn pok(mut self) -> Self {
        self.settings.expansions.prophecy_of_kings = true;
        self
    }

    /// Enable all three codices.
    pub fn codex(mut self) -> Self {
        self.settings.expansions.codex_1 = true;
        self.settings.expansions.codex_2 = true;
        self.settings.expansions.codex_3 = true;
        self
    }

    /// Enable Thunder's Edge.
    pub fn thunders_edge(mut self) -> Self {
        self.settings.expansions.thunders_edge = true;
        self
    }

    /// Set exactly which expansions are enabled.
    pub fn expansions(mut self, expansions: Expansions) -> Self {
        self.settings.expansions = expansions;
        self
    }

    /// Set the points required to win.
    pub fn max_points(mut self, points: u32) -> Self {
        self.settings.max_points = points;
        self
    }

    /// Add a player. They get the first color not taken by an earlier player.
    pub fn player(mut self, name: &str, faction: Faction) -> Self {
        self.players.push((name.to_string(), faction, None));
        self
    }

    /// Add a player with a specific color.
    pub fn player_with_color(mut self, name: &str, faction: Faction, color: Color) -> Self {
        self.players.push((name.to_string(), faction, Some(color)));
        self
    }

    /// Import the game from milty instead of adding players.
    pub fn milty(mut self, milty_data: MiltyData) -> Self {
        self.milty = Some(milty_data);
        self
    }

    /// Choose the starting speaker (defaults to the first player).
    pub fn speaker(mut self, name: &str) -> Self {
        self.setup.speaker = Some(name.to_string());
        self
    }

    /// Choose the initial objectives (defaults to Corner the Market and Develop Weaponry).
    pub fn objectives(mut self, first: PublicObjective, second: PublicObjective) -> Self {
        self.setup.objectives = (first, second);
        self
    }

    /// Choose the starting techs of a Winnu, Argent Flight or Keleres player.
    pub fn setup_techs(mut self, name: &str, techs: impl Into<Vec<Technology>>) -> Self {
        self.setup
            .setup_techs
            .push((name.to_string(), techs.into()));
        self
    }

    /// Choose the faction Council Keleres replicates with The Tribunii.
    pub fn tribunii(mut self, faction: Faction) -> Self {
        self.setup.tribunii = Some(faction);
        self
    }

    /// How far the clock advances per event (defaults to one minute).
    pub fn clock_step(mut self, step: TimeDelta) -> Self {
        self.clock_step = step;
        self
    }

    /// Create the game and add the players, staying in the creation phase.
    #[track_caller]
    pub fn create(self) -> TestGame {
        let mut game = TestGame {
            game: Game::default(),
            clock: Clock::new(self.clock_step),
            setup: self.setup,
        };

        if let Some(milty_data) = self.milty {
            game.apply(Event::ImportFromMilty {
                max_points: self.settings.max_points,
                milty_data: Box::new(milty_data),
            });
            return game;
        }

        game.apply(Event::SetSettings {
            settings: self.settings,
        });

        let mut taken: Vec<Color> = self.players.iter().filter_map(|(_, _, c)| *c).collect();
        for (name, faction, color) in self.players {
            let color = color.unwrap_or_else(|| {
                let color = Color::iter()
                    .find(|c| !taken.contains(c))
                    .expect("ran out of colors");
                taken.push(color);
                color
            });
            game.apply(Event::AddPlayer {
                player: NewPlayer {
                    name,
                    faction,
                    color,
                },
            });
        }
        game
    }

    /// Create the game and finish the creation phase, entering the setup phase.
    #[track_caller]
    pub fn setup(self) -> TestGame {
        let mut game = self.create();
        game.finish_creation();
        game
    }

    /// Create, set up and start the game, entering the strategy phase of round 1.
    #[track_caller]
    pub fn start(self) -> TestGame {
        let mut game = self.setup();
        game.auto_setup().start_game();
        game
    }
}
