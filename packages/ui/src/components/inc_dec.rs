use dioxus::prelude::*;

use crate::components::button::Button;

const INC_DEC_SCSS: Asset = asset!("/assets/styling/components/inc_dec.scss");

#[derive(Debug, Clone, PartialEq, Props)]
pub struct IncDecViewProps {
    points: i8,
    change_points: EventHandler<i8>,
}

/// The points, with buttons to raise them by 1 or lower them by 1 (down to 0).
#[component]
pub fn IncDecView(
    IncDecViewProps {
        points,
        change_points,
    }: IncDecViewProps,
) -> Element {
    rsx! {
        document::Stylesheet { href: INC_DEC_SCSS }

        Button {
            onclick: move |_| change_points(points + 1),
            class: "inc-dec-button",
            "^"
        }
        p { "{points}" }
        Button {
            onclick: move |_| change_points(points - 1),
            class: "inc-dec-button",
            disabled: points == 0,
            "v"
        }
    }
}
