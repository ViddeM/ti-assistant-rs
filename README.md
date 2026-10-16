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
└─ system_tests/       # Scenario tests for the game logic (see Testing)
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

## Testing

```bash
cargo test --workspace
```

The game logic is covered by scenarios in `libs/system_tests/tests/scenarios/`. A scenario is a function that plays a game through a small DSL and asserts on the parts it cares about:

```rust
pub fn action_strategic_politics_and_imperial() -> TestGame {
    let mut g = three_players().start();
    g.strategy_phase(&[("Alice", StrategyCard::Politics), /* ... */]);
    g.turn("Carol").play_card(StrategyCard::Leadership);
    g.turn("Alice")
        .strategic(StrategyCard::Politics)
        .primary(StrategicPrimaryAction::Politics { new_speaker: p("Bob") })
        .skip_rest()
        .commit()
        .end_turn();
    assert_eq!(g.speaker(), Some("Bob"));
    g
}
```

Events go through the real game logic with timestamps from a fake clock. Use `expect_rejected(event, "part of the error")` for events that should fail.

Scenarios registered with `scenarios![...]` are also checked automatically:

- **Determinism**: building the scenario twice gives the same state.
- **Replay**: replaying the history, also after a JSON round trip like in the DB, gives the same state.
- **Undo**: undoing the last events restores the earlier states.
- **Snapshots**: the final state (`<name>@state.snap`) and a per-event timeline (`<name>@timeline.snap`) in `libs/system_tests/tests/snapshots/` must match.

When a change is intended, review and accept the new snapshots:

```bash
cargo install cargo-insta  # once
cargo insta test -p ti_helper_system_tests --review
```

Without `cargo-insta`, `INSTA_UPDATE=always cargo test -p ti_helper_system_tests` overwrites the snapshots so they can be reviewed with `git diff`.

### Playing scenarios in the UI

To load all scenarios into the database at `DATABASE_URL` (each gets a fixed game id, and seeding again resets them):

```bash
cargo test -p ti_helper_system_tests --features seed-db --test scenarios seed_database -- --ignored --nocapture
```

## Docker

```bash
docker build -t ti-helper .
```

The image serves on port 8080.
