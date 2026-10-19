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
packages/
├─ api/                # Server functions, websocket endpoints, lobbies
├─ ui/                 # Shared Dioxus components and views
└─ web/                # Web entrypoint and routes
```

## Setup

Requirements: Rust, the [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started) (`dx`), and Postgres (optional, see below).

1. Copy the env file: `cp .env.example .env`
2. (Optional) Start a Postgres DB with `docker compose up` in the repo root. Leave `DATABASE_URL` unset to run without one (games are then only kept in memory).
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
| `BIND_HOST` / `BIND_PORT` | Server bind address. |




```bash
```


## Docker

```bash
docker build -t ti-helper .
```

The image serves on port 8080.
