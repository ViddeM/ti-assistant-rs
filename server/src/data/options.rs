use super::common::expansions::Expansions;

pub struct GameOptions {
    winning_score: u32,
    player_count: u32,
    expansions: Vec<Expansions>,
}
