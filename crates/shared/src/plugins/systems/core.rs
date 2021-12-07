//! Systems needed to represent the bare-minimum of the game. Systems here
//! set up the game board, score pieces, and control the [`PlacementTimer`] among other things.

use std::{fs::File, io::Write, time::Duration};

use crate::{prelude::*, CampaignDetails, GameDetails, GameType, NextTransition};
use bevy::{app::Events, asset::AssetPath, prelude::*, render::camera::Camera, utils::Instant};

use super::Label;

pub struct CorePuzzlePlugin;

impl Plugin for CorePuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::default())
            .init_resource::<Score>()
            .init_resource::<ActiveEntity>()
            .init_resource::<Step>()
            .init_resource::<Bag>()
            .init_resource::<Hold>()
            .init_resource::<NextUp>()
            .add_event::<GameEvent>()
            .add_system(process_events_system)
            .add_system_set(
                SystemSet::on_enter(GameState::main())
                    .with_system(setup_system)
                    .label("setup"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::main())
                    .with_system(scorer_system)
                    .with_system(placement_timer_tick_system)
                    .with_system(level_win_system)
                    .label(Label::Process)
                    .after(Label::Listen),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::main())
                    .with_system(destroy_map_system)
                    .with_system(reset_game_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::win()).with_system(handle_transition_system),
            );
    }
}

// if there is 5 full blocks in a full square, remove and score
fn scorer_system(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    state: Res<State<GameState>>,
    full_tiles: Query<(Entity, &Transform), (With<tile_states::Full>, With<GameBoard>)>,
    board: Query<(Entity, &Transform, Option<&tile_states::Full>), With<GameBoard>>,
    transforms: Query<&Transform>,
    modes: Res<Assets<GameMode>>,
) {
    if let GameState::Main(game_type) = state.current() {
        let GameDetails { mode, .. } = game_type.get_details();
        // Important little vec that keeps track of all the scoring tiles that will be added at the end of the system loop
        let mut scoring_tiles = vec![];
        let mode = modes.get(mode).unwrap();

        // do the scoring
        match &mode.scorer {
            Scorer::Square(size) => {
                // Loop through every full tile to see if it is n tiles wide
                full_tiles.for_each(|(_, t)| {
                    let mut possible_tiles = vec![];
                    let mut scored = true;
                    for x in 0..*size {
                        for y in 0..*size {
                            // Get the current block (in all blocks)
                            let mut cmp_translation = t.translation.truncate();
                            cmp_translation.x += x as f32;
                            cmp_translation.y += y as f32;
                            if let Some((entity, _)) = full_tiles
                                .iter()
                                .find(|(_, t)| t.translation.truncate() == cmp_translation)
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
            }
            Scorer::Line(direction) => {
                // Takes in a current tile entity and loops over everything and returns a vec of entities that scored if it did, and nothing if it didnt
                let score_line_recursive = |start, dir| -> Vec<Entity> {
                    let mut scored = vec![start];
                    let in_progress = true;

                    #[allow(unused_assignments)]
                    let mut next_pos: Vec2 = Default::default();

                    // get starting transform
                    if let Ok((_, t, _)) = board.get(start) {
                        // set the start position
                        next_pos = t.board_position();
                        // loop while this bool is true lol
                        loop {
                            if !in_progress {
                                break;
                            }
                            next_pos += dir;
                            if let Some((e, _, is_full)) =
                                board.iter().find(|(_, next_transform, _)| {
                                    next_pos == next_transform.board_position()
                                })
                            {
                                // if it is empty, end early without scoring. otherwise, continue the loop and add the entity to scored... for NOW
                                match is_full.is_some() {
                                    true => {
                                        scored.push(e);
                                    }
                                    false => {
                                        // Failure! return nothing.
                                        return Vec::default();
                                    }
                                }
                            } else {
                                // If nothing, we have hit null which means success
                                return scored;
                            }
                        }
                    }
                    scored
                };

                // map all board positions into a vec for easier comparison (this is probs slow lol)
                let board_positions: Vec<Vec2> =
                    board.iter().map(|(_, t, _)| t.board_position()).collect();

                // check to see which tiles are on the edge (1 away from null)
                // do this by seeing if a full tile minus 1x 1y is *not* a `GameBoard` spot board.
                let border_tiles = |direction: Vec2| -> Vec<Entity> {
                    full_tiles
                        .iter()
                        .filter_map(|(e, t)| {
                            // if the position is not contained in the board, this is an edge
                            if !board_positions.contains(&(t.board_position() - direction)) {
                                Some(e)
                            } else {
                                None
                            }
                        })
                        .collect()
                };

                // get scoring lines from border tiles
                match direction {
                    ScoreDirection::Vertical => border_tiles(Vec2::Y).iter().for_each(|entity| {
                        scoring_tiles.append(&mut score_line_recursive(*entity, Vec2::Y));
                    }),
                    ScoreDirection::Horizontal => border_tiles(Vec2::X).iter().for_each(|entity| {
                        scoring_tiles.append(&mut score_line_recursive(*entity, Vec2::X));
                    }),
                    // This only gets two sides,
                    // but that's fine - because these sides are necessary for all possible combinations and will not be exlucded by checking all 4 sides.
                    ScoreDirection::Both => {
                        border_tiles(Vec2::Y).iter().for_each(|entity| {
                            scoring_tiles.append(&mut score_line_recursive(*entity, Vec2::Y));
                        });
                        border_tiles(Vec2::X).iter().for_each(|entity| {
                            scoring_tiles.append(&mut score_line_recursive(*entity, Vec2::X));
                        })
                    }
                }
            }
        }

        // ensure scoring tiles does not contain duplicates
        scoring_tiles.sort();
        scoring_tiles.dedup();

        for e in scoring_tiles {
            // remove all states from scoring tiles manually
            cmd.entity(e)
                .remove::<tile_styles::Invalid>()
                .remove::<tile_styles::Hover>()
                .remove::<tile_states::Full>()
                .remove::<TileColor>()
                .insert(tile_states::Empty)
                .insert(tile_styles::None);
            // spawn a scoring block
            let mut transform = *transforms
                .get(e)
                .expect("Could not get transform with this entity");
            transform.translation.z = 2f32;
            cmd.spawn_bundle((
                Tile,
                GlobalTransform::from(transform),
                transform,
                tile_states::Scored,
                Timer::new(Duration::from_millis(1000), false),
            ));
            *score += 1;
        }
    }
}

/// Determines whether the specific level being played is win
fn level_win_system(
    mut state: ResMut<State<GameState>>,
    started: Option<Res<GameStarted>>,
    score: Res<Score>,
) {
    if let Some(started) = started {
        if let GameState::Main(game_type) = state.current() {
            let GameDetails { objective, .. } = game_type.get_details();

            let won = match objective {
                Objective::FreePlay => false, // Infinite free play
                Objective::Survive(duration) => {
                    // check to see if the player has surpassed the necessary duration
                    Instant::now().duration_since(*started) >= duration
                }
                Objective::TimeLimit {
                    required_score,
                    duration,
                } => {
                    Instant::now().duration_since(*started) >= duration && *score >= required_score
                }
            };

            if won {
                // Destructure the state (which we know is main, as this system only runs during the main game loop)
                let transition = match game_type.get_campaign() {
                    Some(c) => {
                        // check to see if the last level has been completed
                        match c.next_level() {
                            Some((_, next_level_index)) => {
                                NextTransition::NewLevel(GameType::Campaign(CampaignDetails {
                                    campaign: c.campaign,
                                    current_level_index: next_level_index,
                                    campaign_scores: vec![], // TODO!
                                }))
                            }
                            // TODO: This is basically a whole objective win scenario. Should probably do something better than just transition to menu later
                            None => NextTransition::Menu,
                        }
                    }
                    // no campaign, go back to the menus
                    None => NextTransition::Menu,
                };
                state.replace(GameState::Win(transition)).ok();
            }
        }
    }
}

fn handle_transition_system(mut state: ResMut<State<GameState>>) {
    match state.current() {
        GameState::Win(transition) => {
            // Handle win screen
            let next_state = match transition {
                NextTransition::Menu => GameState::Menu,
                NextTransition::NewLevel(next) => {
                    let next_campaign = next
                        .get_campaign()
                        .expect("Should only use `NewLevel` when using campaigns");
                    GameState::Main(GameType::Campaign(next_campaign))
                }
            };
            state.replace(next_state).ok();
        }
        _ => (),
    }
}

/// Sets up everything needed to run the main game loop. It also checks to ensure nothing will be overwritten,
/// so states can be pushed and popped as needed.
fn setup_system(
    mut cmd: Commands,
    mut events: EventWriter<GameEvent>,
    mut bag: ResMut<Bag>,
    mut hold: ResMut<Hold>,
    mut next: ResMut<NextUp>,
    mut bounds: ResMut<Bounds<Vec2>>,
    modes: Res<Assets<GameMode>>,
    maps: Res<Assets<Map>>,
    state: Res<State<GameState>>,
    settings: Res<Assets<SettingsAsset>>,
    current_setting: Res<Handle<SettingsAsset>>,
    patterns: Res<Assets<Pattern>>,
    cameras: Query<Entity, With<Camera>>,
) {
    let settings = settings.get(current_setting.clone()).unwrap();
    cmd.insert_resource(GameStarted::now());
    if let GameState::Main(game_type) = state.current() {
        let GameDetails { map, mode, .. } = game_type.get_details();
        let mode = modes.get(mode).unwrap();
        let map = maps.get(map).unwrap();
        if mode.patterns.is_empty() {
            panic!("Current GameMode provides no patterns")
        }
        // calculate screen position from already calculated world bounds
        let mut rect = map.calculate_rect();
        // adjust to get corners of tiles instead of center
        rect.expand(0.5);
        // this rect is now our world coordinates! Woohoo, easy.
        bounds.world = rect; // assign world coords for now
                             // lets get local screen coordinates from this world coordinates later on when we are positive a camera exists

        // Use unpadded bounds here just so we can successfully center the camera
        let center = rect.center();
        // Create camera if none exists. Reset the transform since the map may have changed
        let camera_entity = cameras.get_single().unwrap_or_else(|_| cmd.spawn().id());

        // Set the position and scale of the camera on every start
        // Calculate the overall size of the board, and divide to find the center point
        let trans = Transform::from_xyz(center.x, center.y, 10.0);
        let mut camera_bundle = OrthographicCameraBundle::new_2d();
        camera_bundle.orthographic_projection.scale = settings.camera_scale;
        camera_bundle.transform = trans;
        // camera_bundle.global_transform = GlobalTransform::from(trans.clone());
        cmd.entity(camera_entity).insert_bundle(camera_bundle);

        // create game map
        for (x, y) in &map.pattern {
            let transform = Transform::from_xyz(*x as f32, *y as f32, 0f32);
            cmd.spawn_bundle((
                tile_states::Empty,
                transform,
                GameBoard,
                Tile,
                tile_styles::None,
            ));
        }

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
}

fn placement_timer_tick_system(
    mut active_timer: Query<&mut PlacementTimer, With<ActiveEntity>>,
    mut events: EventWriter<GameEvent>,
    time: Res<Time>,
) {
    active_timer
        .get_single_mut()
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

fn process_events_system(
    mut cmd: Commands,
    mut events: ResMut<Events<GameEvent>>,
    mut bag: ResMut<Bag>,
    mut next: ResMut<NextUp>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
    mut state: ResMut<State<GameState>>,
    mut step: ResMut<Step>,
    mut active: Query<(Entity, &mut Transform, &Pattern), With<ActiveEntity>>,
    modes: Res<Assets<GameMode>>,
    position_mode: Res<ActivePositionMode>,
    score: ResMut<Score>,
    settings_handle: Res<Handle<SettingsAsset>>,
    hover: Query<Entity, (With<tile_styles::Hover>, With<GameBoard>)>,
    pattern_assets: Res<Assets<Pattern>>,
    cursor: Res<CursorPosition>,
) {
    let mut send_events = vec![];

    for event in events.drain() {
        match event {
            GameEvent::SetActivePattern {
                pattern,
                unswappable,
            } => {
                // create the transform for our new active (if in keyboard mode)
                let active_transform = active
                    .get_single_mut()
                    .map(|(_, t, _)| *t)
                    .unwrap_or_else(|_| Transform::from_xyz(0f32, 0f32, 7f32));

                // determine the next position
                let transform = match *position_mode {
                    ActivePositionMode::Keyboard => active_transform,
                    ActivePositionMode::Mouse => {
                        Transform::from_xyz(cursor.global.x, cursor.global.y, 7f32)
                    }
                };

                // remove all old actives to prepare to add a new one
                active.for_each_mut(|(e, ..)| cmd.entity(e).despawn_recursive());

                if let GameState::Main(game_type) = state.current() {
                    let GameDetails { mode, .. } = game_type.get_details();
                    let mode = modes.get(mode.clone()).unwrap();
                    let timer = step.create_timer(mode);

                    // Create the new active entity
                    let entity = cmd
                        .spawn_bundle((
                            transform,
                            GlobalTransform::from(transform),
                            pattern.clone(),
                            timer,
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
            }
            GameEvent::CommitActive { loss_on_failure } => {
                // First, check to see if the amount of blocks in our `ActiveEntity` match the amount of hovers. If they do not, this is a failure!
                let (actives, color) = active
                    .get_single_mut()
                    .map(|(.., pattern)| (pattern.blocks.len(), pattern.color.clone()))
                    .unwrap_or((0, Default::default()));

                if hover.iter().count() == actives {
                    // everything is good, commit!
                    hover.for_each(|e| {
                        transition::<tile_states::Empty, tile_states::Full>(&mut cmd, e);
                        transition::<tile_styles::Hover, tile_styles::None>(&mut cmd, e);
                        cmd.entity(e).insert(color.clone());
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

                state.replace(GameState::menu()).ok();
            }
        }
    }

    for event in send_events {
        events.send(event);
    }
}

/// Re-initialize all other needed game state to default
pub(crate) fn reset_game_system(
    mut cmd: Commands,
    mut score: ResMut<Score>,
    mut next: ResMut<NextUp>,
    mut bag: ResMut<Bag>,
    mut step: ResMut<Step>,
    active: Query<Entity, With<ActiveEntity>>,
    cameras: Query<Entity, With<Camera>>,
) {
    // Clean up
    cameras.for_each(|e| cmd.entity(e).despawn_recursive());
    *score = 0;
    *next = Handle::<Pattern>::default();
    *bag = Bag::default();
    step.reset();
    active.for_each(|entity| cmd.entity(entity).despawn_recursive());
}

/// Destroy the game board
pub(crate) fn destroy_map_system(mut cmd: Commands, board: Query<Entity, With<GameBoard>>) {
    board.for_each(|e| {
        cmd.entity(e).despawn_recursive();
    });
}
