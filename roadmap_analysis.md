# Roadmap analysis

A review of [`Roadmap.md`](./Roadmap.md) against the code on branch `thunders-edge` (`6ad0dff`), done 2026-10-25. Companion documents:
- [`mismatches.md`](./mismatches.md): data vs. wiki
- [`TE_CIV_Action_Plan.md`](./TE_CIV_Action_Plan.md): a detailed plan for the remaining Thunder's Edge and Codex IV work
- [`Roadmap_Updated.md`](./Roadmap_Updated.md): a proposed corrected roadmap

Every claim below was checked against the code (file references given). "Display" means the frontend shows it to players.

---

## a) Items that should be marked complete (or that have the wrong status)

| Roadmap item | Says | Should say | Evidence |
|---|---|---|---|
| Map Render → Planet destruction (Stellar converter relic) | ❌ | ✅ | `packages/ui/src/views/map_view_mode/layout.rs:171-280` renders `TiToken::DestroyedPlanet` from `map_data.stellar_converter_destroyed_planets` |
| Descriptions → Objectives | ❌ | ✅ | `InfoButton { info: Info::Objective(..) }` in `status_phase_actions_view.rs` and `secret_objectives_view.rs`; the text comes from `objective.info().condition` |
| Descriptions → Techs | ❌ | ✅ | `tech_view_mode/tech_table.rs:383-385` → `Info::Tech` (effects) |
| Descriptions → Agendas | 🍑 | ✅ | Shown in the frontend: `laws_view_mode.rs:78,178` → `Info::Agenda` |
| Descriptions → Leaders | 🍑 | ✅ | Shown in the frontend: `action_phase.rs:520` → `Info::Leader` |
| Descriptions → Action Cards | ❌ | 🍑 | `ActionCardInfo` has `play_text` and `effect` (`action_card.rs:142-159`), but the UI never shows them |
| Descriptions: Relics, Strategy cards, Breakthroughs (not listed) | — | Relics ✅, Strategy cards ✅ (card image), Breakthroughs 🍑 | `Info::Relic`, `Info::Strategy` (image); breakthrough descriptions exist in data, but there's no `Info::Breakthrough` |
| PoK → updated construction strat card | ❌ | ✅ | The image switches on PoK (`info_context.rs:144`). The PoK change is wording only |
| PoK → updated diplo strat card | ❌ | Move to Codex I, ✅ | The Diplomacy change is **Codex I**, not PoK. The image switches on `codex_1` (`info_context.rs:147-148`) |
| Codex I → updated components: diplo strat card | ❌ | ✅ | As above |
| Codex I → updated components: hypermetabolism | ❌ | ✅ | Hyper Metabolism text matches the wiki (generic tech check in mismatches §7) |
| Codex I → updated components: gravity rifts, rerolls during combat, retreating infantry | ❌ | 👺 | Rule changes to untracked things (movement, combat) |
| PoK → Exploration (parent) | ❌ | ✅ | All three children are ✅. The non-attachment exploration cards are intentionally not modelled |
| Score → Relics → Obsidian | ❌ | 👺 | The roadmap itself says the secret cap isn't tracked; there's nothing to implement |
| Score → Agendas (unlisted but done) | — | ✅ Holy Planet of Ixth, ✅ Shard of the Throne (agenda), ✅ The Crown of Emphidia (agenda) | `ScorableAgenda` in `state/score.rs:149-229` and the score table rows |
| Score → Relics (unlisted) | — | ✅ Book of Latvinia (CIV) | `score.book_of_latvinia` |
| Score → Secret objectives → "Buttons to score non-status phase objective when they happen" | ❌ | ✅ (via the Score view), or 🚱 if a contextual action-phase button is still wanted | `ScoreExtraPublicObjective` (`score_table_view.rs:142,185`) and `ScoreExtraSecretObjective` (`secret_objectives_view.rs:134`) |
| Agenda Phase → VPs from agenda cards | ❌ | ✅ | Mutiny, Seed of an Empire, Political Censure, Holy Planet of Ixth, Shard, Crown are all scored (see above) |
| Agenda Phase → Secret objective scoring | ❌ | 🚱 | Possible through `ScoreExtraSecretObjective` in the Score view (Dictate Policy / Drive the Debate), but not offered in the agenda flow |
| Score → Agendas → Incentive Programs / Classified Document Leaks | ❌ | Still ❌, **but the partial code is wrong** | Stage II gating counts them as laws (`game_state.rs:608-621`); see mismatches §9 |
| TE → thunder's edge → planet / exploration | ❌ | ✅ for the expedition | Expedition flow, completion, admin page and tests (`state/expedition.rs`, `scenarios/thunders_edge.rs`). The planet's ability belongs under "legendary planet abilities" |
| TE → new relic cards | ❌ | ✅ data, ❌ effects | All 7 TE relics exist with the right names and descriptions; only the Quantumcore, Silver Flame and Triad effects are missing |
| TE → New Leaders → Agents | ❌ | ✅ | Both ACTION agents are done; the other TE agents are reactions (nothing to play) |
| TE → new anomaly? | ❌ | ✅ data, 👺 rules | `AnomalyType::EntropicScar` exists (tiles 114, 116). Its status-phase "gain a faction tech" rule can be done from the tech page |
| TE → N'orr Supremacy | ❌ | 👺 | The roadmap itself says "just do it on the tech page"; nothing to build |
| TE → Void Tether / Resonance Generator | ❌ | 👺 (plus a generic action button) | Adjacency and breaches aren't tracked |
| Infra / Gameplay → "Take multiple turns", "Undo", "Strategy cards" | ✅ | ✅ | Confirmed (`TakeAnotherTurn`, undo scenarios) |

## b) Missing major topics

1. **Galactic events are only listed under Codex IV**, but 16 of the 20 are Thunder's Edge. The roadmap needs a galactic events section covering both expansions, plus a framework item (settings, selection UI, info display). See action plan §5.
2. **Thunder's Edge action cards:** 14 new cards (20 in the deck), none in the code, plus 8 Ω rewordings of Codex I cards (Hack Election changes meaning). Brilliance, Overrule, Strategize, Crisis and Puppets on a String touch tracked state. **✅ Done**, including Mercenary Contract; see action plan TE-I.
3. **Thunder's Edge updates to existing factions:**
   - Keleres: Executive Order replaces the I.I.H.Q. tech, Agency Supply Network reworded, Custodia Vigilia Ω 2/2, two hero texts.
   - Naalu agent ΩΩ, Xxcha hero ΩΩ (now an ACTION), Yin ΩΩ ×3.
   - Jol-Nar commander renamed to Agnlan Oln.

   The roadmap's "updated codexes: go over all of them" doesn't say what this involves.
4. **Thunder's Edge faction unit upgrades:** Helios V2, Linkship II and Exile II are missing (data).
5. **Victory target / winning:** `GameSettings.max_points` is ignored and the score hard-codes 10 (`state/score.rs:63`). "Winning the game" is ❌, but this underlying bug isn't on the roadmap. It blocks Civilized Society and Liberation of Ordinian (12/10). → lowest priority (2026-10-27): game end is handled at the table. Revisit with the galactic events.
6. **Firmament "Plots within Plots":** scoring another player's secret must give **no VP** (it creates a plot instead), and Puppets of the Blade needs at least 1 plot. Not mentioned; it affects score correctness.
7. **Crimson Rebellion starts with its breakthrough** (and rolls for the Fracture during setup). Not handled or mentioned.
8. **Styx VP** is listed, but other VP sources added by Thunder's Edge and Codex IV aren't:
   - The Silver Flame (+1 VP)
   - Galactic events: Total War, Zealous Orthodoxy, Monuments
   - Liberate Ordinian
9. **A way to gain a relic outside the "gain relic" action.** Only `GainRelicAction` (an action that ends the turn) exists. Fracture planets, Vaults of the Heir, the Quantumcore, Cultural Exchange and scenario setup all need a non-action relic grant.
10. **Data quality / QA:** the QA section has generic "go over X" items, but specific known gaps aren't listed:
    - Codex III Ω secrets not applied
    - wrong expansion tags (Raise a Fleet, Dictate Policy, Drive the Debate, I.I.H.Q., Agency Supply Network)
    - the Fracture tile bug
    - the Firmament starting planets
    - the inverted elect-planet trait check (likely UI crash)

    See mismatches §1.
11. **Legendary planet abilities in general:** PoK legendaries (Primor, Hope's End, Mallice, Mirage) and Mecatol Rex Ω have abilities with no text in the data. Thunder's Edge lists only 4 of its 10.
12. **Space stations in agendas:** they can't be elected in "elect planet" agendas. Not filtered, not mentioned.
13. **Hero purging:** used heroes stay playable (no purge tracking). Relevant to Director Nel and Ta Zern.
14. **Twilight's Fall** (the game mode in the Thunder's Edge box) isn't mentioned. Mark it 👺 explicitly so it doesn't come up again.
15. **Thunder's Edge includes the Codex cards:** decide whether ticking Thunder's Edge should pre-tick Codex I–IV.
16. **Test coverage gaps worth listing:**
    - no scenario elects a cultural/hazardous/industrial planet (that would have caught the inverted check)
    - none takes a Fracture planet or Thunder's Edge
    - none covers Incentive Program or Classified Document Leaks with the stage II gating
17. **Hyperlane tiles 119A–124B** (Thunder's Edge) are missing from `system.rs`. That may matter for milty map rendering of Thunder's Edge games.

## c) Other discrepancies

- **Structure:** PoK and Codex I–III are nested under *Gameplay → Separation between base/PoK/codexes*, while Thunder's Edge and Codex IV are separate top-level sections with a different indentation style. Suggestion: one *Expansions* section with the same layout for all.
- **Wrong use of status icons:**
  - "Breakthroughs for each faction" and "new factions" are 🚱, but several sub-items are hard ❌, and there are real bugs: Firmament planets, Crimson breakthrough, Plane Splitter prerequisites, missing unit upgrades.
  - "space stations ✅" is fine for the planet cards, but the agenda rule is missing.
- **Political Censure ✅:** the scoring works, but the score table labels its row "Shard of the Throne" (`score_table_view.rs:612-613`).
- **Codex II "3 new relics ✅":** Thunder's Edge reprinted Dynamis Core and Nano-Forge with changed text (mismatches §5).
- **Codex III "updated leaders ✅":** correct, apart from Dannel Ω's ability name ("Spinner Overdrive Ω" should be "Quantum Dissemination"). **✅ Fixed.**
- **Codex IV "relics ✅":** correct. Book of Latvinia's "research 2 techs on gain" could become a prompt instead of manual work (action plan CIV-1).
- **Bottom TODO "Look over all strategy cards…":** Diplomacy and Construction are done; Construction ΩΩ and Warfare Ω (Thunder's Edge) are not. The `thunders_edge` flag is matched in `get_strategy_card_image` but never used, and the Construction secondary hint "place token in system" is wrong under Thunder's Edge.
- **"Faction specific setups → agents ❌":** unclear what this means. No faction setup involves agents. Clarify or remove.
- **"Codex I promissory notes ❌":** promissory notes aren't tracked by design (only Naalu's Gift of Prescience, which changes initiative). None of the five Codex I notes changes tracked state, so this should be 👺.
- **"PoK alliances ❌":** alliance abilities aren't tracked; 👺 (galactic events only need them as reminder text).
- **"Coexistence (map visualization)":** it needs unit tracking, so it should be 👺.
- **Thunder's Edge legendary planets ("Thunder's Edge: perform 1 action at the end of your turn … not very pretty"):** the existing `TakeAnotherTurn` can sit behind a contextual "Jupiter Brain" button. No new turn logic is needed (action plan TE-C2).
- **The stale TODO "Handle Council Keleres"** in `get_starting_planets` (`common/faction.rs:176`): Keleres planets are handled in setup through `SetupTheTribunii`.
- **Typos in the roadmap:** "Mecatroll rex", "promissary", "acquiessence", "magan defence grid", "psycoarch", "sling ray" (Sling Relay), "Coexistance", "Assasinate".
