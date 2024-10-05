use serde::{Deserialize, Serialize};

/// An expansion for Twilight Imperial 4th edition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(export)]
pub enum Expansion {
    /// The base game without any expansions.
    Base,
    /// The Prophecy of Kings expansion.
    ProphecyOfKings,
    /// The first Codex addon.
    Codex,
    /// The second Codex addon.
    CodexII,
    /// The third Codex addon.
    CodexIII,
}

impl Expansion {
    /// Returns the name of the expansion in 'pretty' format.
    pub fn name(&self) -> String {
        String::from(match self {
            Expansion::Base => "Base",
            Expansion::ProphecyOfKings => "Prophecy Of Kings",
            Expansion::Codex => "Codex",
            Expansion::CodexII => "Codex II",
            Expansion::CodexIII => "Codex III",
        })
    }
}
