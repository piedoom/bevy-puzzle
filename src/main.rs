use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use pz::prelude::*;
fn main() {
    App::build()
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
        .add_plugins(FullPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(pz::ui::UiPlugin)
        .run();
}
