use std::path::PathBuf;

use crate::{plugins::EditEvent, prelude::*};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

use crate::GameState;

use super::widgets::SelectAssetWidget;

#[derive(Default)]
pub struct EditUiPlugin;

impl Plugin for EditUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>().add_system_set(
            SystemSet::on_update(GameState::Edit).with_system(edit_menu_system.system()),
        );
    }
}

#[derive(Default)]
struct UiState {
    map_name: String,
    mode: Option<Handle<GameMode>>,
}

fn edit_menu_system(
    mut events: EventWriter<EditEvent>,
    mut ui_state: ResMut<UiState>,
    modes: Res<Assets<GameMode>>,
    ctx: ResMut<EguiContext>,
) {
    egui::Area::new("edit_menu")
        .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            ui.text_edit_singleline(&mut ui_state.map_name);
            if ui.button("Save Map").clicked() {
                events.send(EditEvent::SaveCurrentMap {
                    name: ui_state.map_name.clone(),
                    path: PathBuf::from(ui_state.map_name.clone()),
                })
            }

            ui.add(SelectAssetWidget::<GameMode> {
                name: "Mode selection",
                selection: &mut ui_state.mode,
                assets: &modes,
            });

            if ui.button("Test").clicked() {
                events.send(EditEvent::RunCurrentMap {
                    mode: ui_state.mode.as_ref().unwrap().clone(),
                });
            }
        });
}
