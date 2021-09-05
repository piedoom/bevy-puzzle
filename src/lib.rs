use std::{marker::PhantomData, path::PathBuf};

use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadState, LoadedAsset},
    ecs::component::Component,
    prelude::*,
    reflect::TypeUuid,
    render::camera::{Camera, OrthographicProjection},
};
use bevy_asset_ron::RonAssetPlugin;
use rand::prelude::IteratorRandom;
pub struct PuzzlePlugin;

/// Single unit entity. Use components/children to add effects or whatever idk
pub struct Unit;

/// Component that shows the `Unit` as highlighted
pub struct Highlight;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Load)
            .init_resource::<Score>()
            .init_resource::<ActiveEntity>()
            .init_resource::<BlockResource<states::Empty>>()
            .init_resource::<BlockResource<states::Hover>>()
            .init_resource::<BlockResource<states::Full>>()
            .init_resource::<BlockResource<states::Invalid>>()
            .init_resource::<BlockResource<states::Scored>>()
            .init_resource::<Handle<SettingsAsset>>()
            .init_resource::<CursorPosition>()
            .add_asset::<Pattern>()
            .init_asset_loader::<PatternLoader>()
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
                    .with_system(add_sprite_to_tiles.system())
                    .with_system(highlight_under_cursor.system())
                    .with_system(active_follow_mouse.system())
                    .with_system(commit_on_click.system())
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(scorer.system())
                    .with_system(score_scored.system())
                    .after("main")
                    .before("styles"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(style::<states::Empty>.system())
                    .with_system(style::<states::Hover>.system())
                    .with_system(style::<states::Full>.system())
                    .with_system(style::<states::Invalid>.system())
                    .with_system(style::<states::Scored>.system())
                    .label("styles")
                    .after("main"),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Load,
    Main,
}

fn load_setup(
    assets: Res<AssetServer>,
    mut loading: ResMut<PreloadingAssets>,
    mut settings_handle: ResMut<Handle<SettingsAsset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut empty: ResMut<BlockResource<states::Empty>>,
    mut hover: ResMut<BlockResource<states::Hover>>,
    mut full: ResMut<BlockResource<states::Full>>,
    mut invalid: ResMut<BlockResource<states::Invalid>>,
    mut scored: ResMut<BlockResource<states::Scored>>,
) {
    // load our settings file
    *settings_handle = assets.load("settings.rfg");
    loading.0.push(settings_handle.clone_untyped());

    // load all block patterns
    loading.0.append(&mut assets.load_folder("blocks").unwrap());

    // load textures
    let mut load_tex = |path: &'static str| {
        let texture: Handle<Texture> = assets.load(PathBuf::from(format!("sprites/{}.png", path)));
        loading.0.push(texture.clone_untyped());
        let material: Handle<ColorMaterial> = materials.add(texture.clone().into());
        (texture.clone(), material.clone())
    };

    // Load all blocks
    *empty = BlockResource::<states::Empty>::new(load_tex("empty"));
    *hover = BlockResource::<states::Hover>::new(load_tex("hover"));
    *full = BlockResource::<states::Full>::new(load_tex("full"));
    *invalid = BlockResource::<states::Invalid>::new(load_tex("invalid"));
    *scored = BlockResource::<states::Scored>::new(load_tex("scored"));

    assets
        .watch_for_changes()
        .expect("could not watch for changes");
}

// Track any loading assets and transition to the next game state when ready
fn load_transition(
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
                states::Empty,
                Transform::from_xyz(x as f32, y as f32, 0f32),
                GameBoard,
                Tile,
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
    if let Some((_, pattern)) = patterns.iter().choose(&mut rand::thread_rng()) {
        let a = pattern_builder::<states::Full>(&mut cmd, pattern, Default::default());
        cmd.entity(a).insert(ActiveEntity);
    }
}

#[derive(Default)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

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

pub type Score = u64;

#[derive(Default, Debug, Clone, TypeUuid, serde::Deserialize)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b505b"]
pub struct Pattern {
    pub color: Color,
    pub blocks: Vec<Vec2>,
}

#[derive(Default)]
pub struct PatternLoader;

impl AssetLoader for PatternLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let input = String::from_utf8(bytes.to_vec())?;
            let asset = Pattern::from_emoji(input);
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["block"]
    }
}

impl Pattern {
    pub fn from_emoji(input: impl ToString) -> Self {
        // split at first newline
        let input = input.to_string();
        let (color, pattern) = input.split_once('\n').unwrap();
        let color = Color::hex(color).unwrap();
        let mut blocks = Vec::<Vec2>::default();
        let mut cur = Vec2::ZERO;
        pattern.to_string().chars().for_each(|c| {
            match c {
                '⬛' => {
                    cur.x += 1.0;
                }
                '⬜' => {
                    blocks.push(cur);
                    cur.x += 1.0;
                }
                '\n' => {
                    cur.x = 0f32;
                    cur.y += 1.0;
                }
                e => warn!("unrecognized char \"{}\" in pattern", e),
            };
        });
        Self { color, blocks }
    }
}

#[derive(Default, Clone)]
pub struct BlockResource<T> {
    phantom: PhantomData<T>,
    pub texture: Handle<Texture>,
    pub material: Handle<ColorMaterial>,
}

impl<T> BlockResource<T> {
    pub fn new(tex_mat: (Handle<Texture>, Handle<ColorMaterial>)) -> Self {
        Self {
            texture: tex_mat.0.clone(),
            material: tex_mat.1.clone(),
            phantom: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct Textures {
    pub empty: Handle<Texture>,
    pub hover: Handle<Texture>,
    pub full: Handle<Texture>,
}

#[derive(Default)]
pub struct Materials {
    pub empty: Handle<ColorMaterial>,
    pub hover: Handle<ColorMaterial>,
    pub full: Handle<ColorMaterial>,
}

pub fn add_sprite_to_tiles(mut cmd: Commands, query: Query<(Entity, &Transform), Added<Tile>>) {
    // add sprite bundle
    query.for_each(|(entity, transform)| {
        cmd.entity(entity).insert_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(1f32, 1f32)),
            transform: transform.clone(),
            global_transform: transform.clone().into(),
            ..Default::default()
        });
    });
}

pub fn style<T: Component>(
    mut cmd: Commands,
    query: Query<(Entity, Option<&Color>), Added<T>>,
    style: Res<BlockResource<T>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.for_each_mut(|(entity, color)| {
        let texture = materials
            .get(style.material.clone())
            .unwrap()
            .texture
            .as_ref()
            .unwrap()
            .clone();
        // TODO: lol
        let new = materials.add(ColorMaterial {
            color: *color.unwrap_or(&Color::WHITE),
            texture: Some(texture),
        });
        cmd.entity(entity).insert(new.clone());
    });
}

/// actually builds entities from a block pattern. Returns the parent entity of the newly created block structure
pub fn pattern_builder<T: Component + Default>(
    cmd: &mut Commands,
    pattern: &Pattern,
    transform: Transform,
) -> Entity {
    cmd.spawn_bundle((transform.clone(), GlobalTransform::from(transform.clone())))
        .with_children(|p| {
            for block in pattern.blocks.iter() {
                p.spawn_bundle((
                    T::default(),
                    Transform::from_xyz(block.x, block.y, 1f32),
                    GlobalTransform::from_xyz(block.x, block.y, 1f32),
                    pattern.color.clone(),
                    Tile,
                ));
            }
        })
        .id()
}

pub fn highlight_under_cursor(
    mut cmd: Commands,
    empty: Query<(Entity, &Transform), (With<states::Empty>, With<GameBoard>)>,
    hover: Query<(Entity, &Transform), (With<states::Hover>, With<GameBoard>)>,
    full: Query<(Entity, &Transform), (With<states::Full>, With<GameBoard>)>,
    invalid: Query<(Entity, &Transform), (With<states::Invalid>, With<GameBoard>)>,
    active: Query<Entity, With<ActiveEntity>>,
    children: Query<&Children>,
    transforms: Query<&GlobalTransform>,
) {
    // get all blocks in the active pattern
    active.for_each(|entity| {
        // compare and highlight tiles on the gameboard

        // get all active piece coordinates and collect into a vec
        let active_coordinates: Vec<Vec2> = children
            .get(entity)
            .unwrap()
            .iter()
            .filter_map(|active_entity| match transforms.get(*active_entity) {
                Ok(transform) => Some(Vec2::new(
                    transform.translation.x.round(),
                    transform.translation.y.round(),
                )),
                Err(_) => None,
            })
            .collect();

        let vec_floor = |t: &Transform| Vec2::new(t.translation.x.floor(), t.translation.y.floor());

        // Change to hovering for empty
        empty.for_each_mut(|(entity, transform)| {
            let pos = vec_floor(transform);
            if active_coordinates.contains(&pos) {
                // add hover marker and remove empty
                cmd.entity(entity)
                    .remove::<states::Empty>()
                    .insert(states::Hover);
            }
        });

        // Invalid states do not remove any other states and preserve the underlying block type that is important to the game

        // Add an invalid state to full blocks
        full.for_each_mut(|(entity, transform)| {
            let pos = vec_floor(transform);
            if active_coordinates.contains(&pos) {
                cmd.entity(entity).insert(states::Invalid);
            }
        });

        // Remove invalid state when no longer hovered
        invalid.for_each_mut(|(entity, transform)| {
            let pos = vec_floor(transform);
            if !active_coordinates.contains(&pos) {
                cmd.entity(entity).remove::<states::Invalid>();
            }
        });

        // Unhover blocks that aren't hovered anymore
        hover.for_each_mut(|(entity, transform)| {
            let pos = vec_floor(transform);
            if !active_coordinates.contains(&pos) {
                // remove hover marker and add empty
                cmd.entity(entity)
                    .remove::<states::Hover>()
                    .insert(states::Empty);
            }
        });
    });
}

pub fn commit_on_click(
    mut cmd: Commands,
    invalid: Query<&states::Invalid>,
    hover: Query<Entity, (With<states::Hover>, With<GameBoard>)>,
    mouse: Res<Input<MouseButton>>,
    active: Query<Entity, With<ActiveEntity>>,
    colors: Query<&Color>,
    children: Query<&Children>,
    patterns: Res<Assets<Pattern>>,
    cursor: Res<CursorPosition>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // first, ensure there are no invalid blocks
        if invalid.iter().count() == 0 {
            if let Some(active_entity) = active.iter().next() {
                let active_blocks_count = children
                    .get(active_entity)
                    .map(|x| x.iter().count())
                    .unwrap_or(0);

                // also ensure that the number of hovered blocks matches the number of active
                if hover.iter().count() == active_blocks_count {
                    let mut color = Color::WHITE;
                    if let Some(entity) = active.iter().next() {
                        if let Ok(children) = children.get(entity) {
                            if let Some(child) = children.first() {
                                if let Ok(c) = colors.get(*child) {
                                    color = *c;
                                }
                            }
                        };
                    }

                    hover.for_each(|e| {
                        cmd.entity(e)
                            .remove::<states::Hover>()
                            .insert(states::Full)
                            .insert(color);
                    });

                    // reset the piece
                    active.for_each(|e| cmd.entity(e).despawn_recursive());

                    if let Some((_, pattern)) = patterns.iter().choose(&mut rand::thread_rng()) {
                        let a = pattern_builder::<states::Full>(
                            &mut cmd,
                            pattern,
                            Transform::from_xyz(cursor.global.x, cursor.global.y, 0f32),
                        );
                        cmd.entity(a).insert(ActiveEntity);
                    }
                }
            }
        }
    }
}
// if there is 5 full blocks in a full square, remove and score
pub fn scorer(
    mut cmd: Commands,
    board: Query<(Entity, &GlobalTransform), With<GameBoard>>,
    full_tiles: Query<(Entity, &Transform), With<states::Full>>,
) {
    let mut scoring_tiles = vec![];
    board.for_each(|(_, t)| {
        let mut possible_tiles = vec![];
        let mut scored = true;

        for x in -1..=1 {
            for y in -1..=1 {
                // Get the current block (in all blocks)
                let mut cmp_translation = t.translation.truncate();
                cmp_translation.x += x as f32;
                cmp_translation.y += y as f32;
                if let Some((entity, _)) = full_tiles
                    .iter()
                    .find(|(_, t)| Vec2::from(t.translation) == cmp_translation)
                {
                    possible_tiles.push(entity);
                } else {
                    // couldn't find one! failed.
                    scored = false;
                };
            }
        }
        if scored {
            scoring_tiles.extend_from_slice(&possible_tiles);
        }
    });

    for e in scoring_tiles {
        cmd.entity(e)
            .remove::<states::Full>()
            .insert(states::Scored);
    }
}

pub fn score_scored(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    scored: Query<Entity, With<states::Scored>>,
) {
    scored.for_each(|e| {
        *score += 1;
        cmd.entity(e)
            .remove::<states::Scored>()
            .remove::<Color>()
            .insert(states::Empty);
    });
}

/// Marker that means the block is part of the game board
pub struct GameBoard;

/// Includes every tile ever
pub struct Tile;

pub mod states {
    pub fn all() -> Vec<&'static str> {
        vec!["empty", "full", "hover", "ghost", "invalid", "scored"]
    }
    #[derive(Default)]
    pub struct Empty;
    #[derive(Default)]
    pub struct Full;
    #[derive(Default)]
    pub struct Hover;
    #[derive(Default)]
    pub struct Ghost;
    #[derive(Default)]
    pub struct Invalid;
    /// Already scored tile that will be cleaned up
    #[derive(Default)]
    pub struct Scored;
}

struct ScoreText;
