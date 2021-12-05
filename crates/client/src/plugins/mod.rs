use bevy::prelude::*;
use systems::*;

mod systems;

pub struct PuzzleClientPlugins;

impl PluginGroup for PuzzleClientPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(AudioPlugin)
            .add(InputPlugin)
            .add(StylePlugin)
            .add(ui::UiPlugin);
    }
}
