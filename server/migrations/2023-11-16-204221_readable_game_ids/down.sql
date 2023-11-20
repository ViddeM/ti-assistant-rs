ALTER TABLE game_event
    DROP CONSTRAINT game_event_game_id_fkey;

ALTER TABLE game
    ALTER COLUMN id TYPE INTEGER USING (('x'||id)::bit(32)::int);

ALTER TABLE game_event
    ALTER COLUMN game_id TYPE INTEGER USING (('x'||game_id)::bit(32)::int),
    ADD CONSTRAINT game_event_game_id_fkey
        FOREIGN KEY (game_id) REFERENCES game(id);