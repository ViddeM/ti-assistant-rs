-- Game with 5 players just starting their first status phase after taking some planets & their strategy cards.
-- Game ID: 7d96c760

BEGIN;

INSERT INTO
    game(id, "name")
VALUES
    (
        '7d96c760',
        'Five player status phase start'
    );

INSERT INTO
    game_event(game_id, "event")
VALUES
    ('7d96c760', '{"AddPlayer": {"player": {"name": "Tux", "color": "Purple", "faction": "NekroVirus"}}}'),
    ('7d96c760', '{"AddPlayer": {"player": {"name": "Vidde", "color": "Red", "faction": "VuilRaithCabal"}}}'),
    ('7d96c760', '{"AddPlayer": {"player": {"name": "LP", "color": "Black", "faction": "L1Z1XMindnet"}}}'),
    ('7d96c760', '{"AddPlayer": {"player": {"name": "Gurr", "color": "Orange", "faction": "MahactGeneSorcerers"}}}'),
    ('7d96c760', '{"AddPlayer": {"player": {"name": "Potholes", "color": "Pink", "faction": "EmbersOfMuaat"}}}'),
    ('7d96c760', '"StartGame"'),
    ('7d96c760', '{"TakeStrategyCard": {"card": "Leadership", "player": "Potholes"}}'),
    ('7d96c760', '{"TakeStrategyCard": {"card": "Diplomacy", "player": "Tux"}}'),
    ('7d96c760', '{"TakeStrategyCard": {"card": "Politics", "player": "Vidde"}}'),
    ('7d96c760', '{"TakeStrategyCard": {"card": "Imperial", "player": "LP"}}'),
    ('7d96c760', '{"TakeStrategyCard": {"card": "Technology", "player": "Gurr"}}'),
    ('7d96c760', '"CompleteStrategyPhase"'),
    ('7d96c760', '{"StrategicActionBegin": {"card": "Leadership", "player": "Potholes"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "Tux"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Leadership", "player": "Vidde"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Leadership", "player": "Gurr"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "LP"}}'),
    ('7d96c760', '"StrategicActionCommit"'),
    ('7d96c760', '{"StrategicActionBegin": {"card": "Diplomacy", "player": "Tux"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "LP"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Gurr"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Diplomacy", "player": "Potholes"}}'),
    ('7d96c760', '"StrategicActionCommit"'),
    ('7d96c760', '{"StrategicActionBegin": {"card": "Politics", "player": "Vidde"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Politics", "player": "Tux"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "LP"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "Gurr"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Politics", "player": "Potholes"}}'),
    ('7d96c760', '{"StrategicActionPrimary": {"action": {"Politics": {"newSpeaker": "Tux"}}, "player": "Vidde"}}'),
    ('7d96c760', '"StrategicActionCommit"'),
    ('7d96c760', '{"StrategicActionBegin": {"card": "Technology", "player": "Gurr"}}'),
    ('7d96c760', '{"StrategicActionPrimary": {"action": {"Technology": {"tech": "GravityDrive", "extra": "SarweenTools"}}, "player": "Gurr"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": {"Technology": {"tech": "MagenDefenceGrid"}}, "player": "Tux"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "Vidde"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "LP"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Skip", "player": "Potholes"}}'),
    ('7d96c760', '"StrategicActionCommit"'),
    ('7d96c760', '{"StrategicActionBegin": {"card": "Imperial", "player": "LP"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Imperial", "player": "Tux"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Imperial", "player": "Vidde"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Imperial", "player": "Gurr"}}'),
    ('7d96c760', '{"StrategicActionSecondary": {"action": "Imperial", "player": "Potholes"}}'),
    ('7d96c760', '"StrategicActionCommit"'),
    ('7d96c760', '{"TacticalActionBegin": {"player": "Potholes"}}'),
    ('7d96c760', '{"TacticalActionTakePlanet": {"planet": "MecatolRex", "player": "Potholes"}}'),
    ('7d96c760', '{"TacticalActionCommit": {"player": "Potholes"}}'),
    ('7d96c760', '{"PassAction": {"player": "Tux"}}'),
    ('7d96c760', '{"TacticalActionBegin": {"player": "Vidde"}}'),
    ('7d96c760', '{"TacticalActionTakePlanet": {"planet": "Kraag", "player": "Vidde"}}'),
    ('7d96c760', '{"TacticalActionCommit": {"player": "Vidde"}}'),
    ('7d96c760', '{"TacticalActionBegin": {"player": "Gurr"}}'),
    ('7d96c760', '{"TacticalActionCommit": {"player": "Gurr"}}'),
    ('7d96c760', '{"PassAction": {"player": "LP"}}'),
    ('7d96c760', '{"PassAction": {"player": "Potholes"}}'),
    ('7d96c760', '{"PassAction": {"player": "Vidde"}}'),
    ('7d96c760', '{"PassAction": {"player": "Gurr"}}');

COMMIT;
