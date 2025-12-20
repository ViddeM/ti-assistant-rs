use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use ti_helper_game_data::{game_id::GameId, state::game_state::GameState};
use ti_helper_game_logic::gameplay::game::Game;
use tokio::sync::broadcast;

#[derive(Default, Clone)]
pub struct Lobbies {
    pub list: Arc<RwLock<HashMap<GameId, Arc<RwLock<Lobby>>>>>,
}

pub struct Lobby {
    /// The Game State
    pub game: Game,

    /// Broadcaster to send GameState updates to all websocket clients.
    pub state_updates: broadcast::Sender<Arc<GameState>>,
}

impl Lobby {
    pub fn new(game: Game) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            game,
            state_updates: broadcast::channel(100).0,
        }))
    }
}

pub fn generate_game_name(_id: GameId) -> String {
    // TODO
    "Funny Game".into()
}
