-- Game with 3 players just starting their first action phase
-- Game ID: c561033b
BEGIN;

INSERT INTO
    game(id, "name")
VALUES
    (
        'c561033b',
        'Three player action phase start'
    );

INSERT INTO
    game_event(game_id, "event")
VALUES
    ('c561033b', '{"AddPlayer": {"player": {"name": "Vidde", "color": "Blue", "faction": "BaronyOfLetnev"}}}'),
    ('c561033b', '{"AddPlayer": {"player": {"name": "Pelle", "color": "Black", "faction": "YinBrotherhood"}}}'),
    ('c561033b', '{"AddPlayer": {"player": {"name": "Tux", "color": "Yellow", "faction": "ArgentFlight"}}}'),
    ('c561033b', '"StartGame"'),
    ('c561033b', '{"TakeStrategyCard": {"card": "Leadership", "player": "Vidde"}}'),
    ('c561033b', '{"TakeStrategyCard": {"card": "Construction", "player": "Pelle"}}'),
    ('c561033b', '{"TakeStrategyCard": {"card": "Trade", "player": "Tux"}}'),
    ('c561033b', '{"TakeStrategyCard": {"card": "Technology", "player": "Vidde"}}'),
    ('c561033b', '{"TakeStrategyCard": {"card": "Imperial", "player": "Pelle"}}'),
    ('c561033b', '{"TakeStrategyCard": {"card": "Diplomacy", "player": "Tux"}}'),
    ('c561033b', '"CompleteStrategyPhase"');

COMMIT;
