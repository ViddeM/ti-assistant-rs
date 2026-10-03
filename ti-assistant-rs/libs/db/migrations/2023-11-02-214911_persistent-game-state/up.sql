CREATE TABLE game (
	id CHAR(8) PRIMARY KEY NOT NULL,
	name VARCHAR(128) NOT NULL
);

CREATE TABLE game_event (
	id SERIAL NOT NULL,

	game_id CHAR(8) NOT NULL,

	-- sequence number of this event, events for a game are ordered by this value
	seq INTEGER NOT NULL DEFAULT -1,

	timestamp TIMESTAMP WITH TIME ZONE NOT NULL,

	event JSONB NOT NULL,

	CONSTRAINT game_event_pkey PRIMARY KEY (id),
	CONSTRAINT game_event_unique_seq UNIQUE(seq, game_id),
	CONSTRAINT game_event_game_id_fkey
		FOREIGN KEY (game_id) REFERENCES game(id)
		ON DELETE CASCADE
);

CREATE INDEX game_event_seq_index ON game_event(game_id, seq);

-- trigger to automatically set the correct `seq` value for new events
CREATE FUNCTION next_game_event_seq() RETURNS TRIGGER LANGUAGE plpgsql AS $$
	DECLARE new_seq INTEGER;
	BEGIN
	SELECT COUNT(seq) into new_seq FROM game_event WHERE game_id = new.game_id;
		RETURN (new.id, new.game_id, new_seq, new.timestamp, new.event);
	END
$$;

CREATE TRIGGER set_seq BEFORE INSERT ON game_event
	FOR EACH ROW EXECUTE FUNCTION next_game_event_seq();
