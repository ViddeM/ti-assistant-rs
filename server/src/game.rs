use crate::player::Player;

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Self {
        Self { players }
    }
}
