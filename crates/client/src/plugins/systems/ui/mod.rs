//! This is where various elements of the UI including widgets exist

use bevy::prelude::*;
use bevy_egui::{egui::Visuals, EguiContext};
use shared::prelude::*;

use self::edit::EditUiPlugin;

mod edit;
/// Systems that need to run during the [`crate::GameState::Main`] state
mod game;
/// Systems that need to run during the [`crate::GameState::Menu`] state
mod menu;
/// Widgets needed to render custom UI, such as the "next up" piece
pub mod widgets;

pub(crate) use {game::*, menu::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Paused>()
            .init_resource::<MenuState>()
            .insert_resource(Bounds::<Vec2>::default())
            .add_system_set(
                SystemSet::on_update(GameState::menu())
                    .with_system(ui_menu_system)
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::main())
                    .with_system(ui_main_system)
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::pause())
                    .with_system(ui_pause_menu_system)
                    .label("pause"),
            )
            .add_plugin(EditUiPlugin);
    }
}
