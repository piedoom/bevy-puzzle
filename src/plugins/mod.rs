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
