//! Systems related to editing modes and levels

use bevy::{prelude::*, render::camera::Camera};

use crate::prelude::*;

use super::input::active_piece_position_system;

pub struct EditPlugin;
impl Plugin for EditPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<EditEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::edit()).with_system(setup_system.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::edit())
                    .with_system(preview_system.system())
                    .with_system(active_piece_position_system.system())
                    .with_system(process_events_system.system())
                    .with_system(edit_input_system.system()),
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
    let camera_entity = cameras.single().map(|e| e).unwrap_or(cmd.spawn().id());
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
        Color::NONE,
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
            .single_mut()
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
                    Color::NONE,
                    PreviewTile,
                ));
            });
    }
}

fn process_events_system(
    mut cmd: Commands,
    mut events: EventReader<EditEvent>,
    preview: Query<(Entity, &GlobalTransform), With<PreviewTile>>,
    board: Query<&GlobalTransform, With<GameBoard>>,
) {
    for event in events.iter() {
        match event {
            EditEvent::Place => {
                preview
                    .single()
                    .map(|(e, t)| {
                        // ensure there is not already a tile here in the gameboard
                        if board
                            .iter()
                            .find(|t2| t2.translation.round() == t.translation.round())
                            .is_none()
                        {
                            cmd.entity(e).remove::<PreviewTile>().insert(GameBoard);
                        }
                    })
                    .ok();
            }
        }
    }
}

fn edit_input_system(mut events: EventWriter<EditEvent>, input: Res<Input<MouseButton>>) {
    if input.pressed(MouseButton::Left) {
        events.send(EditEvent::Place);
    }
}

pub enum EditEvent {
    Place,
}

/// Tile to be cleaned up at some point
#[derive(Default)]
struct PreviewTile;
