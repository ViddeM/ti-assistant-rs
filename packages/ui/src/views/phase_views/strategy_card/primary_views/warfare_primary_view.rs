use dioxus::prelude::*;

use crate::views::phase_views::tactical_action::TacticalAction;

#[component]
pub fn WarfarePrimaryView() -> Element {
    return rsx! {
        TacticalAction {}
    };
}
