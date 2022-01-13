#![feature(derive_default_enum)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{asset::AssetServerSettings, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use pz::prelude::*;
// use bevy_egui::EguiPlugin;
// use bevy_kira_audio::AudioPlugin;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            cursor_visible: true,
            cursor_locked: false,
            width: 1920f32,
            height: 1080f32,
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            asset_folder: "/".to_string(),
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.02, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PuzzleUiPlugins)
        .add_plugins(PuzzleGamePlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(EguiPlugin)
        .run();
}
