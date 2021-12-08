use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::Spacing, Ui, Vec2 as EVec2},
    *,
};
use shared::{prelude::*, CampaignDetails};
use shared::{GameType, NextTransition};
/// Resource that tells us if the game is paused or not
pub type Paused = bool;

#[derive(Default)]
pub struct MenuState {
    pub map: Option<Handle<Map>>,
    pub theme: Option<Theme>,
    pub campaign: Option<Campaign>,
    pub save: Option<Save>,
    pub page: MenuPage,
}

/// Determines which page of the initial menu we are on
#[derive(Default, Clone, Copy)]
pub enum MenuPage {
    #[default]
    Initial,
    NewCampaign,
    LoadCampaign,
    NewCustom,
    NewMap,
}

pub(crate) fn ui_menu_system(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    mut ui_settings: ResMut<EguiSettings>,
    mut menu_state: ResMut<MenuState>,
    saves: Res<Assets<Save>>,
    ctx: ResMut<EguiContext>,
    campaigns: Res<Campaigns>,
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
            let current_page = menu_state.page;
            match current_page {
                MenuPage::Initial => {
                    // Main buttons
                    ui.horizontal(|ui| {
                        if ui.button("New Campaign").clicked() {
                            menu_state.page = MenuPage::NewCampaign;
                        }

                        if ui.button("Load Campaign").clicked() {
                            menu_state.page = MenuPage::LoadCampaign;
                        }

                        if ui.button("Custom Game").clicked() {
                            menu_state.page = MenuPage::NewCustom;
                        }

                        if ui.button("Map Editor").clicked() {
                            menu_state.page = MenuPage::NewMap;
                        }
                    });
                }
                MenuPage::NewCampaign => {
                    // Show all campaigns
                    combo_box(
                        &mut menu_state.campaign,
                        ui,
                        "Campaign select",
                        campaigns.clone(),
                        |c| c.name,
                    );
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            menu_state.page = MenuPage::Initial;
                        };
                        // Start selected campaign
                        if ui.button("Start").clicked() {
                            if let Some(campaign) = &menu_state.campaign {
                                if !campaign.levels.is_empty() {
                                    // Set the save file as a current resource
                                    let save = Save::new(campaign, 0);
                                    cmd.insert_resource(save);

                                    // Start game (go to pre-game screen)
                                    state
                                        .set(GameState::PreGame(GameType::Campaign(
                                            CampaignDetails {
                                                campaign: campaign.clone(),
                                                ..Default::default()
                                            },
                                        )))
                                        .ok();
                                }
                            }
                        }
                    });
                }
                MenuPage::LoadCampaign => {
                    // Main buttons
                    ui.vertical(|ui| {
                        combo_box(
                            &mut menu_state.save,
                            ui,
                            "Save select",
                            saves.iter().map(|(_, a)| a.clone()),
                            |s| {
                                format!(
                                    "{} - Playtime: {} minutes",
                                    s.updated_at.to_rfc2822(),
                                    (s.updated_at - s.created_at).num_minutes()
                                )
                            },
                        );
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            menu_state.page = MenuPage::Initial;
                        };
                        if ui.button("Load").clicked() {
                            if let Some(save) = &menu_state.save {
                                campaigns.iter().for_each(|c| {
                                    if c.name == save.campaign {
                                        state
                                            .push(GameState::PreGame(GameType::Campaign(
                                                CampaignDetails {
                                                    campaign: c.clone(),
                                                    level_index: save.level,
                                                    campaign_scores: vec![],
                                                },
                                            )))
                                            .ok();
                                    }
                                });
                            }
                        }
                    });
                }
                MenuPage::NewCustom => todo!(),
                MenuPage::NewMap => todo!(),
            }
        });
}

fn combo_box<T, F>(
    prop: &mut Option<T>,
    ui: &mut Ui,
    label: &str,
    items: impl IntoIterator<Item = T>,
    display: F,
) where
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
            items.into_iter().for_each(|item| {
                if ui
                    .selectable_value(prop, Some(item.clone()), display(item.clone()))
                    .clicked()
                {
                    *prop = Some(item);
                }
            });
        });
}

/// Shows the game information before beginning the level
pub(crate) fn ui_pre_game_menu_system(
    mut state: ResMut<State<GameState>>,
    maps: Res<Assets<Map>>,
    ctx: ResMut<EguiContext>,
) {
    match state.current().clone() {
        GameState::PreGame(game_type) => {
            egui::Area::new("pre_game")
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx.ctx(), |ui| {
                    // Main buttons
                    ui.vertical(|ui| {
                        // Show basic info
                        let details = game_type.get_details();
                        let options = details.options;
                        ui.label(format!("Map: {}", maps.get(details.map).unwrap().name));
                        ui.label(format!(
                        "Options\nHolding: {}\nPeeking: {}\nRotating: {}\nAll patterns: {}\nScorer: {:?}",
                        options.can_hold,
                        options.can_peek,
                        options.can_rotate,
                        options.patterns.is_none(),
                        options.scorer,
                        ));

                        // Show objective details
                        ui.label(match details.objective {
                            Objective::FreePlay => {
                                "Freeplay. Play for as long as you can survive.".to_string()
                            }
                            Objective::Survive(duration) => {
                                format!("Survival. Stay alive for {} seconds.", duration.as_secs())
                            }
                            Objective::TimeLimit {
                                required_score,
                                duration,
                            } => format!(
                                "Time Trial. Reach {} in {} seconds.",
                                required_score,
                                duration.as_secs()
                            ),
                        });

                        // If part of a campaign, show the info here
                        if let Some(c) = game_type.get_campaign() {
                            ui.label(format!("Campaign: {}", c.campaign.name));
                            ui.label(format!(
                                "Level {}/{}",
                                c.level_index + 1,
                                c.campaign.levels.len()
                            ));
                        }
                    });
                    if ui.button("Start").clicked() {
                        if let GameState::PreGame(game_type) = state.current().clone() {
                            state.replace(GameState::Game(game_type)).ok();
                        } else {
                            unreachable!("System enabled for an incorrect state");
                        }
                    }
                });
        }
        _ => unreachable!("Added system to wrong set state"),
    }
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

pub(crate) fn ui_end_screen_system(mut state: ResMut<State<GameState>>, ctx: ResMut<EguiContext>) {
    let mut next_state = || match state.current() {
        GameState::PostGame(transition) => {
            // Handle win screen
            let next = match transition {
                NextTransition::Menu => GameState::Menu,
                NextTransition::NewLevel(next) => {
                    let next_campaign = next
                        .get_campaign()
                        .expect("Should only use `NewLevel` when using campaigns");
                    let save = Save::new(&next_campaign.campaign, next_campaign.level_index);
                    save_to_file(save);
                    GameState::PreGame(GameType::Campaign(next_campaign))
                }
            };

            state.replace(next).ok();
        }
        _ => unreachable!("System ran outside of `GameState::End`"),
    };

    egui::Window::new("End game")
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx.ctx(), |ui| {
            if ui.button("Continue").clicked() {
                next_state();
            }
        });
}
