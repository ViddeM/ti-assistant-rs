# Galactic event test games (2026-11-06)

Open at http://127.0.0.1:8080/game/<id>. In the app list they are named `GE: <event> (<stage>)`.

Milty games: draft `20261101_1f062e422dd7a4e2`. Tux = Council Keleres (Tribunii: Mentak), Vidde = Firmament, Yoda = Crimson Rebellion.
Minor Factions games: `1ac9a493` and `09d3557e` are regular (non-milty) games with the same players, factions and colours. `f1a70001` uses the milty import (same draft as the other milty games).

| Tested | Game | Event (stage) | Where it stops / what to try |
|---|---|---|---|
| ✅ | `099a8a18` | Total War (action) | Action, Tux: Component → Galactic Event → Total War |
| ✅ | `100a9fd2` | Mercenaries for Hire (action) | Action, Tux: Component → Galactic Event → Mercenaries for Hire |
| ✅ | `20e0b4a1` | Stellar Atomics (action) | Action, Tux: Component → Galactic Event → Stellar Atomics |
| ✅ | `942cbdd8` | Stellar Atomics (agenda) | Agenda phase, before "Reveal an agenda". Tux used his token in round 1, so once an agenda is revealed he can't vote |
| ✅ | `ccc97b0b` | Conventions of War Abandoned (action) | Tux has X-89 ΩΩ: Component → Galactic Event → choose a planet. Vidde/Yoda should see "Requires X-89" |
|  | `dc46c53a` | Age of Exploration (action) | Tux has Dark Energy Tap: Component → Galactic Event. The form offers Ring 4, positions 15 and 17 (next to systems 94 and 67) |
|  | `03d042d0` | Wild, Wild Galaxy (Stellar Converter) | Tux owns Stellar Converter: Component → Play Relic |
|  | `d4406362` | Dangerous Wilds (tactical action) | Tux: Tactical, take an unowned hazardous planet (Vefut II, Tequ'ran, Xanhact, Cormund, Sakulag, Olergodt) |
|  | `63b0393b` | Monuments to the Ages (build) | Galactic Events page → Build monument. Tux owns Thibah and Accoen, Vidde owns Lazar and Torkan |
|  | `d8bc49b5` | Zealous Orthodoxy (second secret) | Action. Tux and Vidde each have 1 secret scored; the next secret either of them scores should give the VP |
|  | `0bc7835e` | Hidden Agenda (agenda) | Agenda phase, before Reveal |
|  | `185090c7` | Civilized Society (setup) | Setup: "Select all public objectives" |
|  | `1aac2dae` | Civilized Society (status) | Status phase round 1, all 10 objectives revealed, nothing scored, no reveal step |
|  | `7d8a64d9` | Cultural Exchange Program (setup) | Setup: Cultural Exchange dropdowns for Tux and Yoda only (Vidde starts with The Obsidian) |
|  | `260cb545` | Cultural Exchange Program (leaders) | Tux drew Crimson Rebellion: Component → Play Leader → Ahk Ravin |
|  | `d64c88d1` | Rapid Mobilization (start) | Rapid Mobilization screen. No Fracture roll was needed |
|  | `551dc1fe` | Age of Fighters (setup) | Setup, Start Game enabled (grants Fighter II at start) |
|  | `adf05cd3` | Advent of the War Sun (setup) | Setup, Start Game enabled (grants War Sun at start) |
|  | `1ac9a493` | Minor Factions (setup), non-milty | Setup: three Minor faction dropdowns |
|  | `09d3557e` | Minor Factions (alliance), non-milty | Minor factions: Tux Arborec, Vidde Winnu, Yoda Yin. Tux: Tactical → take Nestphar → alliance reminder |
|  | `f1a70001` | Minor Factions (setup), milty | Setup: each player has a Minor faction dropdown, a "Replaces" dropdown and a Place button. Try Tux Arborec over System 20, Vidde Creuss over System 73 and Yoda Winnu over System 28. The column then shows the faction's name, and only Undo changes it. After Start Game, the Map view shows 5, 17 and 7 in those slots and 51 off-board. Mentak isn't offered (Tux's Tribunii home), and neither is Crimson Rebellion (Yoda plays it) |

Skipped (info only): Age of Commerce, Call of the Void, Cosmic Phenomenae, Weird Wormholes.

## Noticed, not changed

- **Conventions of War planet list:** on a milty map the dropdown lists 133 planets, including ones not on this map (Abaddon, Abyz, …). `planet_location` counts any planet that has a system tile as on the board, and the server check uses the same rule, so it would accept those planets too.
- **Keleres "Waiting for other players…":** in `7d8a64d9`, `1ac9a493` and `f1a70001`, Tux's card shows this text even though his techs are already set. `council_keleres_setup.rs` shows Keleres' techs only after the other players finish setup, including their Cultural Exchange / Minor Faction draws. The server accepted the techs earlier. They should appear once the other players' draws are done.

## Screenshots (same folder)

`ge-age-of-exploration.png`, `ge-rapid-mobilization.png`, `ge-cultural-exchange-setup.png`, `ge-minor-factions-setup.png`, `ge-minor-factions-milty-setup.png`, `ge-minor-factions-milty-map.png`, `ge-monuments.png`, `ge-stellar-atomics-agenda.png`, `ge-civilized-society-setup.png`, `ge-import.png`
