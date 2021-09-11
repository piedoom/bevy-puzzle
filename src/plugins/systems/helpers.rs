use bevy::{ecs::component::Component, prelude::*};

use crate::prelude::*;

/// Transition states in a fn as to avoid invalid states
#[inline(always)]
pub fn transition<F, T>(cmd: &mut Commands, entity: Entity)
where
    F: Component,
    T: Component + Default,
{
    cmd.entity(entity).remove::<F>().insert(T::default());
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

/// Assign a new material to a block via a [`Handle<Texture>`]
pub fn style_with_texture(
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

/// Set the active pattern to the newly provided pattern
pub fn set_active_pattern_helper(
    mut cmd: &mut Commands,
    active: &Query<Entity, With<ActiveEntity>>,
    pattern: &Pattern,
    cursor: Res<CursorPosition>,
) -> Entity {
    active.for_each(|e| cmd.entity(e).despawn_recursive());

    let entity = pattern_builder::<tile_states::Full>(
        &mut cmd,
        pattern,
        Transform::from_xyz(cursor.global.x, cursor.global.y, 7f32),
    );
    cmd.entity(entity).insert(ActiveEntity);
    entity
}
