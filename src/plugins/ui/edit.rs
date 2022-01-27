use std::path::PathBuf;

use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

use super::MenuState;

#[derive(Default)]
pub struct EditUiPlugin;

impl Plugin for EditUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_system_set(
                SystemSet::on_exit(GameState::load()).with_system(set_default_ui_data_system),
            )
            .add_system_set(SystemSet::on_update(GameState::edit()).with_system(edit_menu_system));
    }
}

#[derive(Default)]
struct UiState {
    map_name: String,
    // options: Option<GameOptions>,
}

fn edit_menu_system(
    mut events: EventWriter<EditEvent>,
    mut ui_state: ResMut<UiState>,
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

            todo!()
            // TODO: Game settings selections
            // ui.add(SelectAssetWidget::<GameOptions> {
            //     name: "Mode selection",
            //     selection: &mut ui_state.options,
            //     assets: &modes,
            // });

            // if ui.button("Test").clicked() {
            //     events.send(EditEvent::RunCurrentMap {
            //         mode: ui_state.options.as_ref().unwrap().clone(),
            //     });
            // }
        });
}

fn set_default_ui_data_system(mut menu_state: ResMut<MenuState>, maps: Res<Assets<Map>>) {
    menu_state.map = maps.iter().find_map(|(id, map)| {
        if map.name == Map::default_name() {
            Some(maps.get_handle(id))
        } else {
            None
        }
    });
}
