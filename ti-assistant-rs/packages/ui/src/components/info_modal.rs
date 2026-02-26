use dioxus::prelude::*;

use crate::data::{game_context::GameContext, info_context::InfoContext};

const INFO_MODAL_SCSS: Asset = asset!("/assets/styling/components/info_modal.scss");

#[component]
pub fn InfoModal() -> Element {
    let gc = use_context::<GameContext>();
    let mut info = use_context::<InfoContext>();

    let expansions = use_memo(move || gc.game_state().game_settings.expansions.clone());

    let Some(i) = info.get()() else {
        return rsx! {};
    };

    rsx! {
        document::Stylesheet { href: INFO_MODAL_SCSS }

        div { class: "info-modal-bg", onclick: move |_| info.close(),
            div { class: "info-modal", onclick: |e| e.stop_propagation(),
                h1 { "{i.title()}" }
                hr {}
                h2 { "{i.subtitle()}" }
                {i.description(&expansions()).to_element()}
            }
        }
    }
}
