use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui::*, *};

/// Resource that tells us if the game is paused or not
pub type Paused = bool;

pub(crate) fn ui_menu_system(
    mut state: ResMut<State<GameState>>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
    mut current_mode: ResMut<CurrentGameMode>,
    mut events: EventWriter<GameEvent>,
    ctx: ResMut<EguiContext>,
    settings_handle: Res<Handle<SettingsAsset>>,
    modes: ResMut<Assets<GameMode>>,
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

            // get current mode
            let mode = &modes.get(current_mode.clone());
            ui.vertical(|ui| {
                egui::ComboBox::from_label("Game mode")
                    .selected_text(&mode.map(|x| &x.name).unwrap_or(&"loading".to_string()))
                    .show_ui(ui, |ui| {
                        for (id, mode) in modes.iter() {
                            if ui
                                .selectable_value(
                                    &mut *current_mode,
                                    modes.get_handle(id),
                                    &mode.name,
                                )
                                .clicked()
                            {
                                events.send(GameEvent::SetGameMode(current_mode.clone()));
                            }
                        }
                    });
            });

            // Start game button
            if ui.button("Start").clicked() {
                state.set(GameState::Main).ok();
            }
        });
    });
}

pub(crate) fn ui_pause_menu_system(
    mut state: ResMut<State<GameState>>,
    mut events: EventWriter<GameEvent>,
    ctx: ResMut<EguiContext>,
) {
    egui::Window::new("Paused")
        .collapsible(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            if ui.button("Exit").clicked() {
                events.send(GameEvent::Reset);
                state.replace(GameState::Menu).ok();
            }
            if ui.button("Resume").clicked() {
                state.pop().ok();
            }
        });
}
