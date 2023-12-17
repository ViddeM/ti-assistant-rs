use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Expansion {
    Base,
    ProphecyOfKings,
    Codex,
    CodexII,
    CodexIII,
}

impl Expansion {
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
