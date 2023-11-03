-- Game with 3 players just starting their first action phase
-- Game ID: c561033b
INSERT INTO
    game(id, "name")
VALUES
    (
        -983497925,
        'Three player action phase start'
    );

INSERT INTO
    game_event(id, game_id, "event")
VALUES
(
        1,
        -983497925,
        '{"AddPlayer": {"player": {"name": "Vidde", "color": "Blue", "faction": "BaronyOfLetnev"}}}'
    ),
    (
        2,
        -983497925,
        '{"AddPlayer": {"player": {"name": "Pelle", "color": "Black", "faction": "YinBrotherhood"}}}'
    ),
    (
        3,
        -983497925,
        '{"AddPlayer": {"player": {"name": "Tux", "color": "Yellow", "faction": "ArgentFlight"}}}'
    ),
    (4, -983497925, '"StartGame"'),
    (
        5,
        -983497925,
        '{"TakeStrategyCard": {"card": "Leadership", "player": "Vidde"}}'
    ),
    (
        6,
        -983497925,
        '{"TakeStrategyCard": {"card": "Construction", "player": "Pelle"}}'
    ),
    (
        7,
        -983497925,
        '{"TakeStrategyCard": {"card": "Trade", "player": "Tux"}}'
    ),
    (
        8,
        -983497925,
        '{"TakeStrategyCard": {"card": "Technology", "player": "Vidde"}}'
    ),
    (
        9,
        -983497925,
        '{"TakeStrategyCard": {"card": "Imperial", "player": "Pelle"}}'
    ),
    (
        10,
        -983497925,
        '{"TakeStrategyCard": {"card": "Diplomacy", "player": "Tux"}}'
    ),
    (11, -983497925, '"CompleteStrategyPhase"');
