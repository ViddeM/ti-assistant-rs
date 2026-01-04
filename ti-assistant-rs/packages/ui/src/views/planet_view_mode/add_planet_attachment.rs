use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event,
    common::player_id::PlayerId,
    components::{planet::Planet, planet_attachment::PlanetAttachment, system::SystemType},
};

use crate::{
    components::{
        button::Button,
        dropdown::{PlanetAttachmentDropdown, PlanetDropdown, PlayerDropdown},
    },
    data::{event_context::EventContext, game_context::GameContext},
};

#[component]
pub fn AddPlanetAttachment() -> Element {
    let gc = use_context::<GameContext>();
    let event = use_context::<EventContext>();

    let mut player: Signal<PlayerId> = use_signal(|| "".into());
    let mut planet: Signal<Option<Planet>> = use_signal(|| None);
    let mut attachment: Signal<Option<PlanetAttachment>> = use_signal(|| None);

    let available_players = use_memo(move || {
        let mut players = gc
            .game_state()
            .players
            .iter()
            .map(|(id, p)| (id.clone(), p.clone()))
            .collect::<Vec<_>>();
        players.sort_by(|(a, _), (b, _)| a.cmp(&b));
        players
    });

    let player_options = use_memo(move || {
        available_players()
            .iter()
            .map(|(p, _)| p)
            .cloned()
            .collect::<Vec<_>>()
    });

    let available_planets = use_memo(move || {
        let mut planets = available_players()
            .iter()
            .find(|(id, _)| id.eq(&player()))
            .map(|(_, p)| p.planets.keys().cloned().collect::<Vec<_>>())
            .unwrap_or(vec![]);
        planets.sort();
        planets
    });

    let home_planets = use_memo(move || {
        let mut planets = gc
            .game_options()
            .systems
            .values()
            .filter(|system| matches!(system.system_type, SystemType::HomeSystem(..)))
            .flat_map(|system| system.planets.clone())
            .collect::<Vec<_>>();
        planets.sort();
        planets
    });

    let available_attachments = use_memo(move || {
        let Some(planet) = planet() else {
            return vec![];
        };

        let mut attachments = gc
            .game_options()
            .planet_attachments
            .iter()
            .filter(|(a, _)| a.is_real())
            .filter(|(_, i)| {
                if let Some(t) = i.planet_trait.as_ref() {
                    planet.info().planet_traits.contains(t)
                } else {
                    true
                }
            })
            .filter(|&(a, _)| {
                !(a.eq(&PlanetAttachment::UITheProgenitor) && !planet.eq(&Planet::Elysium))
            })
            .filter(|&(a, _)| {
                !(a.eq(&PlanetAttachment::Terraform)
                    && (planet.is_mecatol_rex() || planet.info().is_legendary))
            })
            .filter(|&(a, _)| {
                !(a.eq(&PlanetAttachment::Terraform)
                    || a.eq(&PlanetAttachment::NanoForge) && home_planets().contains(&planet))
            })
            .map(|(a, _)| a.clone())
            .collect::<Vec<_>>();
        attachments.sort();
        attachments
    });

    use_effect(move || {
        if let Some(p) = planet() {
            if !available_planets().contains(&p) {
                planet.set(None);
            }
        }
    });

    use_effect(move || {
        if let Some(a) = attachment() {
            if !available_attachments().contains(&a) {
                attachment.set(None);
            }
        }
    });

    rsx! {
        div { class: "card add-planet-attachment-container",
            h2 { "Attach to planet" }
            PlayerDropdown {
                value: player(),
                options: player_options(),
                on_select: move |p| player.set(p),
            }
            PlanetDropdown {
                value: planet(),
                disabled: player().is_empty(),
                options: available_planets(),
                on_select: move |p| planet.set(p),
            }
            PlanetAttachmentDropdown {
                value: attachment(),
                disabled: player().is_empty() || planet().is_none(),
                options: available_attachments(),
                on_select: move |a| attachment.set(a),
            }
            Button {
                disabled: player().is_empty() || planet().is_none() || attachment().is_none(),
                onclick: move |_| {
                    event
                        .send_event(Event::AddPlanetAttachment {
                            player: player(),
                            planet: planet().expect("Planet to be set"),
                            attachment: attachment().expect("Attachment to be set"),
                        });
                    attachment.set(None);
                    planet.set(None);
                    player.set("".into());
                },
                "Add attachment"
            }
        }
    }
}
