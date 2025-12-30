use dioxus::prelude::*;
use ti_helper_game_data::{
    common::player_id::PlayerId,
    components::{planet::Planet, planet_attachment::PlanetAttachment, system::SystemType},
};

use crate::{
    components::dropdown::{PlanetDropdown, PlayerDropdown},
    data::game_context::GameContext,
};

#[component]
pub fn AddPlanetAttachment() -> Element {
    let gc = use_context::<GameContext>();

    let player: Signal<PlayerId> = use_signal(|| "".into());
    let planet: Signal<Option<Planet>> = use_signal(|| None);
    let attachment: Signal<Option<PlanetAttachment>> = use_signal(|| None);

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

    let available_planets = use_memo(move || {
        let mut planets = available_players()
            .iter()
            .find(|(id, p)| id.eq(&player()))
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
            .flat_map(|system| system.planets)
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
            .filter(|(a, i)| {
                if let Some(t) = i.planet_trait {
                    planet.info().planet_traits.contains(&t)
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
            PlayerDropdown { value: player(), oninput: move |p| player.set(p) }
            PlanetDropdown {
            }
            Dropdown {
            }
            Button { "Add attachment" }
        }
    }
}
