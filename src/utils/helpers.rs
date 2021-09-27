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
