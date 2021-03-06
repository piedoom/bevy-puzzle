//! This is where various elements of the UI including widgets exist
use bevy::prelude::*;
mod edit;
mod game;
mod menu;
pub mod widgets;

use crate::prelude::*;
use {game::*, menu::*, widgets::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MenuState>()
            .init_resource::<PostGameMenuResource>()
            .insert_resource(Bounds::<Vec2>::default())
            .add_system_set(
                SystemSet::on_update(GameState::menu())
                    .with_system(ui_menu_system)
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::game())
                    .with_system(ui_main_system)
                    .label("main"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::pause())
                    .with_system(ui_pause_menu_system)
                    .label("pause"),
            )
            .add_system_set(
                SystemSet::on_update(GameState::post_game()).with_system(ui_post_game_system),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::post_game()).with_system(ui_post_game_save_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::pre_game()).with_system(ui_pre_game_menu_system),
            )
            .add_plugin(edit::EditUiPlugin);
    }
}
