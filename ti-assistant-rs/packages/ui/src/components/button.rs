use dioxus::prelude::*;

const BUTTON_SCSS: Asset = asset!("/assets/styling/components/button.scss");

#[derive(PartialEq, Debug, Clone, Props)]
pub struct ButtonProps {
    #[props(default, into)]
    class: String,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,

    #[props(extends = GlobalAttributes, extends = button)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn ButtonBase(
    ButtonProps {
        class,
        onclick,
        attributes,
        children,
    }: ButtonProps,
) -> Element {
    let class = format!("button-base {class}");

    rsx! {
        document::Stylesheet { href: BUTTON_SCSS }

        button { class, onclick, ..attributes, {children} }
    }
}

#[component]
pub fn Button(
    ButtonProps {
        class,
        onclick,
        attributes,
        children,
    }: ButtonProps,
) -> Element {
    let class = format!("button {class}");

    rsx! {
        ButtonBase {
            class,
            onclick,
            attributes,
            children,
        }
    }
}
