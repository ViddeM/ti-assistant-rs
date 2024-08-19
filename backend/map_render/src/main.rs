use std::collections::HashMap;

use bevy::{asset::AssetMetaCheck, input::mouse::MouseWheel, prelude::*};
use chrono::Utc;
use system_planets::planet_offset;
use ti_helper_game::{
    data::{
        common::{color, faction::Faction},
        components::{
            planet::Planet,
            system::{systems, SystemId},
        },
    },
    gameplay::{
        event::Event,
        game::Game,
        game_settings::{Expansions, GameSettings},
        game_state::GameState,
        player::{NewPlayer, Player},
    },
};
use tile::{setup_map, tile_pos_to_visual_pos, SystemVisuals};
use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

use crate::tile::tile_offset_to_visual_pos;

pub mod system_planets;
pub mod tile;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn main() {
    /* TODO: Figure out how to run the game from outside webassembly (currently requires calling the start_game method after this). If main runs the game it never returns and which is a problem... */
}

#[derive(Resource)]
struct GameInfo {
    game: Game,
}

macro_rules! ev {
    ($game: expr, $event: expr) => {
        $game.apply($event, Utc::now())
    };
    ($game: expr, $name: literal, $faction: ident, $color: ident) => {
        ev!(
            $game,
            Event::AddPlayer {
                player: NewPlayer {
                    name: $name.into(),
                    faction: Faction::$faction,
                    color: color::Color::$color,
                }
            }
        )
    };
}

#[wasm_bindgen]
pub fn start_game(game_id: &str) -> Result<(), JsValue> {
    run_game(game_id)?;

    Ok(())
}

pub fn run_game(game_id: &str) -> Result<(), String> {
    let ws = WebSocket::new("ws://localhost:5555")
        .map_err(|err| format!("Failed to setup ws connection, err: {err:?}"))?;

    let game_state = GameState::default();

    let mut game = Game {
        players: vec![],
        current: game_state.into(),
        history: vec![],
    };

    ev!(
        game,
        Event::SetSettings {
            settings: GameSettings {
                max_points: 14,
                expansions: Expansions {
                    codex_1: false,
                    codex_2: false,
                    codex_3: false,
                    prophecy_of_kings: true,
                }
            }
        }
    );
    ev!(game, "Gurr", Nomad, Purple);
    ev!(game, "Portals", Winnu, Pink);
    ev!(game, "Tux", XxchaKingdom, Green);
    ev!(game, "Swexbe", EmbersOfMuaat, Red);
    ev!(game, "Sponken", YssarilTribes, Black);
    ev!(game, "Håll", MahactGeneSorcerers, Yellow);
    ev!(game, "Vidde", GhostsOfCreuss, Blue);
    ev!(game, "Hoidi", ArgentFlight, Orange);

    let game_info_res = GameInfo { game };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin::default())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(game_info_res)
        .add_systems(
            Startup,
            (setup_camera, setup_map, display_planet_ownership).chain(),
        )
        .add_systems(Update, zooming)
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

fn display_planet_ownership(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_info: Res<GameInfo>,
    system_visuals_query: Query<&SystemVisuals>,
) {
    let owned_planets = game_info
        .game
        .current
        .players
        .values()
        .map(|player| {
            player
                .planets
                .keys()
                .cloned()
                .map(move |planet| (planet, player))
        })
        .flatten()
        .collect::<HashMap<Planet, &Player>>();

    let system_planets: HashMap<SystemId, Vec<Planet>> = systems()
        .into_iter()
        .map(|(system_id, system)| (system_id, system.planets))
        .collect();

    let font = asset_server.load("slider_regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 24.0,
        color: Color::BLACK,
    };

    for visuals in system_visuals_query.iter() {
        if let Some(planets) = system_planets.get(&visuals.system_id) {
            for planet in planets.iter() {
                if let Some(owner) = owned_planets.get(planet) {
                    println!("OWNER MATCHING PLANET! {planet:?} {owner:?}");

                    let base_pos = tile_pos_to_visual_pos(visuals.tile_pos);
                    let offset = tile_offset_to_visual_pos(planet_offset(planet));

                    let position = base_pos + offset;
                    println!("\tPOSITION: {base_pos:?}, {offset:?}, {position:?}");

                    commands.spawn((Text2dBundle {
                        text: Text::from_section(owner.name.clone(), text_style.clone())
                            .with_justify(JustifyText::Center),
                        transform: Transform::from_translation(position),
                        ..default()
                    },));
                }
            }
        }
    }
}