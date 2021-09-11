//! This is where various elements of the UI including widgets exist

use bevy::prelude::*;

use crate::states::GameState;

/// Systems that need to run during the [`crate::GameState::Main`] state
mod game;
/// Systems that need to run during the [`crate::GameState::Menu`] state
mod menu;
/// Widgets needed to render custom UI, such as the "next up" piece
pub mod widgets;

pub(crate) use {game::*, menu::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<Paused>()
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(ui_menu_system.system())
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Main)
                    .with_system(ui_main_system.system())
                    .label("main"),
            );
    }
}
