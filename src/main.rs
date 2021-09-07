use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use pz::{PreloadingAssets, PuzzlePlugin};
fn main() {
    App::build()
        .init_resource::<PreloadingAssets>()
        .insert_resource(WindowDescriptor {
            cursor_visible: true,
            cursor_locked: true,
            width: 1920f32,
            height: 1080f32,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.02, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PuzzlePlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .run();
}
