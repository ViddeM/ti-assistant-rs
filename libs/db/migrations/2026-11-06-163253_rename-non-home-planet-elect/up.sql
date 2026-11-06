UPDATE game_event
SET event = replace(event::text, '"electKind": "PlanetWithTrait"', '"electKind": "NonHomePlanet"')::jsonb
WHERE event::text LIKE '%"electKind": "PlanetWithTrait"%';
