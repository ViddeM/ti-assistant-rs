use dioxus::prelude::*;
use strum::Display;
use ti_helper_game_data::common::color::Color;

use crate::views::tech_view_mode::{tech_ledger::TechLedger, tech_table::TechTable};

mod tech_ledger;
mod tech_table;

const TECH_VIEW_MODE_SCSS: Asset = asset!("/assets/styling/views/tech_view_mode.scss");

#[component]
pub fn TechViewMode() -> Element {
    rsx! {
        document::Stylesheet { href: TECH_VIEW_MODE_SCSS }

        div { class: "tech-view-container",
            TechLedger {}
            TechTable {}
        }
    }
}

#[derive(Debug, Clone, Display, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum TechSection {
    UnitUpgrade,
    Warfare,
    Propulsion,
    Biotic,
    Cybernetic,
    PlayerBlack,
    PlayerBlue,
    PlayerGreen,
    PlayerRed,
    PlayerYellow,
    PlayerPurple,
    PlayerOrange,
    PlayerPink,
}

impl From<&Color> for TechSection {
    fn from(value: &Color) -> Self {
        match value {
            Color::Pink => Self::PlayerPink,
            Color::Orange => Self::PlayerOrange,
            Color::Green => Self::PlayerGreen,
            Color::Red => Self::PlayerRed,
            Color::Yellow => Self::PlayerYellow,
            Color::Black => Self::PlayerBlack,
            Color::Purple => Self::PlayerPurple,
            Color::Blue => Self::PlayerBlue,
        }
    }
}

impl TechSection {
    pub fn to_tag(&self) -> String {
        format!("#{}", self.to_string())
    }
}
