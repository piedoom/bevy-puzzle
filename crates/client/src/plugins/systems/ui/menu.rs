use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::Spacing, Vec2 as EVec2},
    *,
};
use shared::prelude::*;

use super::widgets::SelectAssetWidget;

/// Resource that tells us if the game is paused or not
pub type Paused = bool;

#[derive(Default)]
pub struct MenuState {
    pub mode: Option<Handle<GameMode>>,
    pub map: Option<Handle<Map>>,
    pub theme: Option<Theme>,
}

pub(crate) fn ui_menu_system(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    mut menu_state: ResMut<MenuState>,
    mut settings_assets: ResMut<Assets<SettingsAsset>>,
    mut ui_settings: ResMut<EguiSettings>,
    maps: ResMut<Assets<Map>>,
    ctx: ResMut<EguiContext>,
    settings_handle: Res<Handle<SettingsAsset>>,
    modes: Res<Assets<GameMode>>,
    themes: Res<Themes>,
) {
    ui_settings.scale_factor = 2f64;
    egui::Area::new("menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            let spacing = ui.spacing_mut();
            *spacing = Spacing {
                item_spacing: EVec2::splat(4f32),
                window_padding: EVec2::new(24.0, 24.0),
                button_padding: EVec2::new(12.0, 6.0),
                // indent: (),
                // slider_width: (),
                // text_edit_width: (),
                // icon_width: (),
                // icon_spacing: (),
                // tooltip_width: (),
                // indent_ends_with_horizontal_line: (),
                // combo_height: (),
                // scroll_bar_width: (),
                ..Default::default()
            };

            let settings = settings_assets.get_mut(settings_handle.clone()).unwrap();
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

            ui.vertical(|ui| {
                ui.add(SelectAssetWidget::<GameMode> {
                    name: "Mode selection",
                    selection: &mut menu_state.mode,
                    assets: &modes,
                });
                ui.add(SelectAssetWidget::<Map> {
                    name: "Map selection",
                    selection: &mut menu_state.map,
                    assets: &maps,
                });

                // themes
                // set the default theme if none
                if menu_state.theme.is_none() {
                    menu_state.theme = (*themes).iter().find(|x| x.name == "default").cloned();
                }
                egui::ComboBox::from_label("Theme selection")
                    .selected_text(
                        menu_state
                            .theme
                            .as_ref()
                            .map(|t| &t.name)
                            .unwrap_or(&"None selected".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        themes.iter().for_each(|theme| {
                            if ui
                                .selectable_value(
                                    &mut menu_state.theme,
                                    Some(theme.clone()),
                                    theme.name.to_string(),
                                )
                                .clicked()
                            {
                                menu_state.theme = Some(theme.clone());
                            }
                        });
                    })
            });

            // Start game button
            ui.horizontal(|ui| {
                if ui.button("Start").clicked() {
                    cmd.insert_resource(menu_state.theme.as_ref().unwrap().clone());
                    state
                        .set(GameState::Main {
                            mode: menu_state.mode.as_ref().unwrap().clone(),
                            map: menu_state.map.as_ref().unwrap().clone(),
                        })
                        .ok();
                }

                // Game editor
                if ui.button("Editor").clicked() {
                    state.set(GameState::Edit).ok();
                }
            });
        });
}

pub(crate) fn ui_pause_menu_system(mut state: ResMut<State<GameState>>, ctx: ResMut<EguiContext>) {
    egui::Window::new("Paused")
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            // if we got here from edit mode, show a special exit button
            if let Some(GameState::Edit) = state.inactives().first() {
                if ui.button("Exit").clicked() {
                    // todo: keep board
                    state.replace(GameState::Edit).ok();
                }
            } else if ui.button("Exit").clicked() {
                state.replace(GameState::Menu).ok();
            }

            if ui.button("Resume").clicked() {
                state.pop().ok();
            }
        });
}
