//! System tests for the game logic.
//!
//! Scenarios are plain functions that build a game through the [TestGame] DSL and return it.
//! Registering them with [scenarios!] runs [check_scenario] on each: determinism, replay,
//! JSON round trip and undo checks, plus snapshots of the final state and the timeline.
//!
//! The scenarios themselves live in `tests/scenarios/`.

pub mod check;
pub mod clock;
pub mod game;
pub mod milty;
pub mod phases;
pub mod timeline;

pub use check::{Scenario, check_scenario};
pub use game::{TestGame, TestGameBuilder};

/// Everything a scenario usually needs.
pub mod prelude {
    pub use chrono::TimeDelta;
    pub use ti_helper_game_data::{
        actions::{
            event::{ActionCardAction, Event, FrontierCardAction, RelicAction},
            strategic::{StrategicPrimaryAction, StrategicSecondaryAction},
        },
        common::{color::Color, faction::Faction, game_settings::Expansions},
        components::{
            action_card::ActionCard,
            agenda::{Agenda, AgendaElect, ForOrAgainst},
            frontier_card::FrontierCard,
            leaders::{Agent, Commander, Hero, Leader},
            objectives::{Objective, public::PublicObjective, secret::SecretObjective},
            phase::Phase,
            planet::Planet,
            planet_attachment::PlanetAttachment,
            relic::Relic,
            strategy_card::StrategyCard,
            system::{System, SystemId},
            tech::Technology,
        },
        state::expedition::{TECompletion, TEExpeditionSlot},
    };

    pub use crate::{
        Scenario, TestGame,
        game::{AGAINST, FOR, elect, p},
        scenarios,
    };
}

/// Register scenario functions (`fn() -> TestGame`) defined in the current module.
///
/// Creates a `SCENARIOS` constant listing them and a test per scenario that runs
/// [check_scenario]. Scenario names are used for the snapshot files, so they must be unique
/// across all modules; prefix them with the module name.
#[macro_export]
macro_rules! scenarios {
    ($($name:ident),* $(,)?) => {
        pub const SCENARIOS: &[$crate::Scenario] = &[
            $($crate::Scenario { name: stringify!($name), build: $name }),*
        ];

        mod checks {
            $(
                #[test]
                fn $name() {
                    $crate::check_scenario(stringify!($name), super::$name);
                }
            )*
        }
    };
}
