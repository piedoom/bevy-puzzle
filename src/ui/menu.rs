use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui::*, *};

/// Resource that tells us if the game is paused or not
pub type Paused = bool;

pub(crate) fn ui_menu_system(
    ctx: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    settings_handle: Res<Handle<SettingsAsset>>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
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
            // Start game button
            if ui.button("Start").clicked() {
                state.set(GameState::Main).ok();
            }
        });
    });
}
