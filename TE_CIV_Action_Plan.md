# Thunder's Edge and Codex IV: action plan

What's left for Thunder's Edge (TE) and Codex IV (CIV), with a concrete plan per task. Written 2026-10-25 against branch `thunders-edge` (`6ad0dff`). Data errors are listed separately in [`mismatches.md`](./mismatches.md); this document only refers to them.

---

## 0. Ground rules: what we track and what we don't

This app is used **alongside** the physical game. Every feature has to be faster than doing the thing at the table, or it shouldn't exist.

**Tracked today:**
- **Game structure:** players, factions, strategy cards, turn order, phases and rounds.
- **Score:** every VP source (public and secret objectives, custodians, imperial, Support for the Throne, agenda VPs, relic VPs, manual adjustments).
- **Board-adjacent state:** planet ownership and attachments, technologies, relics, breakthroughs, laws, agenda votes, leaders (to play actions), the TE expedition, time per player, and the map (milty imports only).

**Not tracked, by design:**
- **Pieces on the board:** units and fleets, command tokens.
- **Economy:** trade goods and commodities, exhausted/readied state.
- **Cards in hand:** action cards, promissory notes, unscored secret objectives, relic fragments.
- **Positional rules:** movement, adjacency.
- **Randomness:** dice. The backend must stay deterministic for undo and replay, so players roll at the table and enter the result.

**Lowest priority:** game end (reaching the VP target, winners, tiebreaks, elimination). Players handle it at the table.

**How each rule effect is classified:**

| Verdict | Meaning |
|---|---|
| **Track** | It changes VP, planets, techs, relics, breakthroughs, leaders, turn/phase flow, agenda voting, or objective reveal/scoring, so model it as state, events and validation. |
| **Assist** | It touches tracked state only occasionally. Give a contextual button or shortcut that reuses an existing event (`AddTechToPlayer`, `SetPlanetOwner`, `TakeAnotherTurn` …) instead of building a new flow. |
| **Info** | It only affects untracked things. Show the text (info modal or reminder) and nothing else. |
| 👺 | Out of scope. |

**Design guidelines (from the existing code):**
- **Derive before you store.** For example, Styx VP = "controls Styx", the same way `HolyPlanetOfIxth` is scored in `state/score.rs`.
- **Optional state needs serde defaults.** New `GameSettings`/`GameState` fields need `#[serde(default)]` so stored games still deserialize (see `codex_4` in `common/game_settings.rs`).
- **Data on events, not new phases.** Multi-step actions should carry their data on the commit event, like `RelicAction` and `FrontierCardAction` in `actions/event.rs`, rather than adding phases.
- **Admin escape hatches.** Rare effects can stay manual: manual VPs, `SetPlanetOwner`, `AddTechToPlayer`/`RemoveTechFromPlayer`, `SetBreakthrough`.
- **Tests.** Every tracked feature gets a scenario in `libs/system_tests/tests/scenarios/` (determinism, replay and undo are checked automatically).

**Priority and effort:** P1 = do next (cheap and/or fixes wrong behaviour) · P2 = should · P3 = nice to have. Effort is S (hours), M (a day or two) or L (more).

---

## 1. Current state

| Area | Status |
|---|---|
| TE expansion toggle, milty import of TE tiles and factions | ✅ |
| TE expedition (commit at end of turn, passing, completion/placement, admin page) | ✅ (`state/expedition.rs`, `thunders_edge.rs` scenarios) |
| Breakthrough assignment (auto-unlock on first expedition slot, with the Thunder's Edge planet or with The Quantumcore, admin dropdown) | ✅. Only the I.I.H.Q. and Stellar Genesis on-gain effects are applied. Admin events don't give breakthroughs as a side effect, only when that is their point (giving The Quantumcore, the dropdown) |
| Firmament → Obsidian flip (Puppets of the Blade, planets, The Sowing/Reaping) | ✅ |
| TE factions, planets, space stations (as planet cards), leaders, relics (data) | ✅ (data bugs: see mismatches) |
| TE leader actions Kan Kip Rel, Ahk Ravin, Sharsiss, Sharsiss Hollow | ✅ (as "no-data" actions) |
| CIV toggle, ΩΩ techs (Magen Defense Grid, X-89) | ✅ |
| CIV relics: Circlet of the Void (no effect), Neuraloop (status phase), Book of Latvinia (research up to 2 techs when gained, VP or speaker) | ✅ (`codex.rs` scenarios) |
| Galactic events (4 CIV + 16 TE = 20) | ❌ nothing |
| Liberation of Ordinian | ❌ nothing |
| TE action cards (14 new, Ω versions of 8 Codex I cards) and the tracked effects of Brilliance, Mercenary Contract, Overrule, Strategize, Crisis and Puppets on a String. Codex IV has no action cards | ✅ (`thunders_edge_action_cards`, `thunders_edge_overrule_and_strategize`, `thunders_edge_crisis_and_puppets_on_a_string` scenarios) |
| Breakthrough effects, legendary planet abilities, Fracture, TE strategy cards, TE leader updates | ❌ |

---

## 2. Cross-cutting enablers (do these first)

Several TE, CIV and galactic-event features need the same building blocks. Building them once keeps every later task small.

| # | Enabler | Why | Plan | P / Effort |
|---|---|---|---|---|
| E1 | **Fix data blockers** | Fracture planets, Thunder's Edge, Avernus and Custodia Vigilia can't be taken in a tactical action. Styx has wrong values. The Firmament has 4 starting planets. | See mismatches §1. In `TacticalActionTakePlanet` (`game_event_handler.rs:404-486`), use `planet.info().expansion` for the expansion check instead of `System::for_planet(..)?.expansion`. Make `planet_system` optional for "token planets": Mirage (already), Thunder's Edge (use `te_expedition.completed.placed_in`), Avernus, Custodia Vigilia. Fix tiles 125–127. Only treat 96A as the Firmament's starting tile. | P1 / S |
| E2 | **`GameState::gain_breakthrough(player)`** | The "unlock breakthrough if none" logic is copy-pasted in 4 places (`ClaimThundersEdgeExpeditionSlot`, `CommitToThundersEdgeExpedition`, `CompleteThundersEdgeExpedition`, `SetThundersEdgeCompletion`), and many new sources are coming (Thunder's Edge planet, Quantumcore, Brilliance, Rapid Mobilization, Crimson setup). | One helper that sets the breakthrough and applies its **on-gain effects**: I.I.H.Q. → Custodia Vigilia (Ω); Stellar Genesis → Avernus; Al'Raith → Fracture in play. It also returns a flag so the UI can show the **"roll for the Fracture"** prompt (see TE-B). 🚱 `GameState::gain_breakthrough` replaces the 4 copies and is used by Brilliance. ✅ It applies the I.I.H.Q. and Stellar Genesis on-gain effects, and so does `SetBreakthrough` when a player goes from no breakthrough to one. ✅ Al'Raith puts the Fracture in play (TE-B2). ✅ The roll for the Fracture is recorded in `GameState::fracture_roll` instead of a returned flag, and it must be answered before anything else happens (TE-B3). | P1 / S |
| E3 | **`on_planet_gained(player, planet, previous_owner)` hook** | Thunder's Edge planet (gain breakthrough), Fracture planets (gain relic), Dangerous Wilds, Minor Factions alliance reminder, Monuments. | Called from `TacticalActionTakePlanet`, `SetPlanetOwner` and the new planet-granting effects. It returns "pending prompts" for the UI. Keep it pure state + flags; no new phases. 🚱 `GameState::set_planet_owner` is the hook: tactical takes, `SetPlanetOwner`, the expedition and the breakthrough on-gain effects go through it, and taking Thunder's Edge in a tactical action or winning the expedition unlocks the breakthrough (`SetPlanetOwner`, `SetThundersEdgeCompletion` and `ClaimThundersEdgeExpeditionSlot` don't). No effect needs a prompt yet. | P1 / S |
| E4 | **`SetRelicOwner { relic, player: Option<PlayerId> }` (any phase)** | Today a relic can only be gained through `GainRelicAction`, which is an *action* (it asserts the Action phase and ends the turn). Many effects hand out relics outside that action: Fracture planets, Vaults of the Heir, Black Market Dealings, Cultural Exchange (The Obsidian), Liberation of Ordinian setup, transactions (Silver Flame). | Admin-style event like `SetShardForTheThroneOwner`, reusing that event's Shard of the Throne bookkeeping. Gaining The Quantumcore calls E2. ✅ `GameState::set_relic_owner` moves a relic to a player or to nobody and keeps the Shard of the Throne point in sync. `SetRelicOwner`, `SetShardForTheThroneOwner` and `GainRelicAction` go through it. | P1 / S |
| E5 | **Victory target and game end** | The configured target is ignored (`state/score.rs:63`), but nothing reads it, and game end is handled at the table. Revisit it with the galactic events (Civilized Society), where it may matter more. | If ever wanted: use `GameSettings::max_points` and show the target. | P3 (lowest) / S |
| E6 | **Data on leader actions** | `LeaderActionBegin` jumps straight to `EndActionTurn` without data (TODO list at `game_event_handler.rs:851-867`). Ta Zern, and later others, need data. | Add `LeaderAction` data, mirroring `RelicAction` + `action_matches_relic`, and use the existing (unused) `Phase::LeaderAction` + `LeaderActionCommit { data }`. Leaders without data keep the one-click path. Also track **purged heroes** (`purged_leaders: BTreeSet<Leader>`) so used heroes disappear from `available_leaders`. | P2 / M |
| E7 | **Galactic event framework** | Prerequisite for §5. | See §5.0. | P2 / M |
| E8 | **A leader-faction override** | Cultural Exchange Program, and Creuss's "Unknown" leaders in Liberation of Ordinian. | `leader_faction: BTreeMap<PlayerId, Faction>` in `GameState`, read by `update_available_leaders` (`state/game_state.rs:649`). | P2 / S |

---

## 3. Thunder's Edge

### TE-A Breakthroughs

Synergy needs **no logic**: the app never validates tech prerequisites or objective conditions. Showing the synergy pair is enough.

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| A1 Crimson Rebellion starts with its breakthrough | Track | ✅ `NewPlayer::setup` gives Crimson Rebellion its breakthrough (Resonance Generator has no on-gain effect). The "roll for the Fracture" prompt during setup (the wiki says to roll during "Gather Starting Components") is TE-B3. ✅ `StartGame` gives it its roll. | P1 / S |
| A2 Breakthrough info | Info | Add `Info::Breakthrough` to `packages/ui/src/data/info_context.rs`. Show the description and synergy colours on the TE page (`thunders_edge_view_mode/breakthrough.rs`) and in the player sidebar. | P2 / S |
| A3 Generic breakthrough action button | Assist | Add a "Breakthrough" entry to the action phase component list (`action_phase.rs` `ComponentSelectRow`) for the current player's breakthrough when `kind == Action`, via event `PlayBreakthroughAction { player, data: Option<BreakthroughAction> }`. Without data it goes straight to `EndActionTurn`, like leaders. Fix Deorbit Barrage's kind (mismatches §8). Covers Psychospore, Deorbit Barrage and Resonance Generator. | P2 / S |
| A4 I.I.H.Q. Modernization (Keleres) | Track | ✅ Through E2: gain the `CustodiaVigiliaOmega` planet (2/2). ✅ Executive Order replaces the I.I.H.Q. *tech* under TE (F1). Keep the neighbours clause as info. | P1 / S |
| A5 Stellar Genesis (Muaat) | Track | ✅ Through E2: gain `Avernus` (token planet, see E1), which other players can then take. Moving the token along with war suns is 👺. | P1 / S |
| A6 Vaults of the Heir (Mahact) | Track | `BreakthroughAction::VaultsOfTheHeir { purged_tech, relic }`: remove the tech, give the relic (reuse the E4 logic, including the "relic already owned" check). | P2 / S |
| A7 Visionaria Select (Deepwrought) | Track | `BreakthroughAction::VisionariaSelect { researched: BTreeMap<PlayerId, Technology> }`. Each listed player gains their tech (non-faction, non-unit-upgrade only) and Deepwrought gains every listed tech it doesn't already own. Trade goods and promissory notes stay at the table. | P2 / M |
| A8 Al'Raith Ix Ianovar (Cabal) | Track (if TE-B2 is built) | Through E2: set `fracture_in_play = true` (no roll). Ingress token moves are 👺. ✅ `apply_breakthrough_gained` puts the Fracture in play, also through `SetBreakthrough`. | P3 / S |
| A9 N'orr Supremacy | Info | "Research a unit upgrade after winning a combat" is already possible from the tech page. Nothing to build. | — |
| A10 Everything else (Void Tether, Wing Transfer, Gravleash, Data Skimmer, Yin Ascendant, The Icon, Corsair/Eidolon/Tables Grace/Absolute Synergy …) | Info / 👺 | Units, movement, action cards, alliances. Covered by A2. | — |

### TE-B The Fracture

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| B1 Data | Track | Tiles 125–127 and Styx 4/0 (E1). Optionally model the egress systems. | P1 / S |
| B2 "Fracture in play" flag | Track (light) | `fracture_in_play: bool` in `GameState` plus `SetFractureInPlay { in_play }` (any phase, admin). Set automatically by A8, Plane Splitter (tech gain; flip/gate), The Silver Flame (TE-D2) and Rapid Mobilization (§5). ✅ Until it's set, `planet_location` keeps Cocytus, Lethe, Phlegethon and Styx off the board, so tactical actions and Mercenary Contract reject them. `SetFractureInPlay` is on the Thunder's Edge page, and A8 sets it; Plane Splitter, The Silver Flame and Rapid Mobilization don't yet. The admin `SetPlanetOwner` isn't restricted. | P2 / S |
| B3 "Roll for the Fracture" prompt | Assist | When E2 reports a new breakthrough and the Fracture isn't in play, show "Roll 1 die. 1 or 10 → the Fracture enters play" with buttons **[It entered play]** (sends B2) and **[No]** (dismiss, UI only). Players forget this roll all the time, so the reminder is worth having. ✅ The roll is forced: `GameState::fracture_roll` holds the player who gained their breakthrough while the Fracture wasn't in play (the Crimson Rebellion from `StartGame`), and every other event is rejected until they answer with `RollForTheFracture { player, entered_play }`. Meanwhile the game view shows the roll instead of the phase, with the buttons for that player, and the Thunder's Edge page can't toggle the Fracture. Undo still works. Admin events only give a roll when gaining the breakthrough is their point (`SetBreakthrough`, giving The Quantumcore). | P2 / S |
| B4 Relic for Fracture planets | Assist | Through E3: a Fracture planet with no previous owner (it came from the planet deck) → "Gain 1 relic" prompt with a relic picker → E4. | P2 / S |
| B5 Hide Fracture planets from pickers until in play | Assist | UI filter in the planet dropdowns and the unclaimed planets table. 🚱 Through B2, the tactical action and action card pickers hide them; the unclaimed planets table and the admin planet dropdown still show them. | P3 / S |
| B6 Ingress/egress tokens, adjacency, neutral units, map placement | 👺 | Positional and unit state. | — |

### TE-C Legendary planets

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| C1 **Styx**: "gain 1 VP when you gain this card, lose it when you lose it" | Track | ✅ Derived: +1 VP for whoever controls Styx in `update_player_points`, plus a "Legendary Planets" section in the score table (Thunder's Edge only). No events needed. | P1 / S |
| C2 **Thunder's Edge** "Jupiter Brain" | Track + Assist | (a) ✅ Gaining the planet → E2 (breakthrough if missing), in a tactical action or by winning the expedition. The admin `SetPlanetOwner` and `SetThundersEdgeCompletion` don't give it. (b) ✅ "Exhaust at end of turn: perform 1 action": in the `EndActionTurn` view (`end_action_phase.rs`), show a **"Jupiter Brain: take 1 more action"** button to the owner that sends the existing `TakeAnotherTurn`. No new event, no exhaust tracking (players police once-per-round themselves). The action is part of the same turn: `TakeAnotherTurn` doesn't end the turn, so the end-of-turn options are offered again rather than re-triggered. A passed player can't use it (LRR: passed players can't perform actions). This replaces the "use Take Another Turn" workaround from the roadmap. | P1 / S |
| C3 **Faunus** "Maxis Central Control" (when you pass, gain a non-home, non-legendary planet with no units or attachments) | Track | `PassAction` already routes to `EndActionTurn` while the expedition is open (`game_event_handler.rs:1232-1238`); extend that condition to "owns Faunus". In that view, show a planet picker (unowned planets plus other players' planets that have no attachments, non-home, non-legendary) → event `UseFaunus { player, planet }` (validates, moves ownership through E3). | P2 / S |
| C4 **Avernus** "The Nucleus" (ACTION: Star Forge without a token) | Info | Nothing tracked. At most a "legendary planet action" entry that just ends the turn. | P3 / S |
| C5 **Mecatol Rex Ω** "The Galactic Council" (swap a secret) | Info | Unscored secrets aren't tracked. `MecatolRexOmega` is already swapped in when TE is on (`game_options.rs:93-99`) and `is_mecatol_rex()` covers both versions. | — |
| C6 Ordinian, Emelpar, Garbozia, Industrex, Tempesta | Info | Action cards, units, readying, movement. | — |
| C7 Legendary ability text | Info | Add `legendary_ability: Option<(&str, &str)>` to `PlanetInfo` (or a separate lookup) and an info button in the planet view. This also covers the PoK legendaries (Primor, Hope's End, Mallice, Mirage). | P3 / S |

### TE-D Relics

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| D1 **The Quantumcore**: "When you gain this card, gain your breakthrough" | Track | `GainRelicAction` and E4 call E2. "Synergy for all types" is info only. ✅ `GameState::set_relic_owner` calls E2 when The Quantumcore gets an owner, so both `GainRelicAction` and `SetRelicOwner` unlock the breakthrough. Losing the relic doesn't remove it. | P1 / S |
| D2 **The Silver Flame**: ACTION, roll; a 10 gives +1 VP, otherwise purge your home system and you can't score public objectives; the Fracture enters play | Track | `RelicAction::SilverFlame { rolled_ten: bool }`. True → `score.silver_flame = Some(player)` (+1 VP, score row). False → remove the home planets from the player and add them to the destroyed-planets list (renders on the map like the Stellar Converter); add the player to `cannot_score_public_objectives`, which `ScorePublicObjective`, `ScoreExtraPublicObjective` and the Imperial primary check. Both outcomes → B2. "Can be exchanged in a transaction" → E4. | P2 / M |
| D3 **The Triad**: a planet-like card worth 3 + the number of fragment types you own | Info / Assist | Fragments aren't tracked. Show a "The Triad (3+X)" badge in the player's resources (`players_sidebar/parts/player_resources.rs`) without adding it to the totals. | P3 / S |
| D4 Dynamis Core / Nano-Forge TE wording | Data | mismatches §5 | P2 / S |
| D5 Metali Void Armaments/Shielding, Lightrail Ordnance, Heart of Ixth | Info | ✅ already shown. | — |

### TE-E Leaders

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| E-1 TE leader card updates (Z'eu ΩΩ, Xxekir Grom ΩΩ, Yin ΩΩ ×3, Odlynn Myrr, Kuuasi Aun Jalatai, Jol-Nar commander renamed to Agnlan Oln) | Data | ✅ The variants are in, chained after the older versions in `Leader`'s `replacements!`. Xxekir Grom ΩΩ is an ACTION hero ("place up to 4 PDS/mechs"), so it shows up as playable. | P1 / S |
| E-2 **Director Nel** (Ral Nel hero, "after the last player passes: no longer passed, purge") | Track | When the last player passes, `advance_turn` (`state/game_state.rs:400-428`) switches straight to Status and clears `passed_players`. Add `PlayDirectorNel { player }`, valid in Status while `status_phase_state` is still untouched, for a Ral Nel player with the hero unlocked and not purged. It sets `phase = Action`, `passed_players = everyone else`, `current_player = player`, clears `status_phase_state`, and purges the hero (E6). The status phase starts again normally when they pass. The Status view shows the button only when it applies. | P2 / M |
| E-3 **Ta Zern** (Deepwrought hero, `TaZernTE`): purge a non-unit-upgrade tech from everyone; those who lost it research another | Track | `LeaderAction::TaZern { tech, replacements: BTreeMap<PlayerId, Technology> }` (E6). Remove `tech` from every player, add the replacements, and record `purged_techs: BTreeSet<Technology>` so pickers hide the tech for the rest of the game. | P2 / M |
| E-4 Sharsiss (plots) | Track (if F2 is built) | `LeaderAction::Sharsiss { add_plot / add_token }`, which edits Firmament plots. Without F2 it stays a no-data action (as today). | P3 / S |
| E-5 Other TE leaders (agents and commanders) | Info | Commanders are passive; agents are mostly reactions. Already shown through `Info::Leader`. | — |

### TE-F Factions

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| F1 Keleres TE update | Data | ✅ Executive Order and Agency Supply Network Ω replace the Codex III techs, Custodia Vigilia Ω (2/2) replaces Custodia Vigilia, and the hero texts are in (E-1). Like the faction, they need Codex III. I.I.H.Q. via breakthrough is A4. | P1 / S |
| F2 **Firmament "Plots within Plots"** | Track | Scoring another player's secret gives **no VP**; instead a plot card with that player's token goes into play. Add `plots: Vec<Plot { secret: SecretObjective, puppeted: BTreeSet<PlayerId> }>` to `FirmamentState` (`state/faction_state.rs`) and an event `AddPlot { player, secret, puppeted }`, offered as "Score as plot" in the secret objectives view. It enables: validating Puppets of the Blade (needs ≥1 plot), Captain Aroz unlock display, the puppeted-player list for The Reaping, and Sharsiss (E-4). Plots must not add VP. | P2 / M |
| F3 Deepwrought "Radical Advancement" (status phase start: swap a non-unit-upgrade tech for a same-colour tech with exactly 1 more prerequisite) | Assist | In the status phase view, if the player owns it, add a "swap tech" form (remove + add, validated with `TechInfo.requirements`). Event `SwapTech { player, remove, add }`, which can be reused for Divert Funding-like effects and Rin. | P3 / S |
| F4 Deepwrought ocean cards (1/1 planet-like cards) | Assist | Optional "ocean cards: N" counter for resource/influence totals. | P3 / S |
| F5 Last Bastion: Revelation (a space station) doesn't count as a planet for objectives or home-system control | Info | Planet counts in the UI could exclude space stations (see G2). | P3 / S |
| F6 Crimson Rebellion | — | A1 plus data. The Sorrow and breaches are 👺. | — |

### TE-G Space stations

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| G1 Can't be elected in "elect planet" agendas | Track | Filter `OrbitalBodyType::SpaceStation` out of the planet options in `state/agenda.rs:~170` (trait-based elects already exclude them because stations have no traits) and in `AgendaElect::parse`. | P2 / S |
| G2 Don't count as planets for objectives | Info | Exclude them from per-trait and planet-count displays (`planet_view_mode/player_planets_grid.rs`). | P3 / S |
| G3 Commodity +1, remote transactions, exhaust for trade goods | 👺 | Economy isn't tracked. | — |

### TE-H Strategy cards ✅

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| H1 Construction ΩΩ and Warfare Ω images | Info | ✅ `StrategyCard::version` picks the newest `StrategyCardVersion` whose expansion is enabled; `get_strategy_card_image` (`info_context.rs`) matches on it. | P1 / S |
| H2 Construction secondary hint | Info | ✅ `get_cost_warning` (`generic_strategy_card.rs`) takes the `StrategyCardVersion`: "place token in system" becomes "pay 1 token" for Construction ΩΩ. | P1 / S |
| H3 The roadmap TODO "always display the correct variant" | — | ✅ Done by H1/H2. | — |

### TE-I Action cards ✅

✅ The 14 cards and the Ω versions of the 8 Codex I cards are in (mismatches §2). The Ω versions replace the Codex I cards when Thunder's Edge is enabled (`ActionCard::is_enabled_in`). Codex IV has no action cards. Most of the new cards are **Info**. These touch tracked state:

| Card | Verdict | Status | P / Effort |
|---|---|---|---|
| **Brilliance** (choose a player to gain their breakthrough) | Track | ✅ `ActionCardAction::Brilliance { player: Option<PlayerId> }` (None = the "ready a planet" option). The player must not have their breakthrough yet; it's unlocked with `GameState::gain_breakthrough`, which applies the on-gain effects (E2). | P2 / S |
| **Overrule** (perform the primary of a readied or unchosen strategy card) / **Strategize** (perform the secondary) | Track | ✅ `ActionCardAction::Overrule { card, primary: Option<StrategicPrimaryAction> }` / `Strategize { card, secondary: StrategicSecondaryAction }`. Both reuse the strategy card handlers (`perform_strategic_primary/secondary`), so only Technology, Politics and Imperial have effects; `primary` is required exactly for those cards (`StrategyCard::has_tracked_primary`). The card must not be exhausted (not in `spent_strategy_cards`), and it stays readied. The UI reuses the strategy card primary views. | P2 / M |
| **Crisis** (skip the next player's turn) | Track | ✅ `PlayCrisis { player }` (the player who plays the card), sent from the End of Turn screen: it ends the turn and skips the next player's turn. It needs at least 2 players who haven't passed. In games with Crisis or Puppets on a String, passing always shows the End of Turn screen, so the cards can also be played after a pass. | P3 / S |
| **Puppets on a String** (a passed player performs 1 action at the end of any player's turn) | Track | ✅ `PlayPuppetsOnAString { player }`, sent from the End of Turn screen, ends the turn, makes the passed player current and stores the next player in `GameState::interrupted_turn`; `advance_turn` hands the turn to them after that action. The passed player can't pass again or take another turn. After the last player passes there's no next player, so the action phase ends after that action. | P3 / M |
| **Mercenary Contract** (the planet's owner returns the planet card) | Assist | ✅ `ActionCardAction::MercenaryContract { planet: Option<Planet> }`: a non-home planet on the board owned by another player loses its owner (attachments go with the card, like `SetPlanetOwner` to nobody). None = the planet wasn't owned by another player. | P3 / S |
| Black Market Dealings | Assist | Relics can change hands → E4 (admin). | — |
| Crash Landing, Exchange Program, Extreme Duress, Lie in Wait, Pirate Contract/Fleet, Rescue | Info | ✅ Data only. | — |
| Hack Election Ω ("you vote last") | Info | ✅ Data only; voting order isn't enforced. | — |

### TE-J Map

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| J1 Show Thunder's Edge where it was placed | Info | ✅ `build_layout` draws `TiToken::ThundersEdge` and its owner label in the system from `GameState::planet_location`. | P2 / S |
| J2 Destroyed and purged planets (Silver Flame, Conventions of War Abandoned) | Info | They already render through `stellar_converter_destroyed_planets` + `TiToken::DestroyedPlanet`. Generalise the field name (keep a serde alias). | P2 / S |
| J3 Fracture tiles, Avernus token, breaches, coexistence | 👺 | Positions aren't tracked. | — |

### TE-K Other

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| K1 Codex content in the TE box | Decision | TE ships "74 cards from the Twilight Codices" (Codex I–IV components). Suggestion: in the new-game form, ticking TE **pre-ticks** Codex I–IV (still editable). No backend change. | P2 / S |
| K2 Alliance cards (31) | Info | Needed as text by Minor Factions, Zealous Orthodoxy, Mercenaries for Hire and Yin Ascendant. Tracking ownership is 👺. If the texts are ever wanted, add an `alliance_ability` per faction. | P3 / M |
| K3 Twilight's Fall | 👺 | A separate game mode with its own cards and strategy cards. | — |

---

## 4. Codex IV

| Task | Verdict | Plan | P / Effort |
|---|---|---|---|
| CIV-1 Book of Latvinia: "When you gain this card, research up to 2 technologies that have no prerequisites" | Assist | After `GainRelicAction`/E4 for this relic, prompt with a tech picker filtered to `requirements.is_empty()`, sending `AddTechToPlayer` ×0–2. Removes the "techs are added manually" caveat. ✅ `set_relic_owner` records the new owner in `GameState::book_of_latvinia_research`, and `ResearchBookOfLatviniaTechs` researches 0–2 techs with no prerequisites (base techs or the player's own faction techs) and clears it (purging the Book clears it too). The prompt is shown in the end-of-turn view and on the Relic Owners card. | P2 / S |
| CIV-2 Neuraloop outside the status phase | Assist | `PlayNeuraloop` only works right after the status phase reveal. Also allow it after `RevealExtraPublicObjective` (Incentive Program, Classified Document Leaks, admin reveals). | P3 / S |
| CIV-3 Circlet of the Void action | Info | "Explore a frontier token" could hand over to the frontier-card flow (Enigmatic Device and Mirage are tracked). Optional. | P3 / S |
| CIV-4 Codex 4.5 ΩΩ techs | ✅ | Tied to `codex_4`. Fine. | — |
| CIV-5 Galactic events (Minor Factions, Total War, Age of Commerce, Age of Exploration) | — | See §5 | — |
| CIV-6 Liberation of Ordinian | — | See §6 | — |

---

## 5. Galactic events (all 20)

Codex IV introduced 4 events; TE adds 16 and reprints the 4 (the "20 Galactic Events" in the TE box). Sources: the wiki's *Galactic Events* page and the official Codex IV rules PDF. Each event lists its complexity (1–3) from the card.

### 5.0 Framework (do this once)

1. **Data:** `components/galactic_event.rs`
   - `enum GalacticEvent` (20 variants).
   - `info()` returns name, complexity, expansion (`CodexIV` | `ThundersEdge`), `requires_pok` (Minor Factions, Age of Exploration) and a description file in `galactic_event/description/` (same pattern as relics and breakthroughs).
2. **Settings:** `GameSettings.galactic_events: BTreeSet<GalacticEvent>` with `#[serde(default)]`.
   - Availability: the CIV events need `codex_4 || thunders_edge`; the TE events need `thunders_edge`.
   - Validate in `SetSettings`.
3. **Milty games:** add a creation-phase event `SetGalacticEvents { events }`, because the milty import builds its settings from the draft. It works for both flows.
4. **New-game UI** (`packages/ui/src/views/new_game.rs` + `packages/api/src/requests/new_game.rs`):
   - A multi-select with complexity badges and a **"Pick random"** button. The randomness lives in the client, so the event stays deterministic.
5. **In-game display:**
   - Add `Info::GalacticEvent` (info modal).
   - Add an "Active galactic events" strip in the game header or sidebar.
6. **Scoring:** event-specific VP rows in `update_player_points` (Total War, Zealous Orthodoxy, Monuments), shown in the score table.
7. **Tests:** `libs/system_tests/tests/scenarios/galactic_events.rs`, with a fixture builder `.galactic_events(&[..])`.

### 5.1 Overview

| Event | Origin | Cx | Verdict | What the app does | P / Effort |
|---|---|---|---|---|---|
| Minor Factions | CIV (PoK) | 2 | Assist | Pick minor factions; their home planets gain all 3 traits; alliance reminder | P2 / M |
| Total War | CIV | 3 | Track | "Discard 10 commodities → 1 VP" action | P2 / S |
| Age of Commerce | CIV | 1 | Assist | "Share technology" shortcut | P3 / S |
| Age of Exploration | CIV (PoK) | 2 | Info | — | — |
| Stellar Atomics | TE | 2 | Track | One-shot action; spent players can't vote | P1 / S |
| Age of Fighters | TE | 3 | Track | Everyone gains Fighter II at setup | P1 / S |
| Civilized Society | TE | 2 | Track | All objectives revealed at setup, unlimited scoring, delayed game end | P1 / M |
| Dangerous Wilds | TE | 1 | Assist | Free tech prompt when taking an unowned hazardous planet | P2 / S |
| Call of the Void | TE | 1 | Info | — | — |
| Hidden Agenda | TE | 2 | Assist | Hide individual votes until resolution | P3 / S |
| Wild, Wild Galaxy | TE | 3 | Info | — | — |
| Cultural Exchange Program | TE | 2 | Track | Players use another faction's leaders; Firmament gets The Obsidian | P2 / M |
| Cosmic Phenomenae | TE | 2 | Info | — | — |
| Advent of the War Sun | TE | 1 | Track | Everyone except Muaat gains War Sun at setup | P1 / S |
| Mercenaries for Hire | TE | 1 | Info | — | — |
| Rapid Mobilization | TE | 1 | Track | Setup: planets, breakthroughs, tech(s); the Fracture enters play | P2 / M |
| Weird Wormholes | TE | 3 | Info | — | — |
| Monuments to the Ages | TE | 2 | Track | Monuments + commodity counter → VP | P2 / M |
| Zealous Orthodoxy | TE | 1 | Track | First to 2 secrets gains +1 VP | P1 / S |
| Conventions of War Abandoned | TE | 3 | Assist | "X-89: purge planet" action | P2 / S |

### 5.2 Per-event plans

#### Minor Factions (CIV, complexity 2, needs PoK): Assist
> Setup: players get 1 fewer blue tile. Each player is dealt a non-played faction and places its home system in ring 2 with 3 neutral infantry. Controlling every planet in a minor faction system gives you that faction's alliance card. **Planets in minor faction systems have all three traits.**

What we track: **planet traits** matter wherever the app reads them:
- elect cultural/hazardous/industrial planet agendas (`state/agenda.rs:174`)
- attachment trait checks (`game_event_handler.rs:~520-531`)
- trait counts in the planet view and sidebar

Plan:
1. Add `GameSettings.minor_factions: Vec<Faction>`, chosen at creation from factions not in play (one per player).
2. Add `GameState::planet_traits(&Planet) -> Vec<PlanetTrait>`, which returns all three traits for minor-faction home planets. Replace the direct `planet.info().planet_traits` reads above with it.
3. Make sure minor-faction home planets show up in planet pickers and the unclaimed table (home planets of factions not in play shouldn't be filtered out).
4. Through E3, when a player controls every planet of a minor faction system, show the reminder "Take {faction}'s alliance card".
5. Alliance ownership itself: 👺.

#### Total War (CIV, complexity 3): Track (VP only)
> Destroying units puts commodities on your home planets. **ACTION: discard 10 commodities from your home system → gain 1 VP.**

Commodities aren't tracked, so only the VP matters.
- Add a "Total War: gain 1 VP" entry in the action phase component list.
- It sends `GalacticEventAction { player, action: TotalWarScore }`, which increments `score.galactic_event_points[player]` (a "Total War" score row) and ends the turn.
- Accepted: the app doesn't verify the 10 commodities (same trust model as everything else).

#### Age of Commerce (CIV, complexity 1): Assist
> Transactions don't need neighbours; no commodity cap; **non-faction technologies can be shared in transactions** (the receiver gains it, the giver keeps it).

- Add a "Share technology" mini-form on the tech page (giver → receiver, filtered to the giver's non-faction techs the receiver lacks) that sends `AddTechToPlayer`.
- Everything else: Info.

#### Age of Exploration (CIV, complexity 2, needs PoK): Info
> Relics need only 2 matching fragments; Naaz-Rokha fragments needn't match; ACTION: exhaust Dark Energy Tap to add a random unused tile at the galaxy edge.

- Fragments aren't tracked, and `GainRelicAction` doesn't validate them, so nothing needs to change.
- New tiles' planets can already be taken (any system tile).
- Adding the tile to a milty hex map is 👺.

#### Stellar Atomics (TE, complexity 2): Track
> Each player starts with a control token on the card. ACTION: discard it to destroy all ground forces and structures on any non-home planet. **Without a token you can't vote or play action cards in the agenda phase.**

Plan:
1. Add `stellar_atomics_used: BTreeSet<PlayerId>`.
2. Add a "Stellar Atomics" action (component list) → `GalacticEventAction { player, action: StellarAtomics { planet } }`. The planet is only recorded for history, since units aren't tracked. It ends the turn.
3. In the agenda phase, `CastAgendaVote` and `AddAgendaPlayerVote` must be abstentions for players in the set. The vote UI shows "cannot vote (Stellar Atomics)" and a reminder about action cards.
4. Undo/replay are naturally fine.

#### Age of Fighters (TE, complexity 3): Track (setup only)
> At setup everyone gains **Fighter II** (Naalu: **Hybrid Crystal Fighter II**). Fighters in the fleet pool get capacity 1; non-fighter ships are purged when destroyed.

- At `StartGame`, add `FighterII` to every player, or `HybridCrystalFighterII` for Naalu. Both techs exist in `tech.rs`.
- The rest: Info.

#### Civilized Society (TE, complexity 2): Track
> At setup **all public objectives are turned face up**. **No limit** on public objectives scored per status phase. The game doesn't end at the VP target; it ends **at the end of that round's status phase**, most VP wins, and ties are broken by influence of controlled planets + unspent trade goods.

Plan (keeping changes small):
1. **Setup.** Add `RevealAllPublicObjectives { stage_one: Vec<PublicObjective>, stage_two: Vec<PublicObjective> }` (5 + 5), used instead of `RevealInitialObjectives` when the event is active. The setup view shows 10 dropdowns. Stage-ordering checks (`expected_objectives_before_stage_two`) don't apply.
2. **Status phase.**
   - `CompleteStatusPhase` must not require `revealed_objective`, and the UI hides the reveal step.
   - For extra scoring, reuse `ScoreExtraPublicObjective` (already an any-phase event). The status view shows a multi-select under the normal single "public objective" slot, so `StatusPhaseState` doesn't have to change.
3. **End of game.** Through E5, evaluate winners at `CompleteStatusPhase` instead of on reaching the target. The tiebreak view shows each tied player's influence from controlled planets (derivable, including attachments). Trade goods are entered or decided at the table.
4. **Interactions.** Incentive Program, Classified Document Leaks and Neuraloop still add or replace objectives (no special handling beyond skipping the stage-order check).

#### Dangerous Wilds (TE, complexity 1): Assist
> Neutral infantry on hazardous planets equal to their resources; they replenish each round. **When you gain a hazardous planet from the planet deck, you may research 1 technology, ignoring prerequisites up to that planet's resources.**

- Through E3, when a hazardous planet (use `planet_traits`, which respects Minor Factions) is taken with **no previous owner**, the tactical-action view shows "Dangerous Wilds: research 1 tech (ignore up to N prerequisites)" with a tech picker → `AddTechToPlayer`.
- No new event. Prerequisites aren't validated anywhere, so N is just shown as a hint.

#### Call of the Void (TE, complexity 1): Info
> Command tokens and movement bonuses in the Fracture.

#### Hidden Agenda (TE, complexity 2): Assist (UI only)
> Only the speaker talks in the agenda phase; votes are written secretly, the speaker reveals only totals (and who voted for what only when needed). Argent's votes are public.

- When the event is active, the agenda views show **only per-outcome totals** until `ResolveAgenda`, except the speaker's own view and Argent's votes. The vote history keeps the full breakdown.
- Caveat: the full state is still sent to every client, so this is "don't show", not real secrecy. That's acceptable for a table helper.

#### Wild, Wild Galaxy (TE, complexity 3): Info
> Changes to 9 action cards (Direct Hit, Flank Speed, Maneuvering Jets, Morale Boost, Sabotage, Shields Holding, Skilled Retreat, War Machine, Diplomatic Pressure) plus Stellar Converter and Nova Seed working on any planet or system.

- Show the list in the event info.
- Optional: when the event is active, let the Stellar Converter flow skip its home/legendary/Mecatol checks (`game_event_handler.rs:1085-1107`). That's the only tracked interaction.

#### Cultural Exchange Program (TE, complexity 2): Track
> At the end of setup, shuffle the reference cards of the factions in play; each player draws one, **takes all of that faction's leaders** for the rest of the game and unlocks that commander. **The Firmament/Obsidian doesn't take part; it starts with The Obsidian relic.**

Plan:
1. Use E8 (`leader_faction` override), set through a setup event `SetupCulturalExchange { player, faction }` (per-player setup view, like `SetupTheTribunii`).
2. Validate: the faction is in play and each faction is drawn at most once. The rules don't say what happens if you draw your own faction (worth asking).
3. `update_available_leaders` uses the drawn faction. The Yssaril "all agents" logic still keys off the real faction.
4. At `StartGame`, give The Obsidian to the Firmament player (E4). The Obsidian relic's own effect (1 extra secret) isn't tracked.
5. Commander unlocking doesn't matter to the app (only ACTION leaders are listed).

#### Cosmic Phenomenae (TE, complexity 2): Info
> Changes to nebulae, asteroid fields, supernovas, gravity rifts and entropic scars.

#### Advent of the War Sun (TE, complexity 1): Track (setup only)
> Every player except Muaat gains the **War Sun** unit upgrade; Muaat purges its promissory note, unlocks its commander and places an extra War Sun.

- At `StartGame`, add `WarSun` to every non-Muaat player.
- The rest: Info.

#### Mercenaries for Hire (TE, complexity 1): Info
> A face-up mercenary deck of alliance cards; ACTION: spend 3 trade goods to take the top one.

- Alliances and trade goods aren't tracked.
- At most an action entry that just ends the turn (P3).

#### Rapid Mobilization (TE, complexity 1): Track (setup)
> After setup the Fracture enters play. Each player: (1) places 1 infantry on, gains and readies each planet adjacent to their home system (or the Creuss Gate / The Sorrow); (2) places a space dock and their flagship + 3 fighters; (3) **gains their breakthrough**; (4) **researches 1 technology** (Nomad: 2).

Plan:
1. Add a per-player setup step, `SetupRapidMobilization { player, planets: Vec<Planet>, techs: Vec<Technology> }`, validated: non-home planets not already owned, 1 tech (2 for Nomad).
2. Adjacency is checked only if a milty hex map exists (`map_data.milty_information`); otherwise trust the picker. A P3 extra: pre-select the adjacent planets from the hex map.
3. At `StartGame`, call E2 for everyone (no Fracture roll, since it's already in play) and set `fracture_in_play = true`.
4. Units: 👺.

#### Weird Wormholes (TE, complexity 3): Info
> Ships change type after moving through wormholes.

#### Monuments to the Ages (TE, complexity 2): Track (VP source)
> Instead of placing a structure on a non-home planet, you may spend 5 trade goods to place a **monument** (max 3 in play). **At the start of the status phase each monument gets 1 commodity; each monument is worth 1 VP per 3 commodities to whoever controls its planet.** The new controller may destroy it.

Plan:
1. State: `monuments: BTreeMap<Planet, u8>` (planet → commodities, at most 3 entries).
2. Events (any phase):
   - `BuildMonument { player, planet }`: the player owns the planet, it's non-home, and there are fewer than 3 monuments.
   - `DestroyMonument { planet }`: offered through E3 when a monument planet changes hands.
3. Commodities: when the status phase begins (`advance_turn`'s "everyone passed" branch, `state/game_state.rs:409-420`; the status phase isn't entered through `change_phase`), add +1 to every monument. Deterministic, so undo-safe.
4. Score: in `update_player_points`, `commodities / 3` VP goes to the current controller of each monument's planet. Score row "Monuments". Planet view badge "Monument (N)".

#### Zealous Orthodoxy (TE, complexity 1): Track (VP)
> **The first player to score 2 secret objectives gains 1 VP.** Their alliance card goes on this card (everyone gains the ability) and their alliance promissory note is purged.

Plan:
1. In `ScoreSecretObjective` and `ScoreExtraSecretObjective`: if `score.zealous_orthodoxy.is_none()` and the player now has 2 scored secrets, set it to that player (+1 VP row).
2. Unscoring doesn't remove it (the VP was gained; use manual points to correct mistakes).
3. Show "Everyone now has {faction}'s alliance ability".
4. Firmament plot scoring (F2) doesn't count.

#### Conventions of War Abandoned (TE, complexity 3): Assist
> BOMBARDMENT hits destroy 3 units. **ACTION (if you own X-89): exhaust it to purge a planet in a system containing your BOMBARDMENT unit, along with its attachments and legendary card.** You're eliminated if all your home planets are purged.

Plan:
1. Add an action "X-89: purge planet" for players owning any X-89 version: `GalacticEventAction { player, action: PurgePlanet { planet } }`.
2. It removes the planet from its owner and adds it to the destroyed-planets list (map rendering is already there). Unlike the Stellar Converter, home planets are allowed.
3. Elimination isn't supported by the app: show a warning only (P3, see roadmap "Handle game end").

---

## 6. Liberation of Ordinian (CIV scenario)

Source: Codex IV rules PDF, pages 17–18 ("Pax Magnifica"). The wiki page is only a stub. The Codex IV *cards* PDF is image-only, so the exact text of the FSS Orlando hero, the three "Unknown" leaders and the Stage II side of the objective must be read off the cards before implementing.

**Rules that matter to the app:**
- **Players and factions:** 6 players, PoK required. Fixed factions: Xxcha, Sol, Creuss, Naaz-Rokha, **Nekro (speaker)**, Nomad. Creuss must be **Red**.
- **Board:** a fixed board with **Ordinian in the centre; Mecatol Rex isn't used**; no custodians token. "Game effects that refer to Mecatol Rex instead refer to Ordinian."
- **Nekro setup:** starts with the **Ordinian planet card plus its legendary card** (the same "Hyperion VI" ability as TE's Ordinian), +1 fleet token, units. It uses the techs **???_NULL_REFERENCE_???** (text = Subatomic Splicer) and **???_ERROR_ERROR_???** (text = Nanomachines) instead of Valefar Assimilator X/Y.
- **Starting relics:** Naaz-Rokha starts with **Circlet of the Void**; Nomad starts with **Neuraloop**.
- **Creuss leaders:** Creuss replaces its leaders with three **"Unknown" leaders**. They're the Crimson Rebellion's leaders in disguise: *Forlorn Shadow* = Ahk Ravin's ACTION; *Wistful Soul* (unlock: "Resolve a combat with another player"); *Wayward Riftwalker*.
- **Sol/Xxcha alliance:** Sol and Xxcha are allied (Codex II alliance variant) and share the hero **F.S.S. Orlando – "Apollo Protocol"** (the PDF shows the unlock "The Nekro Virus has 5 victory points"; verify on the card). Both must agree to use it.
- **Objective:** **Liberate Ordinian** is placed face up next to Stage I: "Win a combat against the Nekro Virus" (ACTION PHASE, 1 VP, more than one player can score it). After the **first Stage II objective is revealed**, it flips to its Stage II side **"Control Ordinian"**. VPs already scored from Stage I are kept, but Stage I can't be scored any more.
- **Winning:** Sol/Xxcha win when **one has 12 VP and the other has 10**; any other player wins at **10 VP**.

**Plan** (P3 overall; it's a fixed 6-player scenario. Do steps 1–5 if someone actually plans to play it.)

| Step | Plan | Effort |
|---|---|---|
| LO-1 Setting | `GameSettings.scenario: Option<Scenario>` (`#[serde(default)]`), `Scenario::LiberationOfOrdinian`, offered when `codex_4 && prophecy_of_kings`. The new-game form pre-fills the 6 factions and Creuss = Red; `AddPlayer`/`CreationDone` validate 6 players and the faction set. | S |
| LO-2 Setup automation | At `CreationDone`/`StartGame`: speaker = Nekro (validate `SetupSpeaker` or set it automatically); Nekro gets `Planet::Ordinian` (the "Hyperion VI" legendary already exists from TE data); Naaz-Rokha gets Circlet of the Void and Nomad gets Neuraloop (E4); Nekro techs = Dacxive + the two scenario techs (new `Technology` variants, `Faction(NekroVirus)`, `Expansion::CodexIV`, enabled only in the scenario). | M |
| LO-3 "Mecatol Rex means Ordinian" | Add `GameState::is_centre_planet(&Planet)` (scenario aware) and use it instead of `Planet::is_mecatol_rex()` wherever rules apply: custodians grant (`game_event_handler.rs:481`; **disabled** in the scenario), Imperial "+1 VP while controlling Mecatol Rex", Stellar Converter exclusion, agenda elect "non-Mecatol" checks. | S |
| LO-4 Liberate/Control Ordinian objective | Add `PublicObjective::LiberateOrdinian` (Stage I, 1 VP, scored through `ScoreExtraPublicObjective` because it's an action-phase score) and `PublicObjective::ControlOrdinian` (Stage II; VP and timing to be read off the card), both scenario-only. Reveal LiberateOrdinian with the initial objectives without counting it towards the 5 Stage I (`expected_objectives_before_stage_two` +1). When the first Stage II is revealed, also reveal ControlOrdinian and reject further scoring of LiberateOrdinian (existing VPs stay). | M |
| LO-5 Victory rule | Info only: show the rule in the scenario info (the Sol+Xxcha pair needs 12 + 10, in either order; everyone else needs 10). The winner is decided at the table. | S |
| LO-6 Leaders | Creuss uses the "Unknown" leaders through E8, mapped to new leader variants (the names differ from the TE ones, so don't reuse `AhkRavin` …; only Forlorn Shadow is an ACTION). Add FSS Orlando as a hero available to both Sol and Xxcha with a custom unlock check (instead of "3 objectives"). | M |
| LO-7 Preset map | Optional: ship a fixed `HexMap` so the map view works without milty (Ordinian tile in the centre). Needs tile art for the Codex Ordinian system. | P3 / M (👺 unless asked for) |

---

## 7. Suggested order of work

1. **Bugs and data (P1, all S).**
   - ✅ mismatches §1 items 1–5 (Fracture tiles, agenda trait checks, Firmament planets, token planets, Styx).
   - ✅ TE-H1 strategy card images and the H2 hint.
   - ✅ TE-E-1 leader updates, TE-F1 Keleres.
   - ✅ The 14 action cards (data only) and the Ω wording.
2. **Core TE effects (P1/P2).**
   - ✅ E2/E3/E4 helpers.
   - ✅ A1 Crimson, A4 I.I.H.Q., A5 Stellar Genesis, C2 Jupiter Brain.
   - ✅ D1 Quantumcore.
   - ✅ CIV-1 Book of Latvinia.
   - ✅ C1 Styx.
3. **Galactic event framework plus the cheap tracked events (P1).**
   - Stellar Atomics, Age of Fighters, Advent of the War Sun, Zealous Orthodoxy, Total War, Civilized Society.
4. **TE actions (P2).**
   - A3/A6/A7 breakthrough actions, C3 Faunus, D2 Silver Flame.
   - ✅ TE-B2/B3 Fracture flag and roll prompt, A8 Al'Raith.
   - TE-B4 relic prompt for Fracture planets.
   - E6 leader data → Ta Zern and Director Nel.
   - ✅ Brilliance, Overrule, Strategize.
   - J1 Thunder's Edge on the map, G1 space stations in agendas.
5. **The heavier events (P2).**
   - Monuments, Cultural Exchange, Rapid Mobilization, Minor Factions, Dangerous Wilds, Conventions of War Abandoned.
   - F2 Firmament plots.
6. **Optional (P3).**
   - Hidden Agenda UI, ✅ Crisis/Puppets on a String, Radical Advancement, legendary ability texts, The Triad badge.
   - Liberation of Ordinian.
   - E5 victory target and game end (lowest priority).

## 8. Open questions

1. Should ticking Thunder's Edge pre-tick Codex I–IV in the new-game form (the TE box contains the codex cards)?
2. Are we fine with **no exhaust tracking** for legendary planets and breakthroughs (buttons are always offered, players police once-per-round)?
3. Is Liberation of Ordinian wanted at all, or should it be 👺?
4. Hidden Agenda: is "UI hides votes" good enough, given that clients receive the full state?
5. Cultural Exchange Program: what happens if a player draws their own faction's reference card? (The card doesn't say.)
