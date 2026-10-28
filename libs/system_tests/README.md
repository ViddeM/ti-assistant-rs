# System tests

Tests for the game logic, written as **scenarios**: functions that play a game through a small DSL and return it. A scenario asserts what it is about, and the framework also checks that the game is deterministic, replays, survives persistence and undo, and matches snapshots of the result.

```bash
cargo test -p ti_helper_system_tests            # everything
cargo test -p ti_helper_system_tests agenda_    # only the scenarios whose names match
```

## How it works

A scenario is a `fn() -> TestGame`. `TestGame` wraps the real `Game` and applies every event with `Game::apply_or_err`, which is the same path the server uses. Timestamps come from a fake clock that starts at 2025-01-01 12:00 UTC and moves one minute forward per accepted event.

`scenarios![...]` generates a `#[test]` for each scenario, and that test runs `check_scenario` (`src/check.rs`):

1. **Determinism**: builds the scenario twice and compares the states and timelines. Each build gets differently seeded HashMaps, so this catches results that depend on iteration order.
2. **Replay**: replaying the history from an empty game gives the same state.
3. **Persistence**: the history still replays the same after a JSON round trip, like when it's stored in the DB.
4. **Undo**: undoing each of the last three events, one at a time, restores the state from before it.
5. **Snapshots**: the result must match two files in `tests/snapshots/`:
   - `<name>@state.snap` holds the final `GameState` as YAML.
   - `<name>@timeline.snap` has one line per event. Each line shows the round, phase and current player after the event, plus any change to the speaker, turn order, custodians or points:
     ```text
       14 CompleteStrategyPhase                    r1 Action             Carol      order=[Carol Alice Bob]
       19 StrategicActionPrimary(Alice)            r1 StrategicAction    Alice      speaker=Bob
     ```

The assertions in a scenario cover what the scenario is about. The snapshots cover everything else, so a rule change shows up as a diff in every scenario it affects.

```text
src/
├─ game.rs        # TestGame + builder: raw events, setup and accessors
├─ phases.rs      # Helpers for the strategy, action, status and agenda phases
├─ check.rs       # check_scenario
├─ timeline.rs    # Renders the timeline snapshot
├─ clock.rs       # The fake clock
└─ milty.rs       # Milty draft fixture
tests/
├─ scenarios/     # One module per area, all in a single test binary
└─ snapshots/
```

## Writing a scenario

1. Add a function to the module for the area in `tests/scenarios/`. Prefix its name with the module name: the name is used for the snapshot files, so it must be unique.
2. Add it to that module's `scenarios![...]`.
3. Run the tests and accept the new snapshots (see [Snapshots](#snapshots)). A new scenario fails until its snapshots are accepted.

For a new area, add a module in `tests/scenarios/main.rs` and add its `SCENARIOS` to `ALL_SCENARIOS`. That list is used by the unique-name check and by the DB seeding.

```rust
/// Every player makes a public and a secret scoring decision before an objective is revealed.
pub fn status_scoring_and_reveal() -> TestGame {
    let mut g = action_phase(three_players());
    g.play_out_action_phase();

    g.expect_rejected(Event::CompleteStatusPhase, "No public objective has been revealed");
    g.status_phase()
        .public("Alice", PublicObjective::CornerTheMarket)
        .secret("Bob", SecretObjective::BecomeTheGatekeeper)
        .public("Carol", PublicObjective::CornerTheMarket)
        .reveal(PublicObjective::ExpandBorders)
        .complete();

    assert_eq!(g.phase(), Phase::Strategy);
    assert_eq!([g.points("Alice"), g.points("Bob"), g.points("Carol")], [1, 1, 1]);
    g
}
```

### The DSL

All of it is documented in `src/game.rs` and `src/phases.rs`. In short:

- **Building a game**:
  - Start from `TestGame::builder()` and add `.player(name, faction)`, `.pok()`, `.codex()`, `.thunders_edge()`, `.max_points(n)`, `.speaker(name)`, `.objectives(a, b)`, `.setup_techs(name, techs)`, `.tribunii(faction)`, `.milty(data)` or `.clock_step(delta)`.
  - Finish with `.create()` to stay in the creation phase, `.setup()` for the setup phase, or `.start()` for round 1's strategy phase. `.start()` fills in defaults for the faction setup, the speaker and the objectives (see `auto_setup`).
- **Strategy phase**: `g.strategy_phase(&[("Alice", card), ...])` checks that every pick is made in turn.
- **Action phase**:
  - `g.turn("Alice")` asserts that it's Alice's turn. Continue with one of:
    - `.tactical().take(planet).attach(planet, attachment).commit()`
    - `.strategic(card).primary(..).secondary("Bob", ..).skip_rest().commit()`
    - `.action_card(..)`, `.leader(..)`, `.frontier(..)`, `.gain_relic(..)` or `.relic(..)`
  - End the turn with `.end_turn()` or `.pass()`.
  - `.play_card(card)` uses a card without choosing any effects and ends the turn.
  - `g.play_out_action_phase()` has every player use their remaining cards that way, then pass.
- **Status phase**: `g.status_phase().public(..).secret(..).reveal(objective).complete()`. `reveal` records "nothing scored" for anyone who hasn't decided yet.
- **Agenda phase**:
  - Vote with `g.agenda(agenda).vote("Alice", FOR, 3).vote_all(AGAINST, 1)`.
  - Then `.resolve_expected()`, `.resolve(outcome)` or `.veto()`.
  - End with `g.complete_agenda_phase()`.
- **Anything else**:
  - `g.apply(event)` applies any event.
  - `g.expect_rejected(event, "part of the error")` also checks that the rejected event left the game untouched.
  - `g.advance(delta)` moves the clock forward.
  - The phase handles have `.expect_rejected` too, so rejections can be checked in the middle of an action.
- **Asserting**: `g.state()`, `phase()`, `round()`, `player(name)`, `points(name)`, `speaker()`, `current_player()`, `turn_order()`.
- **Shorthands**: `p("Alice")` gives a `PlayerId`, plus `elect("Alice")`, `FOR` and `AGAINST`.
- **Fixtures** (`tests/scenarios/fixtures.rs`):
  - `three_players()`: Alice (Sol), Bob (Hacan) and Carol (Xxcha).
  - `action_phase(builder)` and `agenda_phase(builder)` start a game in that phase of round 1.

When the DSL is missing a step, use `g.apply(Event::...)`. If the step will come up often, add a helper to `src/phases.rs`.

Some rules don't fit "one game with a result", such as rules that need several small games, or undo behavior. Write those as plain `#[test]`s in the module, like `creation_rules` in `setup.rs`. They don't get the automatic checks.

## Snapshots

When a change is intended, review and accept the new snapshots:

```bash
cargo install cargo-insta  # once
cargo insta test -p ti_helper_system_tests --review
```

Without cargo-insta, run `INSTA_UPDATE=always cargo test -p ti_helper_system_tests` and review the changes with `git diff`. With `CI=true`, a mismatch fails the test and nothing is written.

Renaming or removing a scenario leaves its old `.snap` files behind, so delete them by hand.

## Maintaining the DSL

The DSL only sends events, and the game logic decides what they do. Keep it that way. A helper may read the state to decide *which* events to send: `skip_rest`, for example, only skips for players who haven't decided yet. A helper should not compute results that the game is responsible for, otherwise the scenarios end up testing the helper. Mark new helpers `#[track_caller]` so that failures point at the scenario.

When the game changes:

- **New event**: no DSL change is required.
  - `g.apply(Event::New { .. })` already works, and the timeline picks up the event's name automatically.
  - If the acting player isn't in a `player` or `giver` field, add that field to `actor` in `src/timeline.rs`.
  - Add a helper only if scenarios will use it often or it is part of a multi-step flow. Action-phase actions go on `Turn` in `src/phases.rs`, next to `frontier` and `relic`.
  - Cover the event in a scenario, including `expect_rejected` for its validation rules.
- **Changed or removed event**: the compiler points out which helpers and scenarios need updating. Also see the note on stored events under [Good to know](#good-to-know).
- **Changed game flow**: these helpers encode how the game flows, and they break with a rejected event when the flow changes.

  | Helper | Assumes |
  | --- | --- |
  | `auto_setup` (`game.rs`) | Winnu, Argent Flight and Keleres are the factions that need setup choices |
  | `Turn::play_card` | Technology, Politics and Imperial are the cards that require a primary |
  | `play_out_action_phase` | A player is done once they've used their strategy cards and passed |
  | `StatusPhase::reveal` | Each player makes one public and one secret decision before the reveal |
  | `AgendaVote::vote_all` | Nekro is the only player who can't vote |

- **New `GameState` field**: every state snapshot changes.
  - Check that the diff only adds the new field, then accept all the snapshots at once.
  - If the field matters to the flow of the game, as the speaker and turn order do, consider adding it to `changes` in `src/timeline.rs`. Doing so changes every timeline snapshot too.
- **New expansion**:
  - Add a builder method like `.thunders_edge()`.
  - Enable the expansion in `milty_data_on_map` (`src/milty.rs`), which turns on all expansions.
- **New component enum** used in a `BTreeSet` or `BTreeMap`: add it to `libs/game_data/tests/ord_consistency.rs`.
- **New types that scenarios use often**: add them to `prelude` in `src/lib.rs`.

## Good to know

- **Not covered: compatibility with stored events.** The JSON round trip only checks the current event format against itself. Renaming an `Event` variant or field breaks games that are already in the DB, and so does adding a field without `#[serde(default)]`. No test here catches either.
- **Determinism failures** print the first path that differs, e.g. `at: .players.Alice.technologies[0]`. To fix one, use a `BTreeSet`/`BTreeMap` or sort before anything depends on order. Types in BTree collections need an `Ord` that agrees with `Eq`. `libs/game_data/tests/ord_consistency.rs` checks this for the component enums, so add any new ones there.
- **Time**:
  - Rejected events don't move the clock, and undo doesn't rewind it.
  - The game rejects timestamps that go backwards.
  - To test time tracking, use `.clock_step(TimeDelta::zero())` and call `g.advance(..)` explicitly (see `time.rs`).
- **Failures point at the scenario line**, because the helpers are `#[track_caller]`. When an event is rejected unexpectedly, the panic shows its index, the error, and the phase, round, current player and turn order.
- **Playing a scenario in the UI**: an ignored test writes every scenario into the DB at `DATABASE_URL`.
  - Each game id is derived from the scenario name, so seeding again resets the games, and renaming a scenario gives it a new game.
  - Timestamps are shifted so the last event is now.
  - The test prints the game ids.

  ```bash
  cargo test -p ti_helper_system_tests --features seed-db --test scenarios seed_database -- --ignored --nocapture
  ```
