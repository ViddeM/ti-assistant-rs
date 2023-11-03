-- Game with 5 players just starting their first status phase after taking some planets & their strategy cards.
-- Game ID: 7d96c760
INSERT INTO
    game(id, "name")
VALUES
    (
        2107033440,
        'Five player status phase start'
    );

INSERT INTO
    game_event(id, game_id, "event")
VALUES
    (
        12,
        2107033440,
        '{"AddPlayer": {"player": {"name": "Tux", "color": "Yellow", "faction": "YssarilTribes"}}}'
    ),
    (
        13,
        2107033440,
        '{"AddPlayer": {"player": {"name": "Vidde", "color": "Red", "faction": "VuilRaithCabal"}}}'
    ),
    (
        14,
        2107033440,
        '{"AddPlayer": {"player": {"name": "LP", "color": "Black", "faction": "ArgentFlight"}}}'
    ),
    (
        15,
        2107033440,
        '{"AddPlayer": {"player": {"name": "Gurr", "color": "Green", "faction": "CouncilKeleres"}}}'
    ),
    (
        16,
        2107033440,
        '{"AddPlayer": {"player": {"name": "Theo", "color": "Purple", "faction": "GhostsOfCreuss"}}}'
    ),
    (17, 2107033440, '"StartGame"'),
    (
        18,
        2107033440,
        '{"TakeStrategyCard": {"card": "Leadership", "player": "Tux"}}'
    ),
    (
        19,
        2107033440,
        '{"TakeStrategyCard": {"card": "Diplomacy", "player": "Vidde"}}'
    ),
    (
        20,
        2107033440,
        '{"TakeStrategyCard": {"card": "Politics", "player": "LP"}}'
    ),
    (
        21,
        2107033440,
        '{"TakeStrategyCard": {"card": "Construction", "player": "Gurr"}}'
    ),
    (
        22,
        2107033440,
        '{"TakeStrategyCard": {"card": "Trade", "player": "Theo"}}'
    ),
    (23, 2107033440, '"CompleteStrategyPhase"'),
    (
        24,
        2107033440,
        '{"StrategicActionBegin": {"card": "Leadership", "player": "Tux"}}'
    ),
    (
        25,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Theo", "didSecondary": true}}'
    ),
    (
        26,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Gurr", "didSecondary": false}}'
    ),
    (
        27,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Vidde", "didSecondary": true}}'
    ),
    (
        28,
        2107033440,
        '{"StrategicActionSecondary": {"player": "LP", "didSecondary": false}}'
    ),
    (29, 2107033440, '"StrategicActionCommit"'),
    (
        30,
        2107033440,
        '{"StrategicActionBegin": {"card": "Diplomacy", "player": "Vidde"}}'
    ),
    (
        31,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Tux", "didSecondary": false}}'
    ),
    (
        32,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Theo", "didSecondary": true}}'
    ),
    (
        33,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Gurr", "didSecondary": false}}'
    ),
    (
        34,
        2107033440,
        '{"StrategicActionSecondary": {"player": "LP", "didSecondary": true}}'
    ),
    (35, 2107033440, '"StrategicActionCommit"'),
    (
        36,
        2107033440,
        '{"StrategicActionBegin": {"card": "Politics", "player": "LP"}}'
    ),
    (
        37,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Tux", "didSecondary": true}}'
    ),
    (
        38,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Theo", "didSecondary": true}}'
    ),
    (
        39,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Gurr", "didSecondary": false}}'
    ),
    (
        40,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Vidde", "didSecondary": false}}'
    ),
    (41, 2107033440, '"StrategicActionCommit"'),
    (
        42,
        2107033440,
        '{"StrategicActionBegin": {"card": "Construction", "player": "Gurr"}}'
    ),
    (
        43,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Tux", "didSecondary": false}}'
    ),
    (
        44,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Theo", "didSecondary": false}}'
    ),
    (
        45,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Vidde", "didSecondary": true}}'
    ),
    (
        46,
        2107033440,
        '{"StrategicActionSecondary": {"player": "LP", "didSecondary": true}}'
    ),
    (47, 2107033440, '"StrategicActionCommit"'),
    (
        48,
        2107033440,
        '{"StrategicActionBegin": {"card": "Trade", "player": "Theo"}}'
    ),
    (
        49,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Tux", "didSecondary": true}}'
    ),
    (
        50,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Gurr", "didSecondary": true}}'
    ),
    (
        51,
        2107033440,
        '{"StrategicActionSecondary": {"player": "Vidde", "didSecondary": false}}'
    ),
    (
        52,
        2107033440,
        '{"StrategicActionSecondary": {"player": "LP", "didSecondary": false}}'
    ),
    (53, 2107033440, '"StrategicActionCommit"'),
    (
        54,
        2107033440,
        '{"TacticalActionBegin": {"player": "Tux"}}'
    ),
    (
        55,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "VefutII", "player": "Tux"}}'
    ),
    (
        56,
        2107033440,
        '{"TacticalActionCommit": {"player": "Tux"}}'
    ),
    (
        57,
        2107033440,
        '{"TacticalActionBegin": {"player": "Vidde"}}'
    ),
    (
        58,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "Everra", "player": "Vidde"}}'
    ),
    (
        59,
        2107033440,
        '{"TacticalActionCommit": {"player": "Vidde"}}'
    ),
    (
        60,
        2107033440,
        '{"TacticalActionBegin": {"player": "LP"}}'
    ),
    (
        61,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "Accoen", "player": "LP"}}'
    ),
    (
        62,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "JoelIr", "player": "LP"}}'
    ),
    (
        63,
        2107033440,
        '{"TacticalActionCommit": {"player": "LP"}}'
    ),
    (
        64,
        2107033440,
        '{"TacticalActionBegin": {"player": "Gurr"}}'
    ),
    (
        65,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "JoelIr", "player": "Gurr"}}'
    ),
    (
        66,
        2107033440,
        '{"TacticalActionCommit": {"player": "Gurr"}}'
    ),
    (
        67,
        2107033440,
        '{"TacticalActionBegin": {"player": "Theo"}}'
    ),
    (
        68,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "Abaddon", "player": "Theo"}}'
    ),
    (
        69,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "Ashtroth", "player": "Theo"}}'
    ),
    (
        70,
        2107033440,
        '{"TacticalActionCommit": {"player": "Theo"}}'
    ),
    (
        71,
        2107033440,
        '{"TacticalActionBegin": {"player": "Tux"}}'
    ),
    (
        72,
        2107033440,
        '{"TacticalActionCommit": {"player": "Tux"}}'
    ),
    (
        73,
        2107033440,
        '{"TacticalActionBegin": {"player": "Vidde"}}'
    ),
    (
        74,
        2107033440,
        '{"TacticalActionTakePlanet": {"planet": "Loki", "player": "Vidde"}}'
    ),
    (
        75,
        2107033440,
        '{"TacticalActionCommit": {"player": "Vidde"}}'
    ),
    (
        76,
        2107033440,
        '{"PassAction": {"player": "LP"}}'
    ),
    (
        77,
        2107033440,
        '{"PassAction": {"player": "Gurr"}}'
    ),
    (
        78,
        2107033440,
        '{"PassAction": {"player": "Theo"}}'
    ),
    (
        79,
        2107033440,
        '{"PassAction": {"player": "Tux"}}'
    ),
    (
        80,
        2107033440,
        '{"PassAction": {"player": "Vidde"}}'
    );