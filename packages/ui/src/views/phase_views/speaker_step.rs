use dioxus::prelude::*;

use crate::{
    components::button::Button,
    data::{game_context::GameContext, player_view::PlayerViewContext},
};

#[component]
pub fn SpeakerStep(
    ready: bool,
    #[props(into)] waiting_for: String,
    #[props(default, into)] class: String,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();

    let speaker = gc.game_state().speaker.clone().filter(|_| ready);

    rsx! {
        if view.is_global_or_speaker() {
            Button { class, disabled: !ready, onclick, {children} }
        } else if let Some(speaker) = speaker {
            p { class, "Waiting for {speaker} to {waiting_for}" }
        }
    }
}
