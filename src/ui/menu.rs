use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui::*, *};

/// Resource that tells us if the game is paused or not
pub type Paused = bool;

#[derive(Default)]
pub struct MenuState {
    pub mode: Option<Handle<GameMode>>,
    pub map: Option<Handle<Map>>,
}

pub(crate) fn ui_menu_system(
    mut state: ResMut<State<GameState>>,
    mut menu_state: ResMut<MenuState>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
    ctx: ResMut<EguiContext>,
    settings_handle: Res<Handle<SettingsAsset>>,
    modes: Res<Assets<GameMode>>,
    maps: Res<Assets<Map>>,
) {
    CentralPanel::default().show(ctx.ctx(), |ui| {
        let settings = settings_assets.get_mut(settings_handle.clone()).unwrap();
        ui.centered_and_justified(|ui| {
            // Show high scores
            ui.vertical(|ui| {
                // Loop over highest scores and display a text line for each
                for score in &settings.leaderboard.leaders {
                    let (name, score) = score;
                    ui.label(format!("{}: {}", name, score));
                }

                // Allow user to input name to be used in highscore table
                ui.text_edit_singleline(&mut settings.active_name);
            });

            // get current mode and map
            let mode = modes.get(menu_state.mode.clone().unwrap_or_default());
            let map = maps.get(menu_state.map.clone().unwrap_or_default());
            ui.vertical(|ui| {
                egui::ComboBox::from_label("Game mode")
                    .selected_text(
                        &mode
                            .map(|x| &x.name)
                            .unwrap_or(&String::from("None selected")),
                    )
                    .show_ui(ui, |ui| {
                        for (id, mode) in modes.iter() {
                            let select_handle = modes.get_handle(id);
                            if ui
                                .selectable_value(
                                    &mut menu_state.mode,
                                    Some(select_handle.clone()),
                                    &mode.name,
                                )
                                .clicked()
                            {
                                menu_state.mode = Some(select_handle.clone());
                            }
                        }
                    });

                egui::ComboBox::from_label("Map")
                    .selected_text(&map.map(|x| &x.name).unwrap_or(&"None selected".to_string()))
                    .show_ui(ui, |ui| {
                        for (id, map) in maps.iter() {
                            let select_handle = maps.get_handle(id);
                            if ui
                                .selectable_value(
                                    &mut menu_state.map,
                                    Some(select_handle.clone()),
                                    &map.name,
                                )
                                .clicked()
                            {
                                menu_state.map = Some(select_handle.clone());
                            }
                        }
                    });
            });

            // Start game button
            if ui.button("Start").clicked() {
                state
                    .set(GameState::Main {
                        mode: mode.unwrap().clone(),
                        map: map.unwrap().clone(),
                    })
                    .ok();
            }
        });
    });
}

pub(crate) fn ui_pause_menu_system(mut state: ResMut<State<GameState>>, ctx: ResMut<EguiContext>) {
    egui::Window::new("Paused")
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            if ui.button("Exit").clicked() {
                state.replace(GameState::menu()).ok();
            }
            if ui.button("Resume").clicked() {
                state.pop().ok();
            }
        });
}
