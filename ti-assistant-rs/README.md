# TI Helper

A helper app to use alongside Twilight Imperium 4th edition. Written in Rust with [Dioxus](https://dioxuslabs.com/) (fullstack, web).

See [Roadmap.md](./Roadmap.md) for the feature status.

## Layout

```
libs/
├─ game_data/          # Game components, state and events (shared by server and client)
├─ game_logic/         # Event handling / game rules
├─ db/                 # Postgres persistence (diesel)
├─ milty/              # Milty draft import
└─ demo_game_creator/  # CLI to snapshot a game from the DB into a demo game
packages/
├─ api/                # Server functions, websocket endpoints, lobbies
├─ ui/                 # Shared Dioxus components and views
└─ web/                # Web entrypoint and routes
demo_games/            # Demo games, used for testing
```

## Setup

Requirements: Rust, the [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) (`dx`), and Postgres (optional, see below).

1. Copy the env file: `cp .env.example .env`
2. (Optional) Start a Postgres DB with `docker compose up` in the repo root. Set `DEMO_GAMES_SKIP_DB=true` and leave `DATABASE_URL` unset to run without one (games are then only kept in memory).
3. Run the app:

```bash
dx serve --package web
```

Migrations run on startup if `MIGRATE_DB=true`.

## Configuration

Set through env vars or `.env`:

| Variable | Description |
| --- | --- |
| `DATABASE_URL` | Postgres URI. |
| `MIGRATE_DB` | Run DB migrations on startup. |
| `MEM_GC_CRON` | Cron string for unloading inactive games from memory. |
| `DEMO_GAMES_DIR` | Directory with the demo games. |
| `DEMO_GAMES_SKIP_DB` | Don't insert demo games into the DB. |
| `OVERWRITE_DB_DEMO_GAMES` | Reset demo games in the DB to their stored state on startup. |
| `BIND_HOST` / `BIND_PORT` | Server bind address. |

## Demo games

Demo games in `demo_games/` are used for manual and automated testing. At startup they are replayed to catch breaking changes, and (unless `DEMO_GAMES_SKIP_DB=true`) inserted into the DB.

To create one from an existing game in the DB:

```bash
cargo run -p demo_game_creator -- <GAME_ID> <GAME_NAME>
```

This needs `DATABASE_URL` and `DEMO_GAMES_DIR` to be set.

## Docker

```bash
docker build -t ti-helper .
```

The image serves on port 8080.
