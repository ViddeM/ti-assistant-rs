mod agent;
mod commander;
mod hero;

use std::fmt::Display;

pub use agent::*;
pub use commander::*;
pub use hero::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::common::{
    component::{Component, replacements},
    expansions::Expansion,
    faction::Faction,
};

/// A leader, i.e. an agent, commander, or hero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Leader {
    Agent(Agent),
    Commander(Commander),
    Hero(Hero),
}

impl Leader {
    pub fn type_name(&self) -> &'static str {
        match self {
            Leader::Agent(..) => "Agent",
            Leader::Commander(..) => "Commander",
            Leader::Hero(..) => "Hero",
        }
    }
}

impl Display for Leader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info().name())
    }
}

/// Information about a leader, i.e. an agent, commander, or hero.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[allow(missing_docs)]
pub enum LeaderInfo {
    Agent(AgentInfo),
    Commander(CommanderInfo),
    Hero(HeroInfo),
}

/// Information about a leader ability, i.e. if it's an Action, etc.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeaderAbilityKind {
    /// Leader ability is an Action.
    Action,

    /// Leader ability is not one of the other alternatives.
    Other,
}

impl Component for Leader {
    const REQUIRES: Expansion = Expansion::ProphecyOfKings;

    replacements! {
        Agent(Agent::Zeu) -> Agent(Agent::ZeuCxIII),
        Commander(Commander::Maban) -> Commander(Commander::MabanCxIII),
        Hero(Hero::XxekirGrom) -> Hero(Hero::XxekirGromCxIII),
        Agent(Agent::BrotherMilor) -> Agent(Agent::BrotherMilorCxIII),
        Commander(Commander::BrotherOmar) -> Commander(Commander::BrotherOmarCxIII),
        Hero(Hero::DannelOfTheTenth) -> Hero(Hero::DannelOfTheTenthCxIII),
    }

    fn expansion(&self) -> Expansion {
        match self {
            Leader::Agent(Agent::ZeuCxIII | Agent::BrotherMilorCxIII)
            | Leader::Commander(Commander::MabanCxIII | Commander::BrotherOmarCxIII)
            | Leader::Hero(Hero::XxekirGromCxIII | Hero::DannelOfTheTenthCxIII) => {
                Expansion::CodexIII
            }
            _ => self.info().faction().expansion(),
        }
    }
}

impl Leader {
    /// Get the [LeaderInfo] of this [Leader].
    pub fn info(&self) -> LeaderInfo {
        match self {
            Leader::Agent(a) => a.info().into(),
            Leader::Commander(c) => c.info().into(),
            Leader::Hero(h) => h.info().into(),
        }
    }

    /// Returns an iterator over all leaders.
    pub fn iter() -> impl Iterator<Item = Leader> {
        (Agent::iter().map(Leader::from))
            .chain(Commander::iter().map(Leader::from))
            .chain(Hero::iter().map(Leader::from))
    }
}

impl LeaderInfo {
    /// Get the pretty name of this leader.
    pub fn name(&self) -> String {
        match self {
            LeaderInfo::Agent(l) => &l.name,
            LeaderInfo::Commander(l) => &l.name,
            LeaderInfo::Hero(l) => &l.name,
        }
        .to_string()
    }

    /// Get the [Faction] that this [Leader] is a part of.
    pub fn faction(&self) -> Faction {
        match self {
            LeaderInfo::Agent(l) => l.faction,
            LeaderInfo::Commander(l) => l.faction,
            LeaderInfo::Hero(l) => l.faction,
        }
    }

    /// Get the [LeaderAbilityKind] of this [Leader]s ability.
    pub fn ability_kind(&self) -> LeaderAbilityKind {
        match self {
            LeaderInfo::Agent(l) => l.kind,
            LeaderInfo::Commander(_) => LeaderAbilityKind::Other,
            LeaderInfo::Hero(l) => l.kind,
        }
    }
}

macro_rules! impl_enum {
    (From<$from:ident> for $for:ident::$variant:ident) => {
        impl From<$from> for $for {
            fn from(v: $from) -> $for {
                $for::$variant(v)
            }
        }
    };
}

impl_enum!(From<Agent> for Leader::Agent);
impl_enum!(From<Commander> for Leader::Commander);
impl_enum!(From<Hero> for Leader::Hero);

impl_enum!(From<AgentInfo> for LeaderInfo::Agent);
impl_enum!(From<CommanderInfo> for LeaderInfo::Commander);
impl_enum!(From<HeroInfo> for LeaderInfo::Hero);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// [Leader] is serialized untagged, so a name shared by two leader kinds would deserialize
    /// as the wrong leader (e.g. a hero replayed from the history as a commander).
    #[test]
    fn leaders_survive_a_serde_round_trip() {
        for leader in Leader::iter() {
            let json = serde_json::to_string(&leader).unwrap();
            let parsed: Leader = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("{leader:?} serialized as {json} failed to parse: {e}"));
            assert_eq!(
                parsed, leader,
                "{leader:?} serialized as {json} but parsed as {parsed:?}, \
                 leader names must be unique across agents, commanders and heroes"
            );
        }
    }

    #[test]
    fn leader_info_matches_leader() {
        for leader in Leader::iter() {
            let tag: Leader = match leader.info() {
                LeaderInfo::Agent(i) => i.tag.into(),
                LeaderInfo::Commander(i) => i.tag.into(),
                LeaderInfo::Hero(i) => i.tag.into(),
            };
            assert_eq!(tag, leader, "{leader:?} has info for {tag:?}");
        }
    }

    /// Descriptions are read from a file named after the variant, so leaders with the same name
    /// would silently share a description.
    #[test]
    fn leaders_have_their_own_description() {
        let mut seen: HashMap<String, Leader> = HashMap::new();
        for leader in Leader::iter() {
            let description = match leader.info() {
                LeaderInfo::Agent(i) => i.description,
                LeaderInfo::Commander(i) => i.description,
                LeaderInfo::Hero(i) => i.description,
            };
            assert!(
                !description.trim().is_empty(),
                "{leader:?} has no description"
            );
            if let Some(other) = seen.insert(description, leader) {
                panic!("{leader:?} and {other:?} have the same description");
            }
        }
    }
}
