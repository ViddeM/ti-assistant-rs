use bevy::{input::mouse::MouseWheel, prelude::*};
use tile::setup_map;

pub mod tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#map_render_canvas".into()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_map))
        .add_systems(Update, zooming)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

const SCROLL_STEP: f32 = 0.08;
const MAX_SCALE: f32 = 3.5;
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
