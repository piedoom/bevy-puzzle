use std::time::Duration;

use crate::prelude::*;
use bevy::{prelude::*, render::camera::Camera};

use super::{helpers::set_active_pattern_helper, Label};

pub struct CorePuzzlePlugin;

impl Plugin for CorePuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Load)
            .init_resource::<Score>()
            .init_resource::<ActiveEntity>()
            .init_resource::<Bag>()
            .init_resource::<Hold>()
            .init_resource::<NextUp>()
            .add_system_set(SystemSet::on_enter(GameState::Main).with_system(setup_system.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(scorer_system.system())
                    .with_system(placement_timer_tick_system.system())
                    .label(Label::Process)
                    .after(Label::Listen),
            );
    }
}

// if there is 5 full blocks in a full square, remove and score
fn scorer_system(
    mut cmd: Commands,
    full_tiles: Query<(Entity, &Transform), With<tile_states::Full>>,
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
            .remove::<tile_styles::Invalid>()
            .remove::<tile_styles::Hover>()
            .remove::<tile_states::Full>()
            .remove::<Color>()
            .insert(tile_states::Empty)
            .insert(tile_styles::None);
        // spawn a scoring block
        let mut transform = transforms.get(e).unwrap().clone();
        transform.translation.z = 2f32;
        cmd.spawn_bundle((
            Tile,
            GlobalTransform::from(transform.clone()),
            transform.clone(),
            tile_states::Scored,
            Timer::new(Duration::from_millis(1000), false),
        ));
        *score += 1;
    }
}

fn setup_system(
    mut cmd: Commands,
    active: Query<Entity, With<ActiveEntity>>,
    cursor: Res<CursorPosition>,
    settings: Res<Assets<SettingsAsset>>,
    settings_handle: Res<Handle<SettingsAsset>>,
    patterns: Res<Assets<Pattern>>,
    board: Query<Entity, With<GameBoard>>,
    cameras: Query<Entity, With<Camera>>,
    mut bag: ResMut<Bag>,
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
                    tile_states::Empty,
                    Transform::from_xyz(x as f32, y as f32, 0f32),
                    GameBoard,
                    Tile,
                    tile_styles::None,
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

    // Add pieces to the bag if the bag has not yet been initialized
    if bag.queue.len() == 0 {
        *bag = Bag::new(
            patterns
                .iter()
                .map(|(x, _)| patterns.get_handle(x))
                .collect(),
        );
        *next_up = bag.next().unwrap();
        set_active_pattern_helper(
            &mut cmd,
            &active,
            patterns.get(next_up.clone()).unwrap(),
            cursor,
        );
        *next_up = bag.next().unwrap();
    }
}

fn placement_timer_tick_system(
    time: Res<Time>,
    mut active_timer: Query<&mut PlacementTimer, With<ActiveEntity>>,
) {
    active_timer
        .single_mut()
        .map(|mut t| {
            t.tick(time.delta());
        })
        .ok();
}
