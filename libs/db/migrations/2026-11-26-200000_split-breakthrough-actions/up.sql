ALTER TABLE game_event DISABLE TRIGGER set_seq;

DO $$
DECLARE
	r RECORD;
	player TEXT;
	action JSONB;
	new_events JSONB[];
	researched JSONB;
	other TEXT;
	shift INTEGER;
	i INTEGER;
BEGIN
	FOR r IN
		SELECT id, game_id, seq, timestamp, event
		FROM game_event
		WHERE event ? 'PlayBreakthroughAction'
		ORDER BY game_id, seq DESC
	LOOP
		player := r.event->'PlayBreakthroughAction'->>'player';
		action := r.event->'PlayBreakthroughAction'->'action';
		new_events := ARRAY[jsonb_build_object('BreakthroughActionBegin', jsonb_build_object('player', player))];

		IF jsonb_typeof(action) = 'object' AND action ? 'VisionariaSelect' THEN
			researched := coalesce(action->'VisionariaSelect'->'researched', '{}'::jsonb);
			FOR other IN
				SELECT DISTINCT p.name
				FROM (
					SELECT e.event->'AddPlayer'->'player'->>'name' AS name,
						e.event->'AddPlayer'->'player'->>'faction' AS faction
					FROM game_event e
					WHERE e.game_id = r.game_id AND e.seq < r.seq AND e.event ? 'AddPlayer'
					UNION ALL
					SELECT m.key, m.value->>'faction'
					FROM game_event e,
						jsonb_each(e.event->'ImportFromMilty'->'milty_data'->'players') m
					WHERE e.game_id = r.game_id AND e.seq < r.seq AND e.event ? 'ImportFromMilty'
				) p
				WHERE p.name <> player AND p.faction <> 'NekroVirus'
				ORDER BY p.name
			LOOP
				new_events := new_events || jsonb_build_object('VisionariaSelectDecision',
					jsonb_build_object('player', other, 'tech', researched->other));
			END LOOP;
			action := '"VisionariaSelect"'::jsonb;
		END IF;

		new_events := new_events || jsonb_build_object('BreakthroughActionCommit',
			jsonb_build_object('player', player, 'action', action));

		shift := array_length(new_events, 1) - 1;
		UPDATE game_event SET seq = seq + 1000000
			WHERE game_id = r.game_id AND seq > r.seq;
		UPDATE game_event SET seq = seq - 1000000 + shift
			WHERE game_id = r.game_id AND seq > 1000000;

		UPDATE game_event SET event = new_events[1] WHERE id = r.id;
		FOR i IN 2 .. array_length(new_events, 1) LOOP
			INSERT INTO game_event (game_id, seq, timestamp, event)
				VALUES (r.game_id, r.seq + i - 1, r.timestamp, new_events[i]);
		END LOOP;
	END LOOP;
END
$$;

ALTER TABLE game_event ENABLE TRIGGER set_seq;
