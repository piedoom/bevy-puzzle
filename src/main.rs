use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::AudioPlugin;
// use bevy_egui::EguiPlugin;
// use bevy_kira_audio::AudioPlugin;
use pz::prelude::*;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            cursor_visible: true,
            cursor_locked: false,
            width: 1920f32,
            height: 1080f32,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.02, 0.05)))
        .add_plugins(DefaultPlugins)
        .add_plugins(FullPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(pz::ui::UiPlugin)
        .run();
}
