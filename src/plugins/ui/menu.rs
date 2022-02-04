use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::Spacing, Align, Align2, Ui, Vec2 as EVec2},
    *,
};

#[derive(Default)]
pub struct MenuState {
    pub map: Option<Handle<MapAsset>>,
    pub save: Option<Save>,
    pub page: MenuPage,
}

/// Determines which page of the initial menu we are on
#[allow(dead_code)]
#[derive(Default, Clone)]
pub enum MenuPage {
    #[default]
    Initial,
    Help,
    Endless,
    NewCampaign {
        campaign: Option<Campaign>,
    },
    LoadSavedCampaign,
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
    maps: Res<Assets<MapAsset>>,
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
            let mut next_menu_page: Option<MenuPage> = None;
            match &mut menu_state.page {
                MenuPage::Initial => {
                    // Main buttons
                    ui.with_layout(egui::Layout::top_down_justified(Align::Center), |ui| {
                        if ui.button("New Campaign").clicked() {
                            menu_state.page = MenuPage::NewCampaign {
                                campaign: Default::default(),
                            };
                        }

                        if ui.button("Endless Mode").clicked() {
                            state.replace(GameState::Game(
                                GameType::Endless(
                                    GameDetails{
                                        map: maps.get_handle(maps.iter().find(
                                            |(_,a)|
                                            a.name == MapAsset::default_name()
                                        ).unwrap().0),
                                        options: GameOptions {
                                            can_rotate: true,
                                            can_hold: true,
                                            can_peek: true,
                                            timer_rate: TimerRate::Progressive {
                                                start_rate: Duration::from_millis(3000),
                                                end_rate: Duration::from_millis(500),
                                                delay: 8,
                                                steps: 64,
                                            },
                                            patterns: Some(classic_patterns()),
                                            scorer: Scorer::Square(3),
                                            theme: Default::default(),
                                        },
                                        objective: Objective::FreePlay,
                                    }
                                ))).ok();
                        }

                        if ui.button("Help").clicked() {
                            menu_state.page = MenuPage::Help;
                        }

                        /*
                        if ui.button("Load Campaign").clicked() {
                           // menu_state.page = MenuPage::LoadSavedCampaign;
                        }

                        if ui.button("Map Editor").clicked() {
                           // menu_state.page = MenuPage::NewMap;
                        }
                        */
                    });
                }
                MenuPage::NewCampaign { campaign } => {
                    // Show all campaigns
                    combo_box(campaign, ui, "Campaign select", campaigns.clone(), |c| {
                        c.name
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Back").clicked() {
                            next_menu_page = Some(MenuPage::Initial);
                        };
                        // Start selected campaign
                        if ui.button("Start").clicked() {
                            if let Some(campaign) = &campaign {
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
                MenuPage::LoadSavedCampaign => {
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
                                        cmd.insert_resource(save.clone());
                                    }
                                });
                            }
                        }
                    });
                }
                MenuPage::Help => {
                    ui.vertical(|ui| {
                        if ui.button("Back").clicked() {
                            menu_state.page = MenuPage::Initial;
                        }
                        ui.heading("Help");
                        ui.add_space(32f32);
                        ui.small("OBJECTIVE");
                        ui.label("Increase your score by clearing tiles");
                        ui.add_space(32f32);
                        ui.small("CONTROLS");
                        ui.label("Mouse movement / Arrow keys === Move tile");
                        ui.label("Left mouse button / Space === Place tile");
                        ui.label("A / D === Rotate piece counter clockwise and clockwise");
                        ui.label("Shift / Middle mouse === Swap piece with piece in the hold");
                        ui.add_space(32f32);
                        ui.small("GAMEMODES");
                        ui.label("Freeplay: Play for as long as you can survive");
                        ui.label("Survival: Stay alive for a set time.");
                        ui.label("Total Score: Reach a set total score with no time limit.");
                        ui.label("Time Trial: Reach a set number of points in limited time.");
                        ui.add_space(32f32);
                        ui.small("RULESETS");
                        ui.label("3x3 (most common): Blocks will clear when at least a 3x3 area is filled.");
                        ui.label("Line: Blocks will clear when a continuous line is formed horizontally, vertically, or both.");
                    });
                }
                _ => todo!(),
            }
            if let Some(next_page) = next_menu_page {
                menu_state.page = next_page;
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
    maps: Res<Assets<MapAsset>>,
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
                            Objective::Score(required_score) => {
                                format!("Reach a total score of {}.", required_score)
                            },
                        });

                        // If part of a campaign, show the info here
                        if let Some(c) = game_type.get_campaign() {
                            ui.label(format!("Campaign: {}", c.campaign.name));
                            ui.label(format!(
                                "Level {}/{}",
                                c.level_index + 1,
                                c.campaign.levels.len()
                            ));

                            ui.add(ProgressWidget {
                                current_index: c.level_index,
                                current_completed: false,
                                length: c.campaign.levels.len(),
                                ..Default::default()
                            });
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

/// Saves the game after winning
pub(crate) fn ui_post_game_save_system(
    mut cmd: Commands,
    save: Option<ResMut<Save>>,
    state: Res<State<GameState>>,
) {
    if let GameState::PostGame(details) = state.current() {
        if let Some(campaign) = details.game_type.get_campaign() {
            // Only save the game if we won
            if details.result == GameResult::Win {
                if let Some((_, next_index)) = campaign.next_level() {
                    let new_save = match save {
                        Some(save) => Save {
                            updated_at: chrono::offset::Local::now(),
                            level: next_index,
                            ..save.clone()
                        },
                        None => Save::new(campaign.campaign.name, campaign.level_index),
                    };
                    cmd.insert_resource(save_to_file(new_save));
                }
            }
        }
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

#[derive(Default)]
pub(crate) struct PostGameMenuResource {
    pub name_input: String,
}

pub(crate) fn ui_post_game_system(
    mut cmd: Commands,
    mut state: ResMut<State<GameState>>,
    mut menu_state: ResMut<PostGameMenuResource>,
    mut database_events: EventWriter<DatabaseEvent>,
    username: Option<ResMut<UsernameResource>>,
    ctx: ResMut<EguiContext>,
) {
    if let GameState::PostGame(details) = state.current().clone() {
        egui::containers::Window::new("post_game")
            .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx.ctx(), |ui| {
                ui.heading(match details.result {
                    GameResult::Lose => "Lost",
                    GameResult::Win => "Won",
                });
                ui.label(format!("Score: {}", details.score));

                match &details.game_type {
                    GameType::Campaign(campaign) => {
                        ui.add(ProgressWidget {
                            current_index: campaign.level_index,
                            current_completed: details.result == GameResult::Win,
                            length: campaign.campaign.levels.len(),
                            ..Default::default()
                        });

                        // Specific win-loss ui items
                        match details.result {
                            GameResult::Win => {
                                if let Some((_, next_index)) = campaign.next_level() {
                                    if ui.button("Continue").clicked() {
                                        let mut new_campaign = campaign.clone();
                                        new_campaign.level_index = next_index;
                                        state
                                            .replace(GameState::PreGame(GameType::Campaign(
                                                new_campaign,
                                            )))
                                            .ok();
                                    }
                                } else {
                                    ui.heading("You won this campaign!");
                                }
                            }
                            GameResult::Lose => {
                                if ui.button("Retry").clicked() {
                                    state.replace(GameState::PreGame(details.game_type)).ok();
                                }
                            }
                        }
                    }
                    GameType::Endless(_details) => {
                        // prompt user for their name if we lost and the `UsernameResource` is not set
                        if username.is_none() {
                            // name hasn't been asked for yet, do that now -
                            ui.text_edit_singleline(&mut menu_state.name_input);
                            if ui.button("Add my score").clicked() {
                                // set as resource
                                cmd.insert_resource(UsernameResource::new(
                                    menu_state.name_input.clone(),
                                ));
                                // upload score
                                database_events.send(DatabaseEvent::InsertScore(ScoreRecord {
                                    score: details.score,
                                    name: menu_state.name_input.to_string(),
                                }))
                            }
                        }
                        if ui.button("Replay").clicked() {
                            state.replace(GameState::PreGame(details.game_type)).ok();
                        }
                    }
                    GameType::Other(_details) => {
                        if ui.button("Retry").clicked() {
                            state.replace(GameState::PreGame(details.game_type)).ok();
                        }
                    }
                }

                if ui.button("Return to Menu").clicked() {
                    state.replace(GameState::Menu).ok();
                }
            });
    }
}

fn classic_patterns() -> Vec<String> {
    vec![
        "quad".to_string(),
        "bracket right".to_string(),
        "bracket left".to_string(),
        "pyramid".to_string(),
        "snake left".to_string(),
        "snake right".to_string(),
    ]
}
