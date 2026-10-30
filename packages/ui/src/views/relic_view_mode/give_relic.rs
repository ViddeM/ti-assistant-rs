use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::relic::Relic,
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlayerDropdown, RelicDropdown},
    },
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn GiveRelic() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut player: Signal<Option<PlayerId>> = use_signal(|| None);
    let mut relic: Signal<Option<Relic>> = use_signal(|| None);

    let player_options = use_memo(move || {
        let mut players = gc.game_state().players.keys().cloned().collect::<Vec<_>>();
        players.sort();
        players
    });

    let available_relics = use_memo(move || {
        let owned = player()
            .and_then(|p| gc.game_state().players.get(&p).map(|p| p.relics.clone()))
            .unwrap_or_default();
        let mut relics = gc
            .game_options()
            .relics
            .iter()
            .filter(|r| !owned.contains(r))
            .cloned()
            .collect::<Vec<_>>();
        relics.sort();
        relics
    });

    use_effect(move || {
        if let Some(r) = relic()
            && !available_relics().contains(&r)
        {
            relic.set(None);
        }
    });

    rsx! {
        div { class: "card give-relic-container",
            h2 { "Give relic" }
            PlayerDropdown {
                value: player,
                options: player_options(),
                on_select: move |p| player.set(p),
            }
            RelicDropdown {
                value: relic,
                disabled: player().is_none(),
                options: available_relics(),
                on_select: move |r| relic.set(r),
            }
            Button {
                disabled: player().is_none() || relic().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::SetRelicOwner {
                            relic: relic().expect("Relic to be selected"),
                            player: player(),
                        });
                    relic.set(None);
                    player.set(None);
                },
                "Give relic"
            }
        }
    }
}
