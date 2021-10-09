//! Systems and other data structures related to obtaining user input and modifying the game in some way
use bevy::{input::mouse::MouseMotion, prelude::*, render::camera::*};
use bevy_kira_audio::Audio;

use crate::prelude::*;

use super::Label;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CursorPosition>()
            .insert_resource(ActivePositionMode::Mouse)
            .add_system(pause_system.system())
            .add_system(determine_input_method_system.system())
            .add_system(get_cursor_position_system.system())
            .add_system_set(
                SystemSet::on_update(GameState::main())
                    .with_system(pause_on_lose_focus_system.system())
                    .with_system(rotate_active_system.system())
                    .with_system(add_to_hold_system.system())
                    .with_system(click_commit_system.system())
                    .with_system(update_hovered_system.system())
                    .with_system(active_piece_position_system.system())
                    .label(Label::Listen),
            );
    }
}

/// Get the local and world coordinates of the mouse cursor
fn get_cursor_position_system(
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    cameras: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
    mut cursor_pos: ResMut<CursorPosition>,
) {
    // get the primary window
    if let Some(window) = windows.get_primary() {
        // check if the cursor is in the primary window
        if let Some(pos) = window.cursor_position() {
            // get the size of the window
            let size = Vec2::new(window.width() as f32, window.height() as f32);

            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;

            // assuming there is exactly one main camera entity, so this is OK
            if let Ok((camera_transform, proj)) = cameras.single() {
                // apply the camera transform
                let pos_world =
                    camera_transform.compute_matrix() * proj.scale * p.extend(0.0).extend(1.0);
                let pos_world = Vec2::from(pos_world);
                *cursor_pos.local = *pos_world;
                *cursor_pos.global = *(pos_world + Vec2::from(camera_transform.translation));
            }
        }
    }
}

fn rotate_active_system(
    mut active: Query<&mut Transform, With<ActiveEntity>>,
    state: Res<State<GameState>>,
    keyboard: Res<Input<KeyCode>>,
    modes: Res<Assets<GameMode>>,
    audio: Option<Res<Audio>>,
) {
    if let GameState::Main { mode, .. } = state.current() {
        let mode = modes.get(mode.clone()).unwrap();
        if mode.can_rotate {
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

                active
                    .single_mut()
                    .map(|mut transform| {
                        // rotate the overall piece
                        transform.rotate(rot);
                    })
                    .ok();

                if let GameState::Main { theme, .. } = &state.current() {
                    if let Some(audio) = &audio {
                        audio.play(theme.sfx.grip.clone());
                    }
                }
            }
        }
    }
}

fn add_to_hold_system(
    mut hold: ResMut<Hold>,
    unswappable: Query<&Unswappable>,
    active_pattern: Query<&Pattern, With<ActiveEntity>>,
    keyboard: Res<Input<KeyCode>>,
    patterns: Res<Assets<Pattern>>,
    next_up: Res<NextUp>,
    state: Res<State<GameState>>,
    audio: Option<Res<Audio>>,
    mut events: EventWriter<GameEvent>,
) {
    // TODO: probably should check if unswappable is in the active entity instead of just existing
    if keyboard.just_pressed(KeyCode::LShift) && unswappable.iter().len() == 0 {
        let pattern = hold.swap(active_pattern.single().unwrap().clone());
        let pattern = pattern.unwrap_or(patterns.get(next_up.clone()).unwrap().clone());
        if let GameState::Main { theme, .. } = &state.current() {
            if let Some(audio) = &audio {
                audio.play(theme.sfx.swap.clone());
            }
        }
        events.send(GameEvent::SetActivePattern {
            pattern,
            unswappable: true,
        });
    }
}

/// Commit a piece on click. Failure should not end in a loss.
fn click_commit_system(
    mut events: EventWriter<GameEvent>,
    input: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if input.just_pressed(MouseButton::Left) || keyboard.just_pressed(KeyCode::Space) {
        events.send(GameEvent::CommitActive {
            loss_on_failure: false,
        });
    }
}

fn update_hovered_system(
    mut cmd: Commands,
    active: Query<Entity, With<ActiveEntity>>,
    children: Query<&Children>,
    transforms: Query<&GlobalTransform>,
    blank_tiles: Query<
        (Entity, &Transform),
        (
            With<tile_states::Empty>,
            Without<tile_styles::Hover>,
            With<GameBoard>,
        ),
    >,
    hovered_blank_tiles: Query<
        (Entity, &Transform),
        (
            With<tile_states::Empty>,
            With<tile_styles::Hover>,
            With<GameBoard>,
        ),
    >,
    full_tiles: Query<
        (Entity, &Transform),
        (
            With<tile_states::Full>,
            With<tile_styles::None>,
            With<GameBoard>,
        ),
    >,
    invalid_full_tiles: Query<
        (Entity, &Transform),
        (
            With<tile_states::Full>,
            With<tile_styles::Invalid>,
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
                .map(|r| {
                    r.iter()
                        .filter_map(|active_entity| match transforms.get(*active_entity) {
                            Ok(transform) => Some(transform.board_position()),
                            Err(_) => None,
                        })
                        .collect()
                })
                .unwrap_or_default();

            // add hover to blank tiles that match the active piece transform
            blank_tiles.for_each(|(e, t)| {
                if coords.contains(&t.board_position()) {
                    transition::<tile_styles::None, tile_styles::Hover>(&mut cmd, e);
                }
            });

            // remove hover if coords no longer contains
            hovered_blank_tiles.for_each(|(e, t)| {
                if !coords.contains(&t.board_position()) {
                    transition::<tile_styles::Hover, tile_styles::None>(&mut cmd, e);
                }
            });

            // add invalid to full hovers
            full_tiles.for_each(|(e, t)| {
                if coords.contains(&t.board_position()) {
                    transition::<tile_styles::None, tile_styles::Invalid>(&mut cmd, e);
                }
            });

            // removes invalid from full no longer hovered
            invalid_full_tiles.for_each(|(e, t)| {
                if !coords.contains(&t.board_position()) {
                    transition::<tile_styles::Invalid, tile_styles::None>(&mut cmd, e);
                }
            });
        })
        .ok();
}

pub(crate) fn active_piece_position_system(
    position_mode: Res<ActivePositionMode>,
    active: Query<&mut Transform, With<ActiveEntity>>,
    cursor: Res<CursorPosition>,
    keyboard: Res<Input<KeyCode>>,
) {
    match *position_mode {
        ActivePositionMode::Keyboard => {
            let move_delta = keyboard.get_just_pressed().fold(Vec2::ZERO, |acc, key| {
                acc + match key {
                    KeyCode::Up => Vec2::Y,
                    KeyCode::Down => -Vec2::Y,
                    KeyCode::Right => Vec2::X,
                    KeyCode::Left => -Vec2::X,
                    _ => Vec2::ZERO,
                }
            });
            active.for_each_mut(|mut transform| {
                transform.translation += move_delta.extend(0f32);
            });
        }
        ActivePositionMode::Mouse => {
            active.for_each_mut(|mut transform| {
                transform.translation.x = cursor.global.x;
                transform.translation.y = cursor.global.y;
            });
        }
    }
}

fn pause_system(mut state: ResMut<State<GameState>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match state.current() {
            GameState::Main { .. } => state.push(GameState::Pause).ok(),
            GameState::Edit => state.push(GameState::Pause).ok(),
            GameState::Pause => state.pop().ok(),
            _ => None, // do nothing otherwise
        };
    }
}

fn determine_input_method_system(
    mut position_mode: ResMut<ActivePositionMode>,
    mut mouse_motion: EventReader<MouseMotion>,
    active: Query<&mut Transform, With<ActiveEntity>>,
    keyboard: Res<Input<KeyCode>>,
) {
    // If any of the move keys are pressed, set to keyboard positioning
    match *position_mode {
        ActivePositionMode::Keyboard =>
        // If the mouse has any motion, set mouse positioning mode
        {
            if mouse_motion
                .iter()
                .fold(Vec2::ZERO, |acc, mouse| acc + mouse.delta)
                != Vec2::ZERO
            {
                *position_mode = ActivePositionMode::Mouse;
            }
        }
        ActivePositionMode::Mouse => {
            if keyboard
                .get_pressed()
                .filter(|k| {
                    *k == &KeyCode::Up
                        || *k == &KeyCode::Down
                        || *k == &KeyCode::Left
                        || *k == &KeyCode::Right
                })
                .count()
                > 0
            {
                *position_mode = ActivePositionMode::Keyboard;
                // also set to center the tile
                active.for_each_mut(|mut t| {
                    t.translation = t.translation.round() + Vec3::new(0.5, 0.5, 0.0);
                });
            }
        }
    }
}

/// Automatically pause the game when focus is lost. Opt for adding this to
/// [`SystemSet`]s instead of enabling during all game states
fn pause_on_lose_focus_system(mut state: ResMut<State<GameState>>, windows: Res<Windows>) {
    windows.get_primary().map(|w| {
        if !w.is_focused() {
            state.push(GameState::Pause).ok();
        }
    });
}
