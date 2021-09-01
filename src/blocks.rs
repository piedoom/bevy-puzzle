use std::path::PathBuf;

use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    render::camera::Camera,
    utils::HashMap,
};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillOptions, GeometryBuilder, ShapeColors, StrokeOptions},
    shapes,
};
use rand::prelude::IteratorRandom;
use serde::Deserialize;

use crate::{ActiveEntity, CursorPosition, PreloadingAssets, SettingsAsset};

#[derive(Default, Debug, Clone, TypeUuid, Deserialize)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b505b"]
pub struct Pattern(pub Vec<Vec2>);

#[derive(Default)]
pub struct PatternLoader;

impl AssetLoader for PatternLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let asset = Pattern::from_emoji(String::from_utf8(bytes.to_vec())?);
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
        let mut blocks = Vec::<Vec2>::default();
        let mut cur = Vec2::ZERO;
        input.to_string().chars().for_each(|c| {
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
        Self(blocks)
    }
}

#[derive(Default)]
pub struct BlockResources(HashMap<BlockState, BlockResource>);

#[derive(Clone)]
pub struct BlockResource {
    pub texture: Handle<Texture>,
    pub material: Handle<ColorMaterial>,
}

impl BlockResource {
    pub fn new(texture: Handle<Texture>, material: Handle<ColorMaterial>) -> Self {
        Self { texture, material }
    }
}

impl BlockResources {
    pub fn load(
        &mut self,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        assets: &mut Res<AssetServer>,
        loading: &mut ResMut<PreloadingAssets>,
    ) {
        // Load all blocks
        for state in BlockState::all() {
            let texture: Handle<Texture> = assets
                .load::<Texture, PathBuf>(format!("sprites/{}.png", state.as_str()).into())
                .into();
            loading.0.push(texture.clone_untyped());
            let material: Handle<ColorMaterial> = materials.add(texture.clone().into());
            self.0
                .insert(state, BlockResource::new(texture.clone(), material.clone()));
        }
    }
    pub fn get(&self, state: &BlockState) -> Option<BlockResource> {
        self.0.get(state).cloned()
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

/// enforce styles
pub fn block_style_system(
    mut cmd: Commands,
    blocks: Query<
        (Entity, &BlockState, &Transform, Option<&mut ColorMaterial>),
        Or<(Added<BlockState>, Changed<BlockState>)>,
    >,
    settings_handle: Res<Handle<SettingsAsset>>,
    settings: Res<Assets<SettingsAsset>>,
    block_resources: Res<BlockResources>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let settings = settings.get(settings_handle.clone()).unwrap();
    let shape = shapes::RegularPolygon {
        sides: 4,
        center: Vec2::new(0.5, 0.5),
        feature: shapes::RegularPolygonFeature::SideLength(1f32 - (settings.style.margin * 2f32)),
    };

    blocks.for_each_mut(|(entity, block, transform, sprite)| {
        let resources = block_resources.get(block).unwrap();
        match sprite {
            Some(_) => {
                cmd.entity(entity).insert(resources.material);
            }
            None => {
                // add sprite bundle
                cmd.entity(entity).insert_bundle(SpriteBundle {
                    sprite: Sprite::new(Vec2::new(1f32, 1f32)),
                    transform: transform.clone(),
                    global_transform: transform.clone().into(),
                    material: resources.material,
                    ..Default::default()
                });
            }
        }
    });
}

/// actually builds entities from a block pattern. Returns the parent entity of the newly created block structure
pub fn pattern_builder(
    cmd: &mut Commands,
    pattern: &Pattern,
    block_state: BlockState,
    transform: Transform,
) -> Entity {
    cmd.spawn_bundle((transform.clone(), GlobalTransform::from(transform.clone())))
        .with_children(|p| {
            for block in pattern.0.iter() {
                info!("{}, {}", block.x, block.y);
                p.spawn_bundle((
                    BlockState::Full,
                    Transform::from_xyz(block.x, block.y, 1f32),
                    GlobalTransform::from_xyz(block.x, block.y, 1f32),
                ));
            }
        })
        .id()
}

pub fn highlight_under_cursor(
    mut query: Query<(Entity, &Transform), With<GameBoard>>,
    mut states: Query<&mut BlockState>,
    camera_transform: Query<&Transform, With<Camera>>,
    cursor: Res<CursorPosition>,
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
                    transform.translation.x.floor(),
                    transform.translation.y.floor(),
                )),
                Err(_) => None,
            })
            .collect();

        // loop through game board and highlight transforms that match active coordinates
        query.for_each(|(entity, transform)| {
            // floor and compare transforms or something
            let tile_pos = Vec2::new(
                transform.translation.x.floor(),
                transform.translation.y.floor(),
            );

            if let Ok(mut state) = states.get_mut(entity) {
                *state = if active_coordinates.contains(&tile_pos) {
                    // hovered
                    match *state {
                        BlockState::Empty => BlockState::Hover,
                        BlockState::Full => BlockState::Invalid,
                        BlockState::Hover => BlockState::Hover,
                        BlockState::Ghost => BlockState::Ghost,
                        BlockState::Invalid => BlockState::Invalid,
                    }
                } else {
                    // not hovered
                    match *state {
                        BlockState::Empty => BlockState::Empty,
                        BlockState::Full => BlockState::Full,
                        BlockState::Hover => BlockState::Empty,
                        BlockState::Ghost => BlockState::Ghost,
                        BlockState::Invalid => BlockState::Full,
                    }
                }
            }
        });
    });
}

pub fn commit_on_click(
    mut cmd: Commands,
    query: Query<&mut BlockState, With<GameBoard>>,
    mouse: Res<Input<MouseButton>>,
    active: Query<Entity, With<ActiveEntity>>,
    patterns: Res<Assets<Pattern>>,
    cursor: Res<CursorPosition>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // first, ensure there are no invalid blocks
        query.for_each_mut(|mut state| {
            if *state == BlockState::Hover {
                *state = BlockState::Full;
            }
        });

        active.for_each(|e| cmd.entity(e).despawn_recursive());

        if let Some((handle, pattern)) = patterns.iter().choose(&mut rand::thread_rng()) {
            let a = pattern_builder(
                &mut cmd,
                pattern,
                BlockState::Full,
                Transform::from_xyz(cursor.global.x, cursor.global.y, 0f32),
            );
            cmd.entity(a).insert(ActiveEntity);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BlockState {
    Empty,
    Full,
    Hover,
    Ghost,
    Invalid,
}

impl BlockState {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockState::Empty => "empty",
            BlockState::Full => "full",
            BlockState::Hover => "hover",
            BlockState::Ghost => "ghost",
            BlockState::Invalid => "invalid",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Empty,
            Self::Full,
            Self::Hover,
            Self::Ghost,
            Self::Invalid,
        ]
    }
}
/// Marker that means the block is part of the game board
pub struct GameBoard;
