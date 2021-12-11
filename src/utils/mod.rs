pub mod colors;
mod extensions;
mod helpers;
pub use {extensions::*, helpers::*};

use bevy::prelude::*;
pub struct Bounds<T>
where
    T: Into<Vec2> + From<Vec2> + Default,
{
    pub local: (T, T),
    pub world: (T, T),
}

impl<T> Default for Bounds<T>
where
    T: Into<Vec2> + From<Vec2> + Default,
{
    fn default() -> Self {
        Self {
            local: (T::default(), T::default()),
            world: (T::default(), T::default()),
        }
    }
}
