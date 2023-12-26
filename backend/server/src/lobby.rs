use std::{collections::HashMap, sync::Arc};
use ti_helper_db::game_id::GameId;
use ti_helper_game::gameplay::{game::Game, game_state::GameState};
use tokio::sync::{broadcast, RwLock};

#[derive(Default)]
pub struct Lobbies {
    pub list: RwLock<HashMap<GameId, Arc<RwLock<Lobby>>>>,
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
