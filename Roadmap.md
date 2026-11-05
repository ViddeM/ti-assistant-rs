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

## Gameplay
 - ✅ Track round number
 - ✅ Undo
   - ❌ Display "Undo event {NAME}" popup when undoing so it's clear for everyone
 - ✅ Player-specific view
 - ✅ Update setup to do all required tasks
   - ✅ Choose speaker at start of game
   - ✅ Reveal initial objectives
   - ❌ UI for setting table order
   - ❌ Faction specific setups for planets & techs etc.
     - ✅ planets
     - ✅ agents (available from the start)
     - ✅ techs
     - ❌ show starting techs & units
 - ✅ Planets
   - ✅ Resource Tracking
   - ✅ Planet attachments (PoK)
 - ✅ Relics view (give relics to players or take them away)
 - ❌ Score
   - ✅ Objectives (Stage I and II)
   - ✅ Custodians (Mecatol Rex)
   - ✅ Manually adding score
   - ✅ Secret Objectives (normally up to 3)
     - ✅ Score non-status phase objectives (via the Score view)
       - ❌ Contextual buttons in the action/agenda flow (optional)
   - ✅ Support for the Throne
   - ✅ Imperial
   - ❌ Agendas
     - ✅ Mutiny (Multiple players, add or remove VPs)
     - ❌ Incentive Program (draws a new objective). Manual: reveal the drawn objective in Score → Reveal Objectives
     - ✅ Classified Document Leaks (make a secret objective public)
     - ✅ Seed of an empire (Give 1 VP to player(s) with most/least victory points)
     - ✅ (LAW) Political Censure (Player gains 1 vp and can't play action cards)
     - ✅ Holy Planet of Ixth
     - ✅ Shard of the Throne (agenda)
     - ✅ The Crown of Emphidia (agenda)
   - ✅ Relics
     - 👺 The Obsidian (the secret objective cap isn't tracked, nothing to do)
     - ✅ Shard of the Throne (Player with this card has 1 extra VP)
     - ✅ Crown of Emphidia (Purge to gain 1 VP)
     - ✅ Book of Latvinia (Codex IV)
   - ❌ Winning the game (when reaching the configured score; the target is hard-coded to 10 and nothing reads it yet) (lowest priority: game end is handled at the table).
 - ❌ Action cards
   - ✅ Actions
   - ❌ Other relevant ones
     - ✅ Crisis, Puppets on a String (TE)
 - ❌ Tech actions.
 - ❌ Time tracking
   - ✅ Pause timekeeping
   - ✅ Time spent / player
   - ❌ Time spent / phase
   - ❌ Show breakdown while playing
   - ❌ Show breakdown in post-game view
   - ❌ Make sure everything works and isn't horribly broken by undo
 - ✅ Take multiple turns
 - ❌ Leaders
   - ✅ Play ACTION leaders
   - ❌ Track purged heroes (used heroes stay playable)
   - ❌ Leader actions with data (Ta Zern, Xxekir Grom, Ul, Rin, Hesh and Prit, UNIT.DSGN.FLAYESH, Dannel …; for now the effects are entered by hand, e.g. Hesh and Prit's relic through the Relics view)
 - ❌ Separation between base/PoK/codexes
   - ❌ PoK
     - ✅ Exploration
       - ✅ Exploration cards (attachments only are handled; the rest is intentionally not tracked)
       - ✅ Frontier cards
       - ✅ Relics
     - ✅ Factions (argent, empyrean, mahact, naaz-rokha, nomad, titans of ul, vuilrath)
     - ✅ Leaders (agents, commanders, heroes)
     - 👺 Mechs (probably not relevant?)
     - ✅ Increases player limit to 8
     - ✅ 31 new systems
     - ✅ 8 new techs (psychoarchaeology, bio-stims, dark energy tap, sling relay, scanlink drone network, predictive intelligence, ai dev algorithm, self assembly routines)
       - ❌ Second clauses missing for Predictive Intelligence, AI Development Algorithm, Self Assembly Routines (data)
     - 👺 alliances (use other players' commanders; alliance abilities aren't tracked)
     - ✅ 40 new objectives
     - ✅ 20 new action cards
     - ✅ 13 agendas removed
     - ✅ 13 new agendas
       - ✅ Minister of Antiques (the elected player gains a relic)
     - ✅ updated construction strat card (wording only, the image switches)
   - ✅ Codex I
     - ✅ updated techs: (magmus reactor, wormhole generator, yin spinner, magen defense grid, x-89)
     - 👺 promissory notes (stymie, war funding, cybernetic enhancements, acquiescence, greyfire; promissory notes aren't tracked)
     - ✅ new action cards (blitz, counterstroke, fighter conscription, forward supply base, ghost squad, hack election, harness energy, impersonation, insider info, master plan, plagiarize, rally, reflective shielding, scramble frequency, solar flare, war machine)
     - ✅ updated components
       - ✅ Diplomacy strat card (image switches on Codex I)
       - ✅ Hyper Metabolism
       - 👺 Gravity rifts, rerolls during combat, retreating infantry (rules for untracked things)
   - ✅ Codex II
     - ✅ 3 new relics (PoK), (dynamis core, JR-XS455-O, nano forge)
     - 👺 pax nomina (game mode with teams)
   - ❌ Codex III
     - ✅ updated leaders: (naalu collective agent, commander, xxcha kingdom hero, yin brotherhood agent, commander, hero)
     - 👺 updated mech naalu collective
     - ❌ updated secret objectives (fight with precision, make an example of their world, turn their fleets to dust)
     - ✅ 6 new frontier cards
     - ✅ council keleres faction
 - ✅ Strategy cards (speaker handling etc)
   - ✅ Always display the correct variant (`StrategyCard::version`)
 - ✅ Real names for things (currently we only use the ID for e.g objectives)
   - ✅ Planets
   - ✅ Objectives
   - ✅ Factions
   - ✅ Techs
   - ✅ Planet Attachments
   - ✅ Agendas
   - ✅ Agenda Elect Options
     - ✅ For or Against
     - ✅ Player
     - ✅ Planet
     - ✅ Objective
     - ✅ Agenda
   - ✅ Action Cards
 - ❌ Descriptions for things
   - ❌ Planets
   - ✅ Objectives
   - ❌ Factions
   - ✅ Techs
   - ❌ Planet Attachments
   - ✅ Agendas
   - 🍑 Action Cards (only the end-of-turn card picker shows the effect)
   - ✅ Leaders
   - ✅ Relics
   - ✅ Strategy cards (card image)
   - 🍑 Breakthroughs (description in the data, not shown; synergy is only the tech categories)
   - ❌ Legendary planet abilities (PoK and TE, no ability text in the data yet)
   - ✅ Galactic events
 - ❌ Status phase
   - ✅ Score objectives
   - ✅ Reveal objectives
   - ❌ Display number of action cards to draw & tokens to receive.
 - ❌ Agenda Phase (see below)
 - ❌ Handle game end (lowest priority: game end is handled at the table)
   - ❌ Elimination (needed by Conventions of War Abandoned / Silver Flame edge cases)
 - ❌ Personal views
   - ✅ Frontend only personal views
     - ✅ URL stored views
     - ✅ Limit views to the current person
   - ❌ Backend tracked personal views
     - ❌ Personal undo

## Map Render
 - ❌ Creuss portals
 - ❌ Creuss hero
 - ❌ System destruction (supernova bois)
 - ✅ Planet destruction (Stellar Converter, Conventions of War Abandoned)
 - ✅ Mirage
 - ✅ Thunder's Edge where it was placed
 - ✅ Crimson Rebellion home system off the board, next to its gate
 - ✅ Thunder's Edge planet positions on their tiles
 - ✅ Hyperlane tiles 119A–124B (TE) in the system data
 - ✅ The Fracture above the board, at full size

### Agenda Phase

#### Core functionality
*prio 0*:
- ✅ Reveal agendas
- ✅ Skip vote / Force outcome
      (must include ability to discard agenda with no effect, i.e. 'Deadly Plot')

*prio 1*:
- ✅ Vote on objectives
- ❌ Speaker tiebreak
- ❌ Error handling for invalid agendas. (Should frontend filter these away or should the BE provide a filtered list or just report the error?)

*prio 2*:
- 🚱 Secret objective scoring (possible via the Score view)
- ✅ Track laws in play
- ✅ Repeal laws
- ✅ VPs from agenda cards (see "Score" bullet point)
- ❌ Show available votes (soft limit)
- ❌ Show voting order (but allow votes in any order)
- ✅ Veto (can be solved with Undo, but messes up time tracking)
- 🚱 Agendas outside the agenda phase, e.g. Keleres 'Executive Order' (TE) (the Laws view can add one)

#### Extras (nice to have)
- Abilities that give extra votes (_with a soft cap on castable votes, these aren't required_)
  - ❌ Argent Flight 'Zeal'
  - ❌ Technology 'Predictive Intelligence'
  - ❌ Action Card 'Distinguished Councillor' (play after voting)
  - ❌ Action Card 'Bribery' (play after speaker votes)
  - ❌ The Council Keleres Hero

- Abilities that take away votes (_affected players can abstain instead_)
  - ❌ Nekro Virus 'Galactic Threat'
  - ❌ Action Card 'Assassinate Representative'
  - ❌ Action Card 'Ancient Burial Sites'
  - ❌ Action Card 'Deadly Plot'
  - ❌ 'Rider' Action Cards
  - ❌ Relic: 'Maw of Worlds'
  - ❌ The Council Keleres Promissory
  - ✅ Galactic event 'Stellar Atomics' (players who used their token can't vote)

- Abilities that affect voting order (_with no limit on voting order, this is not required_)
  - ❌ Argent Flight 'Zeal'
  - ❌ Action Card 'Hack Election' (TE Ω: "you vote last")

- Abilities that change outcome (_can use force outcome feature instead_)
  - ❌ Action Card 'Confusing Legal Text'
  - ❌ Action Card 'Confounding Legal Text'
  - ❌ Action Card 'Deadly Plot'

- Abilities that should be shown in GUI but do not need to affect logic
  - ❌ The Mahact Gene Sorcerers 'Genetic Recombination'
        (_affected player just casts votes normally_)
  - ❌ The Xxcha Kingdom faction veto
        (_use regular veto_)
  - ❌ The Nomad 'Future Sight' (_reminder to take 1 trade good_)
  - ❌ The Naalu Collective Agent (_reminder to look at agenda deck_; TE replaces this agent)
  - ❌ The Council Keleres Promissory (_reminder to perform effect_)
  - ❌ The Council Keleres Hero (_reminder to perform effect_)
  - ❌ Action Card 'Construction Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Diplomacy Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Leadership Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Trade Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Warfare Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Sanctions' (_reminder to perform effect_)
  - ✅ Galactic event 'Hidden Agenda' (_the speaker enters the votes as they're read out_)

- When vote is over (_can use "admin view" instead_)
  - ❌ Action Card 'Politics Rider' (_change speaker_)
  - ❌ Action Card 'Imperial Rider' (_1 VP_)
  - ❌ Action Card 'Technology Rider' (_research 1 tech_)
  - ❌ Nekro Virus 'Galactic Threat' (_research 1 tech_)
  - ❌ Secret Objective 'Dictate Policy' (_3 or more laws in play_)
  - ❌ Secret Objective 'Drive the Debate' (_player is elected by agenda_)

## Prettify things
 - ❌ Make things pretty (FE)
   - ❌ Add ways of viewing full information in FE (e.g. seeing what techs, action cards etc does rather than just their names)
   - ❌ Add cheatsheets for things
     - ❌ Strategy card abilities
   - ❌ Add "current player/next player" view somehow
 - ❌ Make things pretty (BE)

## QA
 - ❌ Go over all technologies and see if we have missed any interactions
 - ❌ Go over all action cards and see if we have missed any interactions
 - ❌ Go over all objective cards and see if we have missed any interactions
 - ❌ Go over all planets and see if we have missed any interactions
 - ❌ Go over all factions and see if we have missed anything
 - ❌ Test play with both this and OG ti-assistant
 - ❌ Go over PoK to see if anything has been missed.
 - ✅ Go over TE to see if anything has been missed (done 2026-10-25; the open items are in the Thunder's Edge section)
 - ❌ Missing test scenario: elect a hazardous planet

## Thunder's Edge
 - ✅  Breakthroughs for each faction
  - ✅  Gained automatically (expedition, Thunder's Edge planet, The Quantumcore, Brilliance, Rapid Mobilization); the Crimson Rebellion starts with it
  - ✅  Generic actions that we don't track (just a button in the UI): Psychospore, Deorbit Barrage, Resonance Generator end the turn
  - ✅  I.I.H.Q. Modernization (gain planet Custodia Vigilia Ω)
  - ✅  Resonance Generator (generic action, see above)
  - ✅  Visionaria Select (Coordinate players & research + gain techs)
  - ✅  Stellar Genesis (gain planet Avernus)
  - 👺  Void Tether (adjacency)
  - ✅  Vaults of the heir (Action, purge tech to gain relic)
  - ✅  N'orr Supremacy (nothing to do: research on the tech page)
  - ✅  Al'Raith Ix Ianovar (the Fracture enters play; ingress tokens 👺)
 - ❌  new relic cards, added but some need more handling.
  - ✅  The Quantumcore (gaining it gains the breakthrough)
  - ❌  The Silver Flame
  - ❌  The Triad ("3 + X" badge in the resources; relic fragments aren't tracked)
  - ❌  Dynamis Core / Nano-Forge: TE wording
 - 🚱  new factions
  - ✅  Firmament → Obsidian flip
  - ✅  Keleres TE update
  - ❌  Firmament "Plots within Plots" (plots instead of VP for another player's secret; Sharsiss edits them)
  - ❌  Deepwrought "Radical Advancement" (swap a tech in the status phase)
  - ❌  Deepwrought ocean cards (resource/influence counter)
  - ❌  Unit upgrades: Helios V2, Linkship II, Exile II (data)
 - ✅  new planets
 - ❌  new legendary planet abilities
  - ✅  Thunder's Edge (gaining it gains the breakthrough; "take 1 more action" button at end of turn)
  - ✅  Styx (+1 VP while controlled)
  - ❌  Avernus (Action, just allow it as an option, nothing to do).
  - ❌  Faunus (Gain control of planet when you pass)
  - 👺  Mecatol Rex Ω, Ordinian, Emelpar, Garbozia, Industrex, Tempesta (text only, see Descriptions)
 - 🚱  The Fracture
  - ✅  in-play toggle; its planets stay off the board until then
  - ✅  forced roll when a breakthrough is gained
  - ❌  Plane Splitter puts it into play
  - ❌  gain a relic when taking a Fracture planet from the deck
  - ❌  hide its planets in the unclaimed table and the admin dropdown until in play
  - 👺  ingress/egress tokens, adjacency
 - ✅  space stations.
  - ✅  can't be elected in agendas
  - ❌  don't count as planets in planet counts (incl. Last Bastion's Revelation)
 - 👺  new anomaly (entropic scars: movement and combat)
 - ❌  updated codexes: go over all of them!! (Will have to support double-omega cards :sweat_smile:)
  - ✅  Ω action cards, replace the codex I versions when TE is enabled
  - ✅  new action cards (14 unique, 20 in the deck)
  - ✅  Brilliance (gain breakthrough)
  - ✅  Overrule / Strategize (primary / secondary of a readied or unchosen strategy card)
  - ✅  Crisis (skip a turn), Puppets on a String (passed player performs an action)
  - ✅  Mercenary Contract (owner returns the planet card)
  - ✅  Rest are not handled by this application.
  - ✅  leader updates (Z'eu ΩΩ, Xxekir Grom ΩΩ, Yin ΩΩ, Odlynn Myrr, Kuuasi Aun Jalatai, Agnlan Oln)
 - ✅  updated strategy cards (Construction ΩΩ / Warfare Ω images, Construction secondary hint)
 - ✅  Thunder's Edge expedition and planet (commit, pass, completion, placement on the map, admin page)
 - ✅  update milty import
 - 👺  Coexistence (map visualization)
 - ❌  New Leaders
  - ✅  Agents
    - ✅  Kan Kip Rel (Action)
    - ✅  Ahk Ravin (Action)
  - ✅  Commanders
  - ❌  Heros
    - ❌  Director Nel (Can unpass...)
    - ❌  Ta Zern (Action: Delete a tech from everyone, the ones that got a tech deleted researches again)
    - ✅  Sharsiss (Action)
    - ✅  Sharsiss Hollow (Action)
 - 👺  Twilight's Fall game mode
 - ✅  Galactic events (see "Galactic Events")

## Codex IV
  - ✅  separate expansion toggle
  - ✅  updated techs: (magen defense grid ΩΩ, x-89 bacterial weapon ΩΩ)
  - ✅  relics
    - ✅ Circlet of the Void (the effects are not tracked)
    - ✅ Neuraloop (status phase only, not objectives revealed outside of it)
      - ❌ also after objectives revealed outside the status phase
    - ✅ Book of Latvinia (research prompt for up to 2 techs without prerequisites)
  - ✅  galactic events (see "Galactic Events")
  - 👺  Liberation of Ordinian scenario

## Galactic Events (Codex IV + Thunder's Edge)
 - ✅ Framework: data, choosing the events on the Creation screen (new game and milty), info modal, in-game display, VP rows, admin page; random picks happen at the table
 - Tracked
   - ✅ Stellar Atomics (one-shot action, spent players can't vote)
   - ✅ Age of Fighters (Fighter II for everyone at setup)
   - ✅ Advent of the War Sun (War Sun for everyone but Muaat at setup)
   - 🚱 Civilized Society: ✅ all objectives up at setup, ✅ unlimited scoring, ❌ game ends after that round's status phase, influence tiebreak (lowest priority, with game end)
   - ✅ Zealous Orthodoxy (first to 2 secrets: +1 VP)
   - ✅ Total War (action: +1 VP)
   - ✅ Monuments to the Ages (monuments → VP)
   - ✅ Cultural Exchange Program (use another faction's leaders; Firmament gets The Obsidian)
   - ✅ Age of Exploration (Dark Energy Tap: place the drawn tile on the milty map)
   - ✅ Rapid Mobilization (the Fracture and breakthroughs at Start Game; each player picks planets and researches)
   - ✅ Mercenaries for Hire (action that ends the turn)
 - Assisted
   - ✅ Minor Factions (minor faction planets have all traits, alliance reminder)
   - ✅ Dangerous Wilds (tech prompt for unowned hazardous planets)
   - ✅ Conventions of War Abandoned (X-89: purge planet)
   - ✅ Hidden Agenda (the speaker enters the votes as they're read out)
   - ✅ Wild, Wild Galaxy (the Stellar Converter works on any planet)
 - Info only: Age of Commerce (share a tech by toggling it in the tech view), Call of the Void, Cosmic Phenomenae, Weird Wormholes
