DO $$
DECLARE
	c RECORD;
	begin_seq INTEGER;
	player TEXT;
	action JSONB;
BEGIN
	FOR c IN
		SELECT id, game_id, seq, event
		FROM game_event
		WHERE event ? 'BreakthroughActionCommit'
		ORDER BY game_id, seq DESC
	LOOP
		player := c.event->'BreakthroughActionCommit'->>'player';
		action := c.event->'BreakthroughActionCommit'->'action';

		SELECT max(seq) INTO begin_seq
		FROM game_event
		WHERE game_id = c.game_id AND seq < c.seq
			AND event->'BreakthroughActionBegin'->>'player' = player;

		IF action = '"VisionariaSelect"'::jsonb THEN
			action := jsonb_build_object('VisionariaSelect', jsonb_build_object('researched',
				(SELECT coalesce(jsonb_object_agg(d.event->'VisionariaSelectDecision'->>'player',
					d.event->'VisionariaSelectDecision'->'tech'), '{}'::jsonb)
				FROM game_event d
				WHERE d.game_id = c.game_id AND d.seq > begin_seq AND d.seq < c.seq
					AND d.event ? 'VisionariaSelectDecision'
					AND d.event->'VisionariaSelectDecision'->'tech' <> 'null'::jsonb)));
		END IF;

		UPDATE game_event
			SET event = jsonb_build_object('PlayBreakthroughAction',
				jsonb_build_object('player', player, 'action', action))
			WHERE game_id = c.game_id AND seq = begin_seq;
		DELETE FROM game_event
			WHERE game_id = c.game_id AND seq > begin_seq AND seq <= c.seq
				AND (event ? 'VisionariaSelectDecision' OR event ? 'BreakthroughActionCommit');
	END LOOP;

	UPDATE game_event SET seq = seq + 1000000;
	UPDATE game_event e SET seq = n.new_seq
		FROM (
			SELECT id, row_number() OVER (PARTITION BY game_id ORDER BY seq) - 1 AS new_seq
			FROM game_event
		) n
		WHERE e.id = n.id;
END
$$;
