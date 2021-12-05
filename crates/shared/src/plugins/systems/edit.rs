//! Systems related to editing modes and levels

use std::{fs::File, io::Write};

use bevy::{prelude::*, render::camera::Camera};

use crate::prelude::*;

use super::core::{destroy_map_system, reset_game_system};

pub struct EditPlugin;
impl Plugin for EditPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EditEvent>()
            .add_system_set(SystemSet::on_enter(GameState::edit()).with_system(setup_system))
            .add_system_set(
                SystemSet::on_update(GameState::edit())
                    .with_system(preview_system)
                    .with_system(process_events_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::edit())
                    .with_system(destroy_map_system)
                    .with_system(reset_game_system),
            );
    }
}

// create everything needed on system initialization
fn setup_system(
    mut cmd: Commands,
    cameras: Query<Entity, With<Camera>>,
    settings_assets: Res<Assets<SettingsAsset>>,
    settings_handle: Res<Handle<SettingsAsset>>,
) {
    let settings = settings_assets.get(settings_handle.clone()).unwrap();
    // Create camera if none exists. Reset the transform since the map may have changed
    let camera_entity = cameras.get_single().unwrap_or_else(|_| cmd.spawn().id());
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = settings.camera_scale;
    // camera_bundle.global_transform = GlobalTransform::from(trans.clone());
    cmd.entity(camera_entity).insert_bundle(camera_bundle);

    // active tile
    cmd.spawn_bundle((
        ActiveEntity,
        Tile,
        tile_states::Empty,
        tile_styles::None,
        Transform::default(),
        GlobalTransform::default(),
        TileColor(Color::NONE),
    ));
}

// Spawn a temporary tile behind the cursor to indicate what would happen if the user were to click
fn preview_system(
    mut cmd: Commands,
    cursor: Res<CursorPosition>,
    mut preview: Query<&mut GlobalTransform, With<PreviewTile>>,
) {
    // check if preview tile (if it exists) is still under the cursor
    if cursor.is_changed() {
        preview
            .get_single_mut()
            .map(|mut t| {
                let cur = cursor.global.round();
                match cur == t.translation.round().truncate() {
                    true => (), // all good, do nothing
                    false => {
                        // update position
                        t.translation = cur.extend(0f32);
                    }
                }
            })
            .unwrap_or_else(|_| {
                let pos = cursor.global.round().extend(0f32);
                cmd.spawn_bundle((
                    Tile,
                    tile_states::Empty,
                    tile_styles::None,
                    Transform::from_translation(pos),
                    GlobalTransform::from_translation(pos),
                    TileColor(Color::NONE),
                    PreviewTile,
                ));
            });
    }
}

fn process_events_system(
    mut cmd: Commands,
    mut events: EventReader<EditEvent>,
    mut state: ResMut<State<GameState>>,
    mut maps: ResMut<Assets<Map>>,
    preview: Query<(Entity, &GlobalTransform), With<PreviewTile>>,
    board: Query<(Entity, &GlobalTransform), With<GameBoard>>,
) {
    for event in events.iter() {
        match event {
            EditEvent::PlaceActive => {
                preview
                    .get_single()
                    .map(|(e, t)| {
                        // ensure there is not already a tile here in the gameboard
                        if !board
                            .iter()
                            .any(|(_, t2)| t2.translation.round() == t.translation.round())
                        {
                            cmd.entity(e).remove::<PreviewTile>().insert(GameBoard);
                        }
                    })
                    .ok();
            }
            EditEvent::Clear(pos) => {
                board
                    .iter()
                    .filter(|(_, t)| t.translation.truncate() == pos.round())
                    .for_each(|(e, _)| cmd.entity(e).despawn_recursive());
            }
            EditEvent::SaveCurrentMap { path, name } => {
                // assemble map into a vec
                let pattern: Vec<(isize, isize)> = current_tiles_to_vec(&board);
                // save
                let data = Map {
                    name: name.clone(),
                    pattern,
                };
                let serialized = ron::to_string(&data).unwrap();
                let mut file = File::create(format!("assets/maps/{}.map", path.display())).unwrap();
                writeln!(file, "{}", serialized).unwrap();
            }
            EditEvent::RunCurrentMap { mode } => {
                let map = Map {
                    pattern: current_tiles_to_vec(&board),
                    ..Default::default()
                };
                state
                    .replace(GameState::Main {
                        mode: mode.clone(),
                        map: maps.add(map),
                    })
                    .ok();
            }
        }
    }
}

fn current_tiles_to_vec(
    board: &Query<(Entity, &GlobalTransform), With<GameBoard>>,
) -> Vec<(isize, isize)> {
    board
        .iter()
        .map(|(_, t)| {
            (
                t.translation.x.round() as isize,
                t.translation.y.round() as isize,
            )
        })
        .collect()
}
/// Tile to be cleaned up at some point
#[derive(Default, Component)]
struct PreviewTile;
