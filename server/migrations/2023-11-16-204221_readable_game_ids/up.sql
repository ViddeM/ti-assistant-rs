ALTER TABLE game_event
    DROP CONSTRAINT game_event_game_id_fkey;

ALTER TABLE game
    ALTER COLUMN id TYPE CHAR(8) USING (LPAD(TO_HEX(id), 8, '0'));

ALTER TABLE game_event
    ALTER COLUMN game_id TYPE CHAR(8) USING (LPAD(TO_HEX(game_id), 8, '0')),
    ADD CONSTRAINT game_event_game_id_fkey
        FOREIGN KEY (game_id) REFERENCES game(id) ON DELETE CASCADE;