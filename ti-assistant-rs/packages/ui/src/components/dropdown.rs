use std::str::FromStr;

use dioxus::prelude::*;
use ti_helper_game_data::{
    common::faction::Faction,
    components::{objectives::Objective, tech::Technology},
};

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

#[component]
pub fn TechDropdown(
    value: ReadSignal<Option<Technology>>,
    options: Vec<Technology>,
    on_select: EventHandler<Option<Technology>>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|t| t.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let tech = Technology::from_str(&new_value).expect("Unexpected tech");
            on_select(Some(tech));
        }
    };

    rsx! {
        Dropdown { value: "{current_value()}", oninput,
            option { value: "", "--Select technology--" }
            {
                options
                    .iter()
                    .map(|t| {
                        rsx! {
                            option { value: "{t}", "{t.info().name}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn FactionDropdown(
    value: ReadSignal<Option<Faction>>,
    options: Vec<Faction>,
    on_select: EventHandler<Option<Faction>>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|t| t.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let faction = Faction::from_str(&new_value).expect("Unexpected faction");
            on_select(Some(faction));
        }
    };

    rsx! {
        Dropdown { value: "{current_value()}", oninput,
            option { value: "", "--Select Faction--" }
            {
                options
                    .iter()
                    .map(|f| {
                        rsx! {
                            option { value: "{f}", "{f.name()}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn ObjectiveDropdown(
    value: ReadSignal<Option<Objective>>,
    options: Vec<Objective>,
    on_select: EventHandler<Option<Objective>>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|t| t.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let objective = Objective::from_str(&new_value).expect("Unexpected faction");
            on_select(Some(objective));
        }
    };

    rsx! {
        Dropdown { value: "{current_value()}", oninput,
            option { value: "", "--Select Objective--" }
            {
                options
                    .iter()
                    .map(|o| {
                        rsx! {
                            option { value: "{o}", "{o.info().name}" }
                        }
                    })
            }
        }
    }
}
