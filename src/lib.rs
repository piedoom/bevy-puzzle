#![feature(duration_zero)]
use std::{
    collections::{hash_map::DefaultHasher, VecDeque},
    fs::File,
    hash::{Hash, Hasher},
    io::Write,
    iter::Peekable,
    marker::PhantomData,
    path::PathBuf,
    time::Duration,
};

use bevy::{
    asset::{AssetLoader, AssetPath, BoxedFuture, LoadContext, LoadState, LoadedAsset},
    ecs::component::Component,
    input::keyboard::KeyboardInput,
    prelude::*,
    reflect::TypeUuid,
    render::camera::{Camera, OrthographicProjection},
    utils::{AHasher, Instant},
};
use bevy_asset_ron::RonAssetPlugin;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    thread_rng,
};
mod timer;
pub mod ui;

pub use timer::{PlacementTimer, Timer};
pub struct PuzzlePlugin;

/// Single unit entity. Use components/children to add effects or whatever idk
pub struct Unit;

/// Component that shows the `Unit` as highlighted
pub struct Highlight;

#[derive(Default)]
pub struct Hold(Option<Pattern>);

impl Hold {
    pub fn get(&self) -> Option<&Pattern> {
        self.0.as_ref()
    }

    pub fn set(&mut self, pattern: Pattern) {
        self.0 = Some(pattern);
    }

    pub fn swap(&mut self, pattern: Pattern) -> Option<Pattern> {
        let ret = self.get().cloned();
        self.set(pattern);
        ret
    }
}

/// A random distribution of all pieces is in the bag
#[derive(Default, Clone)]
pub struct Bag {
    // All possible patterns that can be played
    patterns: Vec<Handle<Pattern>>,
    pub queue: VecDeque<Handle<Pattern>>,
}

impl Bag {
    pub fn new(patterns: Vec<Handle<Pattern>>) -> Self {
        let mut s = Self {
            patterns,
            queue: Default::default(),
        };
        s.next();
        s
    }
}

pub type NextUp = Handle<Pattern>;

impl Iterator for Bag {
    type Item = Handle<Pattern>;

    fn next(&mut self) -> Option<Self::Item> {
        // add more pieces if we have no more
        if self.queue.len() == 0 {
            self.patterns.shuffle(&mut thread_rng());
            for pattern in &self.patterns {
                self.queue.push_back(pattern.clone());
            }
        }
        self.queue.pop_front()
    }
}

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Load)
            .init_resource::<Score>()
            .init_resource::<ActiveEntity>()
            .init_resource::<BlockResources>()
            .insert_resource(PlacementTimer::from(Duration::from_millis(3000)))
            .init_resource::<Bag>()
            .init_resource::<Handle<SettingsAsset>>()
            .init_resource::<CursorPosition>()
            .init_resource::<Hold>()
            .init_resource::<NextUp>()
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
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(ui::menu_ui.system()))
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(game_setup.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(input_system.system())
                    .with_system(rotate_active.system())
                    .with_system(add_to_hold.system())
                    .with_system(add_sprite_to_tiles.system())
                    .with_system(active_follow_mouse.system())
                    .with_system(commit.system())
                    .with_system(update_hovered_board_pieces.system())
                    .with_system(ui::ui.system())
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(scorer.system())
                    .with_system(scored_effect.system())
                    .with_system(animate_active.system())
                    .after("main")
                    .before("styles"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(style_blocks.system())
                    .label("styles")
                    .after("main"),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Load,
    Menu,
    Main,
}

fn load_setup(
    assets: Res<AssetServer>,
    mut loading: ResMut<PreloadingAssets>,
    mut settings_handle: ResMut<Handle<SettingsAsset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut style: ResMut<BlockResources>,
) {
    // load our settings file
    *settings_handle = assets.load("settings.rfg");
    loading.0.push(settings_handle.clone_untyped());

    // load all block patterns
    let patterns = &mut assets.load_folder("blocks").unwrap();
    loading.0.append(patterns);

    // load textures
    let mut load_tex = |path: &'static str| {
        let texture: Handle<Texture> = assets.load(PathBuf::from(format!("sprites/{}.png", path)));
        loading.0.push(texture.clone_untyped());
        let material: Handle<ColorMaterial> = materials.add(texture.clone().into());
        (texture.clone(), material.clone())
    };

    *style = BlockResources {
        empty: BlockResource::new(load_tex("empty")),
        full: BlockResource::new(load_tex("full")),
        hover: BlockResource::new(load_tex("hover")),
        invalid: BlockResource::new(load_tex("invalid")),
        scored: BlockResource::new(load_tex("scored")),
    };

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
        state.set(GameState::Menu).ok();
    }
}

fn game_setup(
    mut cmd: Commands,
    active: Query<Entity, With<ActiveEntity>>,
    cursor: Res<CursorPosition>,
    settings: Res<Assets<SettingsAsset>>,
    settings_handle: Res<Handle<SettingsAsset>>,
    patterns: Res<Assets<Pattern>>,
    board: Query<Entity, With<GameBoard>>,
    cameras: Query<Entity, With<Camera>>,
    mut bag: ResMut<Bag>,
    mut timer: ResMut<PlacementTimer>,
    mut next_up: ResMut<NextUp>,
) {
    let settings = settings.get(settings_handle.clone()).unwrap();
    let (size_x, size_y) = (settings.board_size.x, settings.board_size.y);

    // Create the board if it doesn't already exist
    if board.iter().count() == 0 {
        // create game grid
        for x in 0..size_x as usize {
            for y in 0..size_y as usize {
                // add a square
                cmd.spawn_bundle((
                    states::Empty,
                    Transform::from_xyz(x as f32, y as f32, 0f32),
                    GameBoard,
                    Tile,
                    selection::None,
                ));
            }
        }
    }

    // Create camera if none exists
    if cameras.iter().count() == 0 {
        let camera_entity = cmd.spawn().id();
        // Calculate the overall size of the board, and divide to find the center point
        let trans = Transform::from_xyz(size_x / 2f32, size_y / 2f32, 10.0);
        let mut camera_bundle = OrthographicCameraBundle::new_2d();
        camera_bundle.orthographic_projection.scale = settings.camera_scale;
        cmd.entity(camera_entity)
            .insert_bundle(camera_bundle)
            .insert(trans);
    };

    // Add pieces to the bag
    *bag = Bag::new(
        patterns
            .iter()
            .map(|(x, _)| patterns.get_handle(x))
            .collect(),
    );

    // add a block to follow around the cursor or something
    *next_up = bag.next().unwrap();
    let entity = set_active_pattern_helper(
        &mut cmd,
        &active,
        patterns.get(next_up.clone()).unwrap(),
        cursor,
    );
    *next_up = bag.next().unwrap();

    // reset the timer on game start
    timer.reset();
}

#[derive(Default)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);

#[derive(serde::Deserialize, serde::Serialize, TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268fb"]
pub struct SettingsAsset {
    pub style: Style,
    pub board_size: Vec2,
    pub camera_scale: f32,
    pub leaderboard: Leaderboard,
}

#[derive(serde::Deserialize, serde::Serialize)]
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

pub type Score = usize;
/// An always sorted collection of highest scores

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Leaderboard {
    leaders: Vec<(String, usize)>,
    pub max_length: usize,
}

impl Leaderboard {
    /// Add an entry to the leaderboard if it is better than the worst score already on the leaderboards
    pub fn add(&mut self, name: &str, score: usize) -> bool {
        // obtain a hash to see if anything changes
        let mut before_hash = AHasher::default();
        self.leaders.hash(&mut before_hash);
        // push the entry and then truncate by our max length to get the new leaderboard
        self.leaders.push((name.to_string(), score));
        self.leaders.sort_by(|a, b| a.1.cmp(&b.1));
        self.leaders.truncate(self.max_length);
        // If the hash is not the same, it has been added!
        let mut after_hash = AHasher::default();
        self.leaders.hash(&mut after_hash);
        before_hash.finish() != after_hash.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populated_leaderboard() -> Leaderboard {
        Leaderboard {
            leaders: vec![
                ("Name1".into(), 300),
                ("Name2".into(), 200),
                ("Name3".into(), 100),
            ],
            max_length: 3,
        }
    }

    #[test]
    fn dont_add_bad_score_to_leaderboard() {
        let mut leaderboard = populated_leaderboard();
        let entry = ("Name4".to_string(), 50);
        let added = leaderboard.add(&entry.0, entry.1);
        assert_eq!(added, false);
        assert!(!leaderboard.leaders.contains(&entry));
    }

    #[test]
    fn add_new_to_leaderboard() {}
}

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
                    cur.y -= 1.0;
                }
                e => warn!("unrecognized char \"{}\" in pattern", e),
            };
        });
        Self { color, blocks }
    }
}

#[derive(Default, Clone)]
pub struct BlockResources {
    pub empty: BlockResource,
    pub full: BlockResource,
    pub hover: BlockResource,
    pub invalid: BlockResource,
    pub scored: BlockResource,
}

#[derive(Default, Clone)]
pub struct BlockResource {
    pub texture: Handle<Texture>,
    pub material: Handle<ColorMaterial>,
}

impl BlockResource {
    pub fn new(tex_mat: (Handle<Texture>, Handle<ColorMaterial>)) -> Self {
        Self {
            texture: tex_mat.0.clone(),
            material: tex_mat.1.clone(),
        }
    }
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

/// actually builds entities from a block pattern. Returns the parent entity of the newly created block structure
pub fn pattern_builder<T: Component + Default>(
    cmd: &mut Commands,
    pattern: &Pattern,
    transform: Transform,
) -> Entity {
    cmd.spawn_bundle((
        transform.clone(),
        GlobalTransform::from(transform.clone()),
        pattern.clone(),
    ))
    .with_children(|p| {
        for block in pattern.blocks.iter() {
            // TODO: adjust the 0.5 constant offset to allow for more natural (and dynamic) rotations
            // based off of block size. We likely will need to determine this when loading the asset
            let transform = Transform::from_xyz(block.x - 0.5, block.y + 0.5, 1f32);
            p.spawn_bundle((
                T::default(),
                transform,
                GlobalTransform::from(transform),
                pattern.color.clone(),
                Tile,
            ));
        }
    })
    .id()
}

pub type ActiveCoordinates = Vec<Vec2>;

pub fn update_hovered_board_pieces(
    mut cmd: Commands,
    active: Query<Entity, With<ActiveEntity>>,
    children: Query<&Children>,
    transforms: Query<&GlobalTransform>,
    blank_tiles: Query<
        (Entity, &Transform),
        (
            With<states::Empty>,
            Without<selection::Hover>,
            With<GameBoard>,
        ),
    >,
    hovered_blank_tiles: Query<
        (Entity, &Transform),
        (With<states::Empty>, With<selection::Hover>, With<GameBoard>),
    >,
    full_tiles: Query<
        (Entity, &Transform),
        (With<states::Full>, With<selection::None>, With<GameBoard>),
    >,
    invalid_full_tiles: Query<
        (Entity, &Transform),
        (
            With<states::Full>,
            With<selection::Invalid>,
            With<GameBoard>,
        ),
    >,
) {
    // Our active entity contains children of the actual tiles which we get here
    active
        .single()
        .map(|entity| {
            // get all blocks in the active pattern
            // compare and highlight tiles on the gameboard
            let coords: Vec<Vec2> = children
                .get(entity)
                .unwrap()
                .iter()
                .filter_map(|active_entity| match transforms.get(*active_entity) {
                    Ok(transform) => Some(transform.board_position()),
                    Err(_) => None,
                })
                .collect();

            // add hover to blank tiles that match the active piece transform
            blank_tiles.for_each(|(e, t)| {
                if coords.contains(&t.board_position()) {
                    transition::<selection::None, selection::Hover>(&mut cmd, e);
                }
            });

            // remove hover if coords no longer contains
            hovered_blank_tiles.for_each(|(e, t)| {
                if !coords.contains(&t.board_position()) {
                    transition::<selection::Hover, selection::None>(&mut cmd, e);
                }
            });

            // add invalid to full hovers
            full_tiles.for_each(|(e, t)| {
                if coords.contains(&t.board_position()) {
                    transition::<selection::None, selection::Invalid>(&mut cmd, e);
                }
            });

            // removes invalid from full no longer hovered
            invalid_full_tiles.for_each(|(e, t)| {
                if !coords.contains(&t.board_position()) {
                    transition::<selection::Invalid, selection::None>(&mut cmd, e);
                }
            });
        })
        .ok();
}

fn animate_active(
    active: Query<&Children, With<ActiveEntity>>,
    mut transforms: Query<&mut Transform>,
    placement_timer: Res<PlacementTimer>,
) {
    active
        .single()
        .map(|p| {
            p.iter().for_each(|e| {
                transforms
                    .get_mut(*e)
                    .map(|mut t| {
                        t.scale =
                            Vec3::new(0.95, 0.95, 0.0).lerp(Vec3::ONE, placement_timer.normalized())
                    })
                    .ok();
            })
        })
        .ok();
}

fn style_blocks(
    mut cmd: Commands,
    styles: Res<BlockResources>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    full: Query<(Entity, Option<&Color>), Added<states::Full>>,
    empty: Query<Entity, (Added<states::Empty>, With<selection::None>, With<GameBoard>)>,
    scored: Query<Entity, Added<states::Scored>>,
    invalid: Query<
        Entity,
        (
            With<states::Full>,
            Added<selection::Invalid>,
            With<GameBoard>,
        ),
    >,
    hovered: Query<
        Entity,
        (
            With<states::Empty>,
            Added<selection::Hover>,
            With<GameBoard>,
        ),
    >,
    unhovered: Query<Entity, (With<states::Empty>, Added<selection::None>, With<GameBoard>)>,
    uninvalidated: Query<
        (Entity, Option<&Color>),
        (With<states::Full>, Added<selection::None>, With<GameBoard>),
    >,
    mut transforms: Query<&mut Transform>,
) {
    full.iter()
        .chain(uninvalidated.iter())
        .for_each(|(entity, color)| {
            style(
                &mut cmd,
                entity,
                styles.full.texture.clone(),
                color.cloned(),
                &mut materials,
            );
            let mut t = transforms.get_mut(entity).unwrap();
            t.translation.z = 7.0;
        });

    empty.iter().chain(unhovered.iter()).for_each(|entity| {
        style(
            &mut cmd,
            entity,
            styles.empty.texture.clone(),
            None,
            &mut materials,
        );
        let mut t = transforms.get_mut(entity).unwrap();
        t.translation.z = 7.0;
    });
    scored.for_each(|entity| {
        style(
            &mut cmd,
            entity,
            styles.scored.texture.clone(),
            None,
            &mut materials,
        );
    });
    invalid.for_each(|entity| {
        style(
            &mut cmd,
            entity,
            styles.invalid.texture.clone(),
            None,
            &mut materials,
        );
        let mut t = transforms.get_mut(entity).unwrap();
        t.translation.z = 8.0;
    });
    hovered.for_each(|entity| {
        style(
            &mut cmd,
            entity,
            styles.hover.texture.clone(),
            None,
            &mut materials,
        );
        let mut t = transforms.get_mut(entity).unwrap();
        t.translation.z = 8.0;
    });
}

fn style(
    cmd: &mut Commands,
    entity: Entity,
    texture: Handle<Texture>,
    color: Option<Color>,
    materials: &mut Assets<ColorMaterial>,
) {
    // TODO: lol
    let new_material = materials.add(ColorMaterial {
        color: color.unwrap_or(Color::WHITE),
        texture: Some(texture),
    });
    cmd.entity(entity).insert(new_material.clone());
}

pub fn commit(
    mut cmd: Commands,
    mut timer: ResMut<PlacementTimer>,
    mouse: Res<Input<MouseButton>>,
    colors: Query<&Color>,
    children: Query<&Children>,
    patterns: Res<Assets<Pattern>>,
    cursor: Res<CursorPosition>,
    settings_handle: Res<Handle<SettingsAsset>>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
    tiles: QuerySet<(
        // Board pieces
        Query<Entity, With<GameBoard>>,
        // Hovered empty game board pieces
        Query<Entity, (With<selection::Hover>, With<states::Empty>, With<GameBoard>)>,
        // Invalid (full) game board pieces
        Query<Entity, With<selection::Invalid>>,
        // Active entity
        Query<Entity, With<ActiveEntity>>,
    )>,
    mut score: ResMut<Score>,
    mut state: ResMut<State<GameState>>,
    mut next_up: ResMut<NextUp>,
    mut bag: ResMut<Bag>,
) {
    let (board, hover, invalid, active) = (tiles.q0(), tiles.q1(), tiles.q2(), tiles.q3());
    let timer_done = timer.done();
    let mouse_pressed = mouse.just_pressed(MouseButton::Left);
    if mouse_pressed || timer_done {
        let mut lose = false;
        // first, ensure there are no invalid blocks
        if invalid.iter().count() == 0 {
            if let Some(active_entity) = active.iter().next() {
                // also ensure that the number of hovered blocks matches the number of active
                let active_blocks_count = children
                    .get(active_entity)
                    .map(|x| x.iter().count())
                    .unwrap_or(0);
                if hover.iter().count() == active_blocks_count {
                    // get block color to commit to the game board
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
                        transition::<states::Empty, states::Full>(&mut cmd, e);
                        transition::<selection::Hover, selection::None>(&mut cmd, e);
                        cmd.entity(e).insert(color);
                    });
                    set_active_pattern_helper(
                        &mut cmd,
                        &active,
                        patterns.get(next_up.clone()).unwrap(),
                        cursor,
                    );
                    // progress the next up
                    *next_up = bag.next().unwrap();
                    if mouse_pressed {
                        // reset the timer early if the mouse was pressed and we were successful
                        timer.reset();
                    }
                } else {
                    // the tile was off the gameboard
                    if timer_done {
                        // if the timer is done and there are invalid blocks, we fuckin lose LOL cringe...
                        lose = true;
                    }
                }
            }
        } else {
            if timer_done {
                // if the timer is done and there are invalid blocks, we fuckin lose LOL cringe...
                lose = true;
            }
        }

        if lose {
            // Set high score
            let settings = settings_assets.get_mut(settings_handle.clone()).unwrap();
            // If it changed...
            if settings.leaderboard.add("rustacean", *score) {
                // Save asset for leaderboard
                if let Ok(text) = ron::to_string(settings) {
                    let path = AssetPath::from("assets/settings.rfg");
                    let mut file = File::create(path.path()).unwrap();
                    file.write_all(text.as_bytes()).ok();
                }
            }
            // Clean up
            *score = 0;
            board.for_each(|e| {
                cmd.entity(e).despawn_recursive();
            });
            // For now, kick player back to the menu
            state.set(GameState::Menu).ok();
        }
    }
}

/// Set the active pattern to the newly provided pattern
fn set_active_pattern_helper(
    mut cmd: &mut Commands,
    active: &Query<Entity, With<ActiveEntity>>,
    pattern: &Pattern,
    cursor: Res<CursorPosition>,
) -> Entity {
    active.for_each(|e| cmd.entity(e).despawn_recursive());

    let entity = pattern_builder::<states::Full>(
        &mut cmd,
        pattern,
        Transform::from_xyz(cursor.global.x, cursor.global.y, 7f32),
    );
    cmd.entity(entity).insert(ActiveEntity);
    entity
}

// if there is 5 full blocks in a full square, remove and score
pub fn scorer(
    mut cmd: Commands,
    full_tiles: Query<(Entity, &Transform), With<states::Full>>,
    transforms: Query<&Transform>,
    mut score: ResMut<Score>,
) {
    let mut scoring_tiles = vec![];
    full_tiles.for_each(|(_, t)| {
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

    // ensure scoring tiles does not contain duplicates
    scoring_tiles.sort();
    scoring_tiles.dedup();

    for e in scoring_tiles {
        // remove all states from scoring tiles manually
        cmd.entity(e)
            .remove::<selection::Invalid>()
            .remove::<selection::Hover>()
            .remove::<states::Full>()
            .remove::<Color>()
            .insert(states::Empty)
            .insert(selection::None);
        // spawn a scoring block
        let mut transform = transforms.get(e).unwrap().clone();
        transform.translation.z = 2f32;
        cmd.spawn_bundle((
            Tile,
            GlobalTransform::from(transform.clone()),
            transform.clone(),
            states::Scored,
            Timer::from(Duration::from_millis(1000)),
        ));
        *score += 1;
    }
}

fn scored_effect(
    mut cmd: Commands,
    scored: Query<(Entity, &mut Transform, &Timer), With<states::Scored>>,
) {
    scored.for_each_mut(|(e, mut t, timer)| {
        // shrink and delete when scale is too small
        t.scale = t
            .scale
            .lerp(Vec3::new(0f32, 0f32, 2f32), timer.normalized());
        if timer.done() {
            cmd.entity(e).despawn_recursive();
        }
    });
}

fn rotate_active(
    mut active: Query<&mut Transform, With<ActiveEntity>>,
    keyboard: Res<Input<KeyCode>>,
) {
    let right_pressed = keyboard.just_pressed(KeyCode::D);
    let left_pressed = keyboard.just_pressed(KeyCode::A);
    if right_pressed || left_pressed {
        let multiplier = if right_pressed {
            -1f32
        } else if left_pressed {
            1f32
        } else {
            0f32
        };
        let rot = Quat::from_rotation_z(multiplier * 90f32.to_radians());

        active.single_mut().map(|mut t| t.rotate(rot)).ok();
    }
}

fn add_to_hold(
    mut cmd: Commands,
    mut hold: ResMut<Hold>,
    unswappable: Query<&Unswappable>,
    active: Query<Entity, With<ActiveEntity>>,
    active_pattern: Query<&Pattern, With<ActiveEntity>>,
    keyboard: Res<Input<KeyCode>>,
    cursor_pos: Res<CursorPosition>,
    patterns: Res<Assets<Pattern>>,
    next_up: Res<NextUp>,
    mut placement_timer: ResMut<PlacementTimer>,
) {
    // TODO: probably should check if unswappable is in the active entity instead of just existing
    if keyboard.just_pressed(KeyCode::LShift) && unswappable.iter().len() == 0 {
        let new_pattern = hold.swap(active_pattern.single().unwrap().clone());
        let active_entity = set_active_pattern_helper(
            &mut cmd,
            &active,
            &new_pattern.unwrap_or(patterns.get(next_up.clone()).unwrap().clone()),
            cursor_pos,
        );
        cmd.entity(active_entity).insert(Unswappable);
        // also reset the timer when swapping
        placement_timer.reset();
    }
}

/// Marker that means the block is part of the game board
pub struct GameBoard;

/// Includes every tile ever
pub struct Tile;

/// Only one of the components here should be used
pub mod states {
    use bevy::{ecs::component::Component, prelude::*};

    #[derive(Default)]
    pub struct Empty;
    #[derive(Default)]
    pub struct Full;
    /// Already scored tile that will be cleaned up
    #[derive(Default)]
    pub struct Scored;
}

pub mod selection {
    #[derive(Default)]
    pub struct None;
    #[derive(Default)]
    pub struct Hover;

    #[derive(Default)]
    pub struct Invalid;
}

/// Transition states in a fn as to avoid invalid states
#[inline(always)]
pub fn transition<F, T>(cmd: &mut Commands, entity: Entity)
where
    F: Component,
    T: Component + Default,
{
    cmd.entity(entity).remove::<F>().insert(T::default());
}

struct ScoreText;

trait TransformExt {
    /// Returns a two-dimentional rounded coordinates useful for comparing game pieces  
    fn board_position(&self) -> Vec2;
}

impl TransformExt for Transform {
    fn board_position(&self) -> Vec2 {
        let t = self.translation;
        Vec2::new(t.x.round(), t.y.round())
    }
}

impl TransformExt for GlobalTransform {
    fn board_position(&self) -> Vec2 {
        let t = self.translation;
        Vec2::new(t.x.round(), t.y.round())
    }
}

/// Marks an active entity as unswappable. This is useful to prevent constant swapping between the hold
#[derive(Default)]
struct Unswappable;
