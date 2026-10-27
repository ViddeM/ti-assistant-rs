# Game data vs. wiki: mismatches

Audit of `libs/game_data` against the [Twilight Imperium wiki](https://twilight-imperium.fandom.com/wiki/Twilight_Imperium_Wiki), done 2026-10-25 on branch `thunders-edge` (commit `6ad0dff`).

## How this was checked

- **Source:** The wiki was read through its MediaWiki API (`api.php?action=parse&prop=wikitext`). The HTML pages block automated fetches; the API works with `curl`. Liberation of Ordinian only has a stub page, so that information comes from the official Codex IV rules PDF instead (see `TE_CIV_Action_Plan.md`).
- **Scope:** Pages marked UNOFFICIAL, First/Second/Third Edition and Twilight's Fall were ignored.
- **Coverage:** Action cards, agendas, objectives, relics, exploration/frontier cards, technologies, strategy cards, breakthroughs, planets, systems, factions and leaders.
- **Method:** Names, numbers and categories were compared in full. Effect texts were compared by script and the differences then checked by hand. Only meaningful differences are listed (wrong numbers, missing clauses, different targets, outdated text), not punctuation.
- **Re-verified by hand:** Rows marked ✔ were re-checked by reading the code directly. Everything else comes from the comparison pass. Line numbers are accurate as of the commit above.
- **Wiki reliability:** The wiki is not always right. Where its own pages disagree, the conflict is listed in [Wiki inconsistencies](#wiki-inconsistencies-check-against-the-printed-cards) instead of being counted against the code.

Paths below are relative to `libs/game_data/src/components/` unless stated otherwise.

### Severity legend

| | Meaning |
|---|---|
| 🔴 | Breaks app functionality (panic, a blocked flow, wrong score) |
| 🟠 | Wrong or missing data that players will see or that changes a rule |
| 🟡 | Outdated or partial text; the meaning changes slightly |
| ⚪ | Cosmetic (typo, capitalisation) |

---

## 1. Top issues (fix first)

| # | Sev | Issue | Where | |
|---|---|---|---|---|
| 1 | 🔴 | **Fracture tiles 125, 126 and 127 all contain `Planet::Industrex`** (copy-paste error). Cocytus, Styx, Lethe and Phlegethon are in no system. `System::for_planet` errors if a planet is in 0 or more than 1 systems, so `TacticalActionTakePlanet` rejects Industrex and every Fracture planet (`game_event_handler.rs:410,475`). **✅ Fixed.** | `system.rs:801-818` | ✔ |
| 2 | 🔴 | **The planet-trait checks for elect votes are inverted.** `ensure!(!…contains(&PlanetTrait::Cultural))` (and the Hazardous and Industrial equivalents) rejects planets that *have* the trait. `VoteOptionDropdown` calls `AgendaElect::parse(..).expect(..)` (`packages/ui/src/components/dropdown.rs:443`), so picking a valid option should crash the UI for these 9 agendas: Core Mining, Demilitarized Zone, Holy Planet of Ixth, Research Team (×4), Senate Sanctuary, Terraforming Initiative. No test covers it. **✅ Fixed.** The real symptom was different: `VoteState::new` listed the candidates as plain `Planet` outcomes, which the vote handler rejects ("invalid vote kind") for these agendas. So nobody could vote on them, and Holy Planet of Ixth could never give its VP. The parse check was removed rather than flipped, because traits added by attachments (Terraform) are only known to `VoteState::new`. | `agenda/mod.rs:198-227` | ✔ |
| 3 | 🔴 | **The Firmament starts with 4 planets** (Cronos, Tallin, Cronos Hollow, Tallin Hollow), because tiles 96A and 96B are both `HomeSystem(FirmamentObsidian)` and `get_starting_planets` collects every home-system tile for the faction. The wrong state is already baked into `setup_milty_import@state.snap`. **✅ Fixed.** | `common/faction.rs:175-186`, `system.rs:652-665` | ✔ |
| 4 | 🔴 | **Some planets are in no system, so they can't be taken with a tactical action:** Thunder's Edge, Avernus, Custodia Vigilia and the Fracture planets. Mirage is special-cased; the others are not. Today the only way to give them to a player is the admin `SetPlanetOwner` event. **✅ Fixed** (Fracture planets in #1). Thunder's Edge can now be taken once the expedition has placed it (in that system), and Avernus once someone controls it (system not tracked, like Mirage). Correction: Custodia Vigilia should *not* be takeable; it has no tile and the Keleres "cannot lose" it. It should be granted with I.I.H.Q. instead (action plan E2), which is still open. | `system.rs` (no entries), `game_event_handler.rs:410` | ✔ |
| 5 | 🟠 | **Styx has 3 resources; it should have 4** (4/0 on 4 separate wiki pages). **✅ Fixed** | `planet.rs:968` | ✔ |
| 6 | 🟠 | **14 Thunder's Edge action cards are missing** (20 cards in total; see §2). **✅ Fixed**, including the Ω versions of 8 Codex I cards. | `action_card.rs:15-112` | ✔ |
| 7 | 🟠 | **Wrong expansion tags:** Raise a Fleet (public), Dictate Policy and Drive the Debate (secrets) are tagged `Base` but are PoK, so they show up in base-only games. I.I.H.Q. Modernization and Agency Supply Network are tagged `Base` but are Codex III. | `objectives/public.rs:216`, `objectives/secret.rs:312,318`, `tech.rs:1031,1101` | ✔ |
| 8 | 🟠 | **Keleres hasn't been updated for Thunder's Edge.** The I.I.H.Q. Modernization *tech* stays enabled alongside the new I.I.H.Q. *breakthrough*. The replacement tech Executive Order is missing, Agency Supply Network has the Codex III text, and Custodia Vigilia should be 2/2 under Thunder's Edge. | `tech.rs:1026-1105`, `planet.rs:786-796` | |
| 9 | 🟠 | **None of Thunder's Edge's updated leader cards exist:** Naalu agent, Xxcha hero, all three Yin leaders, two Keleres heroes, and the Jol-Nar commander rename. `Leader::is_enabled_in` only knows about the Codex III swaps. | `components/leaders.rs:86-115` | |
| 10 | 🟠 | **Plane Splitter needs 2 cybernetic prerequisites; the code has 1.** | `tech.rs:1231` | ✔ |

Code problems found during the audit that aren't data-vs-wiki issues are in [§9](#9-other-code-issues-found-during-the-audit).

---

## 2. Action cards (`action_card.rs`)

### Missing: Thunder's Edge cards (none of the 14 unique cards exist)

**✅ Fixed.** All 14 cards exist, tagged Thunder's Edge. A Thunder's Edge game with Codex I and PoK has the 140-card deck from the wiki (checked by a unit test). The texts come from the wiki's *Action Cards* page, with obvious typos fixed ("daugfhter", "pirtaes", "hunder", "niether", "gives/shows you" in Extreme Duress, "player's hands" in Lie in Wait). Brilliance, Mercenary Contract, Overrule, Strategize, Crisis and Puppets on a String change tracked state and are implemented (see the action plan, TE-I). The others are text only.

| Card | # in deck | Timing | Effect (short) | Relevant to the app? |
|---|---|---|---|---|
| Black Market Dealings | 1 | When negotiating a transaction | Relics, action cards and unscored secrets may be part of the transaction | Relic ownership can change (relics are tracked) |
| Brilliance | 1 | Action | Ready a tech-specialty planet **or choose 1 player to gain their breakthrough** | **Yes**: grants a breakthrough |
| Crash Landing | 1 | Last ship in active system destroyed | Place 1 ground force on a planet (not Mecatol Rex); coexist | No |
| Crisis | 1 | End of any player's turn, if at least 2 players haven't passed | **Skip the next player's turn** | **Yes**: changes turn order |
| Exchange Program | 1 | Action | Infantry swap into coexistence, or both discard a fleet token | No |
| Extreme Duress | 1 | Start of another player's turn, if they have a readied strategy card | If their action isn't strategic: discard action cards, give trade goods, show secrets | No |
| Lie in Wait | 1 | After 2 of your neighbours transact | Take 1 action card from each | No |
| Mercenary Contract | 1 | Action | Spend 2 trade goods: 2 neutral infantry on an empty non-home planet; **its owner returns the planet card** | **Yes**: planet loses its owner |
| Pirate Contract | 4 | Action | Place a neutral destroyer | No |
| Pirate Fleet | 1 | Action | Spend 3 resources: neutral fleet | No |
| Puppets on a String | 1 | End of any player's turn, if you have passed | **Perform 1 action** | **Yes**: a passed player takes an action |
| Rescue | 1 | After another player moves into your system | Move 1 ship in | No |
| Strategize | 4 | Action | **Perform the secondary** of any readied or unchosen strategy card | **Yes** for the Technology secondary |
| Overrule | 1 | Action | **Perform the primary** of any readied or unchosen strategy card | **Yes** for Technology, Politics and Imperial primaries |

> Note: the wiki's Thunder's Edge page spells it "Brillance"; the Action Cards page has "Brilliance".

### Outdated text: Thunder's Edge Ω rewordings of Codex I cards

| Card | Sev | Code (Codex I) | Thunder's Edge Ω | Location |
|---|---|---|---|---|
| Hack Election | 🟠 | "voting begins with the player to the right of the speaker and continues counterclockwise" | "During this agenda, **you vote last**." | `:808-816` ✔ |
| Counterstroke | 🟡 | "After **a** player activates…" | "After **another** player activates…" | `:771-776` |
| Ghost Squad | 🟡 | "After **a** player commits units…" | "After **another** player commits units…" | `:799-804` |
| War Machine | 🟡 | "+4 to the total PRODUCTION value of **your** units" | "…of **those** units" | `:913-919` |
| Fighter Conscription | 🟡 | "space docks or units that have capacity" | "space docks or **ships that have a capacity value**" | `:780-787` |
| Solar Flare | ⚪ | "During this movement…" | "During the "Movement" step of this tactical action…" | `:904-910` |
| Master Plan | ⚪ | "You may perform an additional action this turn." | "Perform an additional action." | `:847-854` |
| Blitz | ⚪ | "…that do not have… gain" | "…that does not have… gains" (grammar) | `:762-768` |

**✅ Fixed.** The Ω versions are separate cards (`BlitzOmega` …) tagged Thunder's Edge. `ActionCard::is_enabled_in` swaps them in when Thunder's Edge is enabled, so games without Thunder's Edge keep the Codex I wording. The Ω versions are still Codex I cards, so like the originals they need Codex I enabled. Playing a replaced version is rejected.

### Questionable internal timing categories

| Card | Code `play` | Card trigger | Location |
|---|---|---|---|
| Master Plan | `AfterActionCardIsPlayed` | "After you perform an action" | `:853` |
| Coup d'Etat | `AfterActionCardIsPlayed` | "When another player would perform a strategic action". `AfterStrategyCardIsPlayed` exists but nothing uses it | `:949` |

### Verified OK
- **93 cards:** All 93 code entries (59 Base, 17 Codex I, 17 PoK) match on name, deck count, timing text and effect text.
- **Errata:** The errata for Bribery, Direct Hit, Unstable Planet and Veto are applied.
- **Typos only:** "Chose" in Signal Jamming (`:630`, copied from the wiki) and a missing space ".Then" in Deadly Plot (`:960`).
- **Codex IV:** It added no action cards (its components are relics and galactic events), and Thunder's Edge removed none.

---

## 3. Agendas (`agenda/`)

| Item | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| Shared Research | 🟠 | description | *empty file* | "FOR: Each player's units can move through nebulae. AGAINST: Each player places a command token from their reinforcements in their home system, if able." | `agenda/description/SharedResearch` ✔ |
| Terraforming Initiative | 🟠 | description | *empty file* | "Attach this card to the elected planet's card. The resource and influence values of this planet are increased by 1." | `agenda/description/TerraformingInitiative` ✔ |
| Homeland Defense Act | ⚪ | name | "Homeland Defence Act" | "Homeland Defense Act" | `agenda/mod.rs:437` ✔ |
| Covert Legislation | 🟡 | elect | `ForOrAgainst` | none of its own: the outcomes come from the hidden agenda drawn next | `agenda/mod.rs:681-685` |
| Colonial Redistribution | 🟡 | elect | `PlanetWithTrait` (only checks that the planet has a trait) | "Non-Home Planet Other Than Mecatol Rex" (home planets and Mecatol aren't excluded) | `agenda/mod.rs:567-571, 190-196` |
| Representative Government (PoK) | ⚪ | name | "(PoK)" suffix | no suffix | `agenda/mod.rs:661` (deliberate disambiguation) |

### Verified OK
- **63 agendas:** All match on name, law/directive split, expansion and elect type, apart from the rows above.
- **PoK removals:** `disabled_in_pok()` matches the wiki's 13 PoK-removed agendas exactly.
- **Descriptions:** The other 61 descriptions match.
- **Thunder's Edge:** It added no agendas.

---

## 4. Objectives (`objectives/`)

| Item | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| Raise a Fleet | 🟠 | expansion | Base | PoK | `public.rs:212-217` ✔ |
| Dictate Policy | 🟠 | expansion | Base | PoK (PoK introduced agenda-phase secrets) | `secret.rs:308-313` ✔ |
| Drive the Debate | 🟠 | expansion | Base | PoK | `secret.rs:314-319` ✔ |
| Intimidate Council | ⚪ | condition | "Mectrol Rex's System" | "Mecatol Rex's System" | `public.rs:132` ✔ |
| Amass Wealth | ⚪ | name | "Amass wealth" | "Amass wealth" (the wiki has the same capitalisation; the card is probably "Amass Wealth") | `public.rs:158` |

With the three fixes the split becomes 20 Base / 20 PoK secrets (the code currently has 22/18).

### Codex III Ω secret objectives: not applied (`// TODO` at `secret.rs:75`)

| Objective | Code (original) | Codex III Ω | Location |
|---|---|---|---|
| Fight With Precision | "Use ANTI-FIGHTER BARRAGE to destroy the last of a player's fighters in a system." | "Destroy the last of a player's fighters in the active system during the anti-fighter barrage step." | `secret.rs:140-145` |
| Make an Example of Their World | "Use BOMBARDMENT to destroy the last of a player's ground forces on a planet." | "Destroy the last of a player's ground forces on a planet during the bombardment step." | `secret.rs:86-91` |
| Turn Their Fleets to Dust | "Use SPACE CANNON to destroy the last of a player's ships in a system." | "Destroy the last of a player's **non-fighter** ships in the **active system** during the space cannon offense step." | `secret.rs:98-103` |

### Verified OK
- **Public objectives:** All 40 match on name, stage and VP. Condition texts match apart from the typo above.
- **Secret objectives:** All 40 match on name, 1 VP and scoring phase (12 Action, 26 Status, 2 Agenda).
- **Thunder's Edge and Codex IV:** Neither added or changed objectives.

---

## 5. Relics (`relic/`)

| Item | Sev | Code | Wiki (Thunder's Edge reprint) | Location |
|---|---|---|---|---|
| Dynamis Core | 🟡 | "Purge this card to gain trade goods equal to your **printed** commodity value **+2**." (Codex II) | "ACTION: Gain trade goods equal to your commodity value then purge this card." | `relic/description/DynamisCore` |
| Nano-Forge | 🟡 | (Codex II text) | adds "This action cannot be performed once attached." | `relic/description/NanoForge` |
| JR-XS455-O | ⚪ | "JR-XS455-0" (zero) | Codex page has the letter "O"; Relics and Exploration pages have a zero (the wiki contradicts itself) | `relic/mod.rs:140` |
| The Crown of Emphidia | ⚪ | the word "CrownSymbol" left in the text (from copying a wiki image link) | crown icon | `relic/description/TheCrownOfEmphidia` |

### Verified OK
- **All 23 relics:** All present with the right names and expansion tags (10 PoK, 3 Codex II, 3 Codex IV, 7 Thunder's Edge).
- **Descriptions:** All others match.

---

## 6. Exploration, frontier cards and attachments

| Item | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| Lazax Survivors | 🟠 | name (and the enum `LasaxSurvivors`) | "Lasax Survivors" | "Lazax Survivors" | `planet_attachment.rs:43, 211-212` ✔ |
| Ul, the Progenitor (Geoform) | 🟠 | name | "**UI**, the Progenitor (Geoform)" (capital I) | "**Ul**, the Progenitor" (lowercase L, as in Titans of Ul) | `planet_attachment.rs:237` ✔ |
| Mining World / Rich World | ⚪ | name | "Mining world" / "Rich world" | "Mining World" / "Rich World" | `planet_attachment.rs:214-215` |
| Mirage (frontier card) | 🟡 | `frontier_type` | `Action` | not an ACTION card (you place the token, gain the planet card and purge) | `frontier_card.rs:108` |

**Not modelled, by design:** non-attachment exploration cards (Freelancers, Mercenary Outfit, Abandoned Warehouses, Functioning Base, Local Fabricators, Core Mine, Expedition, Volatile Fuel Source, relic fragments, Gamma Wormhole). Frontier cards have no effect-text field.

### Verified OK
- **Frontier cards:** All 13 unique cards (20 total) match on name, count and expansion.
- **Attachment values:** Every attachment value, trait requirement and tech specialty matches.
- **Thunder's Edge:** It added no exploration cards.

---

## 7. Technologies (`tech.rs`)

### Missing
| Item | Sev | Wiki | Notes |
|---|---|---|---|
| Executive Order (Keleres, Thunder's Edge) | 🟠 | Cybernetic, prerequisite C: "ACTION: Exhaust this card and draw the top or bottom card of the agenda deck. Players immediately vote on this agenda as if you were the speaker; you can spend trade goods and resources on this agenda as if they were votes." | Thunder's Edge replaced the I.I.H.Q. Modernization tech with this |
| 4X41C "Helios" V2 (Last Bastion space dock upgrade) | 🟠 | prerequisites CC | Thunder's Edge faction unit upgrade |
| Linkship II (Ral Nel destroyer upgrade) | 🟠 | prerequisites WW | Thunder's Edge faction unit upgrade |
| Exile II (Crimson Rebellion destroyer upgrade) | 🟠 | prerequisites WW | Thunder's Edge faction unit upgrade |
| Agency Supply Network (Thunder's Edge version) | 🟡 | "Once per action, when you resolve a unit's PRODUCTION ability, you may resolve another of your unit's PRODUCTION abilities in any system." | Only the Codex III text exists |
| Valefar Assimilator X / Y (Nekro) | ⚪ | faction techs with no prerequisites | Probably left out on purpose. They matter for Liberation of Ordinian, where they get replaced |
| Obsidian sides of Neural Parasite / Plane Splitter | ⚪ | flipped text, "cannot be researched" | `tech.rs:1216-1237` (informational) |

### Should be disabled when Thunder's Edge is on
| Item | Sev | Notes |
|---|---|---|
| I.I.H.Q. Modernization (tech) | 🟠 | It's always enabled, so Keleres gets both the tech and the breakthrough with the same name. `is_enabled_in` needs a Thunder's Edge swap list (remove I.I.H.Q. tech, add Executive Order and Agency Supply Network Ω) like the Codex I and IV lists |

### Wrong values
| Item | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| Plane Splitter | 🟠 | prerequisites | 1× Cybernetic | 2× Cybernetic | `:1231` ✔ |
| Magmus Reactor (base) | 🟠 | name | "Magmus Reactor/ Magmus Reactor Ω" | "Magmus Reactor" (the base version shows an Ω label) | `:1165` ✔ |
| I.I.H.Q. Modernization | 🟠 | expansion | Base | Codex III | `:1031` ✔ |
| Agency Supply Network | 🟠 | expansion | Base | Codex III | `:1101` ✔ |
| Non-Euclidean Shielding | ⚪ | name | "Non-Euclid**i**an Shielding" | "Non-Euclid**e**an Shielding" | `:1157` ✔ |
| I.I.H.Q. Modernization | ⚪ | name | "I.I.H.Q Modernization" (the breakthrough is "I.I.H.Q. Modernization") | "I.I.H.Q. Modernization" | `:1027` |
| Super-Dreadnought II | ⚪ | name | no hyphen | hyphenated on the Unit Upgrade and L1Z1X pages; no hyphen on the Technology page | `:724` |

### Effect text: missing second clauses or outdated text
| Item | Sev | Missing or outdated | Location |
|---|---|---|---|
| Predictive Intelligence | 🟠 | missing "When you cast votes during the agenda phase, you may cast 3 additional votes; if you do, and the outcome you voted for is not resolved, exhaust this card." | `:496-498` |
| AI Development Algorithm | 🟡 | missing the PRODUCTION cost-reduction clause | `:536-538` |
| Self Assembly Routines | 🟡 | missing "After 1 of your mechs is destroyed, gain 1 trade good." | `:577-579` |
| Chaos Mapping (Saar) | 🟡 | missing "At the start of your turn during the action phase, you may produce 1 unit in a system that contains at least 1 of your units that has PRODUCTION." | `:946-948` |
| Aerie Hololattice (Argent) | 🟡 | missing "Each planet that contains 1 or more of your structures gains the PRODUCTION 1 ability…" | `:1006` |
| I.I.H.Q. Modernization (tech) | 🟡 | missing "Gain the Custodia Vigilia planet card and its legendary planet ability card…" | `:1032-1034` |
| Hegemonic Trade Policy (Winnu) | 🟡 | old text "…until the end of your turn". Errata: "…during that use of 'Production.'" | `:1073` |
| Strike Wing Alpha II | ⚪ | empty effects (every other upgrade has a summary) | `:805-811` |

### Codex replacement logic (`Technology::is_enabled_in`)
- **Codex I swaps:** The 5 Codex I swaps (Magen, X-89, Magmus, Wormhole Generator, Yin Spinner) are correct.
- **Codex IV / 4.5 ΩΩ swaps:** The ΩΩ swaps (Magen, X-89) are correct. Strictly these came with "Codex 4.5" (13 June 2025), but tying them to `codex_4` is fine.
- **Thunder's Edge swaps:** There are none, but the Keleres changes above need them.
- **Design question:** The Thunder's Edge box includes "74 cards from the Twilight Codices". Should enabling Thunder's Edge imply the Codex content (or at least default the codex toggles to on)? See `roadmap_analysis.md`.

### Verified OK
- **Generic techs:** All 24 match on name, colour, prerequisites and expansion. Their text matches apart from the rows above.
- **Ω/ΩΩ versions:** All match.
- **Unit upgrades:** The 9 generic and 14 existing faction unit upgrades match on prerequisites, faction and expansion.
- **Faction techs:** 44 non-unit faction techs match on faction, colour, prerequisites and expansion, apart from the rows above.
- **Spatial Conduit Cylinder:** Spelled right in the code; most wiki pages misspell it "Spacial".

---

## 8. Strategy cards, breakthroughs, planets, systems, factions, leaders

### Strategy cards (`strategy_card.rs`, images in `packages/ui/src/data/info_context.rs`)
The code only stores names and initiative numbers, which are correct. The changed card texts are only reflected through the card images:

| Card | Change | Source | Reflected? |
|---|---|---|---|
| Diplomacy | Primary readies "up to 2 exhausted planets you control" | **Codex I** (the roadmap lists it under PoK) | ✔ The image switches on `codex_1` |
| Construction | Secondary reworded, same effect | PoK | ✔ The image switches on `prophecy_of_kings` |
| **Construction ΩΩ** | Primary: place a structure **or** use the PRODUCTION of 1 space dock, then place a structure. Secondary: "Spend 1 token from your strategy pool to place 1 structure" (**no longer places a token in a system**) | Thunder's Edge | ❌ No image. `expansions.thunders_edge` is matched but never used (`info_context.rs:141-145`). The secondary hint "place token in system" (`generic_strategy_card.rs:134`) is wrong under Thunder's Edge |
| **Warfare Ω** | Primary: tactical action without placing a command token (the system still counts as activated) and redistribute tokens. Secondary: PRODUCTION in home system | Thunder's Edge | ❌ Always the base image (`info_context.rs:151`) |

### Breakthroughs (`breakthrough/`)
| Item | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| N'orr Supremacy | ⚪ | name | "Norr Supremacy" | "N'orr Supremacy" | `breakthrough/mod.rs:290` ✔ |
| Deorbit Barrage | 🟡 | kind | `Other` | its text starts with "ACTION:" | `breakthrough/mod.rs:133-139` |

Verified OK:
- **Coverage:** All 33 breakthroughs (30 factions plus the flip sides Corsair, The Reaping and Eidolon Maximum) match on faction and synergy pair.
- **Descriptions:** All match word for word.

### Planets (`planet.rs`)
| Item | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| Styx | 🟠 | resources | 3 | **4** | `:968` ✔ |
| Custodia Vigilia Ω | 🟠 | values | 2/3 (Codex III) | **2/2** under Thunder's Edge (now gained through the Keleres breakthrough) | `:786-796` |
| El'Nath | ⚪ | name | "Elnath" | "El'Nath" (the Tiles List also writes "Elnath") | `:1049` |
| Legendary abilities | 🟡 | data model | not modelled | e.g. Styx "A Song Like Marrow", Thunder's Edge "Jupiter Brain", Mecatol Rex Ω "The Galactic Council" | `PlanetInfo` |
| Deepwrought ocean cards (5) | ⚪ | not modelled | 1/1 planet-like cards | Arguably not planets |

Verified OK:
- **Base and PoK:** All 99 base and PoK planets match.
- **Thunder's Edge:** All 43 Thunder's Edge planets match (including space stations, home planets, Mecatol Rex Ω, Thunder's Edge 5/1, Ordinian 0/0 and Avernus 2/0), apart from Styx.

### Systems (`system.rs`)
| Item | Sev | Issue | Location |
|---|---|---|---|
| Tiles 125, 126, 127 | 🔴 | All contain Industrex. They should be 125: Cocytus + egress; 126: Lethe + Phlegethon + egress; 127: empty / Styx / empty. **✅ Fixed** (planets only; egress not modelled) | `:801-818` ✔ |
| Fracture structure | 🟡 | The Fracture is 7 systems on 3 tiles; the code models 3 single systems with no egress | `:801-818` |
| Hyperlanes 119A–124B | 🟠 | missing (12 tile sides) | after `:800` |
| Tile 56 (Empyrean home), tile 92 (Last Bastion home) | 🟡 | also nebulae, but `SystemType` can't be both a home system and an anomaly | `:452-457`, `:623-629` |
| Tile 82 (Wormhole Nexus) | 🟡 | the inactive side (82A, γ only) isn't modelled | `:598-604` |
| `valid_thunders_edge_systems` | 🟡 | excludes `Supernova` but **not `MuaatSupernova`** (tile 81), so it can be picked as a Thunder's Edge spot | `:84-101` ✔ |

Verified OK:
- **Base and PoK tiles:** Tiles 1–81 and PoK hyperlanes 83A–91B match.
- **Thunder's Edge tiles:** Tiles 92–118 match.

### Factions (`common/faction.rs`)
| Item | Sev | Issue | Location |
|---|---|---|---|
| Firmament / Obsidian | 🔴 | Starts with 4 planets (see top issue 3) | `:175-186` ✔ |
| Crimson Rebellion | 🟠 | Should **start with its breakthrough unlocked** (and roll for the Fracture during setup); the code starts every faction at `breakthrough: None` | `state/player.rs` (`NewPlayer::setup`) ✔ |
| Deepwrought Scholarate | ⚪ | Starting-tech choices ignore prerequisites; the card says "research technology twice" | `:333-341` |
| Nekro Virus | ⚪ | Valefar Assimilator X/Y not modelled (see §7) | `:211` |
| Council Keleres | — | Starts with no planets; they're chosen in setup through `SetupTheTribunii`, so this is by design (the TODO at `:176` is stale) | |

Verified OK:
- **Faction list:** 30 factions with correct names and expansions.
- **Starting techs:** Fixed and choice-based starting techs match.
- **Home systems:** The home-system mapping is correct for everything except the Firmament.

### Leaders (`leaders/`)

Missing: Thunder's Edge updates (the Omegas page, Thunder's Edge section)

| Leader | Sev | Thunder's Edge change |
|---|---|---|
| Z'eu ΩΩ (Naalu agent) | 🟠 | New ability: "After any player's command token is placed in a system: You may exhaust this card to return that token to that player's reinforcements." |
| Xxekir Grom ΩΩ (Xxcha hero) | 🟠 | "PLANETARY DEFENSE NEXUS — ACTION: Place any combination of up to 4 PDS or mechs onto planets you control; ready each planet that you place a unit on. Then, purge this card." |
| Brother Milor ΩΩ / Brother Omar ΩΩ / Dannel of the Tenth ΩΩ (Yin) | 🟡 | Rewordings: "during combat"; "ignore **all** prerequisites"; "resolve ground combats…, players cannot use SPACE CANNON…" |
| Odlynn Myrr (Keleres hero) | 🟡 | "For each player that **abstains or** votes for another outcome…" |
| Kuuasi Aun Jalatai (Keleres hero) | ⚪ | "…any combination of up to 2 cruisers or destroyers…" |
| Jol-Nar commander | 🟡 | Thunder's Edge renames "Ta Zern" to **"Agnlan Oln"** (same ability). The code has "Ta-Zern" (with a hyphen) at `commander.rs:198` ✔ |
| Version switching | 🟠 | `Leader::is_enabled_in` only has Codex III swap lists (`leaders.rs:86-115`) |

Wrong values

| Leader | Sev | Field | Code | Wiki | Location |
|---|---|---|---|---|---|
| Dannel of the Tenth Ω | 🟡 | ability name | "Spinner Overdrive Ω" | "QUANTUM DISSEMINATION" | `hero.rs:256` ✔ |
| Sharsiss Hollow | ⚪ | ability name | "Teh Blade Revealed" | "The Blade Revealed" | `hero.rs:326` ✔ |
| M'aban Ω | ⚪ | unlock | "…ground units…Mexatol Rex…" | "…ground forces…Mecatol Rex…" | `commander.rs:169` ✔ |
| Brother Omar Ω | ⚪ | description | "his card satisfies…" | "This card satisfies…" | `description/BrotherOmarCxIII` |
| Field Marshal Mercer | ⚪ | name | "Field Marshall" | "Field Marshal" | `agent.rs:245` ✔ |
| Lyra Keen | ⚪ | ability name | Entity 4X41A Apollo | ENTITY 4X41A "APOLLO" | `hero.rs:333` |
| Riftwalker Meian | ⚪ | description | stray `"` at the end | | `description/RiftwalkerMeian` |

Verified OK:
- **PoK leaders:** All PoK faction leaders match on name, faction, ability kind, unlock and text.
- **Codex III leaders:** These match (apart from the rows above).
- **Thunder's Edge leaders:** All 18 match the faction pages. The wiki's *Leaders* page is out of date for Thunder's Edge, so the faction pages were used.

---

## 9. Other code issues found during the audit

These aren't data-vs-wiki mismatches, but they came up while checking:

| Sev | Issue | Location |
|---|---|---|
| 🟡 | `Score::update_player_points` hard-codes `self.max_points = 10; // TODO set up score`. `GameSettings.max_points` (set in new game / milty) is never used, and nothing in the UI shows the VP target. Low priority: nothing reads it and game end is handled at the table | `state/score.rs:63` ✔ |
| 🟠 | The Political Censure row in the score table renders the heading "Shard of the Throne" (copy-paste). **✅ Fixed.** | `packages/ui/src/views/score_view_mode/score_table_view.rs:612-613` ✔ |
| 🟠 | The **stage II gating** (`expected_objectives_before_stage_two`) counts Incentive Program and Classified Document Leaks among the *laws in play*. Incentive Program is a **directive**, so it's never counted: an extra Stage I revealed through `RevealExtraPublicObjective` makes the status phase demand Stage II one round early. Classified Document Leaks (a law) adds +1, but nothing ever turns the elected secret into a public objective (`RevealExtraPublicObjective` only takes a `PublicObjective`), so the app demands a 6th Stage I instead. Found by reading the code; no test covers it | `state/game_state.rs:608-621`, `game_event_handler.rs:1328-1340` ✔ |
| 🟡 | Heroes are never marked as purged, so a used hero stays in the playable leader list | `state/game_state.rs:649-678` ✔ |
| 🟡 | Tech prerequisites are never validated anywhere. This is fine and keeps things simple, but it means breakthrough synergy needs no logic at all; see the action plan | — |
| 🟡 | The comment `// TODO: Handle Council Keleres` in `get_starting_planets` is stale (setup handles it) | `common/faction.rs:176` |

---

## Wiki inconsistencies (check against the printed cards)

Where the wiki disagrees with itself, the code was **not** counted as wrong:

- **Tiamat / Faunus:** one tech specialty or two? (The Planets page says two; the Thunder's Edge and Legendary pages say one.) The code has one each.
- **Spelling conflicts:**
  - "Mez Lo Orz Fei Zsha" vs "…Pei Zsha"
  - "Rep Lo Orz Qet" vs "…Oet"
  - "Corneeq" vs "Coorneeq"
  - "Ssruu" vs "Clever Clever Ssruu"
  - "JR-XS455-O" vs "-0"
  - "Brilliance" vs "Brillance"
- **Tiles List:** Its values for Shalloq, Arnor/Lor and Tiamat contradict every other page (the code follows the majority).
- **Proxima Targeting VI:** The faction page (which the code follows) and the tech pages word it differently.
- **Pirate Fleet:** "no non-neutral ships" vs "no ships".
- **Out-of-date or wrong pages:** The wiki *Leaders* page is out of date for Thunder's Edge (old Last Bastion hero name, "???" heroes, old Ahk Siever unlock). The Objectives page's Ω footnote mentions "promissory note" (copy-paste).

## Not verified

- **Flavor text:** Not checked.
- **Unit stats:** Not compared; the code only stores summary blurbs.
- **Codex IV cards PDF:** Couldn't be text-extracted; it's image-only.
- **Thunder's Edge wording:** Thunder's Edge pages are recently edited and contain typos, so treat the exact Thunder's Edge wording as medium confidence.
