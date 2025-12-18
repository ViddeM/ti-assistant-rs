use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::common::expansions::Expansion;

use super::phase::Phase;

/// An action card.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, EnumIter)]
#[allow(missing_docs)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub enum ActionCard {
    /* Base */
    AncientBurialSites,
    AssassinateRepresentative,
    Bribery,
    Bunker,
    ConfusingLegalText,
    ConstructionRider,
    CourageousToTheEnd,
    CrippleDefenses,
    DiplomacyRider,
    DirectHit,
    Disable,
    DistinguishedCouncilor,
    EconomicInitiative,
    EmergencyRepairs,
    ExperimentalBattlestation,
    FighterPrototype,
    FireTeam,
    FlankSpeed,
    FocusedResearch,
    FrontlineDeployment,
    GhostShip,
    ImperialRider,
    InTheSilenceOfSpace,
    IndustrialInitiative,
    Infiltrate,
    Insubordination,
    Intercept,
    LeadershipRider,
    LostStarChart,
    LuckyShot,
    ManeuveringJets,
    MiningInitiative,
    MoraleBoost,
    Parley,
    Plague,
    PoliticalStability,
    PoliticsRider,
    PublicDisgrace,
    ReactorMeltdown,
    Reparations,
    RepealLaw,
    RiseOfAMessiah,
    Sabotage,
    Salvage,
    ShieldsHolding,
    SignalJamming,
    SkilledRetreat,
    Spy,
    Summit,
    TacticalBombardment,
    TechnologyRider,
    TradeRider,
    UnexpectedAction,
    UnstablePlanet,
    Upgrade,
    Uprising,
    Veto,
    WarEffort,
    WarfareRider,
    /* Codex */
    Blitz,
    Counterstroke,
    FighterConscription,
    ForwardSupplyBase,
    GhostSquad,
    HackElection,
    HarnessEnergy,
    Impersonation,
    InsiderInformation,
    MasterPlan,
    Plagiarize,
    Rally,
    ReflectiveShielding,
    Sanction,
    ScrambleFrequency,
    SolarFlare,
    WarMachine,
    /* Prophecy of Kings */
    ArchaeologicalExpedition,
    ConfoundingLegalText,
    CoupDetat,
    DeadlyPlot,
    DecoyOperation,
    DiplomaticPressure,
    DivertFunding,
    ExplorationProbe,
    ManipulateInvestments,
    NavSuite,
    RefitTroops,
    RevealPrototype,
    ReverseEngineer,
    Rout,
    Scuttle,
    SeizeArtifact,
    Waylay,
}

/// When an action card can be played.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub enum ActionCardPlay {
    StartOfPhase(Phase),
    Agenda(AgendaStagePlay),
    StatusPhaseReturnStrategyCards,
    Action,
    AfterActionCardIsPlayed,
    AfterStrategyCardIsPlayed,
    NotImplemented,
}

/// When, during the agenda phase, an action card can be played.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(missing_docs)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub enum AgendaStagePlay {
    WhenReveal,
    AfterReveal,
    AfterYouCastVotes,
    AfterSpeakerVotes,
    AfterElected,
    WhenOutcomeResolve,
}

/// All relevant information for an action card.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ActionCardInfo {
    /// Which action card this is in regards to.
    pub card: ActionCard,
    /// The 'pretty' name of the action card.
    pub name: String,
    /// Which expansion the action card came from.
    pub expansion: Expansion,
    /// How many copies of the action card that exists in the action card deck.
    pub num_in_deck: usize,
    /// Human readable text for when the action card can be played.
    pub play_text: String,
    /// When the action card can be played.
    pub play: ActionCardPlay,
    /// What effect happens when the action card can be played in human readable format.
    pub effect: String,
    /// The flavor text of the action card.
    pub flavor_text: String,
}

macro_rules! ai {
    ($card:expr, $name:literal, $exp:expr, $num:literal, $play_text:literal, $effect:literal, $flavor:literal) => {
        ai!(
            $card,
            $name,
            $exp,
            $num,
            $play_text,
            ActionCardPlay::NotImplemented,
            $effect,
            $flavor
        )
    };

    ($card:expr, $name:literal, $exp:expr, $num:literal, $play_text:literal, $play:expr, $effect:literal, $flavor:literal) => {
        ActionCardInfo {
            card: $card,
            name: $name.into(),
            expansion: $exp,
            num_in_deck: $num,
            play_text: $play_text.into(),
            play: $play,
            effect: $effect.into(),
            flavor_text: $flavor.into(),
        }
    };
}

impl ActionCard {
    /// Returns the [ActionCardInfo] for the action card.
    pub fn info(&self) -> ActionCardInfo {
        match self {
            ActionCard::AncientBurialSites => ai!(
                ActionCard::AncientBurialSites,
                "Ancient Burial Sites",
                Expansion::Base,
                1,
                r#"At the start of the agenda phase"#,
                ActionCardPlay::StartOfPhase(Phase::Agenda),
                r#"Choose 1 player. Exhaust each cultural planet owned by that player."#,
                r#"The images depicted a race that Rin had never seen before. Curious. Could it be that this was a race that was exterminated by the Lazax?"#
            ),
            ActionCard::AssassinateRepresentative => ai!(
                ActionCard::AssassinateRepresentative,
                "Assassinate Representative",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"Choose 1 player. That player cannot vote on this agenda."#,
                r#"With a sickening crunch of bone and metal, unit.desgn.FLAYESH extracted its stinger from the blood-drenched skull of the Jol-Nar councilor."#
            ),
            ActionCard::Bribery => ai!(
                ActionCard::Bribery,
                "Bribery",
                Expansion::Base,
                1,
                r#"After the speaker votes on an agenda"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterSpeakerVotes),
                r#"Spend any number of trade goods. For each trade good spent, cast 1 additional vote for the outcome on which you voted."#,
                r#""We think that this initiative would spell disaster for the galaxy, not just the Creuss." Taivra said, quietly slipping Z'eu an envelope. "Don't you agree?""#
            ),
            ActionCard::Bunker => ai!(
                ActionCard::Bunker,
                "Bunker",
                Expansion::Base,
                1,
                r#"At the start of an invasion"#,
                r#"During this invasion, apply -4 to the result of each Bombardment roll against planets you control."#,
                r#"Elder Junn crossed his arms and steadied his breathing. The bombs could not reach them, not this far down. At least, that is what the soldiers had told him."#
            ),
            ActionCard::ConfusingLegalText => ai!(
                ActionCard::ConfusingLegalText,
                "Confusing Legal Text",
                Expansion::Base,
                1,
                r#"When you are elected as the outcome of an agenda"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterElected),
                r#"Choose 1 player. That player is the elected player instead."#,
                r#"Somehow, even after the Council had adjourned, none of them were any closer to understanding the strange and confusing events of the day."#
            ),
            ActionCard::ConstructionRider => ai!(
                ActionCard::ConstructionRider,
                "Construction Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, place 1 space dock from your reinforcements on a planet you control."#,
                r#"The vote was nearly unanimous. The council would provide funding for the restoration. Ciel was furious."#
            ),
            ActionCard::CourageousToTheEnd => ai!(
                ActionCard::CourageousToTheEnd,
                "Courageous to the End",
                Expansion::Base,
                1,
                r#"After 1 of your ships is destroyed during a space combat"#,
                r#"Roll 2 dice. For each result equal to or greater than that ship's combat value, your opponent must choose and destroy 1 of their ships."#,
                r#"Coughing up blood and covered in burns, Havvat collapsed against the rapidly overheating ship core. She smiled. She would be remembered."#
            ),
            ActionCard::CrippleDefenses => ai!(
                ActionCard::CrippleDefenses,
                "Cripple Defenses",
                Expansion::Base,
                1,
                r#"Action: Choose 1 planet."#,
                ActionCardPlay::Action,
                r#"Destroy each PDS on that planet."#,
                r#"Titanic vines burst forth from the ground, wrapping themselves around the system's primary firing cylinder, breaking it free from its moorings, and bringing it crashing down upon the installation below."#
            ),
            ActionCard::DiplomacyRider => ai!(
                ActionCard::DiplomacyRider,
                "Diplomacy Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, choose 1 system that contains a planet you control. Each other player places a command token from their reinforcements in that system."#,
                r#"Ciel evaluated the other senators. Weak, all of them. This was his game to win."#
            ),
            ActionCard::DirectHit => ai!(
                ActionCard::DirectHit,
                "Direct Hit",
                Expansion::Base,
                4,
                r#"After another player's ship uses Sustain Damage to cancel a hit produced by your units or abilities"#,
                r#"Destroy that ship."#,
                r#"(1) There it was! An opening! Neekuaq gestured wildly, a rare display for one normally quite reserved. "Fire the main battery!"(2) The Loncara Ssodu's main battery flared to life, firing a volley directly into the flickering starboard shield of the Letnev dreadnought.(3) For a moment, it looked as if the dreadnought's shield would hold, but a moment later, the ship began to come apart where the attack had pierced its hull.(4) Neekuaq watched, satisfied, as the ship was wracked by a series of explosions from within, huge armored plates and other debris hurtling off into the darkness."#
            ),
            ActionCard::Disable => ai!(
                ActionCard::Disable,
                "Disable",
                Expansion::Base,
                1,
                r#"At the start of an invasion in a system that contains 1 or more of your opponents' PDS units"#,
                r#"Your opponents' PDS units lose Planetary Shield and Space Cannon during this invasion."#,
                r#""Ssruu has met their systems and fixed them." Ssruu dropped a handful of ripped wires and broken circuitry on the pedestal before Q'uesh Sish. "He will await his next task aboard his ship.""#
            ),
            ActionCard::DistinguishedCouncilor => ai!(
                ActionCard::DistinguishedCouncilor,
                "Distinguished Councilor",
                Expansion::Base,
                1,
                r#"After you cast votes on an outcome of an agenda"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterYouCastVotes),
                r#"Cast 5 additional votes for that outcome."#,
                r#"Elder Junn was a wonder to behold. He knew the name of every senator, made small talk effortlessly, and commanded attention when he spoke. Magmus hated everything about him."#
            ),
            ActionCard::EconomicInitiative => ai!(
                ActionCard::EconomicInitiative,
                "Economic Initiative",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Ready each cultural planet you control."#,
                r#"Large sum of Hacan currency flowed freely into the project, and somehow, defying all logic, the returns were even greater."#
            ),
            ActionCard::EmergencyRepairs => ai!(
                ActionCard::EmergencyRepairs,
                "Emergency Repairs",
                Expansion::Base,
                1,
                r#"At the start or end of a combat round"#,
                r#"Repair all of your units that have Sustain Damage in the active system."#,
                r#""What do you mean 'It's fine!?'" Dahla said, nearly tripping over a damaged bulkhead as the ship rocked from the explosive barrage."#
            ),
            ActionCard::ExperimentalBattlestation => ai!(
                ActionCard::ExperimentalBattlestation,
                "Experimental Battlestation",
                Expansion::Base,
                1,
                r#"After another player moves ships into a system during a tactical action"#,
                r#"Choose 1 of your space docks that is either in or adjacent to that system. That space dock uses Space Cannon 5 (x3) against ships in the active system."#,
                r#""Arm the relay!" Mendosa bared his fangs. "The Saar will not submit!""#
            ),
            ActionCard::FighterPrototype => ai!(
                ActionCard::FighterPrototype,
                "Fighter Prototype",
                Expansion::Base,
                1,
                r#"At the start of the first round of a space combat"#,
                r#"Apply +2 to the result of each of your fighters' combat rolls during this combat round."#,
                r#"Suffi vaulted excitedly off the prototype's sleek, azure wing, landing effortlessly in its cockpit. "Let's see those damn snakes keep up with us now!""#
            ),
            ActionCard::FireTeam => ai!(
                ActionCard::FireTeam,
                "Fire Team",
                Expansion::Base,
                1,
                r#"After your ground forces make combat rolls during a round of ground combat"#,
                r#"Reroll any number of your dice."#,
                r#"Grinning, Jael spun towards the familiar noise of heavy boots hitting the ground. Backup had arrived."#
            ),
            ActionCard::FlankSpeed => ai!(
                ActionCard::FlankSpeed,
                "Flank Speed",
                Expansion::Base,
                4,
                r#"After you activate a system"#,
                r#"Apply +1 to the move value of each of your ships during this tactical action."#,
                r#"(1) Mendosa smirked. "Don't ye fret now, girl. It sounds like ye've got somewhere ta be in a hurry, and it just se happens that I ken get ye there right quick."(2) There was a low-pitched hum throughout the station and the gigantic, bulky constructions on the hull spat blue flame. Massive ion thrusters! So this was how the Saar moved the damned things.(3) Mendosa grinned at the young Mentak pilot's astonishment. "Mobility-the Clan's greatest strength. Ye'd be amazed at how fast this thing ken go."(4) With a sound like thunder and a flash of neon blue light, the thrusters accelerated full bore, nearly knocking Suffi off her feet."#
            ),
            ActionCard::FocusedResearch => ai!(
                ActionCard::FocusedResearch,
                "Focused Research",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Spend 4 trade goods to research 1 technology."#,
                r#"Stunned, Rin looked over the data, scarcely able to believe what he was seeing. It was nothing short of a major breakthrough. The Headmaster would be pleased."#
            ),
            ActionCard::FrontlineDeployment => ai!(
                ActionCard::FrontlineDeployment,
                "Frontline Deployment",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Place 3 infantry from your reinforcements on 1 planet you control."#,
                r#"The soldiers poured fire on the writhing mass, but as each tendril vaporized, two more took its place. Panic wormed into the troopers' minds; was the creature growing?"#
            ),
            ActionCard::GhostShip => ai!(
                ActionCard::GhostShip,
                "Ghost Ship",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Place 1 destroyer from your reinforcements in a non-home system that contains a wormhole and does not contain other players' ships."#,
                r#"Reports of Creuss vessels sighted in the area had surely been exaggerated. After all, the Creuss had no business this far from the Shaleri passage, and usually kept to themselves."#
            ),
            ActionCard::ImperialRider => ai!(
                ActionCard::ImperialRider,
                "Imperial Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, gain 1 victory point."#,
                r#"In a sweeping victory that surprised absolutely no one, the Winnu representative singlehandedly elected himself to oversee the analysis of a recently uncovered Lazax data cache."#
            ),
            ActionCard::InTheSilenceOfSpace => ai!(
                ActionCard::InTheSilenceOfSpace,
                "In The Silence Of Space",
                Expansion::Base,
                1,
                r#"After you activate a system"#,
                r#"Choose 1 system. During this tactical action, your ships in the chosen system can move through systems that contain other players' ships."#,
                r#"The Barony ships seemed to defy reality, bending light around them at impossible angles - naught but inky-black contours could be seen."#
            ),
            ActionCard::IndustrialInitiative => ai!(
                ActionCard::IndustrialInitiative,
                "Industrial Initiative",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Gain 1 trade good for each industrial planet you control."#,
                r#"The Gashlai reactors proved to be extremely effective. With the conversion rate practically doubled, the factories' output was immense."#
            ),
            ActionCard::Infiltrate => ai!(
                ActionCard::Infiltrate,
                "Infiltrate",
                Expansion::Base,
                1,
                r#"When you gain control of a planet"#,
                r#"Replace each PDS and space dock that is on that planet with a matching unit from your reinforcements."#,
                r#"The 1X had scarcely interfaced with the mainframe before it belonged to them completely."#
            ),
            ActionCard::Insubordination => ai!(
                ActionCard::Insubordination,
                "Insubordination",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Remove 1 token from another player's tactic pool and return it to their reinforcements."#,
                r#"Kilik had never been particularly good at following orders. It only seemed natural to get paid for exercising that trait, as long as she didn't think to hard about where the money was coming from."#
            ),
            ActionCard::Intercept => ai!(
                ActionCard::Intercept,
                "Intercept",
                Expansion::Base,
                1,
                r#"After your opponent declares a retreat during a space combat"#,
                r#"Your opponent cannot retreat during this round of space combat."#,
                r#"The space before the fleeing ship shimmered and warped, revealing the bow of the Y'sia Yssrila bearing down on its position. There would be no escape today."#
            ),
            ActionCard::LeadershipRider => ai!(
                ActionCard::LeadershipRider,
                "Leadership Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, gain 3 command tokens."#,
                r#"Rev's impassioned speech stunned the other councilors. Perhaps there was more to these humans than they had initially thought."#
            ),
            ActionCard::LostStarChart => ai!(
                ActionCard::LostStarChart,
                "Lost Star Chart",
                Expansion::Base,
                1,
                r#"After you activate a system"#,
                r#"During this tactical action, systems that contain alpha and beta wormholes are adjacent to each other."#,
                r#"The ship's sensors showed that the ancient star-map had somehow led them beyond the Mahact plateau."#
            ),
            ActionCard::LuckyShot => ai!(
                ActionCard::LuckyShot,
                "Lucky Shot",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Destroy 1 dreadnought, cruiser, or destroyer in a system that contains a planet you control."#,
                r#"The missile - an insignificant spark against the darkness - struck the hull of the dreadnought. Moments later, the starboard flank erupted in flames."#
            ),
            ActionCard::ManeuveringJets => ai!(
                ActionCard::ManeuveringJets,
                "Maneuvering Jets",
                Expansion::Base,
                4,
                r#"Before you assign hits produced by another player's Space Cannon roll"#,
                r#"Cancel 1 hit."#,
                r#"(1) Something was wrong. T'esla sensed a flicker of movement on the planet's surface, followed by a faint glint of light. Reflexively, she veered her fighter to the side - an action that saved her life.(2) The fighter's thruster boosted her just out of the cannon's firing solution, and a metal slug the size of a building rocketed past her crystalline ship.(3) T'esla breathed a sigh of relief as the massive slug disappeared into the distance. A touch slower and she would have been annihilated.(4) The flickering red warning lights on her control panel reflected in her scales as she regained her composure. "Well," she said aloud. "Zat was close.""#
            ),
            ActionCard::MiningInitiative => ai!(
                ActionCard::MiningInitiative,
                "Mining Initiative",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Gain trade goods equal to the resource value of 1 planet you control."#,
                r#"To support its voracious war machine, the N'orr empire cracked the planet's crust and began mining its very core."#
            ),
            ActionCard::MoraleBoost => ai!(
                ActionCard::MoraleBoost,
                "Morale Boost",
                Expansion::Base,
                4,
                r#"At the start of a combat round"#,
                r#"Apply +1 to the result of each of your unit's combat rolls during this combat round."#,
                r#"(1) Harrugh stood before his warriors, searching for the words that would express the pride that swelled within him.(2) "We have done the impossible. I am proud to call you my brothers. My equals. My betters." Harrugh's whiskers bristled with energy as he spoke.(3) "Today we turn these invaders to ash and take back what is ours!" Harrugh paused, noting the murmurs of approval rippling through the ranks.(4) The whispering gradually died down and his warriors watched Harrugh expectantly. Every muscle in his body tensed, and he thrust his gyro-spear skyward. "For Kenara!""#
            ),
            ActionCard::Parley => ai!(
                ActionCard::Parley,
                "Parley",
                Expansion::Base,
                1,
                r#"After another player commits units to land on a planet you control"#,
                r#"Return the committed units to the space area."#,
                r#"The human admiral returned to his ship, struggling to remember precisely how his meeting with the Collective had gone."#
            ),
            ActionCard::Plague => ai!(
                ActionCard::Plague,
                "Plague",
                Expansion::Base,
                1,
                r#"Action: Choose 1 planet that is controlled by another player."#,
                ActionCardPlay::Action,
                r#"Roll 1 die for each infantry on that planet. For each result of 6 or greater, destroy 1 of those units."#,
                r#"The letani dipped a mossy limb beneath the surface of the reservoir, feeling the flow of the particles within, shifting them, changing them into something different. Something dangerous."#
            ),
            ActionCard::PoliticalStability => ai!(
                ActionCard::PoliticalStability,
                "Political Stability",
                Expansion::Base,
                1,
                r#"When you would return your strategy card(s) during the status phase"#,
                ActionCardPlay::StatusPhaseReturnStrategyCards,
                r#"Do not return your strategy card(s). You do not choose strategy cards during the next strategy phase."#,
                r#"The Winnu councilor breathed a sigh of relief. This opportunity would not be wasted. The peace of the Lazax was within grasp."#
            ),
            ActionCard::PoliticsRider => ai!(
                ActionCard::PoliticsRider,
                "Politics Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, draw 3 action cards and gain the speaker token."#,
                r#"The Dirzuga had managed to negotiate themselves into a position of power amongst the other councilors, all of whom still looked terribly uncomfortable in their presence."#
            ),
            ActionCard::PublicDisgrace => ai!(
                ActionCard::PublicDisgrace,
                "Public Disgrace",
                Expansion::Base,
                1,
                r#"When another player chooses a strategy card during the strategy phase"#,
                r#"That player must choose a different strategy card instead, if able."#,
                r#"In the months that followed the report, the Council was quick to censure the Jol-Nar for their unethical practices."#
            ),
            ActionCard::ReactorMeltdown => ai!(
                ActionCard::ReactorMeltdown,
                "Reactor Meltdown",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Destroy 1 space dock in a non-home system."#,
                r#"Flames engulfed the shipyard, a vortex of fire and shrapnel that left nothing untouched."#
            ),
            ActionCard::Reparations => ai!(
                ActionCard::Reparations,
                "Reparations",
                Expansion::Base,
                1,
                r#"After another player gains control of a planet you control"#,
                r#"Exhaust 1 planet that player controls and ready 1 planet you control."#,
                r#"Gila oversaw the reconstruction of the Hacan outpost, but neglected to inform the N'orr representative that the damage was not nearly as severe as he was led to believe."#
            ),
            ActionCard::RepealLaw => ai!(
                ActionCard::RepealLaw,
                "Repeal Law",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Discard 1 law from play."#,
                r#"Eventually, the N'orr diplomat came to an agreement with Hacan Garrus, and the embargo was repealed. Still, he couldn't shake the feeling that Garrus had gotten the better end of the deal."#
            ),
            ActionCard::RiseOfAMessiah => ai!(
                ActionCard::RiseOfAMessiah,
                "Rise of a Messiah",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Place 1 infantry from your reinforcements on each planet you control."#,
                r#"Harrugh stood atop the dunes, looking down at the thousands of Hacan warriors mustered below. Gathering his strength, he let loose a mighty roar, the will of his people throughout the galaxy echoing forth from within."#
            ),
            ActionCard::Sabotage => ai!(
                ActionCard::Sabotage,
                "Sabotage",
                Expansion::Base,
                4,
                r#"When another player plays an action card other than "Sabotage""#,
                ActionCardPlay::AfterActionCardIsPlayed,
                r#"Cancel that action card."#,
                r#"(1) Q'uesh Sish tapped her claws impatiently. "You were sssaying?" But the confused Jol-Nar envoy stood silent. It was as if the words had escaped his thoughts entirely.        (2) trjn.desgn.ALIZARIN introduced itself to each data nodes, incorporating them into its platform, expunging any data it deemed a threat to the Virus.        (3) unit.desgn.BEELZEBUL followed the group of men through the winding hallways until they stopped at an intersection. Investigators would later find its cracked oculus at the center of a smoking crater, singed, but still functional.        (4) Unlenn smashed a clenched fist into the bulkhead, green eyes ablaze with rage. The entire plan had been turned on its head. Such incompetence was unfathomable."#
            ),
            ActionCard::Salvage => ai!(
                ActionCard::Salvage,
                "Salvage",
                Expansion::Base,
                1,
                r#"After you win a space combat"#,
                r#"Your opponent gives you all of their commodities."#,
                r#"When the Coalition raiders had finally returned to their ships, scarcely a bolt remained to salvage from the destroyed vessel."#
            ),
            ActionCard::ShieldsHolding => ai!(
                ActionCard::ShieldsHolding,
                "Shields Holding",
                Expansion::Base,
                4,
                r#"Before you assign hits to your ships during a space combat"#,
                r#"Cancel up to 2 hits."#,
                r#"(1) T'ro stared unflinching at the incoming barrage through the bridge's observation deck. The shield would hold.(2) The blast from the enemy's main cannon slammed into the space in front of the warship, splitting into two streams across the bow.(3) The ship's shields sizzled and sparked but remained active despite the heavy assault.(4) As the assault died down and it became clear that the ship sustained no major damage, T'ro clicked his mandibles in anticipation. "Return fire!""#
            ),
            ActionCard::SignalJamming => ai!(
                ActionCard::SignalJamming,
                "Signal Jamming",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Chose 1 non-home system that contains or is adjacent to 1 of your ships. Place a command token from another player's reinforcements in that system."#,
                r#"Meian was practically aglow, wisps of her ether crackling with delight. It was amazing what could be achieved with a well-timed electromagnetic discharge."#
            ),
            ActionCard::SkilledRetreat => ai!(
                ActionCard::SkilledRetreat,
                "Skilled Retreat",
                Expansion::Base,
                4,
                r#"At the start of a combat round"#,
                r#"Move all of your ships from the active system into an adjacent system that does not contain another player's ships; the space combat ends in a draw. Then, place a command token from your reinforcements in that system."#,
                r#"(1) In an instant, the Sol fleet vanished from the Barony's scanners.        (2) The Creuss fleet was gone. No trace remained of their passage.        (3) T'ro gave the order, and the entire fleet withdrew without question.        (4) "We have no choice, captain." Ciel looked nervous."We must retreat.""#
            ),
            ActionCard::Spy => ai!(
                ActionCard::Spy,
                "Spy",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Choose 1 player. That player gives you 1 random action card from their hand."#,
                r#""I'm no spy," Connor said, casually stuffing the data drives into his pack. "Now, how can I get out of here without being seen?""#
            ),
            ActionCard::Summit => ai!(
                ActionCard::Summit,
                "Summit",
                Expansion::Base,
                1,
                r#"At the start of the strategy phase"#,
                ActionCardPlay::StartOfPhase(Phase::Strategy),
                r#"Gain 2 command tokens."#,
                r#"At the end of the twelfth day, Rev returned home from the summit in New Moscow feeling physically exhausted and mentally drained. But she was also satisfied. They had a plan - finally - and it was a good plan."#
            ),
            ActionCard::TacticalBombardment => ai!(
                ActionCard::TacticalBombardment,
                "Tactical Bombardment",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Choose 1 system that contains 1 or more of your units that have Bombardment. Exhaust each planet controlled by other players in that system."#,
                r#"A6's red eyes flickered with activity and a string of numbers flashed briefly across the screen before he spoke. "Bombardment complete. All enemy installations have been neutralized.""#
            ),
            ActionCard::TechnologyRider => ai!(
                ActionCard::TechnologyRider,
                "Technology Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, research 1 technology."#,
                r#"The Council granted the Yin funding to research an effective countermeasure to combat the virus. After all, keeping the Yin occupied with the virus meant killing two birds with one stone."#
            ),
            ActionCard::TradeRider => ai!(
                ActionCard::TradeRider,
                "Trade Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, gain 5 trade goods."#,
                r#"The Letnev councilor begrudgingly accepted the N'orr's core mining contract, if only so that the "gracious" Hacan senator didn't secure yet another source of income for his clan."#
            ),
            ActionCard::UnexpectedAction => ai!(
                ActionCard::UnexpectedAction,
                "Unexpected Action",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Remove 1 of your command tokens from the game board and return it to your reinforcements."#,
                r#"M'aban's voice was barely a whisper, cracking as hundreds of flashing lights appeared on her scanner. "We've been tricked.""#
            ),
            ActionCard::UnstablePlanet => ai!(
                ActionCard::UnstablePlanet,
                "Unstable Planet",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Choose 1 hazardous planet. Exhaust that planet and destroy up to 3 infantry on it."#,
                r#"Another quake shook the planet, and a wave of dirt and rock rose up to swallow the Tekklar legion."#
            ),
            ActionCard::Upgrade => ai!(
                ActionCard::Upgrade,
                "Upgrade",
                Expansion::Base,
                1,
                r#"After you activate a system that contains 1 or more of your ships"#,
                r#"Replace one of your cruisers in that system with one of your dreadnoughts from your reinforcements."#,
                r#""More guns?" Connor shook his head. "Not more guns. Let's try something bigger.""#
            ),
            ActionCard::Uprising => ai!(
                ActionCard::Uprising,
                "Uprising",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Exhaust 1 non-home planet controlled by another player. Then gain trade goods equal to its resource value."#,
                r#"Harrugh leapt down from the ledge, gyro-spear blazing in the sunlight. In front, the L1Z1X began to take notice. Behind his warriors stood at the ready. Win or lose, this would be the end."#
            ),
            ActionCard::Veto => ai!(
                ActionCard::Veto,
                "Veto",
                Expansion::Base,
                1,
                r#"When an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::WhenReveal),
                r#"Discard that agenda and reveal 1 agenda from the top of the deck. Players vote on this agenda instead."#,
                r#"Magmus batted the now charred corpse aside with the back of his golden gauntlet. "The Muuat reject your proposal.""#
            ),
            ActionCard::WarEffort => ai!(
                ActionCard::WarEffort,
                "War Effort",
                Expansion::Base,
                1,
                r#"As an Action"#,
                ActionCardPlay::Action,
                r#"Place 1 cruiser from your reinforcements in a system that contains 1 or more of your ships."#,
                r#"The N'orr may not be the most proficient shipwrights, but the willingness of their citizens to do their part far exceeds that of the other great races."#
            ),
            ActionCard::WarfareRider => ai!(
                ActionCard::WarfareRider,
                "Warfare Rider",
                Expansion::Base,
                1,
                r#"After an agenda is revealed"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, place 1 dreadnought from your reinforcements in a system that contains 1 or more of your ships."#,
                r#"Elder Junn sighed. "Disarmament, it seems, has fallen out of fashion in these dark days.""#
            ),
            /* Codex */
            ActionCard::Blitz => ai!(
                ActionCard::Blitz,
                "Blitz",
                Expansion::Codex,
                1,
                r#"At the start of an invasion:"#,
                r#"Each of your non-fighter ships in the active system that do not have BOMBARDMENT gain BOMBARDMENT 6 until the end of the invasion."#,
                r#""T-these weren't meant to be used like this. T-this is a t-terrible idea." Tai mumbled, wringing his hands. "Well 'course it is," Dart grinned, slamming his fist down on the release lock. "But we've got about sixty kilotons of 'surprise' and nothing to lose!""#
            ),
            ActionCard::Counterstroke => ai!(
                ActionCard::Counterstroke,
                "Counterstroke",
                Expansion::Codex,
                1,
                r#"After a player activates a system that contains 1 of your command tokens:"#,
                r#"Return that command token to your tactic pool."#,
                r#""That's the thing about loyalty - " Viktor mused, turning one of the pieces backward, facing its own side. "It can be bought.""#
            ),
            ActionCard::FighterConscription => ai!(
                ActionCard::FighterConscription,
                "Fighter Conscription",
                Expansion::Codex,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Place 1 fighter from your reinforcements in each system that contains 1 or more of your space docks or units that have capacity; they cannot be placed in systems that contain other players' ships."#,
                r#"T'esla grinned as her shimmering vessel lifted off the flight deck - at last, that damned Viscount would experience the superiority of Naalu dogfighting. "Get ready," she hissed into the comms."#
            ),
            ActionCard::ForwardSupplyBase => ai!(
                ActionCard::ForwardSupplyBase,
                "Forward Supply Base",
                Expansion::Codex,
                1,
                r#"After another player activates a system that contains your units:"#,
                r#"Gain 3 trade goods. Then, choose another player to gain 1 trade good."#,
                r#"All eyes turned toward the darkening skies as the only surviving supply truck rolled through the gates. "Get these checked in and send the last shipment to our allies!" Harrugh yelled over the din of the engines."#
            ),
            ActionCard::GhostSquad => ai!(
                ActionCard::GhostSquad,
                "Ghost Squad",
                Expansion::Codex,
                1,
                r#"After a player commits units to land on a planet you control:"#,
                r#"Move any number of ground forces from any planet you control in the active system to any other planet you control in the active system."#,
                r#"The L1 advance team was expecting to find the outpost abandoned. What they found - unfortunately for them - was Connor."#
            ),
            ActionCard::HackElection => ai!(
                ActionCard::HackElection,
                "Hack Election",
                Expansion::Codex,
                1,
                r#"After an agenda is revealed:"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"During this agenda, voting begins with the player to the right of the speaker and continues counterclockwise."#,
                r#"YOUR [NULL ID] STATELY GAME WILL BE FOR NAUGHT<<[VAR:42687] ORDER WILL ARISE FROM [VAR:89001] CHAOS<< IT IS OUR HAND THAT CONTROLS THE OUTCOME"#
            ),
            ActionCard::HarnessEnergy => ai!(
                ActionCard::HarnessEnergy,
                "Harness Energy",
                Expansion::Codex,
                1,
                r#"After you activate an anomaly:"#,
                r#"Replenish your commodities."#,
                r#""Manipulating the cosmos was a power reserved for the gods." Rowl spat at the ground as his contraption whirred to life. "Not anymore.""#
            ),
            ActionCard::Impersonation => ai!(
                ActionCard::Impersonation,
                "Impersonation",
                Expansion::Codex,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Spend 3 influence to draw 1 secret objective."#,
                r#""Also," Connor continued, bemused. "Someone just handed me this as I was leaving. I think it was meant for our dear friend Sucaban. We should be able to put it to good use.""#
            ),
            ActionCard::InsiderInformation => ai!(
                ActionCard::InsiderInformation,
                "Insider Information",
                Expansion::Codex,
                1,
                r#"After an agenda is revealed:"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"Look at the top card of the agenda deck."#,
                r#"M'aban yawned. Another filibuster. Politics could be so flavorless at times. Besides, she already knew how this one was going to turn out. Not well. Not well at all..."#
            ),
            ActionCard::MasterPlan => ai!(
                ActionCard::MasterPlan,
                "Master Plan",
                Expansion::Codex,
                1,
                r#"After you perform an action:"#,
                ActionCardPlay::AfterActionCardIsPlayed,
                r#"You may perform an additional action this turn."#,
                r#"Viktor laced his long fingers with a wicked smile. Unlenn would be proud. The tactician's reputation was well-earned; even pawns could unseat kings."#
            ),
            ActionCard::Plagiarize => ai!(
                ActionCard::Plagiarize,
                "Plagiarize",
                Expansion::Codex,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Spend 5 influence and choose a non-faction technology owned by 1 of your neighbors; gain that technology."#,
                r#"The Yssaril spies handed over their prize to Connor, who in turn passed it to the Sol engineers with a grim smile. "Now then, boys," he said, one hand on his rifle, "about those loose ends...""#
            ),
            ActionCard::Rally => ai!(
                ActionCard::Rally,
                "Rally",
                Expansion::Codex,
                1,
                r#"After you activate a system that contains another player's ships:"#,
                r#"Place 2 command tokens from your reinforcements in your fleet pool."#,
                r#"The Federation advance would not be halted - not today. Claire set her broadcast to play on all open channels. She began, her voice fierce and unwavering "Friends! On this day, our names shall be inscribed in the pages of legend!""#
            ),
            ActionCard::ReflectiveShielding => ai!(
                ActionCard::ReflectiveShielding,
                "Reflective Shielding",
                Expansion::Codex,
                1,
                r#"When one of your ships uses SUSTAIN DAMAGE during combat:"#,
                r#"Produce 2 hits against your opponent's ships in the active system."#,
                r#"Prit scrambled up Hesh's shoulder, gleefully clapping as the entropic shielding tore the mercenary cruisers apart."#
            ),
            ActionCard::Sanction => ai!(
                ActionCard::Sanction,
                "Sanction",
                Expansion::Codex,
                1,
                r#"After an agenda is revealed:"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterReveal),
                r#"You cannot vote on this agenda. Predict aloud an outcome of this agenda. If your prediction is correct, each player that voted for that outcome returns 1 command token from their fleet supply to their reinforcements."#,
                r#"Two can play at this game."#
            ),
            ActionCard::ScrambleFrequency => ai!(
                ActionCard::ScrambleFrequency,
                "Scramble Frequency",
                Expansion::Codex,
                1,
                r#"After another player makes a BOMBARDMENT, SPACE CANNON, or ANTI-FIGHTER BARRAGE roll:"#,
                r#"That player rerolls all of their dice."#,
                r#""I will not accept this from that worthless scum!" Feng growled as his enigmatic companion interfaced with the ship's nav suite. The Memoria's position flickered and shifted, and the blast flashed harmlessly by."#
            ),
            ActionCard::SolarFlare => ai!(
                ActionCard::SolarFlare,
                "Solar Flare",
                Expansion::Codex,
                1,
                r#"After you activate a system:"#,
                r#"During this movement, other players cannot use SPACE CANNON against your ships."#,
                r#"Meian braced for combat as the fleet entered the Ordinian system, but there were no Valefar droneships waiting to meet them. The solar flare gambit had succeeded - they were completely undetected."#
            ),
            ActionCard::WarMachine => ai!(
                ActionCard::WarMachine,
                "War Machine",
                Expansion::Codex,
                4,
                r#"When 1 or more of your units use PRODUCTION:"#,
                r#"Apply +4 to the total PRODUCTION value of your units and reduce the combined cost of the produced units by 1."#,
                r#"(1) "This would take weeks to replace if you had run it backwards," Varish called as she rotated the massive capacitor bank. "Okay, fire it up."(2) The station vibrated as the device sputtered to life, emitting a high-pitched buzz to Varish's ears. "Micro-wormholes are cycling at 19 kilohertz," Cole read. "The entropic field tap's stable."(3) The station's production facilities spooled up to new intensities, lights flaring as the assembly line drones increased their speeds to keep up.(4) "Varish, let the Commanders know they can contact the Tetrarchy. Tell them the fighter squadron assembly will be done ahead of schedule, and under budget.""#
            ),
            /* Prophecy of Kings */
            ActionCard::ArchaeologicalExpedition => ai!(
                ActionCard::ArchaeologicalExpedition,
                "Archaeological Expedition",
                Expansion::ProphecyOfKings,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Reveal the top 3 cards of an exploration deck that matches a planet you control; gain any relic fragments that you reveal and discard the rest."#,
                r#"Hiari pressed the activation glyphs, and the ancient door hissed open. As she stepped into the pitch-black interior of the Mahact tomb, she felt a shiver of anticipation. Perhaps, finally, she would find the Codex."#
            ),
            ActionCard::ConfoundingLegalText => ai!(
                ActionCard::ConfoundingLegalText,
                "Confounding Legal Text",
                Expansion::ProphecyOfKings,
                1,
                r#"When another player is elected as the outcome of an agenda:"#,
                ActionCardPlay::Agenda(AgendaStagePlay::AfterElected),
                r#"You are the elected player instead."#,
                r#""Unfortunately," Cinnabian said in a level voice, "the Galactic Council requires the dispute be arbitrated by a truly neutral party. As the Titans were still slumbering when your war began, we will oversee your negotiations... instead of your friends on Jord.""#
            ),
            ActionCard::CoupDetat => ai!(
                ActionCard::CoupDetat,
                "Coup d'Etat",
                Expansion::ProphecyOfKings,
                1,
                r#"When another player would perform a strategic action:"#,
                ActionCardPlay::AfterActionCardIsPlayed,
                r#"End that player's turn; the strategic action is not resolved and the strategy card is not exhausted."#,
                r#"Artuno shoved the slug pistol into Huro's face as armored mercenaries stormed into the office. "I'm afraid the station's undergoing a change in management.""#
            ),
            ActionCard::DeadlyPlot => ai!(
                ActionCard::DeadlyPlot,
                "Deadly Plot",
                Expansion::ProphecyOfKings,
                1,
                r#"During the agenda phase, when an outcome would be resolved:"#,
                ActionCardPlay::Agenda(AgendaStagePlay::WhenOutcomeResolve),
                r#"If you voted for or predicted another outcome, discard the agenda instead; the agenda is resolved with no effect and it is not replaced.Then, exhaust all of your planets."#,
                r#"The hooded envoy erupted into a nightmare of black metal, lunging at the shocked diplomats."#
            ),
            ActionCard::DecoyOperation => ai!(
                ActionCard::DecoyOperation,
                "Decoy Operation",
                Expansion::ProphecyOfKings,
                1,
                r#"After another player activates a system that contains 1 or more of your structures:"#,
                r#"Remove up to 2 of your ground forces from the game board and place them on a planet you control in the active system."#,
                r#"Enemy fire lashed the abandoned positions on the ridgeline as Trilossa's flock slipped unseen around their flanks."#
            ),
            ActionCard::DiplomaticPressure => ai!(
                ActionCard::DiplomaticPressure,
                "Diplomatic Pressure",
                Expansion::ProphecyOfKings,
                4,
                r#"When an agenda is revealed:"#,
                ActionCardPlay::Agenda(AgendaStagePlay::WhenReveal),
                r#"Choose another player; that player must give you 1 promissory note from their hand."#,
                r#"(1) The Arborec emissary leaned close, and Brother Milor flinched at the faint whiff of decay. "You do not understand," Dirzuga Ohao rasped. "You must support this measure."(2) Brother Milor shook his head. "The Brotherhood cannot support your war against this extra-dimensional threat." Dirzuga Ohao raised an eyebrow. "Are you certain?"(3) "It would be unfortunate," the Arborec Dirzuga said, "if your supplies of diraxium were to dry up. How would your fleet defend you if we stopped supplying its fuel?"(4) "Damn you!" Brother Milor hissed. "You leave us no choice but to fight in your war." Dirzuga Ohao smiled, dead skin stretching over her teeth."#
            ),
            ActionCard::DivertFunding => ai!(
                ActionCard::DivertFunding,
                "Divert Funding",
                Expansion::ProphecyOfKings,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Return a non-unit upgrade, non-faction technology that you own to your technology deck. Then, research another technology."#,
                r#""I am sorry, Academician," Arbiter Berekon shrugged. "I am sure psychoarchaeology is an interesting subject, but the supreme leader feels your budget would be better spent on more militaristic research.""#
            ),
            ActionCard::ExplorationProbe => ai!(
                ActionCard::ExplorationProbe,
                "Exploration Probe",
                Expansion::ProphecyOfKings,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Explore a frontier token that is in or adjacent to a system that contains 1 or more of your ships."#,
                r#"With a dull thud, the probe shot from the cruiser and accelerated into the writhing maelstrom of the gravity rift. Admiral DeLouis paced on the bridge. Hopefully, this time the probe would return."#
            ),
            ActionCard::ManipulateInvestments => ai!(
                ActionCard::ManipulateInvestments,
                "Manipulate Investments",
                Expansion::ProphecyOfKings,
                1,
                r#"At the start of the Strategy Phase:"#,
                ActionCardPlay::StartOfPhase(Phase::Strategy),
                r#"Place a total of 5 trade goods from the supply on strategy cards of your choice; you must place these tokens on at least 3 different cards."#,
                r#""But my good fellow," the envoy sputtered. "Surely you can't expect me to agree to those prices!" Durruq leaned in close. "Check again. You may find your goods are trading for less than you think.""#
            ),
            ActionCard::NavSuite => ai!(
                ActionCard::NavSuite,
                "Nav Suite",
                Expansion::ProphecyOfKings,
                1,
                r#"After you activate a system:"#,
                r#"During the "Movement" step of this tactical action, ignore the effects of anomalies."#,
                r#""The field is too thick!" Sikosk gasped. Trrakan's plume flared. "Not for me, and not for Iruth." A flick of his talon, and the destroyer flipped and dove for the asteroid field."#
            ),
            ActionCard::RefitTroops => ai!(
                ActionCard::RefitTroops,
                "Refit Troops",
                Expansion::ProphecyOfKings,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Choose 1 or 2 of your infantry on the game board; replace each of those infantry with mechs."#,
                r#"Sek'kus leapt from the trenches, the plasma projectors on her Valkyrie ekoskeleton howling as they scoured the field ahead of her."#
            ),
            ActionCard::RevealPrototype => ai!(
                ActionCard::RevealPrototype,
                "Reveal Prototype",
                Expansion::ProphecyOfKings,
                1,
                r#"At the start of a combat:"#,
                r#"Spend 4 resources to research a unit upgrade technology of the same type as 1 of your units that is participating in this combat."#,
                r#"Varish and Cole completed the final linkages, activating the field harvester. The cruiser's beams roared with renewed power, cutting through the enemy ship."#
            ),
            ActionCard::ReverseEngineer => ai!(
                ActionCard::ReverseEngineer,
                "Reverse Engineer",
                Expansion::ProphecyOfKings,
                1,
                r#"When another player discards an action card that has a component action:"#,
                r#"Take that action card from the discard pile."#,
                r#"Xuange spread their hands over the captured harvester. Mirroring their careful gestures, dark energy fields unfolded and began delicately disassembling the device."#
            ),
            ActionCard::Rout => ai!(
                ActionCard::Rout,
                "Rout",
                Expansion::ProphecyOfKings,
                1,
                r#"At the start of the "Announce Retreats" step of space combat, if you are the defender:"#,
                r#"Your opponent must announce a retreat, if able."#,
                r#"Vuil'raith fleshships swarmed the weary defenders, sending unease and sweeping through the telepathic links. But Z'eu projected a powerful feeling of calm, and the Naalu fighters began to drive the Cabal back toward the rift."#
            ),
            ActionCard::Scuttle => ai!(
                ActionCard::Scuttle,
                "Scuttle",
                Expansion::ProphecyOfKings,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Choose 1 or 2 of your non-fighter ships on the game board and return them to your reinforcements; gain trade goods equal to the combined cost of those ships."#,
                r#"Rear Admiral Farran saluted as he watched the blasted starship being towed to the scrap yard. "You served me well," he whispered. "Now you can serve the Barony once more.""#
            ),
            ActionCard::SeizeArtifact => ai!(
                ActionCard::SeizeArtifact,
                "Seize Artifact",
                Expansion::ProphecyOfKings,
                1,
                r#"As an Action:"#,
                ActionCardPlay::Action,
                r#"Choose 1 of your neighbors that has 1 or more relic fragments. That player must give you 1 relic fragment of your choice."#,
                r#"Armored dropships crashed into the main dock. Hatches blew open, and Captain Mentarion and her commando raider charged into the facility's heart."#
            ),
            ActionCard::Waylay => ai!(
                ActionCard::Waylay,
                "Waylay",
                Expansion::ProphecyOfKings,
                1,
                r#"Before you roll dice for ANTI-FIGHTER BARRAGE:"#,
                r#"Hits from this roll are produced against all ships (not just fighters)."#,
                r#"The destroyers sprayed fire with their secondary batteries as they advanced. The Mahact dreadnought began to list, fire and atmosphere gushing from a thousand tiny wounds."#
            ),
        }
    }
}
