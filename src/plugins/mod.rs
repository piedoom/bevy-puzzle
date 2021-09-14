//! Plugin groups for building the game. Within this module also exists private system functions (except for UI systems).
//! This pattern reminds us to favor exposing plugins over systems, as it will A. ensure that the end result is more simple to consume,
//! and B. will ensure we are always thinking in terms of "How can this be idiomatically composed"? In this case, we're ensuring
//! that any sort of multiplayer server can load all the core elements of the game without requiring any graphical components
//! or systems that affect visuals.

mod systems;

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};
pub use systems::{
    assets::AssetPlugin, core::CorePuzzlePlugin, input::InputPlugin, style::StylePlugin,
};

pub struct FullPlugins;

impl PluginGroup for FullPlugins {
    fn build(&mut self, app: &mut PluginGroupBuilder) {
        app.add(AssetPlugin)
            .add(CorePuzzlePlugin)
            .add(InputPlugin)
            .add(StylePlugin);
    }
}
