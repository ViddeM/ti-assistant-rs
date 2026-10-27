# Roadmap
- ❌ = Feature not yet implemented
- ✅ = Feature Implemented
- 🍈 = Implemented in frontend only
- 🍑 = Implemented in backend only
- 🚱 = Feature is untested or needs polish
- 👺 = OUT OF SCOPE, go away, I have no more spare time to give 😢

> Updated 2026-10-25 from the analysis in [`roadmap_analysis.md`](./roadmap_analysis.md). Item IDs in brackets (e.g. `[TE-C1]`, `[GE]`) point to [`TE_CIV_Action_Plan.md`](./TE_CIV_Action_Plan.md); data errors are listed in [`mismatches.md`](./mismatches.md).

## Known bugs (see mismatches.md §1 and §9)
 - ✅ Fracture tiles 125–127 all contain Industrex; Fracture planets can't be taken in a tactical action
 - ✅ Elect cultural/hazardous/industrial planet: the trait check is inverted (the vote dropdown likely panics)
 - ✅ Firmament starts with 4 planets (tiles 96A + 96B)
 - ✅ Thunder's Edge / Avernus can't be taken in a tactical action (no system) `[E1]`; Custodia Vigilia should be granted through I.I.H.Q. instead `[A4]`
 - ✅ Styx resources 3 → 4
 - ❌ Configured victory points ignored (score hard-codes 10) (lowest priority: game end is handled at the table) `[E5]`
 - ❌ Stage II gating wrong with Incentive Program / Classified Document Leaks
 - ✅ Political Censure score row is labelled "Shard of the Throne"
 - ❌ Wrong expansion tags: Raise a Fleet, Dictate Policy, Drive the Debate (→ PoK); I.I.H.Q. Modernization, Agency Supply Network (→ Codex III)

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
     - 🚱 planets (Firmament bug, see Known bugs)
     - ❌ agents (unclear what this means: clarify or remove)
     - ✅ techs
     - ❌ show starting techs & units
 - ✅ Planets
   - ✅ Resource Tracking
   - ✅ Planet attachments (PoK)
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
     - ✅ Mutiny (multiple players, add or remove VPs)
     - ❌ Incentive Program (draws a new objective). Partial code exists but is wrong (Known bugs)
     - ❌ Classified Document Leaks (make a secret objective public). Partial code exists but is wrong (Known bugs)
     - ✅ Seed of an Empire (1 VP to player(s) with most/least victory points)
     - ✅ (LAW) Political Censure (player gains 1 VP and can't play action cards)
     - ✅ Holy Planet of Ixth
     - ✅ Shard of the Throne (agenda)
     - ✅ The Crown of Emphidia (agenda)
   - ❌ Relics
     - 👺 The Obsidian (we don't track the secret objective cap, so there's nothing to do)
     - ✅ Shard of the Throne (player with this card has 1 extra VP)
     - ✅ Crown of Emphidia (purge to gain 1 VP)
     - ✅ Book of Latvinia (Codex IV)
     - ❌ The Silver Flame (TE) `[TE-D2]`
   - ❌ Legendary planets
     - ✅ Styx (TE) `[TE-C1]`
   - ❌ Galactic event VP sources (Total War, Zealous Orthodoxy, Monuments) `[GE]`
   - ❌ Use the configured victory points (currently hard-coded to 10) (lowest priority: game end is handled at the table) `[E5]`
   - ❌ Winning the game (when reaching the winning score) (lowest priority: game end is handled at the table) `[E5]`
 - ❌ Action cards
   - ✅ Actions
   - ❌ Other relevant ones
     - ✅ Crisis (skip a turn), Puppets on a String (a passed player performs an action) (TE) `[TE-I]`
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
   - ❌ Track purged heroes (used heroes stay playable) `[E6]`
   - ❌ Leader actions with data (Ta Zern, Xxekir Grom, Ul, Rin, …) `[E6]`
 - ✅ Strategy cards (speaker handling etc)
   - 🚱 Always display the correct variant (Diplomacy and Construction ✅, Thunder's Edge Construction ΩΩ and Warfare Ω ❌) `[TE-H]`
 - ✅ Real names for things
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
   - ❌ Planets (incl. legendary planet abilities) `[TE-C7]`
   - ✅ Objectives
   - ❌ Factions
   - ✅ Techs
   - ❌ Planet Attachments
   - ✅ Agendas
   - 🍑 Action Cards
   - ✅ Leaders
   - ✅ Relics
   - ✅ Strategy cards (card image)
   - 🍑 Breakthroughs `[TE-A2]`
   - ❌ Galactic events `[GE]`
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
 - ✅ Planet destruction (Stellar Converter relic)
 - ✅ Mirage
 - ❌ Thunder's Edge planet where it was placed `[TE-J1]`
 - ❌ Hyperlane tiles 119A–124B (TE) missing from the system data
 - 👺 Fracture tiles / ingress tokens / Avernus token / coexistence

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
- ❌ Fix the inverted trait check for elect cultural/hazardous/industrial planet (Known bugs)
- ❌ Space stations can't be elected (TE) `[TE-G1]`

*prio 2*:
- 🚱 Secret objective scoring (possible via the Score view)
- ✅ Track laws in play
- ✅ Repeal laws
- ✅ VPs from agenda cards (see "Score" bullet point)
- ❌ Show available votes (soft limit)
- ❌ Show voting order (but allow votes in any order)
- ✅ Veto (can be solved with Undo, but messes up time tracking)
- ❌ Stellar Atomics: players without a token can't vote (Galactic event) `[GE]`

#### Extras (nice to have)
- Abilities that give extra votes (_with a soft cap on castable votes, these aren't required_)
  - ❌ Argent Flight 'Zeal'
  - ❌ Technology 'Predictive Intelligence'
  - ❌ Action Card 'Distinguished Councillor' (play after voting)
  - ❌ Action Card 'Bribery' (play after speaker votes)
  - ❌ The Council Keleres Hero
  - ❌ Keleres 'Executive Order' (TE): vote on an agenda during the action phase

- Abilities that take away votes (_affected players can abstain instead_)
  - ❌ Nekro Virus 'Galactic Threat'
  - ❌ Action Card 'Assassinate Representative'
  - ❌ Action Card 'Ancient Burial Sites'
  - ❌ Action Card 'Deadly Plot'
  - ❌ 'Rider' Action Cards
  - ❌ Relic: 'Maw of Worlds'
  - ❌ The Council Keleres Promissory

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
  - ❌ Galactic event 'Hidden Agenda' (_hide individual votes until resolved_) `[GE]`

- When vote is over (_can use "admin view" instead_)
  - ❌ Action Card 'Politics Rider' (_change speaker_)
  - ❌ Action Card 'Imperial Rider' (_1 VP_)
  - ❌ Action Card 'Technology Rider' (_research 1 tech_)
  - ❌ Nekro Virus 'Galactic Threat' (_research 1 tech_)
  - ❌ Secret Objective 'Dictate Policy' (_3 or more laws in play_)
  - ❌ Secret Objective 'Drive the Debate' (_player is elected by agenda_)

## Expansions

### Prophecy of Kings
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
 - 👺 Alliances (use other players' commanders)
 - 🚱 40 new objectives (Raise a Fleet / Dictate Policy / Drive the Debate tagged as Base)
 - ✅ 20 new action cards
 - ✅ 13 agendas removed
 - ✅ 13 new agendas
 - ✅ Updated construction strat card (wording only, image switches)
 - ❌ Legendary planet ability texts (Primor, Hope's End, Mallice, Mirage) `[TE-C7]`

### Codex I
 - ✅ Updated techs: (magmus reactor, wormhole generator, yin spinner, magen defense grid, x-89)
 - 👺 Promissory notes (stymie, war funding, cybernetic enhancements, acquiescence, greyfire): promissory notes aren't tracked
 - ✅ New action cards (blitz, counterstroke, fighter conscription, forward supply base, ghost squad, hack election, harness energy, impersonation, insider info, master plan, plagiarize, rally, reflective shielding, scramble frequency, solar flare, war machine)
 - 🚱 Updated components
   - ✅ Diplomacy strat card (image switches on Codex I)
   - ✅ Hyper Metabolism
   - 👺 Gravity rifts, rerolls during combat, retreating infantry (rules for untracked things)

### Codex II
 - 🚱 3 new relics (dynamis core, JR-XS455-O, nano-forge). TE reprinted Dynamis Core and Nano-Forge with new text
 - 👺 Pax Nomina (game mode with teams)

### Codex III
 - ✅ Updated leaders: (naalu collective agent, commander, xxcha kingdom hero, yin brotherhood agent, commander, hero)
 - 👺 Updated mech naalu collective
 - ❌ Updated secret objectives (fight with precision, make an example of their world, turn their fleets to dust)
 - ✅ 6 new frontier cards
 - ✅ Council Keleres faction

### Codex IV
 - ✅ Separate expansion toggle
 - ✅ Updated techs (Codex 4.5): magen defense grid ΩΩ, x-89 bacterial weapon ΩΩ
 - ✅ Relics
   - ✅ Circlet of the Void (the effects are not tracked)
   - ✅ Neuraloop (status phase only, not objectives revealed outside of it) `[CIV-2]`
   - ✅ Book of Latvinia (the techs gained with it are added manually) `[CIV-1]`
 - ✅ Action cards: Codex IV has none
 - ❌ Galactic events: Minor Factions, Total War, Age of Commerce, Age of Exploration (see "Galactic Events") `[GE]`
 - ❌ Liberation of Ordinian scenario `[LO]`

### Thunder's Edge
 - 🚱 Breakthroughs for each faction (assigning them works, the effects below are not handled)
   - ❌ Gain breakthrough helper with on-gain effects + "roll for the Fracture" prompt `[E2, TE-B3]`
   - ❌ Generic breakthrough actions (just a button in the UI) `[TE-A3]`
   - ❌ Crimson Rebellion starts with its breakthrough `[TE-A1]`
   - ❌ I.I.H.Q. Modernization (gain Custodia Vigilia planet) `[TE-A4]`
   - ❌ Stellar Genesis (gain Avernus planet) `[TE-A5]`
   - ❌ Vaults of the Heir (Action: purge tech to gain relic) `[TE-A6]`
   - ❌ Visionaria Select (other players research, you gain the same techs) `[TE-A7]`
   - ❌ Al'Raith Ix Ianovar (the Fracture enters play) `[TE-A8]`
   - 👺 Resonance Generator, Void Tether (breaches / adjacency; generic action button only)
   - 👺 N'orr Supremacy (research from the tech page)
   - ❌ Show breakthrough description + synergy `[TE-A2]`
 - ✅ New relic cards (data)
   - ❌ The Quantumcore (gain breakthrough when gained) `[TE-D1]`
   - ❌ The Silver Flame (roll: +1 VP or lose home system / can't score publics) `[TE-D2]`
   - ❌ The Triad (badge only, relic fragments aren't tracked) `[TE-D3]`
   - ❌ Non-action "gain relic" event (Fracture planets, Vaults, Cultural Exchange …) `[E4]`
 - 🚱 New factions (missing unit upgrades; ✅ Firmament planet bug, ✅ Plane Splitter prereqs)
   - ❌ Firmament "Plots within Plots" (no VP for scoring others' secrets, plots enable Puppets of the Blade) `[TE-F2]`
   - ❌ Deepwrought Radical Advancement tech swap (optional) `[TE-F3]`
   - ❌ Unit upgrades: Helios V2, Linkship II, Exile II (data)
 - ❌ Updates to existing factions
   - ❌ Keleres: Executive Order replaces I.I.H.Q. tech, Agency Supply Network Ω, Custodia Vigilia Ω `[TE-F1]`
   - ❌ Leaders ΩΩ: Naalu agent, Xxcha hero, Yin agent/commander/hero; Keleres heroes; Jol-Nar commander → Agnlan Oln `[TE-E-1]`
 - ✅ New planets
 - 🚱 Space stations (planet cards ✅, can't be elected in agendas ❌) `[TE-G]`
 - ✅ New anomaly (entropic scar, data only)
 - ❌ New legendary planet abilities
   - ❌ Thunder's Edge: gain breakthrough on gain + "Jupiter Brain" button reusing Take Another Turn `[TE-C2]`
   - ✅ Styx (1 extra point as long as you hold the planet) `[TE-C1]`
   - 👺 Avernus (Action, nothing to track)
   - ❌ Faunus (gain control of a planet when you pass) `[TE-C3]`
   - 👺 Mecatol Rex Ω, Ordinian, Emelpar, Garbozia, Industrex, Tempesta (text only)
 - ❌ The Fracture
   - ❌ Fix Fracture tile data (Known bugs)
   - ❌ "Fracture in play" flag + relic prompt when taking Fracture planets `[TE-B]`
 - 🚱 Updated codex content: ΩΩ techs ✅ (Codex IV), Ω action cards ✅, ΩΩ leaders ❌
 - ❌ Updated strategy cards: Construction ΩΩ, Warfare Ω (image + secondary hint) `[TE-H]`
 - ✅ New action cards (14 unique / 20 cards) + Ω versions of 8 Codex I cards, which replace the old ones when TE is enabled
   - ✅ Brilliance (gain breakthrough) `[TE-I]`
   - ✅ Overrule / Strategize (perform another strategy card's primary / secondary) `[TE-I]`
   - ✅ Crisis (skip a turn), Puppets on a String (extra action when passed) `[TE-I]`
   - ✅ Mercenary Contract (the owner returns the planet card) `[TE-I]`
 - ✅ Thunder's Edge expedition (commit, pass, complete, admin page)
 - ✅ Update milty import
 - 👺 Coexistence (map visualization)
 - ❌ New Leaders
   - ✅ Agents
     - ✅ Kan Kip Rel (Action)
     - ✅ Ahk Ravin (Action)
   - ✅ Commanders
   - ❌ Heroes
     - ❌ Director Nel (can unpass…) `[TE-E-2]`
     - ❌ Ta Zern (Action: delete a tech from everyone, the ones that lost a tech research again) `[TE-E-3]`
     - ✅ Sharsiss (Action)
     - ✅ Sharsiss Hollow (Action)
 - ❌ Galactic events: 16 new (see "Galactic Events") `[GE]`
 - ❌ Decide: should ticking Thunder's Edge pre-tick Codex I–IV (the box contains the codex cards)? `[TE-K1]`
 - 👺 Twilight's Fall game mode

### Galactic Events (Codex IV + Thunder's Edge)
 - ❌ Framework: settings, selection (incl. random) in new game / milty, info display, VP rows `[GE 5.0]`
 - Tracked
   - ❌ Stellar Atomics (one-shot action, spent players can't vote)
   - ❌ Age of Fighters (Fighter II for everyone at setup)
   - ❌ Advent of the War Sun (War Sun for everyone but Muaat at setup)
   - ❌ Civilized Society (all objectives up at setup, unlimited scoring, game ends after status phase)
   - ❌ Zealous Orthodoxy (first to 2 secrets: +1 VP)
   - ❌ Total War (action: +1 VP)
   - ❌ Monuments to the Ages (monuments → VP)
   - ❌ Cultural Exchange Program (use another faction's leaders; Firmament gets The Obsidian)
   - ❌ Rapid Mobilization (setup planets, breakthroughs, techs; the Fracture enters play)
 - Assisted
   - ❌ Minor Factions (minor faction planets have all traits, alliance reminder)
   - ❌ Dangerous Wilds (tech prompt for unowned hazardous planets)
   - ❌ Conventions of War Abandoned (X-89: purge planet)
   - ❌ Age of Commerce (share technology shortcut)
   - ❌ Hidden Agenda (hide votes until resolved)
 - Info only: Age of Exploration, Call of the Void, Wild Wild Galaxy, Cosmic Phenomenae, Mercenaries for Hire, Weird Wormholes

## Prettify things
 - ❌ Make things pretty (FE)
   - ❌ Add ways of viewing full information in FE (e.g. seeing what techs, action cards etc does rather than just their names)
   - ❌ Add cheatsheets for things
     - ❌ Strategy card abilities
   - ❌ Add "current player/next player" view somehow
 - ❌ Make things pretty (BE)

## QA
 - ❌ Fix the data mismatches in `mismatches.md`
 - ❌ Go over all technologies and see if we have missed any interactions
 - ❌ Go over all action cards and see if we have missed any interactions
 - ❌ Go over all objective cards and see if we have missed any interactions
 - ❌ Go over all planets and see if we have missed any interactions
 - ❌ Go over all factions and see if we have missed anything
 - ❌ Test play with both this and OG ti-assistant
 - ❌ Go over PoK to see if anything has been missed.
 - 🚱 Go over TE to see if anything has been missed (done in `TE_CIV_Action_Plan.md`)
 - ❌ Missing test scenarios: elect trait planets, take Fracture / Thunder's Edge planets, Incentive Program / Classified Document Leaks stage II gating
