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
   - ❌ Faction specific setups for planets & techs etc.
     - ✅ planets
     - ❌ agents
     - ✅ techs
     - ❌ show starting techs & units
   - ✅🚱 Max points (10 / 14) :: `Score::max_points` is hardcoded to 10, the game setting is ignored.
 - ✅ Planets
   - ✅ Resource Tracking
   - ✅ Planet attachments (PoK)
 - ❌ Score
   - ✅ Objectives (Stage I and II)
     - ❌ Can't score public objectives without controlling all planets in your home system (rule 61.16)
     - ❌ Max 1 objective scored per combat during the action phase (rule 61.7)
   - ✅ Custodians (Mecatol Rex)
   - ✅ Manually adding score
   - ✅🚱 Secret Objectives (normally up to 3) :: The cap of 3 (scored + unscored) is not enforced and status phase secrets are not checked to be of the status phase kind.
     - 🚱 Buttons to score non-status phase objective when they happen. (Manual score buttons exist, no prompts when they happen)
   - ✅🚱 Support for the Throne :: Can be given to oneself (only the FE prevents it), no way to return the card (losing the VP when activating the giver's system or when the giver is eliminated).
   - ✅ Imperial
   - ❌ Agendas
     - ✅ Mutiny (Multiple players, add or remove VPs)
     - ❌ Incentive Programs (Draws new objective cards)
     - ❌ Classified Document Leaks (make a secret objective public)
       - ❌ If discarded the objective is no longer public (FAQ)
     - ✅ Seed of an empire (Give 1 VP to player(s) with most/least victory points)
     - ✅🚱 (LAW) Political Censure (Player gains 1 vp and can't play action cards) :: Not prevented from playing action cards.
     - ✅🚱 Repealing a law removes its VP, should only happen for Political Censure (rule 98.6 & FAQ: Holy Planet of Ixth, Shard of the Throne and The Crown of Emphidia keep their VP)
     - ❌ (LAW) Shard of the Throne / The Crown of Emphidia changing owner (winning combat / taking a planet in the owner's home system)
   - ❌ Relics
     - ❌ Obsidian (Draw and have 1 extra secret objective) :: Note we don't track the cap atm so technically it is not required to be implemented?
     - ✅🚱 Shard of the Throne (Player with this card has 1 extra VP) :: VP is not moved along with the relic when using `SetPlanetOwner`, and gaining the relic when taking a planet in the owner's home system (not only legendary) is not handled.
     - ✅🚱 Crown of Emphidia (Purge to gain 1 VP) :: The relics step is only reached once custodians have been taken, so it can't be purged before that.
   - ❌ Winning the game (when reaching the winning score).
     - ❌ Game ends immediately when a player reaches the winning score, in any phase (rule 98.7)
     - ❌ Simultaneous winners: tiebreak by initiative order (agenda phase: closest to the speaker) (rule 98.7)
     - ❌ Game ends when there are no more public objectives to reveal (rule 61.15)
     - ❌ Can't get more VPs than the winning score (rule 98.4), negative VPs.
   - ❌ Stage I -> Stage II threshold is miscounted (Incentive Program is a directive so is never counted, Classified Document Leaks adds one, manually revealed stage I objectives).
 - ❌ Action cards
   - ✅ Actions
   - ❌ Other relevant ones
 - ❌ Tech actions.
 - ❌ Time tracking
   - ✅ Pause timekeeping
   - ✅ Time spent / player (only during strategy & action phase)
   - ❌ Time spent / phase
   - ❌ Show breakdown while playing
   - ❌ Show breakdown in post-game view
   - ❌ Make sure everything works and isn't horribly broken by undo
 - ✅ Take multiple turns
 - ✅🚱 Separation between base/PoK/codexes (expansion toggles exist, see sub items for what's missing)
   - ✅ PoK
     - ✅ Exploration
       - ✅ Exploration cards (attachments only are handled)
       - ✅ Frontier cards
       - ✅ Relics
       - ❌ Relic fragments
     - ✅ Factions (Argent Flight, Empyrean, Mahact, Naaz-Rokha, Nomad, Titans of Ul, Vuil'raith)
     - ✅ Leaders (agents, commanders, heroes)
     - ❌👺 Mechs (probably not relevant?)
     - ✅ Increases player limit to 8
     - ✅ 31 new systems
     - ✅ 8 new techs (Psychoarchaeology, Bio-Stims, Dark Energy Tap, Sling Relay, Scanlink Drone Network, Predictive Intelligence, AI Development Algorithm, Self-Assembly Routines)
     - ❌ alliances ( use other players commanders)
     - ✅ 40 new objectives
     - ✅ 20 new action cards
     - ✅ 13 agendas removed
     - ✅ 13 new agendas
     - 🍈 updated construction strat card
     - ❌ Legendary planet ability cards (exhaustable, follow the planet) (rule 53)
   - ❌ Codex I
     - ✅ updated techs: (Magmus Reactor Ω, Wormhole Generator Ω, Yin Spinner Ω, Magen Defense Grid Ω, X-89 Bacterial Weapon Ω)
     - ❌ promissory notes (Stymie, War Funding, Cybernetic Enhancements, Acquiescence, Greyfire Mutagen)
     - ✅ new action cards (blitz, counterstroke, fighter conscription, forward supply base, ghost squad, hack election, harness energy, impersonation, insider information, master plan, plagiarize, rally, reflective shielding, scramble frequency, solar flare, war machine)
     - ❌ updated components: (diplo strat card, hyper metabolism, gravity rifts, rerolls during combat, retreating infantry)
       - 🍈 updated diplo strat card
       - ✅ hyper metabolism
   - ❌ Codex II
     - ✅ 3 new relics (PoK), (dynamis core, JR-XS455-O, nano forge)
     - ❌👺 pax nomina (game mode with teams)
   - ❌ Codex III
     - ✅ updated leaders: (naalu collective agent, commander, xxcha kingdom hero, yin brotherhood agent, commander, hero)
     - ❌👺 updated mech naalu collective
     - ❌ updated secret objectives (fight with precision, make an example of their world, turn their fleets to dust) :: Descriptions use the Ω text, but there are no Ω variants / codex toggle.
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
   - ✅ Objectives
   - ❌ Factions
   - ✅ Techs
   - ❌ Planet Attachments
   - ✅ Agendas
   - 🍑 Action Cards
   - ✅ Leaders
   - ✅ Relics
 - ❌ Status phase
   - ✅ Score objectives
   - ✅ Reveal objectives
   - ❌ Display number of action cards to draw & tokens to receive.
 - ❌ Agenda Phase (see below)
 - ❌ Handle game end
 - ❌ Player elimination (rule 33)
   - ❌ Discard the player's laws (e.g. Political Censure, Shard of the Throne, The Crown of Emphidia)
   - ❌ Return their promissory notes (Support for the Throne)
   - ❌ Shuffle their secret objectives back
   - ❌ Pass the speaker token to the left
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
 - ✅ Planet destruction (Stellar converter relic)
 - ✅ Mirage
 - ❌ Wormhole Nexus flip (rule 100.2), nexus is currently always shown, even without PoK
 - ❌ Frontier tokens (e.g. Ion Storm, Gamma Relay wormholes)
 - ❌ Custodians token on Mecatol Rex

### Agenda Phase

#### Core functionality
*prio 0*:
- ✅ Reveal agendas
- 🍑 Skip vote / Force outcome
      (must include ability to discard agenda with no effect, i.e. 'Deadly Plot')
      :: The FE can't resolve before everyone has voted and can't pick 'discard' when there is a clear winner.

*prio 1*:
- ✅ Vote on objectives
- ✅🚱 Vote on planets :: Planet with trait agendas (Holy Planet of Ixth, Core Mining, Demilitarized Zone, Senate Sanctuary, Terraforming Initiative, Research Teams, Colonial Redistribution) can't be voted on as the candidates are created as `AgendaElect::Planet`. The trait checks in `AgendaElect::parse` are also inverted.
- ❌ Speaker tiebreak (also when no votes are cast) (rule 8.19), currently defaults to discard.
- 🍑 Error handling for invalid agendas. (Should frontend filter these away or should the BE provide a filtered list or just report the error?)
  - ❌ New Constitution (discard if no laws in play)

*prio 2*:
- 🍑 Secret objective scoring (manual scoring works, no prompt for 'Dictate Policy' / 'Drive the Debate')
- ✅ Track laws in play
- ✅ Repeal laws
- ✅ VPs from agenda cards (see "Score" bullet point)
- ❌ Show available votes (soft limit)
- ✅ Show voting order (but allow votes in any order)
- ✅ Veto
- ❌ Casting 0 votes should count as abstaining (FAQ), currently counts as a vote (e.g. for Mutiny)
- ❌ Recalculate voting order when the speaker changes during the agenda phase (Public Execution, Politics Rider, Hack Election)
- ❌ Directives / laws that discard other laws (Judicial Abolishment, New Constitution)
- ❌ Covert Legislation (outcomes come from the hidden agenda, currently For / Against)

#### Extras (nice to have)
- Abilities that give extra votes (_with a soft cap on castable votes, these aren't required_)
  - ❌ Argent Flight 'Zeal'
  - ❌ Technology 'Predictive Intelligence'
  - ❌ Action Card 'Distinguished Councilor' (play after voting)
  - ❌ Action Card 'Bribery' (play after speaker votes)
  - ❌ The Council Keleres Hero
  - ❌ The Xxcha Kingdom Commander (1 vote per planet, can't be prevented from voting)
  - ❌ The Emirates of Hacan Commander (trade goods give 2 votes each)
  - ❌ Agenda 'Representative Government' (1 vote each)

- Abilities that take away votes (_affected players can abstain instead_)
  - ✅ Nekro Virus 'Galactic Threat'
  - ❌ Action Card 'Assassinate Representative'
  - ❌ Action Card 'Ancient Burial Sites'
  - ❌ 'Rider' Action Cards
  - ❌ Relic: 'Maw of Worlds'
  - ❌ The Council Keleres Promissory
  - ❌ Agenda 'Public Execution' (also loses the speaker token)

- Abilities that affect voting order (_with no limit on voting order, this is not required_)
  - ✅ Argent Flight 'Zeal'
  - ❌ Action Card 'Hack Election'

- Abilities that change outcome (_can use force outcome feature instead_)
  - ❌ Action Card 'Confusing Legal Text'
  - ❌ Action Card 'Confounding Legal Text'
  - ❌ Action Card 'Deadly Plot'

- Abilities that should be shown in GUI but does not need to affect logic
  - ❌ The Mahact Gene Sorcerers 'Genetic Recombination'
        (_affected player just casts votes normally_)
  - ❌ The Xxcha Kingdom 'Quash' & 'Political Favor' promissory
        (_use regular veto_)
  - ❌ The Xxcha Kingdom Hero (discard a law, resolve 2 agendas)
  - ❌ The Nomad 'Future Sight' (_reminder to take 1 trade good_)
  - ❌ The Naalu Collective Agent (_reminder to look at agenda deck_)
  - ❌ Action Card 'Insider Information' (_reminder to look at agenda deck_)
  - ❌ The Council Keleres Promissory (_reminder to perform effect_)
  - ❌ The Council Keleres Hero (_reminder to perform effect_)
  - ❌ Action Card 'Construction Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Diplomacy Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Leadership Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Trade Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Warfare Rider' (_reminder to perform effect_)
  - ❌ Action Card 'Sanction' (_reminder to perform effect_)

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
 - ❌ Go over PoK to see if anything has been missed.
   - ❌ Verify PoK action card count (17 PoK tagged in code vs 20) & expansion of 'Sanction'
   - ❌ Verify expansion tags (e.g. 'Raise a Fleet', 'Dictate Policy', 'Drive the Debate' are tagged as base)
 - ❌ Go over TE to see if anything has been missed.

## Thunder's Edge
 - ❌  Breakthroughs for each faction
  - ❌  Generic actions that we don't track (just a button in the UI)
  - ❌  I.I.H.Q. Modernization (gain planet)
  - ❌  Resonance Generator (maybe for map purposes? LOW PRIO)
  - ❌  Visionaria Select (Coordinate players & research + gain techs)
  - ❌  Stellar Genesis (gain planet)
  - ❌  Void Tether (I have no idea, maybe for map purposes? Double check)
  - ❌  Vaults of the heir (Action, purge tech to gain relic)
  - ❌  N'orr Supremacy (Can research after winning a combat so litteraly whenever, just do it on the tech page instead).
  - ❌  Al'Raith Ix Ianovar - Fracture enter play (if tracked) also ingress tokens if we want them on the map.
 - ❌  new relic cards, added but some need more handling.
  - ❌  The quantumcore, depends on if we will handle breakthroughs and/or synergies.
  - ❌  The Silver Flame
  - ❌  The Triad (maybe we'll want to show it on the planet page somehow? But we don't track relic fragments...)
 - ❌  new factions (added, missing leaders)
 - ✅  new techs
 - ✅  new planets
 - ❌  new legendary planet abilities
  - ❌  Thunder's Edge (PERFORM 1 ACTION AT THE END OF YOUR TURN??? Can be worked around atm by taking another turn but not very pretty, also if we should handle breakthroughs)
  - ❌  Styx (Have an extra point as long as you hold the planet)
  - ❌  Avernus (Action, just allow it as an option, nothing to do).
  - ❌  Faunus (Gain control of planet when you pass)
 - ❌  fracture? (option for picking relic after taking those planets?)
 - ✅  space stations.
 - ❌  new anomaly?
 - ❌  updated codexes: go over all of them!! (Will have to support double-omega cards :sweat_smile:)
 - ❌  updated strategy cards
 - ❌  thunder's edge
   - ❌  planet
   - ❌  exploration
 - ✅ update milty import
 - ❌  Coexistance (map visualization)

## Codex IV
  - ❌  galactic events (could be difficult!)
  - ❌  relics, added but need handling.
    - ❌ Circlet of the Void
    - ❌ Neuraloop
    - ❌ Book of Latvinia
  - TODO

TODO: Look overall strategy cards to see that we always display the correct variant.
TODO: Display "Undo event {NAME}" popup or smth when undoing so that it is more clear for other players (and oneself).
