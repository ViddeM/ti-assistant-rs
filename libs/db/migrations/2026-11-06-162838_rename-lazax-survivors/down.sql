UPDATE game_event
SET event = replace(event::text, '"LazaxSurvivors"', '"LasaxSurvivors"')::jsonb
WHERE event::text LIKE '%"LazaxSurvivors"%';
