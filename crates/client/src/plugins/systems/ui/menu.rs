use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::Spacing, Ui, Vec2 as EVec2},
    *,
};
use shared::GameType;
use shared::{prelude::*, CampaignDetails};
/// Resource that tells us if the game is paused or not
pub type Paused = bool;

#[derive(Default)]
pub struct MenuState {
    pub mode: Option<Handle<GameMode>>,
    pub map: Option<Handle<Map>>,
    pub theme: Option<Theme>,
    pub campaign: Option<Campaign>,
    pub save: Option<Save>,
}

pub(crate) fn ui_menu_system(
    mut state: ResMut<State<GameState>>,
    mut ui_settings: ResMut<EguiSettings>,
    ctx: ResMut<EguiContext>,
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
                ..Default::default()
            };

            // Main buttons
            ui.horizontal(|ui| {
                // Select campaign screen
                if ui.button("New").clicked() {
                    state.set(GameState::StartOptions).ok();
                }

                if ui.button("Load").clicked() {
                    state.set(GameState::LoadOptions).ok();
                }

                // Game editor
                if ui.button("Edit").clicked() {
                    state.set(GameState::EditOptions).ok();
                }
            });
        });
}

pub(crate) fn ui_start_options_system(
    mut menu_state: ResMut<MenuState>,
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    ctx: ResMut<EguiContext>,
    campaigns: Res<Campaigns>,
) {
    egui::Area::new("ui_start_options_menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            combo_box(
                &mut menu_state.campaign,
                ui,
                "Campaign select",
                campaigns.clone(),
                |c| c.name,
            );
            // Start selected campaign
            if ui.button("Start").clicked() {
                if let Some(campaign) = &menu_state.campaign {
                    if !campaign.levels.is_empty() {
                        // Create save file
                        let save = save_game(campaign, 0);

                        // Set the save file as a current resource
                        cmd.insert_resource(save);

                        // Start game
                        state
                            .set(GameState::Main(GameType::Campaign(CampaignDetails {
                                campaign: campaign.clone(),
                                ..Default::default()
                            })))
                            .ok();
                    }
                }
            }
        });
}

fn combo_box<T, F>(prop: &mut Option<T>, ui: &mut Ui, label: &str, items: Vec<T>, display: F)
where
    T: Clone + PartialEq,
    F: Fn(T) -> String,
{
    egui::ComboBox::from_label(label)
        .selected_text(
            prop.as_ref()
                .map(|t| display(t.clone()))
                .unwrap_or_else(|| "None selected".to_string()),
        )
        .show_ui(ui, |ui| {
            items.iter().for_each(|item| {
                if ui
                    .selectable_value(prop, Some(item.clone()), display(item.clone()))
                    .clicked()
                {
                    *prop = Some(item.clone());
                }
            });
        });
}

pub(crate) fn ui_load_options_system(
    mut menu_state: ResMut<MenuState>,
    mut state: ResMut<State<GameState>>,
    campaigns: Res<Campaigns>,
    saves: Res<Assets<Save>>,
    ctx: ResMut<EguiContext>,
) {
    egui::Area::new("load_options")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            // Main buttons
            ui.vertical(|ui| {
                combo_box(
                    &mut menu_state.save,
                    ui,
                    "Save select",
                    saves.iter().map(|(_, a)| a.clone()).collect(),
                    |s| {
                        format!(
                            "{} - Playtime: {} minutes",
                            s.updated_at.to_rfc2822(),
                            (s.updated_at - s.created_at).num_minutes()
                        )
                    },
                );
            });
            if ui.button("Load").clicked() {
                if let Some(save) = &menu_state.save {
                    campaigns.iter().for_each(|c| {
                        if c.name == save.campaign {
                            state
                                .push(GameState::Main(GameType::Campaign(CampaignDetails {
                                    campaign: c.clone(),
                                    current_level_index: save.level,
                                    campaign_scores: vec![],
                                })))
                                .ok();
                        }
                    });
                }
            }
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
