use bevy::{ecs::component::Component, prelude::*};

/// Transition states in a fn as to avoid invalid states
#[inline(always)]
pub(crate) fn transition<F, T>(cmd: &mut Commands, entity: Entity)
where
    F: Component,
    T: Component + Default,
{
    cmd.entity(entity).remove::<F>().insert(T::default());
}

/// Assign a new material to a block via a [`Handle<Texture>`]
pub(crate) fn style_with_texture(
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
