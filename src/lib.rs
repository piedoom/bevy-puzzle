use bevy::{
    asset::LoadState,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCamera, Camera, OrthographicProjection},
        draw,
    },
    sprite::Rect,
};
use bevy_asset_ron::RonAssetPlugin;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use blocks::{commit_on_click, pattern_builder, BlockResources, BlockState, GameBoard, Pattern};
use rand::prelude::IteratorRandom;
pub mod blocks;
pub struct PuzzlePlugin;

/// Single unit entity. Use components/children to add effects or whatever idk
pub struct Unit;

/// Component that shows the `Unit` as highlighted
pub struct Highlight;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Load)
            .init_resource::<ActiveEntity>()
            .init_resource::<Handle<SettingsAsset>>()
            .init_resource::<CursorPosition>()
            .init_resource::<BlockResources>()
            .add_asset::<blocks::Pattern>()
            .init_asset_loader::<blocks::PatternLoader>()
            .add_plugin(RonAssetPlugin::<SettingsAsset>::new(&["rfg"]))
            .add_system_set(
                // Load setup
                SystemSet::on_enter(GameState::Load).with_system(load_setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Load).with_system(load_transition.system()),
            )
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(game_setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(input_system.system())
                    .with_system(blocks::block_style_system.system())
                    .with_system(blocks::highlight_under_cursor.system())
                    .with_system(active_follow_mouse.system())
                    .with_system(commit_on_click.system()),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Load,
    Main,
}

fn load_setup(
    mut cmd: Commands,
    mut assets: Res<AssetServer>,
    mut loading: ResMut<PreloadingAssets>,
    mut settings_handle: ResMut<Handle<SettingsAsset>>,
    mut block_resources: ResMut<BlockResources>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // load our settings file
    *settings_handle = assets.load("settings.rfg");
    loading.0.push(settings_handle.clone_untyped());

    // load all block patterns
    loading.0.append(&mut assets.load_folder("blocks").unwrap());

    // load textures
    block_resources.load(&mut materials, &mut assets, &mut loading);

    assets
        .watch_for_changes()
        .expect("could not watch for changes");
}

// Track any loading assets and transition to the next game state when ready
fn load_transition(
    mut cmd: Commands,
    loading: Res<PreloadingAssets>,
    assets: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
) {
    if loading
        .0
        .iter()
        .filter(|h| assets.get_load_state(*h) == LoadState::Loading)
        .count()
        == 0
    {
        // We are done loading! transition state.
        state.set(GameState::Main).ok();
    }
}

fn game_setup(
    mut cmd: Commands,
    settings: Res<Assets<SettingsAsset>>,
    settings_handle: Res<Handle<SettingsAsset>>,
    patterns: Res<Assets<Pattern>>,
) {
    let settings = settings.get(settings_handle.clone()).unwrap();
    let (size_x, size_y) = (settings.board_size.x, settings.board_size.y);

    // create game grid
    for x in 0..size_x as usize {
        for y in 0..size_y as usize {
            // add a square
            cmd.spawn_bundle((
                BlockState::Empty,
                Transform::from_xyz(x as f32, y as f32, 0f32),
                GameBoard,
            ));
        }
    }

    // Set up the camera to be centerd on the game board
    // Calculate the overall size of the board, and divide to find the center point
    let trans = Transform::from_xyz(size_x / 2f32, size_y / 2f32, 10.0);
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = settings.camera_scale;
    cmd.spawn_bundle(camera_bundle).insert(trans);

    // add a block to follow around the cursor or something
    if let Some((handle, pattern)) = patterns.iter().choose(&mut rand::thread_rng()) {
        let a = pattern_builder(&mut cmd, pattern, BlockState::Full, Default::default());
        cmd.entity(a).insert(ActiveEntity);
    }
}

#[derive(Default)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

fn move_cursor(keyboard_input: Res<Input<KeyCode>>) {
    // let mut input = Vec2::default();
    // if keyboard_input.just_pressed(KeyCode::Up) {
    //     input.y + 1f32;
    // }
    // if keyboard_input.just_pressed(KeyCode::Down) {
    //     input.y - 1f32;
    // }
    // if keyboard_input.just_released(KeyCode::Left) {
    //     input.x - 1f32;
    // }
    // if keyboard_input.just_released(KeyCode::Right) {
    //     input.x + 1f32;
    // }
    // cursor.reposition(input);
}

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268fb"]
pub struct SettingsAsset {
    pub style: Style,
    pub board_size: Vec2,
    pub camera_scale: f32,
}

#[derive(serde::Deserialize)]
pub struct Style {
    pub outline: Color,
    pub line_width: f32,
    pub margin: f32,
}

fn input_system(
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    cameras: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
    mut cursor_pos: ResMut<CursorPosition>,
) {
    // get the primary window
    let window = windows.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = window.cursor_position() {
        // get the size of the window
        let size = Vec2::new(window.width() as f32, window.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let (camera_transform, proj) = cameras.single().unwrap();

        // apply the camera transform
        let pos_world = camera_transform.compute_matrix() * proj.scale * p.extend(0.0).extend(1.0);
        let pos_world = Vec2::from(pos_world);
        *cursor_pos.local = *pos_world;
        *cursor_pos.global = *(pos_world + Vec2::from(camera_transform.translation));
    }
}

#[derive(Default)]
pub struct CursorPosition {
    pub local: Vec2,
    pub global: Vec2,
}

#[derive(Default)]
pub struct ActiveEntity;

pub fn active_follow_mouse(
    active: Query<&mut Transform, With<ActiveEntity>>,
    cursor: Res<CursorPosition>,
) {
    active.for_each_mut(|mut transform| {
        transform.translation.x = cursor.global.x;
        transform.translation.y = cursor.global.y;
    });
}
