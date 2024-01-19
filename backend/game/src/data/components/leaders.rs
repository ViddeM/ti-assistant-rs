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

/// Information about a leader ability, i.e. if it's an Action, etc.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum LeaderAbilityKind {
    /// Leader ability is an Action.
    Action,

    /// Leader ability is not one of the other alternatives.
    Other,
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
        use {Agent::*, Commander::*, Hero::*};

        // leaders are a PoK feature
        if !expansions.prophecy_of_kings {
            return false;
        }

        // check if the expansion that adds this leaders faction is enabled.
        let faction_source = self.info().faction().expansion();
        if !expansions.is_enabled(&faction_source) {
            return false;
        }

        // check if leader is patched in codex 3

        let removed_in_codex_3 = &[
            Zeu.into(),
            Maban.into(),
            XxekirGrom.into(),
            BrotherMilor.into(),
            BrotherOmar.into(),
            DannelOfTheTenth.into(),
        ];

        let added_in_codex_3 = &[
            ZeuCxIII.into(),
            MabanCxIII.into(),
            XxekirGromCxIII.into(),
            BrotherMilorCxIII.into(),
            BrotherOmarCxIII.into(),
            DannelOfTheTenthCxIII.into(),
        ];

        let is_removed_in_codex_3 = removed_in_codex_3.contains(self);
        let is_added_in_codex_3 = added_in_codex_3.contains(self);

        if expansions.codex_3 && is_removed_in_codex_3 {
            return false;
        }

        if !expansions.codex_3 && is_added_in_codex_3 {
            return false;
        }

        true
    }
}

impl LeaderInfo {
    /// Get the pretty name of this leader.
    pub fn name(&self) -> &'static str {
        match self {
            LeaderInfo::Agent(l) => l.name,
            LeaderInfo::Commander(l) => l.name,
            LeaderInfo::Hero(l) => l.name,
        }
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
