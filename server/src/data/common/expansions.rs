pub enum Expansions {
    Base,
    ProphecyOfKings,
    Codex,
    CodexII,
    CodexIII,
}

impl Expansions {
    pub fn name(&self) -> String {
        String::from(match self {
            Expansions::Base => "Base",
            Expansions::ProphecyOfKings => "Prophecy Of Kings",
            Expansions::Codex => "Codex",
            Expansions::CodexII => "Codex II",
            Expansions::CodexIII => "Codex III",
        })
    }
}
