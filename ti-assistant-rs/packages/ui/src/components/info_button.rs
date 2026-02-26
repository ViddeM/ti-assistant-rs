use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaInfo, Icon};

use crate::data::info_context::{Info, InfoContext};

const INFO_BUTTON_SCSS: Asset = asset!("/assets/styling/components/info_button.scss");

#[derive(PartialEq, Clone, Props)]
pub struct InfoButtonProps {
    info: Info,

    #[props(default, into)]
    class: String,

    #[props(extends = GlobalAttributes, extends = button)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn InfoButton(
    InfoButtonProps {
        info,
        class,
        attributes,
    }: InfoButtonProps,
) -> Element {
    let mut info_ctx = use_context::<InfoContext>();

    let class = format!("{class} info-button");

    let onclick = move |_event: MouseEvent| {
        info_ctx.open(info.clone());
    };

    rsx! {
        document::Stylesheet { href: INFO_BUTTON_SCSS }

        button { class, onclick, ..attributes,
            Icon {
                icon: FaInfo,
                width: None,
                height: None,
                class: "inline-icon",
            }
        }
    }
}
