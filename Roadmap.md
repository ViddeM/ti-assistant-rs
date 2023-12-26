# Roadmap

## Infra
 - [x] Multiple games
 - [x] Multiple WS clients per game
 - [x] Persist gamestate

## Gameplay
 - [ ] Track round number
 - [ ] Choose speaker at start of game
 - [x] Planets
   - [x] Resource Tracking
   - [ ] Planet attachments (PoK)
 - [ ] Score
   - [ ] Objectives (Stage I and II)
   - [ ] Custodians (Mecatroll rex)
   - [ ] Manually adding score
   - [ ] Secret Objectives (normally up to 3)
   - [ ] Support for the Throne
   - [ ] Imperial
   - [ ] Agendas
     - [ ] Mutiny (Multiple players, add or remove VPs)
     - [ ] Incentive Programs (Draws new objective cards)
     - [ ] Classified Document Leaks (make a secret objective public)
     - [ ] Seed of an empire (Give 1 VP to player(s) with most/least victory points)
     - [ ] (LAW) Political Censure (Player gains 1 vp and can't play action cards)
   - [ ] Relics
     - [ ] Obsidian (Draw and have 1 extra secret objective)
     - [ ] Shard of the Throne (Player with this card has 1 extra VP)
     - [ ] Crown of Emphydia (Purge to gain 1 VP)
 - [ ] Action cards
   - [x] Actions
   - [ ] Other relevant ones
 - [ ] Time tracking
   - [x] Time spent / player
   - [ ] Time spent / phase
   - [ ] Show breakdown while playing
   - [ ] Show breakdown in post-game view
 - [ ] Agenda Phase (see below)
 - [ ] Relics
 - [ ] Separation between base/PoK/codexes
 - [x] Strategy cards (speaker handling etc)
 - [ ] Odd faction specific starting planets / tech (e.g argent)
 - [ ] Real names for things (currently we only use the ID for e.g objectives)

### Agenda Phase

#### Core functionality
*prio 0*:
- [ ] Reveal objectives
- [ ] Skip vote / Force outcome
      (must include ability to discard agenda with no effect, i.e. 'Deadly Plot')

*prio 1*:
- [ ] Vote on objectives
- [ ] Speaker tiebreak

*prio 2*:
- [ ] Secret objective scoring
- [ ] Track laws in play
- [ ] VPs from agenda cards (see "Score" bullet point)
- [ ] Show available votes (soft limit)
- [ ] Show voting order (but allow votes in any order)
- [ ] Veto (can be solved with Undo, but messes up time tracking)

#### Extras (nice to have)
- Abilities that give extra votes (_with a soft cap on castable votes, these aren't required_)
  - [ ] Argent Flight 'Zeal'
  - [ ] Technology 'Predictive Intelligence'
  - [ ] Action Card 'Distinguished Councillor' (play after voting)
  - [ ] Action Card 'Bribery' (play after speaker votes)
  - [ ] The Council Keleres Hero

- Abilities that take away votes (_affected players can abstain instead_)
  - [ ] Nekro Virus 'Galactic Threat'
  - [ ] Action Card 'Assasinate Representative'
  - [ ] Action Card 'Ancient Burial Sites'
  - [ ] Action Card 'Deadly Plot'
  - [ ] 'Rider' Action Cards
  - [ ] Relic: 'Maw of Worlds'
  - [ ] The Council Keleres Promisary

- Abilities that affect voting order (_with no limit on voting order, this is not required_)
  - [ ] Argent Flight 'Zeal'
  - [ ] Action Card 'Hack Election'

- Abilities that change outcome (_can use force outcome feature instead_)
  - [ ] Action Card 'Confusing Legal Text'
  - [ ] Action Card 'Confounding Legal Text'
  - [ ] Action Card 'Deadly Plot'

- Abilities that should be shown in GUI but does not need to affect logic
  - [ ] The Mahact Gene Sorcerers 'Genetic Recombination'
        (_affected player just casts votes normally_)
  - [ ] The Xxcha Kingdom faction veto
        (_use regular veto_)
  - [ ] The Nomad 'Future Sight' (_reminder to take 1 trade good_)
  - [ ] The Naalu Collective Agent (_reminder to look at agenda deck_)
  - [ ] The Council Keleres Promisary (_reminder to perform effect_)
  - [ ] The Council Keleres Hero (_reminder to perform effect_)
  - [ ] Action Card 'Construction Rider' (_reminder to perform effect_)
  - [ ] Action Card 'Diplomacy Rider' (_reminder to perform effect_)
  - [ ] Action Card 'Leadership Rider' (_reminder to perform effect_)
  - [ ] Action Card 'Trade Rider' (_reminder to perform effect_)
  - [ ] Action Card 'Warfare Rider' (_reminder to perform effect_)
  - [ ] Action Card 'Sanctions' (_reminder to perform effect_)

- When vote is over (_can use "admin view" instead_)
  - [ ] Action Card 'Politics Rider' (_change speaker_)
  - [ ] Action Card 'Imperial Rider' (_1 VP_)
  - [ ] Action Card 'Technology Rider' (_research 1 tech_)
  - [ ] Nekro Virus 'Galactic Threat' (_research 1 tech_)
  - [ ] Secret Objective 'Dictate Policy' (_3 or more laws in play_)
  - [ ] Secret Objective 'Drive the Debate' (_player is elected by agenda_)

## Prettify things
 - [ ] Make things pretty (FE)
 - [ ] Make things pretty (BE)

## QA
 - [ ] Go over all technologies and see if we have missed any interactions
 - [ ] Go over all action cards and see if we have missed any interactions
 - [ ] Go over all objective cards and see if we have missed any interactions
 - [ ] Go over all planets and see if we have missed any interactions
 - [ ] Go over all factions and see if we have missed anything
 - [ ] Test play with both this and OG ti-assistant
