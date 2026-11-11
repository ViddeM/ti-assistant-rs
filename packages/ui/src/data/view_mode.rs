use dioxus::prelude::*;
use strum::{Display, EnumString};

#[derive(PartialEq, Debug, Clone, Copy, Default, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum ViewMode {
    #[default]
    Game,
    Score,
    Techs,
    Planets,
    Relics,
    Laws,
    Map,
    ThundersEdge,
    GalacticEvents,
    History,
}

impl ViewMode {
    pub fn label(&self) -> &'static str {
        match self {
            ViewMode::Game => "Game",
            ViewMode::Score => "Score",
            ViewMode::Techs => "Techs",
            ViewMode::Planets => "Planets",
            ViewMode::Relics => "Relics",
            ViewMode::Laws => "Laws",
            ViewMode::Map => "Map",
            ViewMode::ThundersEdge => "Thunder's Edge",
            ViewMode::GalacticEvents => "Galactic Events",
            ViewMode::History => "History",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ViewModeContext {
    current: ReadSignal<ViewMode>,
    set_view_mode: Callback<ViewMode>,
}

impl ViewModeContext {
    pub fn new(current: ReadSignal<ViewMode>, set_view_mode: Callback<ViewMode>) -> Self {
        Self {
            current,
            set_view_mode,
        }
    }

    pub fn get(&self) -> ViewMode {
        (self.current)()
    }

    pub fn set(&self, view_mode: ViewMode) {
        (self.set_view_mode)(view_mode);
    }
}
