use dioxus::prelude::*;
use ti_helper_game_data::{
    actions::event::Event, common::player_id::PlayerId, components::tech::TechOrigin,
};

use crate::{
    components::{button::Button, info_button::InfoButton},
    data::{
        event_context::EventContext, game_context::GameContext, info_context::Info,
        player_view::PlayerViewContext,
    },
    views::select_tech::SelectTechView,
};

const ENTROPIC_SCAR_PHASE_SCSS: Asset =
    asset!("/assets/styling/views/phase_views/entropic_scar_phase.scss");

#[component]
pub fn EntropicScarPhaseView() -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let players = use_memo(move || gc.game_state().turn_order.to_vec());

    rsx! {
        document::Stylesheet { href: ENTROPIC_SCAR_PHASE_SCSS }

        div { class: "column card",
            h2 { "Entropic Scar" }
            for player in players() {
                EntropicScarPlayer { key: "{player}", player }
            }
            Button {
                class: "margin-top",
                disabled: !view.is_global_or_speaker(),
                onclick: move |_| event.send_event(Event::CompleteEntropicScarPhase),
                "Continue"
            }
        }
    }
}

#[component]
fn EntropicScarPlayer(player: ReadSignal<PlayerId>) -> Element {
    let gc = use_context::<GameContext>();
    let view = use_context::<PlayerViewContext>();
    let event = use_context::<EventContext>();

    let mut picking = use_signal(|| false);

    let color = use_memo(move || {
        gc.game_state()
            .players
            .get(&player())
            .map(|p| p.color.name())
            .unwrap_or_default()
    });
    let gained_tech = use_memo(move || {
        gc.game_state()
            .status_phase_state
            .as_ref()
            .and_then(|state| state.entropic_scar_techs.get(&player()).cloned())
    });
    let has_faction_tech_left = use_memo(move || {
        let state = gc.game_state();
        let Some(p) = state.players.get(&player()) else {
            return false;
        };
        gc.game_options()
            .technologies
            .iter()
            .any(|t| t.info().origin == TechOrigin::Faction(p.faction) && !p.has_tech(t))
    });
    let base_techs = use_memo(move || {
        gc.game_options()
            .technologies
            .iter()
            .filter(|t| t.info().origin == TechOrigin::Base)
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        fieldset { class: "player-color-border-{color} entropic-scar-fieldset",
            legend { class: "player-color-border-{color} entropic-scar-legend", "{player}" }
            if let Some(tech) = gained_tech() {
                p {
                    "{tech.info().name}"
                    InfoButton { info: Info::Tech(tech) }
                }
            } else if view.display_for(&player()) {
                p { "Ships in an entropic scar?" }
                p { class: "warning-text", "Spend a strategy token" }
                if !has_faction_tech_left() {
                    p { "No faction technologies left to gain." }
                } else if picking() {
                    SelectTechView {
                        player_id: player(),
                        filtered_techs: base_techs(),
                        on_select: move |tech| {
                            event
                                .send_event(Event::GainEntropicScarTech {
                                    player: player(),
                                    tech,
                                });
                        },
                    }
                } else {
                    Button { onclick: move |_| picking.set(true), "Pick tech" }
                }
            } else {
                p { "No tech gained" }
            }
        }
    }
}
