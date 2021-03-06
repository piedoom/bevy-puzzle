#![feature(derive_default_enum)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
use pz::prelude::*;

fn main() {
    dotenv::dotenv().ok();
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.02, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PuzzleUiPlugins)
        .add_plugins(PuzzleGamePlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(EguiPlugin);

    app.run();
}
