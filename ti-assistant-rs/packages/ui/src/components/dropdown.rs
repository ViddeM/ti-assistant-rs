use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct DropdownProps {
    #[props(default, into)]
    class: String,

    #[props(default)]
    oninput: EventHandler<FormEvent>,

    #[props(extends = GlobalAttributes, extends = select)]
    attributes: Vec<Attribute>,

    children: Element,
}

#[component]
pub fn Dropdown(
    DropdownProps {
        class,
        oninput,
        attributes,
        children,
    }: DropdownProps,
) -> Element {
    let class = class;

    rsx! {
        select { class, oninput, ..attributes, {children} }
    }
}
