use bevy::prelude::*;

mod game;
mod misc;
mod shaders;
pub mod ui;

pub use {game::*, misc::*, ui::*};

pub struct PuzzleUiPlugins;

impl PluginGroup for PuzzleUiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(ui::UiPlugin);
    }
}

pub struct PuzzleGamePlugins;

impl PluginGroup for PuzzleGamePlugins {
    fn build(&mut self, app: &mut bevy::app::PluginGroupBuilder) {
        app.add(game::AssetPlugin)
            .add(game::CorePuzzlePlugin)
            .add(game::EditPlugin)
            .add(game::InputPlugin)
            .add(game::StylePlugin)
            .add(shaders::ShadersPlugin);

        #[cfg(not(target_arch = "wasm32"))]
        app.add(misc::http::HttpPlugin);

        #[cfg(target_arch = "wasm32")]
        app.add(misc::resize::ViewportResizedPlugin);
    }
}

#[derive(SystemLabel, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Label {
    /// The first stage (gathering input)
    Listen,
    /// The second stage (moving, modifying entities)
    Process,
    /// The third stage (styling)
    React,
}
