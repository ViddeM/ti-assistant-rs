UPDATE game_event
SET event = replace(event::text, '"LasaxSurvivors"', '"LazaxSurvivors"')::jsonb
WHERE event::text LIKE '%"LasaxSurvivors"%';
