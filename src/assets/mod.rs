mod mode;
mod pattern;
mod settings;

use bevy::prelude::HandleUntyped;
pub use mode::*;
pub use pattern::*;
pub use settings::*;

#[derive(Default)]
pub struct PreloadingAssets(pub Vec<HandleUntyped>);
