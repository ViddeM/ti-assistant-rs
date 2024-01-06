# Twilight Imperium 4th edition helper application
An application intended to be used alongside the Twilight Imperium 4th ed game.

## Development
How to dev?

### Setup DB
Run `docker compose up` in the root directory of the project.

### Run backend
 1) Ensure that you have a `backend/.env` file, you can copy the `backend/.env.example` file to get reasonable default settings.
 2) Enter the `backend/server` directory and run `cargo run` (note that the directory is relevant when using the `DEMO_GAMES_SKIP_DB` env variable is `false`)

### Run frontend
 1) Ensure that you have a `frontend/.env` file, you can copy the `frontend/.env.example` file to get reasonable default settings.
 2) Download dependencies run `yarn` in the `frontend` directory.
 3) Run frontend by running `yarn dev` in the `frontend` directory.

### Notes on the backend
The backend uses a system of "demo games" both for manual and automated testing purposes. Such demo games are located in the `demo_games` directory and can be created using a CLI script in the `backend/demo_game_creator` (see "Creating demo games" below).
If the `VERIFY_DEMO_GAMES` env variable is set, all demo games will be replayed during compile-time to alert for breaking changes to the codebase.
Furthermore if the `DEMO_GAMES_SKIP_DB` env variable is not set to `true`, the demo games will be inserted into the database upon startup, if the `OVERWRITE_DB_DEMO_GAMES` is also set then it will also reset those games to the state that they are stored in.
 
### Creating demo games
Creating a demo game is done by going to the `backend/demo_game_creator` directory and running `cargo run <GAME_ID> <GAME_NAME>` where the `GAME_ID` argument must be a game that exists in the database (a snapshot of which will become the demo game) and the `GAME_NAME` argument is a name that will identify the demo game. Note: this script requires the env variables `DATABASE_URL` and `DEMO_GAMES_DIR` to exist.
