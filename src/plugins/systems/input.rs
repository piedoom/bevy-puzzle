//! Systems and other data structures related to obtaining user input and modifying the game in some way

use std::{fs::File, io::Write};

use bevy::{asset::AssetPath, prelude::*, render::camera::*};

use crate::prelude::*;

use super::{
    helpers::{set_active_pattern_helper, transition},
    Label,
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CursorPosition>().add_system_set(
            SystemSet::on_update(GameState::Main)
                .with_system(get_cursor_position_system.system())
                .with_system(rotate_active_system.system())
                .with_system(add_to_hold_system.system())
                .with_system(commit_active_system.system())
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

fn rotate_active_system(
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

fn add_to_hold_system(
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

fn commit_active_system(
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
        Query<
            Entity,
            (
                With<tile_styles::Hover>,
                With<tile_states::Empty>,
                With<GameBoard>,
            ),
        >,
        // Invalid (full) game board pieces
        Query<Entity, With<tile_styles::Invalid>>,
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
                        transition::<tile_states::Empty, tile_states::Full>(&mut cmd, e);
                        transition::<tile_styles::Hover, tile_styles::None>(&mut cmd, e);
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

fn active_piece_position_system(
    active: Query<&mut Transform, With<ActiveEntity>>,
    cursor: Res<CursorPosition>,
) {
    active.for_each_mut(|mut transform| {
        transform.translation.x = cursor.global.x;
        transform.translation.y = cursor.global.y;
    });
}
