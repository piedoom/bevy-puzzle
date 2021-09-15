//! Systems needed to represent the bare-minimum of the game. Systems here
//! set up the game board, score pieces, and control the [`PlacementTimer`] among other things.

use std::{fs::File, io::Write, time::Duration};

use crate::prelude::*;
use bevy::{app::Events, asset::AssetPath, prelude::*, render::camera::Camera};

use super::Label;

pub struct CorePuzzlePlugin;

impl Plugin for CorePuzzlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Load)
            .init_resource::<Score>()
            .init_resource::<CurrentGameMode>()
            .init_resource::<ActiveEntity>()
            .init_resource::<Step>()
            .init_resource::<Bag>()
            .init_resource::<Hold>()
            .init_resource::<NextUp>()
            .add_event::<GameEvent>()
            .add_system(process_events_system.system())
            .add_system_set(
                SystemSet::on_exit(GameState::Load).with_system(set_default_mode_system.system()),
            )
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
    mut score: ResMut<Score>,
    full_tiles: Query<(Entity, &Transform), With<tile_states::Full>>,
    transforms: Query<&Transform>,
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

/// Sets up everything needed to run the main game loop. It also checks to ensure nothing will be overwritten,
/// so states can be pushed and popped as needed.
fn setup_system(
    mut cmd: Commands,
    mut events: EventWriter<GameEvent>,
    mut bag: ResMut<Bag>,
    mut hold: ResMut<Hold>,
    mut score: ResMut<Score>,
    mut active: Query<Entity, With<ActiveEntity>>,
    mut next: ResMut<NextUp>,
    settings: Res<Assets<SettingsAsset>>,
    settings_handle: Res<Handle<SettingsAsset>>,
    patterns: Res<Assets<Pattern>>,
    current_mode: ResMut<CurrentGameMode>,
    modes: ResMut<Assets<GameMode>>,
    queries: QuerySet<(Query<Entity, With<GameBoard>>, Query<Entity, With<Camera>>)>,
) {
    let (board, cameras) = (queries.q0(), queries.q1());
    let settings = settings.get(settings_handle.clone()).unwrap();
    let mode = modes.get(current_mode.clone()).unwrap();
    let (size_x, size_y) = (mode.board_size.0, mode.board_size.1);

    // reset everything for good measure
    reset_game(
        &mut cmd,
        &mut score,
        &mut active,
        &mut next,
        &mut bag,
        &board,
    );

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

    // Create camera if none exists
    if cameras.iter().count() == 0 {
        let camera_entity = cmd.spawn().id();
        // Calculate the overall size of the board, and divide to find the center point
        let trans = Transform::from_xyz(size_x as f32 / 2f32, size_y as f32 / 2f32, 10.0);
        let mut camera_bundle = OrthographicCameraBundle::new_2d();
        camera_bundle.orthographic_projection.scale = settings.camera_scale;
        cmd.entity(camera_entity)
            .insert_bundle(camera_bundle)
            .insert(trans);
    };

    *bag = Bag::new(
        patterns
            .iter()
            .filter(|(_, pattern)| mode.patterns.contains(&pattern.name))
            .map(|(x, _)| patterns.get_handle(x))
            .collect(),
    );
    *next = bag.next().unwrap();
    events.send(GameEvent::SetActivePattern {
        pattern: patterns.get(next.clone()).unwrap().clone(),
        unswappable: false,
    });
    *next = bag.next().unwrap();

    // remove any piece from the hold
    hold.clear();
}

fn placement_timer_tick_system(
    mut active_timer: Query<&mut PlacementTimer, With<ActiveEntity>>,
    mut events: EventWriter<GameEvent>,
    time: Res<Time>,
) {
    active_timer
        .single_mut()
        .map(|mut t| {
            t.tick(time.delta());
            if t.just_finished() {
                // Commit the piece
                events.send(GameEvent::CommitActive {
                    loss_on_failure: true,
                });
            }
        })
        .ok();
}

/// Trigger any other stuff that needs to be done after the loading stage
fn set_default_mode_system(
    mut modes: ResMut<Assets<GameMode>>,
    mut events: EventWriter<GameEvent>,
    patterns: Res<Assets<Pattern>>,
) {
    // The current mode is unset. Find the asset titled "default" or load in the default asset
    let user_default = modes
        .iter()
        .find(|(_, mode)| mode.name == GameMode::default_name())
        .map(|(id, _)| modes.get_handle(id));

    let mode =
        user_default.unwrap_or_else(|| modes.add(GameMode::default_with_patterns(&*patterns)));

    events.send(GameEvent::SetGameMode(mode));
}

fn process_events_system(
    mut cmd: Commands,
    mut events: ResMut<Events<GameEvent>>,
    mut current_mode: ResMut<CurrentGameMode>,
    mut bag: ResMut<Bag>,
    mut next: ResMut<NextUp>,
    mut score: ResMut<Score>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
    mut state: ResMut<State<GameState>>,
    mut step: ResMut<Step>,
    mut active: Query<Entity, With<ActiveEntity>>,
    settings_handle: Res<Handle<SettingsAsset>>,
    tiles: QuerySet<(
        Query<Entity, (With<tile_styles::Hover>, With<GameBoard>)>,
        Query<Entity, With<GameBoard>>,
        Query<(Entity, &Pattern), With<ActiveEntity>>,
    )>,
    modes: Res<Assets<GameMode>>,
    pattern_assets: Res<Assets<Pattern>>,
    cursor: Res<CursorPosition>,
) {
    let (hover, board, active_pattern) = (tiles.q0(), tiles.q1(), tiles.q2());
    let mut send_events = vec![];
    for event in events.drain() {
        match event {
            GameEvent::SetActivePattern {
                pattern,
                unswappable,
            } => {
                // Set the active pattern to the newly provided pattern
                active_pattern.for_each(|(e, _)| cmd.entity(e).despawn_recursive());

                let transform = Transform::from_xyz(cursor.global.x, cursor.global.y, 7f32);

                // Create the new active entity
                let entity = cmd
                    .spawn_bundle((
                        transform.clone(),
                        GlobalTransform::from(transform.clone()),
                        pattern.clone(),
                        PlacementTimer::new(Duration::from_millis(3000), false),
                        ActiveEntity,
                    ))
                    .with_children(|p| {
                        for block in pattern.blocks.iter() {
                            // TODO: adjust the 0.5 constant offset to allow for more natural (and dynamic) rotations
                            // based off of block size. We likely will need to determine this when loading the asset
                            let local_transform =
                                Transform::from_xyz(block.x - 0.5, block.y + 0.5, 1f32);
                            p.spawn_bundle((
                                tile_states::Full,
                                local_transform,
                                GlobalTransform::from(local_transform),
                                pattern.color.clone(),
                                Tile,
                            ));
                        }
                    })
                    .id();
                if unswappable {
                    cmd.entity(entity).insert(Unswappable);
                }
            }
            GameEvent::SetGameMode(mode_handle) => {
                // Set the active game mode
                *current_mode = mode_handle.clone();
                let mode = modes.get(mode_handle).unwrap();

                // Reset the bag and its pieces
                let patterns: Vec<Handle<Pattern>> = pattern_assets
                    .iter()
                    .filter_map(|(id, pattern)| {
                        // Only include pieces specified in the mode
                        if mode.patterns.contains(&pattern.name) {
                            Some(pattern_assets.get_handle(id))
                        } else {
                            None
                        }
                    })
                    .collect();

                *bag = Bag::new(patterns);
                *next = bag.next().unwrap();
            }
            GameEvent::CommitActive { loss_on_failure } => {
                // First, check to see if the amount of blocks in our `ActiveEntity` match the amount of hovers. If they do not, this is a failure!
                let (actives, color) = active_pattern
                    .single()
                    .map(|(_, pattern)| (pattern.blocks.len(), pattern.color))
                    .unwrap_or((0, Color::WHITE));

                if hover.iter().count() == actives {
                    // everything is good, commit!
                    hover.for_each(|e| {
                        transition::<tile_states::Empty, tile_states::Full>(&mut cmd, e);
                        transition::<tile_styles::Hover, tile_styles::None>(&mut cmd, e);
                        cmd.entity(e).insert(color);
                    });

                    // This check is needed in case the event is processed after a change that resets our next piece
                    if let Some(pattern) = pattern_assets.get(next.clone()) {
                        // Set active to our next up piece
                        send_events.push(GameEvent::SetActivePattern {
                            pattern: pattern.clone(),
                            unswappable: false,
                        });

                        // Advance the step counter
                        step.next();

                        // Advance the pieces
                        *next = bag.next().unwrap();
                    }
                }
                // If the event is set to lose on failure to place, send a loss event
                else if loss_on_failure {
                    send_events.push(GameEvent::Loss);
                }
            }
            GameEvent::Loss => {
                // Set high score
                let settings = settings_assets.get_mut(settings_handle.clone()).unwrap();
                // If it changed...
                let name = {
                    if settings.active_name.is_empty() {
                        "rustacean"
                    } else {
                        &settings.active_name
                    }
                };
                if settings.leaderboard.add(name, *score) {
                    // Save asset for leaderboard
                    if let Ok(text) = ron::to_string(settings) {
                        let path = AssetPath::from("assets/settings.rfg");
                        let mut file = File::create(path.path()).unwrap();
                        file.write_all(text.as_bytes()).ok();
                    }
                }
                reset_game(
                    &mut cmd,
                    &mut score,
                    &mut active,
                    &mut next,
                    &mut bag,
                    &board,
                );
                state.replace(GameState::Menu).ok();
            }
        }
    }
    for event in send_events {
        events.send(event);
    }
}
