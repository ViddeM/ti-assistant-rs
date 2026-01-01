use std::str::FromStr;

use dioxus::prelude::*;
use ti_helper_game_data::{
    common::{faction::Faction, player_id::PlayerId},
    components::{
        action_card::ActionCard, agenda::Agenda, objectives::Objective, planet::Planet,
        planet_attachment::PlanetAttachment, relic::Relic, tech::Technology,
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
    disabled: Option<bool>,
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
                            option { key: "{t}", value: "{t}", "{t.info().name}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn FactionDropdown(
    id: Option<String>,
    required: Option<bool>,
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
        Dropdown {
            id,
            required,
            value: "{current_value()}",
            oninput,
            option { value: "", "--Select Faction--" }
            {
                options
                    .iter()
                    .map(|f| {
                        rsx! {
                            option { key: "{f}", value: "{f}", "{f.name()}" }
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
                            option { key: "{o}", value: "{o}", "{o.info().name}" }
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
                            option { key: "{o}", value: "{o}", "{o.info().name}" }
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
                            option { key: "{r}", value: "{r}", "{r.info().name}" }
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
                            option { key: "{p}", value: "{p}", "{p.info().name}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn AgendaDropdown(
    value: ReadSignal<Option<Agenda>>,
    options: Vec<Agenda>,
    on_select: EventHandler<Option<Agenda>>,
    disabled: Option<bool>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|p| p.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let agenda = Agenda::from_str(&new_value).expect("Unexpected agenda");
            on_select(Some(agenda));
        }
    };

    rsx! {
        Dropdown {
            value: "{current_value()}",
            disabled: disabled.unwrap_or(false),
            oninput,
            option { value: "", "--Select Agenda--" }
            {
                options
                    .iter()
                    .map(|a| {
                        rsx! {
                            option { key: "{a}", value: "{a}", "{a.info().name}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn PlayerDropdown(
    value: ReadSignal<PlayerId>,
    options: Vec<PlayerId>,
    on_select: EventHandler<PlayerId>,
    disabled: Option<bool>,
) -> Element {
    let mut current_value = use_memo(move || value());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        current_value.set(new_value.into());
        on_select(current_value());
    };

    rsx! {
        Dropdown {
            value: "{current_value()}",
            disabled: disabled.unwrap_or(false),
            oninput,
            option { value: "", "--Select Player--" }
            {
                options
                    .iter()
                    .map(|p| {
                        rsx! {
                            option { key: "{p}", value: "{p}", "{p}" }
                        }
                    })
            }
        }
    }
}

#[component]
pub fn PlanetAttachmentDropdown(
    value: ReadSignal<Option<PlanetAttachment>>,
    options: Vec<PlanetAttachment>,
    on_select: EventHandler<Option<PlanetAttachment>>,
    disabled: Option<bool>,
) -> Element {
    let current_value =
        use_memo(move || value().as_ref().map(|p| p.to_string()).unwrap_or_default());

    let oninput = move |event: FormEvent| {
        let new_value = event.value();
        if new_value.is_empty() {
            on_select(None);
        } else {
            let attachment =
                PlanetAttachment::from_str(&new_value).expect("Unexpected planet attachment");
            on_select(Some(attachment));
        }
    };

    rsx! {
        Dropdown {
            value: "{current_value()}",
            disabled: disabled.unwrap_or(false),
            oninput,
            {
                if options.is_empty() {
                    rsx! {
                        option { value: "", "No attachments available" }
                    }
                } else {
                    rsx! {
                        option { value: "", "--Select Attachment--" }
                        {
                            options
                                .iter()
                                .map(|p| {
                                    rsx! {
                                        option { key: "{p}", value: "{p}", "{p.info().name}" }
                                    }
                                })
                        }
                    }
                }
            }
        }
    }
}
