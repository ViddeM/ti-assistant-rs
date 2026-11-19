use std::sync::Arc;

use api::messages::WsMessageIn;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use ti_helper_game_data::{actions::event::Event, state::time_stats::TimeStats};

pub type History = Arc<Vec<(Event, DateTime<Utc>)>>;

#[derive(Clone, Copy)]
pub struct ServerDataContext {
    request: Callback<WsMessageIn>,
    history: ReadSignal<Option<History>>,
    time_stats: ReadSignal<Option<Arc<TimeStats>>>,
}

impl ServerDataContext {
    pub fn new(
        request: Callback<WsMessageIn>,
        history: ReadSignal<Option<History>>,
        time_stats: ReadSignal<Option<Arc<TimeStats>>>,
    ) -> Self {
        Self {
            request,
            history,
            time_stats,
        }
    }

    pub fn request_history(&self) {
        self.request.call(WsMessageIn::RequestHistory);
    }

    pub fn history(&self) -> Option<History> {
        (self.history)()
    }

    pub fn request_time_stats(&self) {
        self.request.call(WsMessageIn::RequestTimeStats);
    }

    pub fn time_stats(&self) -> Option<Arc<TimeStats>> {
        (self.time_stats)()
    }
}
