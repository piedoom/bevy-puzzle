mod extensions;
pub mod helpers;

use bevy::prelude::*;
pub use extensions::*;
pub use helpers::*;

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

// pub(crate) struct Bounds {
//     pub(crate) world: egui::Rect,
// }

// impl Default for Bounds {
//     fn default() -> Self {
//         Self {
//             world: egui::Rect::NOTHING,
//         }
//     }
// }
