use crate::{
    data::{
        common::{color::Color, faction::Faction},
        components::strategy_card::StrategyCard,
    },
    game::{Event, Game},
    player::Player,
};

pub fn play_example_game() -> Game {
    let mut game = Game::new(vec![]);

    let events = vec![
        Event::AddPlayer {
            player: Player {
                name: "Tux".into(),
                color: Color::Black,
                faction: Faction::SardakkNorr,
                planets: vec![],
            },
        },
        Event::AddPlayer {
            player: Player {
                name: "Vidde".into(),
                color: Color::Purple,
                faction: Faction::UniversitiesOfJolNar,
                planets: vec![],
            },
        },
        Event::AddPlayer {
            player: Player {
                name: "Gurr".into(),
                color: Color::Green,
                faction: Faction::NekroVirus,
                planets: vec![],
            },
        },
        Event::StartGame,
        Event::TakeStrategyCard {
            player: "Tux".into(),
            card: StrategyCard::Leadership,
        },
        Event::TakeStrategyCard {
            player: "Vidde".into(),
            card: StrategyCard::Technology,
        },
        Event::TakeStrategyCard {
            player: "Gurr".into(),
            card: StrategyCard::Diplomacy,
        },
        Event::TakeStrategyCard {
            player: "Tux".into(),
            card: StrategyCard::Construction,
        },
        Event::TakeStrategyCard {
            player: "Vidde".into(),
            card: StrategyCard::Imperial,
        },
        Event::TakeStrategyCard {
            player: "Gurr".into(),
            card: StrategyCard::Politics,
        },
        Event::CompleteStrategyPhase,
        Event::TacticalAction {
            player: "Tux".into(),
        },
        Event::StrategicActionBegin {
            player: "Gurr".into(),
            card: StrategyCard::Diplomacy,
        },
        Event::StrategicActionCommit,
        Event::StrategicActionBegin {
            player: "Vidde".into(),
            card: StrategyCard::Technology,
        },
        Event::StrategicActionCommit,
        Event::StrategicActionBegin {
            player: "Tux".into(),
            card: StrategyCard::Leadership,
        },
        Event::StrategicActionCommit,
        // Event::StrategicActionBegin {
        //     player: "Gurr".into(),
        //     card: StrategyCard::Politics,
        // },
        // Event::StrategicActionCommit,
        // Event::StrategicActionBegin {
        //     player: "Vidde".into(),
        //     card: StrategyCard::Imperial,
        // },
        // Event::StrategicActionCommit,
        // Event::StrategicActionBegin {
        //     player: "Tux".into(),
        //     card: StrategyCard::Construction,
        // },
        // Event::StrategicActionCommit,
        // Event::PassAction {
        //     player: "Gurr".into(),
        // },
        // Event::PassAction {
        //     player: "Vidde".into(),
        // },
        // Event::PassAction {
        //     player: "Tux".into(),
        // },
    ];

    for event in events {
        game.apply(event);
    }

    game
}
