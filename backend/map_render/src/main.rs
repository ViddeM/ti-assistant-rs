use std::collections::HashMap;

use bevy::{input::mouse::MouseWheel, prelude::*};
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

use crate::tile::tile_offset_to_visual_pos;

pub mod system_planets;
pub mod tile;

fn main() {
    run_game();
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
pub fn run_game() {
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#map_render_canvas".into()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(game_info_res)
        .add_systems(
            Startup,
            (setup_camera, setup_map, display_planet_ownership).chain(),
        )
        .add_systems(Update, zooming)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

// const SCROLL_STEP: f32 = 0.0005;
const SCROLL_STEP: f32 = 0.05;
const MAX_SCALE: f32 = 5.0;
const MIN_SCALE: f32 = 0.2;

fn zooming(
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let mut projection = camera_query.single_mut();

    let steps: f32 = scroll_evr.read().map(|e| e.y).sum();

    let new_scale = (projection.scale - steps * SCROLL_STEP)
        .max(MIN_SCALE)
        .min(MAX_SCALE);

    projection.scale = new_scale;
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
