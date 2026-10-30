use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::relic::Relic,
};

use crate::{
    components::{button::Button, dropdown::RelicDropdown},
    data::{
        event_context::EventContext, game_context::GameContext, player_view::PlayerViewContext,
    },
};

#[component]
pub fn MinisterOfAntiques(player_id: PlayerId) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let mut selected_relic: Signal<Option<Relic>> = use_signal(|| None);

    let taken_relics = use_memo(move || {
        gc.game_state()
            .players
            .values()
            .flat_map(|p| p.relics.iter())
            .cloned()
            .collect::<Vec<_>>()
    });

    let available_relics = use_memo(move || {
        let mut relics = gc
            .game_options()
            .relics
            .iter()
            .filter(|r| !taken_relics().contains(r))
            .cloned()
            .collect::<Vec<_>>();
        relics.sort();
        relics
    });

    let gainer = player_id.clone();
    let gain = use_callback(move |relic: Option<Relic>| {
        event.send_event(Event::GainMinisterOfAntiquesRelic {
            player: gainer.clone(),
            relic,
        })
    });

    rsx! {
        div { class: "card column",
            h2 { "Minister of Antiques" }
            p { "{player_id} was elected and gains 1 relic." }
            if view.display_for(&player_id) {
                div { class: "margin-top",
                    RelicDropdown {
                        value: selected_relic,
                        options: available_relics(),
                        on_select: move |r| selected_relic.set(r),
                    }
                }
                div { class: "center-row margin-top",
                    Button {
                        class: "margin-right",
                        disabled: selected_relic().is_none(),
                        onclick: move |_| gain(selected_relic()),
                        "Gain relic"
                    }
                    Button { onclick: move |_| gain(None), "No relic" }
                }
            } else {
                p { "Waiting for {player_id} to gain their relic." }
            }
        }
    }
}
