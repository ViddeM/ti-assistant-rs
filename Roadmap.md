# Roadmap
- ❌ = Feature not yet implemented
- ✅ = Feature Implemented
- 🍈 = Implemented in frontend only
- 🍑 = Implemented in backend only
- 🚱 = Feature is untested or needs polish
- 👺 = OUT OF SCOPE, go away, I have no more spare time to give 😢

## Scope
- The app is used alongside the physical game, so every feature has to be faster than doing the thing at the table.
- **Tracked:** game structure (players, factions, strategy cards, turn order, phases, rounds), every VP source, and board-adjacent state (planets and attachments, techs, relics, breakthroughs, laws, agenda votes, leaders, the Thunder's Edge expedition, time per player, the milty map).
- **Not tracked, by design:** units, fleets and command tokens; trade goods, commodities, exhausted/readied state; cards in hand; movement and adjacency; dice and random draws (players roll or draw at the table and enter the result). Anything that is "secret", we shouldn't have to worry about "leaking" information that is not public to everyone.
- **How effects are handled:** an effect that changes tracked state is modelled. One that touches it rarely gets a shortcut to an existing event. One that only touches untracked things is shown as text.
- **Out of scope (👺):** non-standard game variants (Pax Nomina, Twilight's Fall, Liberation of Ordinian). Galactic events are in scope.
- **Lowest priority:** game end (VP target, winners, tiebreaks, elimination), which is handled at the table.

## Infra
- ✅ Multiple games
- ✅ Multiple WS clients per game
- ✅ Persist gamestate
- ✅ Handle loss of connection

## Base features
Base-game features, whichever expansions are enabled.

### Setup
- ✅ Choose speaker at start of game
- ✅ Reveal initial objectives
- ❌ UI for setting table order
- ❌ Faction-specific setups for planets & techs etc.
  - ✅ Planets
  - ✅ Techs
  - ❌ Show starting techs & units

### Gameplay
- ✅ Track round number
- ✅ Strategy cards (speaker handling etc)
  - ✅ Always display the correct variant (`StrategyCard::version`)
- ✅ Take multiple turns
- ✅ Planets
  - ✅ Resource tracking
- ❌ Action cards
  - ✅ Actions
  - ❌ Other relevant ones
- ❌ Tech actions
- ❌ Status phase
  - ✅ Score objectives
  - ✅ Reveal objectives
  - ❌ Display number of action cards to draw & tokens to receive

### Score
- ✅ Objectives (Stage I and II)
- ✅ Custodians (Mecatol Rex)
- ✅ Manually adding score
- ✅ Secret objectives (normally up to 3)
  - ✅ Score non-status phase objectives (via the Score view)
  - ❌ Contextual buttons in the action/agenda flow (optional)
- ✅ Support for the Throne
- ✅ Imperial
- ❌ Agendas
  - ✅ Mutiny (multiple players, add or remove VPs)
  - ❌ Incentive Program (draws a new objective; manually: reveal the drawn objective in Score → Reveal Objectives)
  - ✅ Classified Document Leaks (make a secret objective public)
  - ✅ Seed of an Empire (give 1 VP to player(s) with most/least victory points)
  - ✅ (LAW) Political Censure (player gains 1 VP and can't play action cards)
  - ✅ Holy Planet of Ixth
  - ✅ Shard of the Throne (agenda)
  - ✅ The Crown of Emphidia (agenda)

### Agenda phase

#### Core functionality
- Prio 0
  - ✅ Reveal agendas
  - ✅ Skip vote / force outcome (must include ability to discard agenda with no effect, i.e. 'Deadly Plot')
- Prio 1
  - ✅ Vote on objectives
  - ❌ Speaker tiebreak
  - ❌ Error handling for invalid agendas (should the frontend filter these away, should the BE provide a filtered list, or should it just report the error?)
- Prio 2
  - 🚱 Secret objective scoring (possible via the Score view)
  - ✅ Track laws in play
  - ✅ Repeal laws
  - ✅ VPs from agenda cards (see Score)
  - ❌ Show available votes (soft limit)
  - ❌ Show voting order (but allow votes in any order)
  - ✅ Veto (can be solved with Undo, but messes up time tracking)

#### Extras (nice to have)
The expansions list their own extras under "Agenda phase extras", tagged with these categories.
- Abilities that give extra votes (_with a soft cap on castable votes, these aren't required_)
  - ❌ Action Card 'Distinguished Councilor' (play after voting)
  - ❌ Action Card 'Bribery' (play after speaker votes)
- Abilities that take away votes (_affected players can abstain instead_)
  - ❌ Nekro Virus 'Galactic Threat'
  - ❌ Action Card 'Assassinate Representative'
  - ❌ Action Card 'Ancient Burial Sites'
  - ❌ 'Rider' Action Cards
- Abilities that affect voting order (_with no limit on voting order, this is not required_): only in the expansions
- Abilities that change the outcome (_can use the force outcome feature instead_)
  - ❌ Action Card 'Confusing Legal Text'
- Abilities that should be shown in the GUI but don't need to affect logic
  - ❌ The Xxcha Kingdom faction veto (_use regular veto_)
  - ❌ Action Card 'Construction Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Diplomacy Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Leadership Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Trade Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Warfare Rider' (_reminder to perform effect_)
- When the vote is over (_can use the "admin view" instead_)
  - ❌ Action Card 'Politics Rider' (_change speaker_)
  - ❌ Action Card 'Imperial Rider' (_1 VP_)
  - ❌ Action Card 'Technology Rider' (_research 1 tech_)
  - ❌ Nekro Virus 'Galactic Threat' (_research 1 tech_)

### Names and descriptions
- ✅ Real names for things (currently we only use the ID for e.g objectives)
  - ✅ Planets
  - ✅ Objectives
  - ✅ Factions
  - ✅ Techs
  - ✅ Agendas
  - ✅ Agenda elect options: for or against, player, planet, objective, agenda
  - ✅ Action cards
- ❌ Descriptions for things
  - ❌ Planets
  - ✅ Objectives
  - ❌ Factions
  - ✅ Techs
  - ✅ Agendas
  - 🍑 Action cards (only the end-of-turn card picker shows the effect)
  - ✅ Strategy cards (card image)

### Time tracking
- ✅ Pause timekeeping
- ✅ Time spent / player
- ❌ Time spent / phase
- ❌ Show breakdown while playing
- ❌ Show breakdown in post-game view
- ❌ Make sure everything works and isn't horribly broken by undo

### Undo and views
- ✅ Undo
  - ❌ Display "Undo event {NAME}" popup when undoing so it's clear for everyone
- ✅ Player-specific view
- ✅ Frontend-only personal views
  - ✅ URL-stored views
    - ✅ The open tab (Score, Techs, …)
  - ✅ Limit views to the current person
- ❌ Backend-tracked personal views
  - ❌ Personal undo

### Map render
- ❌ Creuss portals

### Game end
Lowest priority: game end is handled at the table.
- ❌ Winning the game (when reaching the configured score; the target is hard-coded to 10 and nothing reads it yet)
- ❌ Elimination (needed by Conventions of War Abandoned / Silver Flame edge cases)

## Expansions
Each expansion lists what it adds to the base game. An "X + Y" section holds what X adds to a feature that came with Y, e.g. TE's leaders go under "TE + PoK" since leaders came with PoK.

### Prophecy of Kings
- ✅ Exploration
  - ✅ Exploration cards (attachments only are handled; the rest is intentionally not tracked)
  - ✅ Frontier cards
  - ✅ Map: Mirage
- ❌ Planet attachments
  - ✅ Tracking
  - ✅ Real names
  - ❌ Descriptions
- ✅ Relics
  - ✅ Relics view (give relics to players or take them away)
  - ✅ Descriptions
  - ✅ Shard of the Throne (player with this card has 1 extra VP)
  - ✅ Crown of Emphidia (purge to gain 1 VP)
  - ✅ Map: planet destruction (Stellar Converter; Conventions of War Abandoned uses it too)
- ✅ Factions (argent, empyrean, mahact, naaz-rokha, nomad, titans of ul, vuilrath)
- ❌ Leaders (agents, commanders, heroes)
  - ✅ Agents available from the start
  - ✅ Play ACTION leaders
  - ✅ Descriptions
  - ❌ Track purged heroes (used heroes stay playable)
  - ❌ Leader actions with data (Xxekir Grom, Ul, Rin, Hesh and Prit, UNIT.DSGN.FLAYESH, Dannel …; for now the effects are entered by hand, e.g. Hesh and Prit's relic through the Relics view)
  - ❌ Map: Creuss hero
  - ❌ Map: system destruction (supernova bois)
- ❌ Legendary planet ability descriptions (no ability text in the data yet)
- 👺 Mechs (probably not relevant?)
- 👺 Alliances (use other players' commanders; alliance abilities aren't tracked)
- ✅ Increases player limit to 8
- ✅ 31 new systems
  - ❌ Map: Wormhole Nexus inactive side (82A: γ wormhole only, no Mallice) and flipping it
- ✅ 8 new techs (psychoarchaeology, bio-stims, dark energy tap, sling relay, scanlink drone network, predictive intelligence, ai dev algorithm, self assembly routines)
  - ✅ Second clauses for Predictive Intelligence, AI Development Algorithm, Self Assembly Routines (text only)
- ✅ 40 new objectives
- ✅ 20 new action cards
- ✅ 13 agendas removed
- ✅ 13 new agendas
  - ✅ Minister of Antiques (the elected player gains a relic)
- ✅ Updated Construction strategy card (wording only, the image switches)
- ❌ Agenda phase extras (nice to have)
  - ❌ Argent Flight 'Zeal' (extra votes, voting order)
  - ❌ Technology 'Predictive Intelligence' (extra votes)
  - ❌ Action Card 'Deadly Plot' (takes away votes, changes the outcome)
  - ❌ Action Card 'Confounding Legal Text' (changes the outcome)
  - ❌ Agenda 'Covert Legislation' (_once the vote is over, reveal the drawn agenda directly instead_)
  - ❌ Relic 'Maw of Worlds' (takes away votes)
  - ❌ The Mahact Gene Sorcerers 'Genetic Recombination' (GUI only: the affected player just casts votes normally)
  - ❌ The Nomad 'Future Sight' (GUI only: reminder to take 1 trade good)
  - ❌ The Naalu Collective Agent (GUI only: reminder to look at the agenda deck; TE replaces this agent)
  - ❌ Secret Objective 'Dictate Policy' (when the vote is over: 3 or more laws in play)
  - ❌ Secret Objective 'Drive the Debate' (when the vote is over: player is elected by agenda)

### Codex I
- ✅ Updated techs (magmus reactor, wormhole generator, yin spinner, magen defense grid, x-89)
- 👺 Promissory notes (stymie, war funding, cybernetic enhancements, acquiescence, greyfire; promissory notes aren't tracked)
- ✅ New action cards (blitz, counterstroke, fighter conscription, forward supply base, ghost squad, hack election, harness energy, impersonation, insider info, master plan, plagiarize, rally, reflective shielding, scramble frequency, solar flare, war machine)
- ✅ Updated components
  - ✅ Diplomacy strategy card (image switches on Codex I)
  - ✅ Hyper Metabolism
  - 👺 Gravity rifts, rerolls during combat, retreating infantry (rules for untracked things)
- ❌ Agenda phase extras (nice to have)
  - ❌ Action Card 'Hack Election' (voting order; TE Ω: "you vote last")
  - ❌ Action Card 'Sanction' (GUI only: reminder to perform effect)

### Codex II
- 👺 Pax Nomina (game mode with teams)

#### Codex II + PoK
- ✅ 3 new relics (dynamis core, JR-XS455-O, nano forge)

### Codex III
- ✅ Council Keleres faction
- ✅ Updated secret objectives (fight with precision, make an example of their world, turn their fleets to dust)
- ❌ Agenda phase extras (nice to have)
  - ❌ The Council Keleres Promissory (takes away votes; GUI only: reminder to perform effect)

#### Codex III + PoK
- ✅ Updated leaders (naalu collective agent, commander, xxcha kingdom hero, yin brotherhood agent, commander, hero)
- 👺 Updated mech naalu collective
- ✅ 6 new frontier cards
- ❌ Agenda phase extras (nice to have)
  - ❌ The Council Keleres Hero (extra votes; GUI only: reminder to perform effect)

### Codex IV
- ✅ Separate expansion toggle
- ✅ Updated techs (magen defense grid ΩΩ, x-89 bacterial weapon ΩΩ)
- ✅ Galactic events framework: data, choosing the events on the Creation screen (new game and milty), info modal, in-game display, VP rows, admin page; random picks happen at the table
- Galactic events, tracked
  - ✅ Total War (action: +1 VP)
  - ✅ Age of Exploration (Dark Energy Tap: place the drawn tile on the milty map)
- Galactic events, assisted
  - ✅ Minor Factions (minor faction planets have all traits, alliance reminder, systems placed on the milty map)
- Galactic events, info only
  - ✅ Age of Commerce (share a tech by toggling it in the tech view)
- 👺 Liberation of Ordinian scenario

#### Codex IV + PoK
- ✅ Relics
  - ✅ Circlet of the Void (the effects are not tracked)
  - ✅ Neuraloop (status phase only; ❌ also after objectives revealed outside the status phase)
  - ✅ Book of Latvinia (research prompt for up to 2 techs without prerequisites; its VP is in the Score view)

### Thunder's Edge
- ✅ Breakthroughs for each faction
  - ✅ Gained automatically (expedition, Thunder's Edge planet, The Quantumcore, Brilliance, Rapid Mobilization); the Crimson Rebellion starts with it
  - ✅ Generic actions that we don't track (just a button in the UI): Psychospore, Deorbit Barrage, Resonance Generator end the turn
  - ✅ I.I.H.Q. Modernization (gain planet Custodia Vigilia Ω)
  - ✅ Resonance Generator (generic action, see above)
  - ✅ Visionaria Select (coordinate players & research + gain techs)
  - ✅ Stellar Genesis (gain planet Avernus)
  - 👺 Void Tether (adjacency)
  - ✅ Vaults of the Heir (action, purge tech to gain relic)
  - ✅ N'orr Supremacy (nothing to do: research on the tech page)
  - ✅ Al'Raith Ix Ianovar (the Fracture enters play; ingress tokens 👺)
  - 🍑 Descriptions (in the data, not shown; synergy is only the tech categories)
- 🚱 New factions
  - ✅ Firmament → Obsidian flip
  - ✅ Keleres TE update
  - ✅ Map: Crimson Rebellion home system off the board, next to its gate
  - 🚱 Keleres 'Executive Order': agendas outside the agenda phase (the Laws view can add one)
  - ❌ Firmament "Plots within Plots" (plots instead of VP for another player's secret; Sharsiss edits them)
  - ❌ Deepwrought "Radical Advancement" (swap a tech in the status phase)
  - ❌ Deepwrought ocean cards (resource/influence counter)
  - ✅ Unit upgrades: Helios V2, Linkship II, Exile II (text only)
- ✅ New planets
  - ✅ Map: planet positions on their tiles
- 🚱 The Fracture
  - ✅ In-play toggle; its planets stay off the board until then
  - ✅ Forced roll when a breakthrough is gained
  - ✅ Map: above the board, at full size
  - ❌ Plane Splitter puts it into play
  - ❌ Gain a relic when taking a Fracture planet from the deck
  - ❌ Hide its planets in the unclaimed table and the admin dropdown until in play
  - 👺 Ingress/egress tokens, adjacency
- ✅ Space stations
  - ✅ Can't be elected in agendas
  - ❌ Don't count as planets in planet counts (incl. Last Bastion's Revelation)
- ✅ New anomaly (entropic scars: an Entropic Scar phase before the status phase to gain a faction tech; unit abilities and wormholes 👺)
  - ❌ Reconsider entropic scar visibility / confirmation to avoid missing someone doing tech without being too intrusive
- ✅ Thunder's Edge expedition and planet (commit, pass, completion, placement on the map, admin page)
  - ✅ Map: Thunder's Edge where it was placed
- ✅ Action cards
  - ✅ Ω action cards, replace the Codex I versions when TE is enabled
  - ✅ New action cards (14 unique, 20 in the deck)
  - ✅ Brilliance (gain breakthrough)
  - ✅ Overrule / Strategize (primary / secondary of a readied or unchosen strategy card)
    - ✅ Overrule on Warfare Ω starts a tactical action
  - ✅ Crisis (skip a turn), Puppets on a String (passed player performs an action)
  - ✅ Mercenary Contract (owner returns the planet card)
  - ✅ Rest are not handled by this application
- ❌ Updated codexes: go over all of them!! (Will have to support double-omega cards :sweat_smile:)
- ✅ Updated strategy cards (Construction ΩΩ / Warfare Ω images, Construction secondary hint)
  - ✅ Warfare Ω: the primary is a tactical action, entered in the Warfare card
- ✅ Update milty import
- 👺 Coexistence (map visualization)
- 👺 Twilight's Fall game mode

#### TE + PoK
- ❌ New relic cards, added but some need more handling
  - ✅ The Quantumcore (gaining it gains the breakthrough)
  - ❌ The Silver Flame (roll: a 10 gives +1 VP, otherwise purge your home system and you can't score public objectives; the Fracture enters play)
  - ❌ The Triad ("3 + X" badge in the resources; relic fragments aren't tracked)
  - ✅ Dynamis Core / Nano-Forge: TE wording (Ω versions replace the Codex II cards)
  - 👺 The Obsidian (the secret objective cap isn't tracked, nothing to do)
- ❌ New legendary planet abilities
  - ✅ Thunder's Edge (gaining it gains the breakthrough; "take 1 more action" button at end of turn)
  - ✅ Styx (+1 VP while controlled)
  - ❌ Avernus (Action, just allow it as an option, nothing to do)
  - ❌ Faunus (Gain control of planet when you pass)
  - 👺 Mecatol Rex Ω, Ordinian, Emelpar, Garbozia, Industrex, Tempesta (text only, see Descriptions below)
  - ❌ Descriptions (no ability text in the data yet)
- ❌ Leaders
  - ✅ New agents: Kan Kip Rel (Action), Ahk Ravin (Action)
  - ✅ New commanders
  - ❌ New hero Director Nel (Can unpass...)
  - ❌ New hero Ta Zern (Action: Delete a tech from everyone, the ones that got a tech deleted researches again)
  - ✅ New heroes Sharsiss and Sharsiss Hollow (Action)
  - ✅ Leader updates (Z'eu ΩΩ, Xxekir Grom ΩΩ, Yin ΩΩ, Odlynn Myrr, Kuuasi Aun Jalatai, Agnlan Oln)
- ✅ Map: hyperlane tiles 119A–124B in the system data

#### TE + Codex IV
- Galactic events, tracked
  - ✅ Stellar Atomics (one-shot action, spent players can't vote)
  - ✅ Age of Fighters (Fighter II for everyone at setup)
  - ✅ Advent of the War Sun (War Sun for everyone but Muaat at setup)
  - 🚱 Civilized Society: ✅ all objectives up at setup, ✅ unlimited scoring, ❌ game ends after that round's status phase, influence tiebreak (lowest priority, with game end)
  - ✅ Zealous Orthodoxy (first to 2 secrets: +1 VP)
  - ✅ Monuments to the Ages (monuments → VP)
  - ✅ Cultural Exchange Program (use another faction's leaders; Firmament gets The Obsidian)
  - ✅ Rapid Mobilization (the Fracture and breakthroughs at Start Game; each player picks planets and researches)
  - ✅ Mercenaries for Hire (action that ends the turn)
- Galactic events, assisted
  - ✅ Dangerous Wilds (tech prompt for unowned hazardous planets)
  - ✅ Conventions of War Abandoned (X-89: purge planet)
  - ✅ Hidden Agenda (the speaker enters the votes as they're read out)
  - ✅ Wild, Wild Galaxy (the Stellar Converter works on any planet)
- Galactic events, info only
  - ✅ Call of the Void
  - ✅ Cosmic Phenomenae
  - ✅ Weird Wormholes

## Prettify things
- ❌ Make things pretty (FE)
  - ❌ Add ways of viewing full information in FE (e.g. seeing what techs, action cards etc do rather than just their names)
  - ❌ Add cheatsheets for things, e.g. strategy card abilities
  - ❌ Add "current player/next player" view somehow
- ❌ Make things pretty (BE)

## QA
- ❌ Go over all technologies and see if we have missed any interactions
- ❌ Go over all action cards and see if we have missed any interactions
- ❌ Go over all objective cards and see if we have missed any interactions
- ❌ Go over all planets and see if we have missed any interactions
- ❌ Go over all factions and see if we have missed anything
- ❌ Test play with both this and OG ti-assistant
- ❌ Go over PoK to see if anything has been missed
- ✅ Go over TE to see if anything has been missed (done 2026-10-25; the open items are in the Thunder's Edge section)
- ❌ Missing test scenario: elect a hazardous planet
