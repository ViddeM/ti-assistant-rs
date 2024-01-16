mod agent;
mod commander;
mod hero;

pub use agent::*;
pub use commander::*;
pub use hero::*;
use serde::{Deserialize, Serialize};

use crate::{data::common::faction::Faction, gameplay::game_settings::Expansions};

/// A leader, i.e. an agent, commander, or hero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum Leader {
    Agent(Agent),
    Commander(Commander),
    Hero(Hero),
}

/// Information about a leader, i.e. an agent, commander, or hero.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[allow(missing_docs)]
pub enum LeaderInfo {
    Agent(AgentInfo),
    Commander(CommanderInfo),
    Hero(HeroInfo),
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

    /// Is this leader enabled for the given [Expansions]?
    pub fn is_enabled_in(&self, expansions: &Expansions) -> bool {
        // leaders are a PoK feature
        if !expansions.prophecy_of_kings {
            return false;
        }

        // check if the expansion that adds this leader is enabled.
        let source_expansion = self.info().faction().expansion();
        if !expansions.is_enabled(&source_expansion) {
            return false;
        }

        // check if any enabled expansion explicitly disables this leader
        let is_replaced_in_codex_3 = matches!(
            self,
            Leader::Agent(Agent::Zeu)
                | Leader::Commander(Commander::Maban)
                | Leader::Hero(Hero::XxekirGrom)
                | Leader::Agent(Agent::BrotherMilor)
                | Leader::Commander(Commander::BrotherOmar)
                | Leader::Hero(Hero::DannelOfTheTenth)
        );

        if expansions.codex_3 && is_replaced_in_codex_3 {
            return false;
        }

        true
    }
}

impl LeaderInfo {
    /// Get the [Faction] that this [Leader] is a part of.
    pub fn faction(&self) -> Faction {
        match self {
            LeaderInfo::Agent(l) => l.faction,
            LeaderInfo::Commander(l) => l.faction,
            LeaderInfo::Hero(l) => l.faction,
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
