use std::str::FromStr;

use dioxus::prelude::*;
use ti_helper_game_data::{
    common::faction::Faction,
    components::{
        action_card::ActionCard, objectives::Objective, planet::Planet, relic::Relic,
        tech::Technology,
    },
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
    default_text: Option<&'static str>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|t| t.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let objective = Objective::from_str(&new_value).expect("Unexpected objective");
            on_select(Some(objective));
        }
    };

    let default_text = default_text.unwrap_or("--Select Objective--");

    rsx! {
        Dropdown { value: "{current_value()}", oninput,
            option { value: "", "{default_text}" }
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

#[component]
pub fn ActionCardDropdown(
    value: ReadSignal<Option<ActionCard>>,
    options: Vec<ActionCard>,
    on_select: EventHandler<Option<ActionCard>>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|a| a.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let action_card = ActionCard::from_str(&new_value).expect("Unexpected action card");
            on_select(Some(action_card));
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

#[component]
pub fn RelicDropdown(
    value: ReadSignal<Option<Relic>>,
    options: Vec<Relic>,
    on_select: EventHandler<Option<Relic>>,
    disabled: Option<bool>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|r| r.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let relic = Relic::from_str(&new_value).expect("Unexpected relic");
            on_select(Some(relic));
        }
    };

    rsx! {
        Dropdown {
            value: "{current_value()}",
            disabled: disabled.unwrap_or(false),
            oninput,
            option { value: "", "--Select Relic--" }
            {
                options
                    .iter()
                    .map(|r| {
                        rsx! {
                            option { value: "{r}", "{r.info().name}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn PlanetDropdown(
    value: ReadSignal<Option<Planet>>,
    options: Vec<Planet>,
    on_select: EventHandler<Option<Planet>>,
    disabled: Option<bool>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|p| p.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let planet = Planet::from_str(&new_value).expect("Unexpected planet");
            on_select(Some(planet));
        }
    };

    rsx! {
        Dropdown {
            value: "{current_value()}",
            disabled: disabled.unwrap_or(false),
            oninput,
            option { value: "", "--Select Planet--" }
            {
                options
                    .iter()
                    .map(|p| {
                        rsx! {
                            option { value: "{p}", "{p.info().name}" }
                        }
                    })
            }
        }
    }
}
