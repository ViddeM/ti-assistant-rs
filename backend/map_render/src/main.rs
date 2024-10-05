use std::sync::{
    mpsc::{sync_channel, Receiver, SyncSender},
    Mutex,
};

use bevy::{asset::AssetMetaCheck, input::mouse::MouseWheel, prelude::*};
use serde::Deserialize;
use ti_helper_game_data::components::phase::Phase;
use ti_helper_game_logic::{game_options::GameOptions, gameplay::game_state::GameState};
use tile::{render_map, PlanetOwnerVisuals, SystemVisuals};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, UrlSearchParams, WebSocket};

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
    let window = web_sys::window().expect("Expected there to be a window!");
    let search = window
        .location()
        .search()
        .expect("Failed to find expected search parameters");
    let search_params =
        UrlSearchParams::new_with_str(&search).expect("Failed to create url search params");
    let game_id = search_params
        .get("gameId")
        .expect("Failed to retrieve game ID");
    run_game(&game_id).expect("Failed to start game");
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
    game_state: GameState,
    previous_game_state: Option<GameState>,
}

impl MapInfo {
    fn new(game_state: GameState) -> Self {
        Self {
            game_state,
            previous_game_state: None,
        }
    }

    fn update_game_state(&mut self, new_game_state: GameState) {
        self.previous_game_state = Some(self.game_state.clone());
        self.game_state = new_game_state;
    }
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

    match message {
        WsResponse::GameOptions(opts) => console_log(&format!("Got game options, opts: {opts:?}")), // self.game_options = Some(opts),
        WsResponse::JoinedGame(_) => console_log("Joined game successfully"),
        WsResponse::GameState(state) => {
            console_log("Got game state response");
            tx.send(state)
                .map_err(|err| format!("Failed to send game_state over channel, err: {err:?}"))?;
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
        .add_systems(Startup, (setup_camera, setup_loading_text).chain())
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
    asset_server: Res<AssetServer>,
    loading_text_query: Query<Entity, With<LoadingText>>,
    planet_owner_query: Query<Entity, With<PlanetOwnerVisuals>>,
    tile_visual_query: Query<Entity, With<SystemVisuals>>,
) {
    let receiver = game_info.rx.get_mut().expect("Failed to get mutex");
    let Some(game_state) = receiver.try_iter().last() else {
        // If we haven't received a new gamestate there's not much to do
        return;
    };

    let font = asset_server.load("slider_regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 48.0,
        color: Color::WHITE,
    };

    if let Ok(e) = loading_text_query.get_single() {
        commands.entity(e).despawn();
    }

    if game_state.hex_map.is_none() {
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                "No milty string specified for game, unable to render map",
                text_style,
            )
            .with_justify(JustifyText::Center),
            ..default()
        });
        return;
    }

    if matches!(game_state.phase, Phase::Creation | Phase::Setup) {
        commands.spawn(Text2dBundle {
            text: Text::from_section("Setup must be finalized to view the map.", text_style)
                .with_justify(JustifyText::Center),
            ..default()
        });
        return;
    }

    if let Some(map_info) = game_info.map_info.as_mut() {
        map_info.update_game_state(game_state);
    } else {
        game_info.map_info = Some(MapInfo::new(game_state));
    };
    let game_state = &game_info
        .map_info
        .as_ref()
        .expect("Should have just set game_state!")
        .game_state;

    // TODO: Instead of removing everything and respawning them, only update what has changed!
    planet_owner_query.iter().for_each(|entity| {
        commands
            .get_entity(entity)
            .expect("Entity to exist")
            .despawn()
    });
    tile_visual_query.iter().for_each(|entity| {
        commands
            .get_entity(entity)
            .expect("Entity to exist")
            .despawn()
    });
    render_map(commands, asset_server, game_state);
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
