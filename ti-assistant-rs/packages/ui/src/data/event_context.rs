use api::messages::WsMessageIn;
use dioxus::prelude::*;
use ti_helper_game_data::actions::event::Event;

#[derive(Clone, Copy)]
pub struct EventContext {
    send_event: EventHandler<WsMessageIn>,
}

impl EventContext {
    pub fn new(send_event: EventHandler<WsMessageIn>) -> Self {
        Self { send_event }
    }

    pub fn undo(&self) {
        self.send_event.call(WsMessageIn::Undo);
    }

    pub fn send_event(&self, event: Event) {
        self.send_event.call(WsMessageIn::Event(event));
    }

    pub fn pause(&self) {
        self.send_event
            .call(WsMessageIn::Event(Event::TrackTime { paused: false }))
    }

    pub fn play(&self) {
        self.send_event
            .call(WsMessageIn::Event(Event::TrackTime { paused: true }))
    }
}
