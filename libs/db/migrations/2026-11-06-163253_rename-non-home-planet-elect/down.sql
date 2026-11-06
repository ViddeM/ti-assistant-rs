UPDATE game_event
SET event = replace(event::text, '"electKind": "NonHomePlanet"', '"electKind": "PlanetWithTrait"')::jsonb
WHERE event::text LIKE '%"electKind": "NonHomePlanet"%';
