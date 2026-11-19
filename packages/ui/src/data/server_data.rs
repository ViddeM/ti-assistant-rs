use std::sync::Arc;

use api::messages::WsMessageIn;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use ti_helper_game_data::actions::event::Event;

pub type History = Arc<Vec<(Event, DateTime<Utc>)>>;

#[derive(Clone, Copy)]
pub struct ServerDataContext {
    request: Callback<WsMessageIn>,
    history: ReadSignal<Option<History>>,
}

impl ServerDataContext {
    pub fn new(request: Callback<WsMessageIn>, history: ReadSignal<Option<History>>) -> Self {
        Self { request, history }
    }

    pub fn request_history(&self) {
        self.request.call(WsMessageIn::RequestHistory);
    }

    pub fn history(&self) -> Option<History> {
        (self.history)()
    }
}
