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
- **No compatibility code for stored games.** Don't add `#[serde(default)]`, `#[serde(alias)]` or other code only so that games stored before a change still load. Old games will be handled by a database migration later.
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
| Galactic events (4 CIV + 16 TE = 20) | Framework ✅ (data, `GameState::galactic_events`, the Creation picker, the info modal and the info box list). 4 need nothing, 8 of the 10 small ones are done and the 6 large ones are left (§5.1) |
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
| E4 | **`SetRelicOwner { relic, player: Option<PlayerId> }` (any phase)** | Today a relic can only be gained through `GainRelicAction`, which is an *action* (it asserts the Action phase and ends the turn). Many effects hand out relics outside that action: Fracture planets, Vaults of the Heir, Black Market Dealings, Cultural Exchange (The Obsidian), Liberation of Ordinian setup, transactions (Silver Flame). | Admin-style event like `SetShardForTheThroneOwner`, reusing that event's Shard of the Throne bookkeeping. Gaining The Quantumcore calls E2. ✅ `GameState::set_relic_owner` moves a relic to a player or to nobody and keeps the Shard of the Throne point in sync. `SetRelicOwner`, `SetShardForTheThroneOwner` and `GainRelicAction` go through it. `SetRelicOwner` is sent by the Relics view (one card per player with their relics, and a form to give one), which covers the effects that hand out relics until they get their own events (B4, A6, …). | P1 / S |
| E5 | **Victory target and game end** | The configured target is ignored (`state/score.rs:63`), but nothing reads it, and game end is handled at the table. Revisit it with the galactic events (Civilized Society), where it may matter more. | If ever wanted: use `GameSettings::max_points` and show the target. | P3 (lowest) / S |
| E6 | **Data on leader actions** | `LeaderActionBegin` jumps straight to `EndActionTurn` without data (TODO list at `game_event_handler.rs:851-867`). Ta Zern, and later others, need data. | Add `LeaderAction` data, mirroring `RelicAction` + `action_matches_relic`, and use the existing (unused) `Phase::LeaderAction` + `LeaderActionCommit { data }`. Leaders without data keep the one-click path. Also track **purged heroes** (`purged_leaders: BTreeSet<Leader>`) so used heroes disappear from `available_leaders`. | P2 / M |
| E7 | **Galactic event framework** | Prerequisite for §5. | See §5.0. | P2 / M |
| E8 | **A leader-faction override** | Cultural Exchange Program, and Creuss's "Unknown" leaders in Liberation of Ordinian. | `Player::leaders_from: Option<Faction>`, read by `update_available_leaders` (`state/game_state.rs:879`). Built with Cultural Exchange Program (§5.2.3). | P2 / S |

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
| D1 **The Quantumcore**: "When you gain this card, gain your breakthrough" | Track | `GainRelicAction` and E4 call E2. "Synergy for all types" is info only. ✅ `GameState::set_relic_owner` calls E2 when The Quantumcore gets an owner, so `GainRelicAction`, `SetRelicOwner` and the Minister of Antiques relic (`GainMinisterOfAntiquesRelic`) unlock the breakthrough. Losing the relic doesn't remove it. | P1 / S |
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
| J2 Destroyed and purged planets (Silver Flame, Conventions of War Abandoned) | Info | ✅ They render through `destroyed_planets` + `TiToken::DestroyedPlanet`. | P2 / S |
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
| CIV-1 Book of Latvinia: "When you gain this card, research up to 2 technologies that have no prerequisites" | Assist | After `GainRelicAction`/E4 for this relic, prompt with a tech picker filtered to `requirements.is_empty()`, sending `AddTechToPlayer` ×0–2. Removes the "techs are added manually" caveat. ✅ `set_relic_owner` records the new owner in `GameState::book_of_latvinia_research`, and `ResearchBookOfLatviniaTechs` researches 0–2 techs with no prerequisites (base techs or the player's own faction techs) and clears it (purging the Book clears it too). The prompt is shown in the end-of-turn view, the agenda view and the Relics view. | P2 / S |
| CIV-2 Neuraloop outside the status phase | Assist | `PlayNeuraloop` only works right after the status phase reveal. Also allow it after `RevealExtraPublicObjective` (Incentive Program, Classified Document Leaks, admin reveals). | P3 / S |
| CIV-3 Circlet of the Void action | Info | "Explore a frontier token" could hand over to the frontier-card flow (Enigmatic Device and Mirage are tracked). Optional. | P3 / S |
| CIV-4 Codex 4.5 ΩΩ techs | ✅ | Tied to `codex_4`. Fine. | — |
| CIV-5 Galactic events (Minor Factions, Total War, Age of Commerce, Age of Exploration) | — | See §5 | — |
| CIV-6 Liberation of Ordinian | — | See §6 | — |

---

## 5. Galactic events (all 20)

Codex IV introduced 4 events; TE adds 16 and reprints the 4 (the "20 Galactic Events" in the TE box). Sources: the wiki's *Galactic Events* page and the official Codex IV rules PDF. Each event lists its complexity (1–3) from the card.

### 5.0 Framework (do this once)

1. ✅ **Data:** `components/galactic_event/mod.rs`
   - `enum GalacticEvent` (20 variants).
   - `info()` returns name, complexity, expansion (`CodexIV` | `ThundersEdge`), `also_requires` and a description file in `galactic_event/description/` (same pattern as relics and breakthroughs).
   - Availability goes through `is_enabled_in`, which `GalacticEvent` overrides to check `also_requires` before the default `Component` check: the CIV events need `codex_4` (the Thunder's Edge reprint doesn't count), the TE events need `thunders_edge`. Age of Exploration's `also_requires` is PoK; Minor Factions is playable without it by ignoring the alliance paragraph.
2. ✅ **State:** `GameState::galactic_events: BTreeSet<GalacticEvent>`. A set, because the Codex IV rulebook says advanced players can play with several events at once.
3. ✅ **Choosing:** the creation-phase event `SetGalacticEvents { events }` replaces the whole set (empty clears it) and checks each event with `assert_enabled`. `SetSettings` and `ImportFromMilty` don't touch it.
   - The Creation ("Add Players") screen, which both the new-game and milty flows land on, lists the chosen events in a table (name, complexity bars, info button, remove button) above a dropdown of the remaining enabled events and an Add button. It's hidden when no event is enabled.
4. **Random pick:** a **"Pick random"** button on the Creation screen picker. The randomness lives in the client, so the event stays deterministic.
5. **In-game display:**
   - ✅ `Info::GalacticEvent` (info modal), opened from the Creation screen.
   - ✅ From Setup on, the info box lists the chosen events (name + info button) under a "Galactic event(s):" label. Creation keeps its own table.
6. **Scoring:** `Score::galactic_events` (`GalacticEventScore`) holds the event points, one field per event (Zealous Orthodoxy, Total War), added up in `update_player_points`. Monuments' VP is derived from planet ownership instead. The score table shows them in a "Galactic Events" section.
7. **Tests:** ✅ the builder's `.galactic_events([..])` and the creation tests in `setup.rs`. The per-event scenarios go in `libs/system_tests/tests/scenarios/galactic_events.rs`.
8. ✅ **Admin page:** a "Galactic Events" page (`views/galactic_events_view_mode/`) fixes the tracked state of the events in play without undoing (`galactic_events_admin` scenario). Like the Thunder's Edge page, it's a `ViewMode` with its info box button. The button is shown when an event in play has a section (`has_admin_section`), and the page shows one section per such event:
   - Stellar Atomics: each player's token, used or on the card, with a button that sends `SetStellarAtomicsUsed { player, used }`.
   - Zealous Orthodoxy: a player dropdown for the holder (`SetZealousOrthodoxyOwner { player }`). Clearing it lets the next player to reach 2 secret objectives gain it.
   - Total War: each player's points with `^`/`v` buttons (`SetTotalWarPoints { player, points }`, never negative). The buttons are the score table's `IncDecView`, now in `components/inc_dec.rs`.
   - Conventions of War Abandoned: the "Destroyed Planets" section from the Planets page (`SetPlanetDestroyed`, §5.2.2 item 8).
   - The events are any phase and need their galactic event in play (`GameState::assert_galactic_event`), an existing player and a change.
   - Each later event with tracked state adds its section.

### 5.1 Overview

The events are grouped by how much the app has to do, and each group is listed in the suggested order of work:
- **Nothing to implement:** the rule only touches untracked things (units, movement, commodities, trade goods, action cards, alliances, dice). The info modal and the info box already cover it.
- **Small implementation:** fits an existing flow: a grant at `StartGame`, a check, a score field, or one action in the Galactic Event tab. A few hours and one commit each.
- **Large changes:** new setup or status steps, new state that several subsystems read, or changes to existing checks across the handler and the UI. A day or more each.

Implementation goes event by event, one commit per event. Each tracked event gets a scenario in `libs/system_tests/tests/scenarios/galactic_events.rs`, using the builder's `.galactic_events([..])`.

Paths: handler = `libs/game_logic/src/gameplay/game_event_handler.rs`, state = `libs/game_data/src/state/`, UI = `packages/ui/src/views/`.

**Nothing to implement (4)**

| Event | Origin | Cx | Verdict | Why nothing | P / Effort |
|---|---|---|---|---|---|
| Age of Commerce | CIV | 1 | Info | Transactions and commodities aren't tracked; sharing a tech already works in the tech view | — |
| Call of the Void | TE | 1 | Info | Command tokens and movement | — |
| Cosmic Phenomenae | TE | 2 | Info | Anomaly combat and movement | — |
| Weird Wormholes | TE | 3 | Info | Ship replacement | — |

**Small implementation (10)**

| # | Event | Origin | Cx | Verdict | What the app does | P / Effort |
|---|---|---|---|---|---|---|
| 1 | Age of Fighters | TE | 3 | Track | ✅ Everyone gains Fighter II at `StartGame` (Naalu: Hybrid Crystal Fighter II) | P1 / S |
| 2 | Advent of the War Sun | TE | 1 | Track | ✅ Everyone except Muaat gains War Sun at `StartGame` | P1 / S |
| 3 | Zealous Orthodoxy | TE | 1 | Track | ✅ The first player to score 2 secrets gains 1 VP | P1 / S |
| 4 | Total War | CIV | 3 | Track | ✅ "Total War: gain 1 VP" action; builds the Galactic Event tab | P1 / S |
| 5 | Mercenaries for Hire | TE | 1 | Track | ✅ "Mercenaries for Hire" action that just ends the turn | P1 / S |
| 6 | Stellar Atomics | TE | 2 | Track | ✅ One-shot action; players who used it can't vote | P1 / S |
| 7 | Wild, Wild Galaxy | TE | 3 | Assist | ✅ The Stellar Converter works on any planet | P2 / S |
| 8 | Conventions of War Abandoned | TE | 3 | Track | ✅ "X-89: purge planet" action | P2 / S |
| 9 | Dangerous Wilds | TE | 1 | Track | Free tech when a tactical action takes an unowned hazardous planet | P2 / S |
| 10 | Hidden Agenda | TE | 2 | Assist | Hide individual votes until everyone has voted (UI only) | P3 / S |

**Large changes (6)**

| # | Event | Origin | Cx | Verdict | What the app does | P / Effort |
|---|---|---|---|---|---|---|
| 1 | Civilized Society | TE | 2 | Track | All 10 objectives revealed at setup, no status phase reveal, unlimited scoring; the game end rule comes later with E5 | P1 / M |
| 2 | Cultural Exchange Program | TE | 2 | Track | Players use a drawn faction's leaders (E8); the Firmament gets The Obsidian | P2 / M |
| 3 | Monuments to the Ages | TE | 2 | Track | Monuments with derived commodities → VP | P2 / M |
| 4 | Age of Exploration | CIV (PoK) | 2 | Track | "Dark Energy Tap" action that places the drawn tile on the milty map (tile, ring and position dropdowns; a hex neighbour helper) | P2 / M |
| 5 | Rapid Mobilization | TE | 1 | Track | Breakthroughs and the Fracture at `StartGame`; a per-player planets and research step | P2 / M |
| 6 | Minor Factions | CIV (PoK) | 2 | Track | Pick minor factions; their planets gain all 3 traits everywhere traits are read; alliance reminder | P2 / M |

### 5.2 Per-event plans

#### 5.2.1 Nothing to implement

##### Age of Commerce (CIV, complexity 1): Info
> Transactions don't need neighbours; no commodity cap; **non-faction technologies can be shared in transactions** (the receiver gains it, the giver keeps it).

- Transactions and commodities aren't tracked.
- Sharing a tech means toggling it for the receiver in the tech view (`tech_table.rs` → `AddTechToPlayer`, any phase), which already works. No "share technology" form is needed.

##### Call of the Void (TE, complexity 1): Info
> Command tokens and movement bonuses in the Fracture.

##### Cosmic Phenomenae (TE, complexity 2): Info
> Changes to nebulae, asteroid fields, supernovas, gravity rifts and entropic scars.

- Anomaly combat and movement aren't tracked.

##### Weird Wormholes (TE, complexity 3): Info
> Ships change type after moving through wormholes.

- Ship replacement isn't tracked.

#### 5.2.2 Small implementation

Every event ACTION (Total War, Mercenaries for Hire, Stellar Atomics, Conventions of War Abandoned and Age of Exploration in §5.2.3) goes through one event and one tab. Total War built both: `PlayGalacticEventAction` and the Galactic Event button now exist, with the `TotalWar`, `MercenariesForHire`, `StellarAtomics` and `ConventionsOfWarAbandoned` variants so far. Each later event adds its variant, its `has_action` entry and its form arm.

- **One event:** `Event::PlayGalacticEventAction { player, action: GalacticEventAction }`, with one variant per event, like `RelicAction`:
  - `TotalWar`
  - `MercenariesForHire`
  - `StellarAtomics`
  - `ConventionsOfWarAbandoned { planet }`
  - `AgeOfExploration { tile }`
- **Shared checks, done once** (modelled on `PuppetsOfTheBlade`, handler ~841): Action phase, the player's turn, `action_progress.is_none()`, and the variant's event is active. Then the handler applies the variant and goes to `EndActionTurn`.
- **The Galactic Event tab** in `ComponentSelectRow` (`action_phase.rs`), next to Action Card:
  - It's shown only when an active event has an ACTION. An event without one, like Age of Fighters, would leave the tab empty.
  - It has a dropdown of those actions, then the chosen action's form.
- Tech and promissory note actions still have no entry in the Component list. That gap is cross-cutting and outside this plan.

##### 1. Age of Fighters (TE, complexity 3): Track (setup only) ✅
> At setup everyone gains **Fighter II** (Naalu: **Hybrid Crystal Fighter II**). Fighters in the fleet pool get capacity 1; non-fighter ships are purged when destroyed.

✅ Done as planned below (`galactic_events_age_of_fighters` scenario).

- In `StartGame` (handler ~359), before `change_phase`, give every player `FighterII`; Naalu (`NaaluCollective`) gets `HybridCrystalFighterII`.
- Skip players who already have it.
- This has to happen at `StartGame`, not earlier: `player_initialization_finished` (`state/game_state.rs:855`) counts techs, and `SetupPlayerTechs` needs an empty tech list.
- The rest: Info.

##### 2. Advent of the War Sun (TE, complexity 1): Track (setup only) ✅
> Every player except Muaat gains the **War Sun** unit upgrade; Muaat purges its promissory note, unlocks its commander and places an extra War Sun.

✅ Done as planned below (`galactic_events_advent_of_the_war_sun` scenario).

- The same `StartGame` grant as Age of Fighters: `WarSun` for every player except `EmbersOfMuaat`.
- Muaat's own effects aren't tracked.

##### 3. Zealous Orthodoxy (TE, complexity 1): Track (VP) ✅
> **The first player to score 2 secret objectives gains 1 VP.** Their alliance card goes on this card (everyone gains the ability) and their alliance promissory note is purged.

✅ Done as planned below (`galactic_events_zealous_orthodoxy` and `galactic_events_zealous_orthodoxy_initiative` scenarios).

1. Add `Score::galactic_events.zealous_orthodoxy: Option<PlayerId>`. It has to be stored: `Score.secret_objectives` is a set per player, and the event history isn't in `GameState`. It's a VP, so it belongs in `Score`.
2. Set it with `GameState::gain_zealous_orthodoxy` while it's unset:
   - `ScoreExtraSecretObjective` (action phase, score view corrections): the player who reaches 2 secrets gains it right away.
   - `ScoreSecretObjective` (status phase): players score in initiative order but the app takes the decisions in click order, so it waits until every player has decided. Then the first player in `turn_order` (initiative order) with 2 or more secrets gains it.
3. `update_player_points` adds +1. A read-only row in a new "Galactic Events" section of `score_table_view.rs`, shown when the event is in play.
4. Unscoring doesn't clear it; use undo to fix mistakes.
- The alliance ability stays info only. Firmament plots (F2) don't count as scored secrets.

##### 4. Total War (CIV, complexity 3): Track (VP only) ✅
> Destroying units puts commodities on your home planets. **ACTION: discard 10 commodities from your home system → gain 1 VP.**

✅ Done as planned below (`galactic_events_total_war` scenario).

1. Add `GalacticEventScore::total_war: HashMap<PlayerId, i8>`, the same shape as `imperial`, with a read-only row in the "Galactic Events" score section.
2. Build `PlayGalacticEventAction` and the Galactic Event tab (§5.2.2 intro).
3. `GalacticEventAction::TotalWar` adds +1 to `Score::galactic_events.total_war`. It replaces the separate `TotalWarAction` event of the earlier plan.
- Commodities aren't tracked, so the 10 commodities aren't checked.

##### 5. Mercenaries for Hire (TE, complexity 1): Track ✅
> At the start of the game, the alliance cards of the factions not in play form a face-up mercenary deck. **ACTION: Spend 3 trade goods to gain the top card of the mercenary deck** and place it in your play area. Players can use the abilities of the mercenaries in their play area.

✅ Done as planned below (`galactic_events_mercenaries_for_hire` scenario).

- `GalacticEventAction::MercenariesForHire` has no data and no state. It just ends the turn.
- The card and the 3 trade goods stay at the table: alliances and trade goods aren't tracked.

##### 6. Stellar Atomics (TE, complexity 2): Track ✅
> Each player starts with a control token on the card. ACTION: discard it to destroy all ground forces and structures on any non-home planet. **Without a token you can't vote or play action cards in the agenda phase.**

✅ Done as planned below (`galactic_events_stellar_atomics` scenario).

1. Add `GameState::stellar_atomics_used: BTreeSet<PlayerId>`. It can't be derived, and it isn't a score.
2. Add the `GalacticEventAction::StellarAtomics` variant. It's rejected if the player has already used the token. There's no planet field, since units aren't tracked and nothing would read it.
3. Add `GameState::vote_restriction(&PlayerId) -> Option<VoteRestriction>`: `NekroVirus` for Nekro, otherwise `StellarAtomics` for a used token. It's infallible (an unknown player gives `None`) and returns the reason, which the handlers and views need. Use it instead of the hard-coded Nekro checks:
   - `CastAgendaVote` (handler ~1598) and `AddAgendaPlayerVote` (~1958) reject the vote with the reason;
   - `agenda_actions_view.rs`: the player's vote box shows the reason, and `ResolveOutcome`'s vote count leaves them out;
   - `laws_view_mode.rs`: the override vote form doesn't offer them;
   - the system tests' `vote_all` skips them.
4. `VoteRestriction::reason` gives "Nekro Virus cannot vote on agendas" and "Stellar Atomics token used, cannot vote on agendas". The Galactic Event tab shows "Stellar Atomics token already used" instead of the Play button.
- Recording these players as forced abstentions can't work, because `AddAgendaPlayerVote.outcome` isn't an `Option`.
- The action card restriction stays info only.

##### 7. Wild, Wild Galaxy (TE, complexity 3): Assist ✅
> Changes to 9 action cards (Direct Hit, Flank Speed, Maneuvering Jets, Morale Boost, Sabotage, Shields Holding, Skilled Retreat, War Machine, Diplomatic Pressure) plus Stellar Converter and Nova Seed working on any planet or system.

✅ Done as planned below (`galactic_events_wild_wild_galaxy` scenario).

- When the event is active, the Stellar Converter handler (`RelicAction::StellarConverter`, handler ~1091) skips its Mecatol, home, legendary and legendary-attachment checks but keeps "not already destroyed".
- The `relic_card.rs` picker drops the same filters.
- The action cards and Nova Seed stay info only.

##### 8. Conventions of War Abandoned (TE, complexity 3): Track ✅
> BOMBARDMENT hits destroy 3 units. **ACTION (if you own X-89): exhaust it to purge a planet in a system containing your BOMBARDMENT unit, along with its attachments and legendary card.** You're eliminated if all your home planets are purged.

✅ Done as planned below (`galactic_events_conventions_of_war_abandoned` scenario).

1. ✅ **Prerequisite fix (own commit):** destroyed planets can currently be taken again (the TODO near handler ~1078). Reject them in `TacticalActionTakePlanet` and `SetPlanetOwner`, and filter them out of the tactical take, unclaimed and Stellar Converter pickers.
2. ✅ Rename `MapData::stellar_converter_destroyed_planets` to `destroyed_planets`, so purged planets share the same list and the same map token (TE-J2).
3. Add the `GalacticEventAction::ConventionsOfWarAbandoned { planet }` variant. Besides the shared checks, it checks that the player owns any X-89 version (`X89BacterialWeapon`, `…Omega`, `…OmegaOmega`) and that the planet isn't destroyed yet. Home planets are allowed.
4. It removes the planet (and its attachments) like the Stellar Converter does, then ends the turn.
5. Its form in the Galactic Event tab is a planet dropdown.
6. ✅ The admin `SetPlanetDestroyed { planet, destroyed }` (any phase) destroys a planet on the board or restores a destroyed one (`score_manual_destroyed_planets` scenario). A restored planet is unowned, has no attachments and can be taken again. Its "Destroyed Planets" section (`views/destroyed_planets.rs`) is on the Planets page and the Galactic Events page.
- Elimination is game end, so skip it.

##### 9. Dangerous Wilds (TE, complexity 1): Track
> Neutral infantry on hazardous planets equal to their resources; they replenish each round. **When you gain a hazardous planet from the planet deck, you may research 1 technology, ignoring prerequisites up to that planet's resources.**

1. Follow the tactical attachment pattern (`TacticalActionAttachPlanetAttachment` handler ~497, `TacticalProgress.planet_attachments`, UI `tactical_action.rs` ~126-150).
2. For each taken planet with no previous owner and the hazardous trait, show a tech picker that sends `TacticalActionResearchTechnology { player, planet, tech }`.
3. Checks: event active, the planet is in `taken_planets` with `None`, it's hazardous, at most once per planet (stored in `TacticalProgress`), `assert_can_research`, the tech isn't owned.
4. Prerequisites aren't checked. Normal research doesn't check them either, and exhausting a planet's tech specialty could skip more than N, but exhaustion isn't tracked. Show "ignore up to N prerequisites" as a hint.

##### 10. Hidden Agenda (TE, complexity 2): Assist (UI only)
> Only the speaker talks in the agenda phase; votes are written secretly, the speaker reveals only totals (and who voted for what only when needed). Argent's votes are public.

- While a vote is open, `agenda_actions_view.rs` shows "Voted" instead of "{outcome} - {votes}", except for Argent's votes and the speaker's own player view.
- `agenda_info_view.rs` shows the outcome totals only once every eligible player has voted.
- The vote history after resolving stays complete.
- Caveat: clients still receive the full state, so this is "don't show", not real secrecy (§8 question 4).

#### 5.2.3 Large changes

##### 1. Civilized Society (TE, complexity 2): Track
> At setup **all public objectives are turned face up**. **No limit** on public objectives scored per status phase. The game doesn't end at the VP target; it ends **at the end of that round's status phase**, most VP wins, and ties are broken by influence of controlled planets + unspent trade goods.

1. **Setup.** `RevealAllPublicObjectives { stage_one, stage_two }` (5 + 5) instead of `RevealInitialObjectives` (handler ~330), with 10 dropdowns in `setup_phase.rs` (~242). The stage II gating (`GameState::status_phase_reveals_stage_two`) doesn't apply to it.
2. **Status phase.**
   - `CompleteStatusPhase` (handler ~1355) and the "Next phase" button (`status_phase_instructions_view.rs` ~14-37) both require `revealed_objective` today. Drop that requirement, and reject `RevealPublicObjective`, when the event is active.
   - Hide the reveal fieldset (`status_phase_actions_view.rs` ~110-148), which also hides the Neuraloop prompt.
3. **Unlimited scoring.** Keep the one `ScorePublicObjective` slot and add "score another" in the status view through `ScoreExtraPublicObjective` (any phase, bypasses the slot). `StatusPhaseState` stays unchanged.
4. **Game end.** Do the "ends after the status phase" rule and the influence tiebreak last, separately, with E5. They're not a dependency.

##### 2. Cultural Exchange Program (TE, complexity 2): Track
> At the end of setup, shuffle the reference cards of the factions in play; each player draws one, **takes all of that faction's leaders** for the rest of the game and unlocks that commander. **The Firmament/Obsidian doesn't take part; it starts with The Obsidian relic.**

1. Add `Player::leaders_from: Option<Faction>`. This is E8, which doesn't exist yet.
2. Setup event `SetupCulturalExchange { player, faction }`. Checks: Setup phase, event active, the faction is in play, neither side is `FirmamentObsidian`, each faction is drawn once. Drawing your own faction is allowed, since the card doesn't forbid it.
3. When the event is active, `player_initialization_finished` (`state/game_state.rs:855`) requires the draw (except for the Firmament). Add a per-player dropdown in the setup view.
4. `update_available_leaders` (`state/game_state.rs:879`) uses `leaders_from`. The Yssaril "all agents" rule follows whoever holds Ssruu, which is the drawn faction.
5. At `StartGame`, the Firmament player gets `Relic::TheObsidian` through `set_relic_owner`.
- The Obsidian's own effect (1 extra secret) and commander unlocking aren't tracked.

##### 3. Monuments to the Ages (TE, complexity 2): Track (VP source)
> Instead of placing a structure on a non-home planet, you may spend 5 trade goods to place a **monument** (max 3 in play). **At the start of the status phase each monument gets 1 commodity; each monument is worth 1 VP per 3 commodities to whoever controls its planet.** The new controller may destroy it.

1. Add `GameState::monuments: BTreeMap<Planet, u32>`. The value is the number of status phases already started when the monument was built.
2. Derive the commodities instead of counting them: the status phases started now (`round - 1`, plus 1 once the round's status phase has begun) minus the stored value. No counter is incremented at the status phase.
3. Events:
   - `BuildMonument { player, planet }`: owned, non-home, fewer than 3 monuments.
   - `DestroyMonument { planet }`: offered in the tactical view when a taken planet has a monument, and in the planet view.
4. `update_player_points` gives `commodities / 3` VP to the planet's owner. Add a "Monuments" score row and a planet badge.
- Trade goods aren't tracked, so the 5 trade goods aren't checked.

##### 4. Age of Exploration (CIV, complexity 2, needs PoK): Track
> Relics require only 2 matching fragments be purged instead of 3. The Naaz-Rokha Alliance's FABRICATION faction ability and BLACK MARKET FORGERY promissory note do not require purged fragments to match.
>
> **ACTION: Exhaust DARK ENERGY TAP and choose a non-home edge system that contains your ships to roll 1 die. On a result of 1-4, draw a random unused red tile; on a result of 5-10, draw a random unused blue tile. Place that tile adjacent to the chosen system so that it is touching at least 2 non-home systems.** Place a frontier token in the system if it does not contain any planets.

1. **The action:** `GalacticEventAction::AgeOfExploration { tile: Option<(SystemId, Coordinate)> }`. Besides the shared checks (§5.2.2 intro):
   - the player owns `DarkEnergyTap`;
   - `tile` is required when the game has a milty map (`MapData::milty_information.hex_map`), the same way `FrontierCardAction::Mirage` requires its `system` (handler ~955), and must be `None` otherwise;
   - exhaustion and the roll stay at the table.
2. **Tile dropdown:** systems from enabled expansions that are red or blue, i.e. `SystemType::Normal` or `Anomaly`, never `HomeSystem` or `Hyperlane`.
   - They mustn't already be in `hex_map.tiles`.
   - The special tiles are excluded: Mecatol Rex, the Creuss gate, the Crimson Rebellion gate, the Wormhole Nexus and the Fracture.
   - The player picks the tile they drew.
3. **Position: two dropdowns, ring and position in that ring.**
   - **Explainer** above them: "Ring 0 is Mecatol Rex. Ring N has 6 × N positions, numbered from 0 directly above Mecatol Rex, going clockwise." That's `Coordinate`'s own convention (`common/map.rs`).
   - **Legal positions only:** empty (a gap inside the galaxy, or a spot just outside it) and touching at least 2 non-home systems.
     - The ring dropdown lists the rings that have a legal position.
     - The position dropdown lists that ring's legal positions.
   - Hyperlanes and home systems don't count towards the 2.
   - Every legal position touches a non-home edge system. Whether that system holds the player's ships isn't tracked.
4. **Neighbour helper:** `Coordinate::neighbors()` in `common/map.rs`, using ring/position ↔ axial conversion. There's only the private `outward_neighbor` today, and the ring/position → grid math lives in the UI (`get_tile_position`, `map_view_mode/layout.rs:371`). Test the helper against the map layout's positions.
5. **Handler:** it checks the same rules and appends `Tile { system, position: HexPosition::Pos(coord) }` (rotation 0) to `hex_map.tiles`.
6. **Map view:**
   - The off-board tiles already move out of the way: `place_tiles` (`map_view_mode/outside_galaxy.rs:72`) places every `HexPosition::Pos` tile first, then nudges the Crimson Rebellion home system, the Fracture and the left column away from them. Add a layout test with a tile next to where the Fracture is drawn.
   - `build_layout`'s `empty_slots` (`layout.rs:325`) draws placeholders for every ring up to the outermost tile, so a tile one ring out would add a whole placeholder ring. Limit it to the imported rings (`HexMap::ring_count`).
7. **Info only:**
   - Planet ownership needs no change: `GameState::planet_location` (`state/game_state.rs:579`) already counts any planet with a system tile as on the board.
   - The frontier token stays at the table, since the map doesn't draw frontier tokens.
   - The 2-fragment relic rules stay at the table, since fragments aren't tracked and `GainRelicAction` doesn't check them.

##### 5. Rapid Mobilization (TE, complexity 1): Track (setup)
> After setup the Fracture enters play. Each player: (1) places 1 infantry on, gains and readies each planet adjacent to their home system (or the Creuss Gate / The Sorrow); (2) places a space dock and their flagship + 3 fighters; (3) **gains their breakthrough**; (4) **researches 1 technology** (Nomad: 2).

1. At `StartGame`: set `fracture_in_play` first (so there's no roll), then `gain_breakthrough` for everyone, which applies the I.I.H.Q. and Stellar Genesis effects.
2. The planets and the research (1 tech, Nomad 2) need a per-player step after the faction setup. `player_initialization_finished` counts total techs, so that check has to change first.
3. In milty games, use Age of Exploration's neighbour helper (`Coordinate::neighbors()`) to offer the planets next to the home system. Without a map, trust the picker: non-home, unowned, on the board.
- Units aren't tracked.

##### 6. Minor Factions (CIV, complexity 2, needs PoK): Track
> Setup: players get 1 fewer blue tile. Each player is dealt a non-played faction and places its home system in ring 2 with 3 neutral infantry. Controlling every planet in a minor faction system gives you that faction's alliance card. **Planets in minor faction systems have all three traits.**

1. Minor factions are chosen per game, validated as not in play, one per player.
2. Promote the private `get_planet_traits` (`state/agenda.rs:267`) to `GameState::planet_traits`, which returns all three traits for minor-faction planets. Use it at every trait read:
   - agenda elections;
   - both attachment checks (handler ~497, ~1852);
   - the tactical "Cannot explore" check and attachment filter;
   - the admin attach form;
   - the sidebar counts;
   - the planet grid;
   - Dangerous Wilds' hazardous check.
3. Minor-faction systems "don't count as home systems", so add a home-planet helper that excludes them, for the Stellar Converter, Shard of the Throne, Monuments and Conventions checks.
4. Show an alliance reminder in the tactical view when a player controls the whole system. Alliance ownership is 👺.

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
| LO-1 Setting | `GameSettings.scenario: Option<Scenario>`, `Scenario::LiberationOfOrdinian`, offered when `codex_4 && prophecy_of_kings`. The new-game form pre-fills the 6 factions and Creuss = Red; `AddPlayer`/`CreationDone` validate 6 players and the faction set. | S |
| LO-2 Setup automation | At `CreationDone`/`StartGame`: speaker = Nekro (validate `SetupSpeaker` or set it automatically); Nekro gets `Planet::Ordinian` (the "Hyperion VI" legendary already exists from TE data); Naaz-Rokha gets Circlet of the Void and Nomad gets Neuraloop (E4); Nekro techs = Dacxive + the two scenario techs (new `Technology` variants, `Faction(NekroVirus)`, `Expansion::CodexIV`, enabled only in the scenario). | M |
| LO-3 "Mecatol Rex means Ordinian" | Add `GameState::is_centre_planet(&Planet)` (scenario aware) and use it instead of `Planet::is_mecatol_rex()` wherever rules apply: custodians grant (`game_event_handler.rs:481`; **disabled** in the scenario), Imperial "+1 VP while controlling Mecatol Rex", Stellar Converter exclusion, agenda elect "non-Mecatol" checks. | S |
| LO-4 Liberate/Control Ordinian objective | Add `PublicObjective::LiberateOrdinian` (Stage I, 1 VP, scored through `ScoreExtraPublicObjective` because it's an action-phase score) and `PublicObjective::ControlOrdinian` (Stage II; VP and timing to be read off the card), both scenario-only. Reveal LiberateOrdinian with the initial objectives; it needs no adjustment to the stage II gating, which comes from the round (`GameState::status_phase_reveals_stage_two`). When the first Stage II is revealed, also reveal ControlOrdinian and reject further scoring of LiberateOrdinian (existing VPs stay). | M |
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
   - Disable the Setup screen's "Start Game" button until a speaker is set. In `setup_phase.rs`, `setup_complete` is `speaker_chosen() || revealed_objectives() > 0`, so once the objectives are revealed the button is enabled without a speaker, and `StartGame` is rejected with "No speaker has been set!".
2. **Core TE effects (P1/P2).**
   - ✅ E2/E3/E4 helpers.
   - ✅ A1 Crimson, A4 I.I.H.Q., A5 Stellar Genesis, C2 Jupiter Brain.
   - ✅ D1 Quantumcore.
   - ✅ CIV-1 Book of Latvinia.
   - ✅ C1 Styx.
3. **Galactic event framework plus the small events (§5.2.2), in order.**
   - ✅ The framework (§5.0).
   - ✅ Age of Fighters, Advent of the War Sun, Zealous Orthodoxy, Total War, Mercenaries for Hire, Stellar Atomics, Wild, Wild Galaxy, Conventions of War Abandoned.
   - ✅ The galactic events admin page (§5.0 item 8) (P2).
   - Dangerous Wilds (P2).
   - Hidden Agenda (P3).
4. **TE actions (P2).**
   - A3/A6/A7 breakthrough actions, C3 Faunus, D2 Silver Flame.
   - ✅ TE-B2/B3 Fracture flag and roll prompt, A8 Al'Raith.
   - TE-B4 relic prompt for Fracture planets.
   - E6 leader data → Ta Zern and Director Nel.
   - ✅ Brilliance, Overrule, Strategize.
   - J1 Thunder's Edge on the map, G1 space stations in agendas.
5. **The large events (§5.2.3), in order.**
   - Civilized Society without its game end rule (P1).
   - Cultural Exchange Program, Monuments to the Ages, Age of Exploration, Rapid Mobilization, Minor Factions (P2).
   - F2 Firmament plots.
6. **Optional (P3).**
   - ✅ Crisis/Puppets on a String, Radical Advancement, legendary ability texts, The Triad badge.
   - Liberation of Ordinian.
   - E5 victory target and game end, with Civilized Society's end rule and tiebreak (lowest priority).

## 8. Open questions

1. Should ticking Thunder's Edge pre-tick Codex I–IV in the new-game form (the TE box contains the codex cards)?
2. Are we fine with **no exhaust tracking** for legendary planets and breakthroughs (buttons are always offered, players police once-per-round)?
3. Is Liberation of Ordinian wanted at all, or should it be 👺?
4. Hidden Agenda: is "UI hides votes" good enough, given that clients receive the full state?
5. ✅ Cultural Exchange Program: what happens if a player draws their own faction's reference card? It's allowed, since the card doesn't forbid it (§5.2.3).
