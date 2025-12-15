# Roadmap
- ❌ = Feature not yet implemented
- ✅ = Feature Implemented
- 🍈 = Implemented in frontend only
- 🍑 = Implemented in backend only
- 🚱 = Feature is untested or needs polish
- 👺 = OUT OF SCOPE, go away, I have no more spare time to give 😢

## Infra
 - ✅ Multiple games
 - ✅ Multiple WS clients per game
 - ✅ Persist gamestate
 - ✅ Handle loss of connection

## Gameplay
 - ✅ Track round number
 - ✅ Undo
 - ✅ Player-specific view
 - ✅ Update setup to do all required tasks
   - ✅ Choose speaker at start of game
   - ✅ Reveal initial objectives
   - ❌ UI for setting table order
   - ❌ Faction specific setups for planets & techs
     - ✅ planets
     - ❌ agents
     - ✅ techs
     - ❌ show starting techs & units
 - ✅ Planets
   - ✅ Resource Tracking
   - ✅ Planet attachments (PoK)
 - ❌ Score
   - ✅ Objectives (Stage I and II)
   - ✅ Custodians (Mecatroll rex)
   - ✅ Manually adding score
   - ✅ Secret Objectives (normally up to 3)
     - ❌ Buttons to score non-status phase objective when they happen.
   - ✅ Support for the Throne
   - ✅ Imperial
   - ❌ Agendas
     - ✅ Mutiny (Multiple players, add or remove VPs)
     - ❌ Incentive Programs (Draws new objective cards)
     - ❌ Classified Document Leaks (make a secret objective public)
     - ✅ Seed of an empire (Give 1 VP to player(s) with most/least victory points)
     - ✅ (LAW) Political Censure (Player gains 1 vp and can't play action cards)
   - ❌ Relics
     - ❌ Obsidian (Draw and have 1 extra secret objective) :: Note we don't track the cap atm so technically it is not required to be implemented?
     - ✅ Shard of the Throne (Player with this card has 1 extra VP)
     - ✅ Crown of Emphydia (Purge to gain 1 VP)
   - ❌ Winning the game (when reaching the winning score).
 - ❌ Action cards
   - ✅ Actions
   - ❌ Other relevant ones
 - ❌ Tech actions.
 - ❌ Time tracking
   - ✅ Pause timekeeping
   - ✅ Time spent / player
   - ❌ Time spent / phase
   - ❌ Show breakdown while playing
   - ❌ Show breakdown in post-game view
   - ❌ Make sure everything works and isn't horribly broken by undo
 - ✅ Take multiple turns
 - ❌ Separation between base/PoK/codexes
   - ❌ PoK
     - ❌ Exploration
       - ✅ Exploration cards (attachments only are handled)
       - ✅ Frontier cards
       - ✅ Relics
     - ✅ Factions (argent, empyrean, mahact, naaz-rokha, nomad, titans of ul, vuilrath)
     - ✅ Leaders (agents, commanders, heroes)
     - ❌👺 Mechs (probably not relevant?)
     - ✅ Increases player limit to 8
     - ✅ 31 new systems
     - ✅ 8 new techs (psycoarch, bio stims, dark energy tap, sling ray, scanlink drone network, predictive intelligence, ai dev algos, self assembly)
     - ❌ alliances ( use other players commanders)
     - ✅ 40 new objectives
     - ✅ 20 new action cards
     - ✅ 13 agendas removed
     - ✅ 13 new agendas
     - ❌ updated diplo strat card
     - ❌ updated construction strat card
   - ❌ Codex I
     - ❌ updated techs: (magmus reactor, wormhole generator, yin spinner, magan defence grid, x-89)
     - ❌ promissary notes (stymie, war funding, cybernetic enhancements, acquiessence, greyfire)
     - ✅ new action cards (blitz, counterstroke, fighter conscription, forward supply base, ghost squad, hack election, harness energy, impersonation, insider info, master plan, plagiarize, rally, reflective shielding, scramble frequency, solar flare, war machine)
     - ❌ updated components: (diplo strat card, hypermetabolism, gravity rifts, rerolls during combat, retreating infantry)
   - ❌ Codex II
     - ✅ 3 new relics (PoK), (dynamis core, JR-XS455-O, nano forge)
     - ❌👺 pax nomina (game mode with teams)
   - ❌ Codex III
     - ✅ updated leaders: (naalu collective agent, commander, xxcha kingdom hero, yin brotherhood agent, commander, hero)
     - ❌👺 updated mech naalu collective
     - ❌ updated secret objectives (fight with precision, make an example of their world, turn their fleets to dust)
     - ✅ 6 new frontier cards
     - ✅ council keleres faction
 - ✅ Strategy cards (speaker handling etc)
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
   - ❌ Objectives
   - ❌ Factions
   - ❌ Techs
   - ❌ Planet Attachments
   - 🍑 Agendas
   - ❌ Action Cards
   - 🍑 Leaders
 - ❌ Status phase
   - ✅ Score objectives
   - ✅ Reveal objectives
   - ❌ Display number of action cards to draw & tokens to receive.
 - ❌ Agenda Phase (see below)
 - ❌ Handle game end
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
 - ❌ Planet destruction (Stellar converter relic)
 - ✅ Mirage

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
- ❌ Secret objective scoring
- ✅ Track laws in play
- ✅ Repeal laws
- ❌ VPs from agenda cards (see "Score" bullet point)
- ❌ Show available votes (soft limit)
- ❌ Show voting order (but allow votes in any order)
- ✅ Veto (can be solved with Undo, but messes up time tracking)

#### Extras (nice to have)
- Abilities that give extra votes (_with a soft cap on castable votes, these aren't required_)
  - ❌ Argent Flight 'Zeal'
  - ❌ Technology 'Predictive Intelligence'
  - ❌ Action Card 'Distinguished Councillor' (play after voting)
  - ❌ Action Card 'Bribery' (play after speaker votes)
  - ❌ The Council Keleres Hero

- Abilities that take away votes (_affected players can abstain instead_)
  - ❌ Nekro Virus 'Galactic Threat'
  - ❌ Action Card 'Assasinate Representative'
  - ❌ Action Card 'Ancient Burial Sites'
  - ❌ Action Card 'Deadly Plot'
  - ❌ 'Rider' Action Cards
  - ❌ Relic: 'Maw of Worlds'
  - ❌ The Council Keleres Promisary

- Abilities that affect voting order (_with no limit on voting order, this is not required_)
  - ❌ Argent Flight 'Zeal'
  - ❌ Action Card 'Hack Election'

- Abilities that change outcome (_can use force outcome feature instead_)
  - ❌ Action Card 'Confusing Legal Text'
  - ❌ Action Card 'Confounding Legal Text'
  - ❌ Action Card 'Deadly Plot'

- Abilities that should be shown in GUI but does not need to affect logic
  - ❌ The Mahact Gene Sorcerers 'Genetic Recombination'
        (_affected player just casts votes normally_)
  - ❌ The Xxcha Kingdom faction veto
        (_use regular veto_)
  - ❌ The Nomad 'Future Sight' (_reminder to take 1 trade good_)
  - ❌ The Naalu Collective Agent (_reminder to look at agenda deck_)
  - ❌ The Council Keleres Promisary (_reminder to perform effect_)
  - ❌ The Council Keleres Hero (_reminder to perform effect_)
  - ❌ Action Card 'Construction Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Diplomacy Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Leadership Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Trade Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Warfare Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Sanctions' (_reminder to perform effect_)

- When vote is over (_can use "admin view" instead_)
  - ❌ Action Card 'Politics Rider' (_change speaker_)
  - ❌ Action Card 'Imperial Rider' (_1 VP_)
  - ❌ Action Card 'Technology Rider' (_research 1 tech_)
  - ❌ Nekro Virus 'Galactic Threat' (_research 1 tech_)
  - ❌ Secret Objective 'Dictate Policy' (_3 or more laws in play_)
  - ❌ Secret Objective 'Drive the Debate' (_player is elected by agenda_)

## Prettify things
 - ❌ Make things pretty (FE)
   - ❌ Add ways of viewing full information in FE (e.g. seing what techs, action cards etc does rather than just their names)
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
 - x  Go over PoK to see if anything has been missed.

## Thunder's Edge
 - x  Breakthroughs for each faction
 - x  new relic cards
 - x  new factions
 - x  new planets
 - x  new legendary planet abilities
 - x  fracture? (option for picking relic after taking those planets?)
   - x styx planet score
 - x  new anomaly?
 - x  galactic events (could be difficult!)
 - x  updated codexes: go over all of them!! (Will have to support double-omega cards :sweat_smile:)
 - x  updated strategy cards
 - x  thunder's edge
   - x  planet
   - x  exploration
 - x  update milty import
