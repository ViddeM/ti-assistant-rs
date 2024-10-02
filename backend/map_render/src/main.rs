use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, sync_channel, Receiver, Sender, SyncSender, TryRecvError},
        Arc, Mutex,
    },
    thread::yield_now,
    time::Duration,
};

use bevy::{
    a11y::AccessibilityPlugin,
    app::{PanicHandlerPlugin, PluginGroupBuilder},
    asset::AssetMetaCheck,
    diagnostic::DiagnosticsPlugin,
    ecs::system::EntityCommands,
    input::{mouse::MouseWheel, InputPlugin},
    log::LogPlugin,
    prelude::*,
    time::TimePlugin,
};
use chrono::Utc;
use serde::Deserialize;
use system_planets::planet_offset;
use ti_helper_game::{
    data::{
        common::{color, faction::Faction},
        components::{
            planet::Planet,
            system::{systems, SystemId},
        },
    },
    game_options::GameOptions,
    gameplay::{
        event::Event,
        game::Game,
        game_settings::{Expansions, GameSettings},
        game_state::{self, GameState},
        player::{NewPlayer, Player},
    },
};
use tile::{setup_map, tile_pos_to_visual_pos, SystemVisuals};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

use crate::tile::tile_offset_to_visual_pos;

pub mod system_planets;
pub mod tile;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(s: &str);
}

fn main() {
    /* TODO: Figure out how to run the game from outside webassembly (currently requires calling the start_game method after this). If main runs the game it never returns and which is a problem... */
    // TODO: READ game id from query param
    run_game("9f4c4174").expect("Failed to start game");
}

#[derive(Resource, Debug)]
struct GameInfo {
    rx: Mutex<Receiver<GameState>>,
    map_info: Option<MapInfo>,
}

impl GameInfo {
    fn new(rx: Receiver<GameState>) -> Self {
        Self {
            rx: Mutex::new(rx),
            map_info: None,
        }
    }
}

#[derive(Debug)]
struct MapInfo {
    last_game_state: GameState,
}

#[derive(Debug, Clone, Deserialize, serde::Serialize)]
enum WsResponse {
    GameOptions(GameOptions),
    JoinedGame(String),
    GameState(GameState),
}

fn handle_message(e: MessageEvent, tx: SyncSender<GameState>) -> Result<(), String> {
    let data = e.data();
    let Some(data) = data.as_string() else {
        return Err(format!(
            "Data was not string, not sure what to do. Data: {data:?}"
        ));
    };

    let message: WsResponse = serde_json::from_str(&data)
        .map_err(|err| format!("Failed to deserialize message, err: {err:?}"))?;

    console_log(&format!("HELLO message: {:?}", message));

    match message {
        WsResponse::GameOptions(opts) => console_log(&format!("Got game options, opts: {opts:?}")), // self.game_options = Some(opts),
        WsResponse::JoinedGame(_) => console_log("Joined game successfully"),
        WsResponse::GameState(state) => {
            console_log("Got game state response");
            tx.send(state)
                .map_err(|err| format!("Failed to send game_state over channel, err: {err:?}"))?;
            console_log("SENT");
        }
    }

    Ok(())
}

fn run_game(game_id: &str) -> Result<(), String> {
    console_log(&format!("Running game with game_id: {game_id}"));
    let ws = WebSocket::new("ws://localhost:5555")
        .map_err(|err| format!("Failed to setup ws connection, err: {err:?}"))?;

    let (tx, rx) = sync_channel::<GameState>(2);

    let on_message_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        let tx = tx.clone();
        if let Err(err) = handle_message(e, tx) {
            console_error(&format!("Failed to handle websocket message, err: {err:?}"))
        }
    });
    ws.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
    on_message_callback.forget();

    let ws_clone = ws.clone();
    let game_id_clone = game_id.to_string();

    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log("Socket opened !");
        let data = format!("{{\"JoinGame\": \"{game_id_clone}\"}}");
        match ws_clone.send_with_str(&data) {
            Ok(_) => console_log("Message sent successfully"),
            Err(err) => console_error(&format!(
                "Failed to send join game message to server, err: {err:?}"
            )),
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    let game_info = GameInfo::new(rx);

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(game_info)
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_loading_text,
                // setup_map,
                // display_planet_ownership
            )
                .chain(),
        )
        .add_systems(Update, (zooming, update_map_from_channel))
        .run();

    Ok(())
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

const SCROLL_STEP: f32 = 0.001;
const MAX_SCALE: f32 = 5.0;
const MIN_SCALE: f32 = 0.2;

fn zooming(
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut projection = camera_query.single_mut();
    let steps: f32 = scroll_evr.read().map(|e| e.y).sum();
    if steps == 0. {
        return;
    }

    let new_projection =
        (projection.scale - SCROLL_STEP * steps * projection.scale).clamp(MIN_SCALE, MAX_SCALE);
    projection.scale = new_projection;
}

fn update_map_from_channel(
    mut game_info: ResMut<GameInfo>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    loading_text: Query<Entity, With<LoadingText>>,
) {
    let receiver = game_info.rx.get_mut().expect("Failed to get mutex");
    let Some(game_state) = receiver.try_iter().last() else {
        // If we haven't received a new gamestate there's not much to do
        return;
    };

    if game_info.map_info.is_none() {
        // This was the first time we got a game_state, delete the loading text.
        let entity = loading_text.single();
        commands.entity(entity).despawn();

        let font = asset_server.load("slider_regular.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 48.0,
            color: Color::WHITE,
        };
        commands.spawn((
            Text2dBundle {
                text: Text::from_section("LOADED!", text_style).with_justify(JustifyText::Center),
                ..default()
            },
            LoadingText,
        ));
    }

    game_info.map_info = Some(MapInfo {
        last_game_state: game_state,
    });

    // Update the gamestate

    // If map exists, do nothing for now
    // Otherwise, create the map.
}

#[derive(Component)]
struct LoadingText;

fn setup_loading_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("slider_regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 48.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Loading...", text_style).with_justify(JustifyText::Center),
            ..default()
        },
        LoadingText,
    ));
}

// fn display_planet_ownership(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     game_info: Res<GameInfo>,
//     system_visuals_query: Query<&SystemVisuals>,
// ) {
//     let owned_planets = game_info
//         .game
//         .current
//         .players
//         .values()
//         .map(|player| {
//             player
//                 .planets
//                 .keys()
//                 .cloned()
//                 .map(move |planet| (planet, player))
//         })
//         .flatten()
//         .collect::<HashMap<Planet, &Player>>();

//     let system_planets: HashMap<SystemId, Vec<Planet>> = systems()
//         .into_iter()
//         .map(|(system_id, system)| (system_id, system.planets))
//         .collect();

//     let font = asset_server.load("slider_regular.ttf");
//     let text_style = TextStyle {
//         font: font.clone(),
//         font_size: 24.0,
//         color: Color::BLACK,
//     };

//     for visuals in system_visuals_query.iter() {
//         if let Some(planets) = system_planets.get(&visuals.system_id) {
//             for planet in planets.iter() {
//                 if let Some(owner) = owned_planets.get(planet) {
//                     let base_pos = tile_pos_to_visual_pos(visuals.tile_pos);
//                     let offset = tile_offset_to_visual_pos(planet_offset(planet));
//                     let position = base_pos + offset;

//                     commands.spawn((Text2dBundle {
//                         text: Text::from_section(owner.name.clone(), text_style.clone())
//                             .with_justify(JustifyText::Center),
//                         transform: Transform::from_translation(position),
//                         ..default()
//                     },));
//                 }
//             }
//         }
//     }
// }
