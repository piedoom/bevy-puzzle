use crate::prelude::colors::GREEN_COLOR;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};
use bevy::utils::Instant;
use bevy_egui::egui::{self, *};
use bevy_egui::{EguiContext, EguiSettings};

use super::PatternWidget;

/// Draw the UI for our main game state
pub(crate) fn ui_main_system(
    step: Res<Step>,
    windows: Res<Windows>,
    camera: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
    ctx: ResMut<EguiContext>,
    score: Res<Score>,
    active: Query<(Entity, &PlacementTimer, &GlobalTransform), With<ActiveEntity>>,
    hold: Res<Hold>,
    next_up: Res<NextUp>,
    patterns: Res<Assets<Pattern>>,
    state: Res<State<GameState>>,
    started: Res<GameStarted>,
    ui_settings: Res<EguiSettings>,
) {
    // get current mode
    if let GameState::Game(game_type) = state.current() {
        let GameDetails { options, .. } = game_type.get_details();
        if let Ok((camera, _projection, camera_transform)) = camera.get_single() {
            // cursor
            // get active position
            active
                .get_single()
                .map(|(_, timer, active_transform)| {
                    if let Some(pos) = camera.world_to_screen(
                        &windows,
                        camera_transform,
                        active_transform.translation,
                    ) {
                        let window_height =
                            windows.get_primary().map(|x| x.height()).unwrap_or(0f32);
                        let offset = 48f32 * ui_settings.scale_factor as f32;
                        let pos = egui::Vec2::new(pos.x - offset, window_height - pos.y - offset)
                            / ui_settings.scale_factor as f32;

                        // Show the placement timer and overall speed next to the current cursor
                        // TODO: make this work for arrow keys / console

                        egui::containers::Area::new("timer")
                            .interactable(false)
                            .current_pos(Pos2::new(pos.x, pos.y))
                            .show(ctx.ctx(), |ui| {
                                ui.add(PlacementTimerWidget {
                                    size: egui::Vec2::splat(24f32),
                                    timer_percent: timer.percent(),
                                    timer_stroke: Stroke::new(5f32, GREEN_COLOR),
                                    speed_percent: step.percent(&options).unwrap_or(1f32),
                                    speed_stroke: Stroke::new(4f32, CONTRAST_HIGH_COLOR),
                                    background_color: BACKGROUND_LIGHT_COLOR,
                                });
                            });
                    }
                })
                .ok();

            let details = game_type.get_details();

            egui::containers::Area::new("time")
                .anchor(Align2::CENTER_TOP, egui::Vec2::ZERO)
                .show(ctx.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(TimeWidget {
                            current_time: Instant::now().duration_since(*started).as_secs(),
                            time_limit: match details.objective {
                                Objective::FreePlay | Objective::Score(_) => None,
                                Objective::Survive(duration)
                                | Objective::TimeLimit { duration, .. } => Some(duration.as_secs()),
                            },
                        });

                        if let Objective::TimeLimit { duration, .. }
                        | Objective::Survive(duration) = details.objective
                        {
                            let range = match details.objective {
                                Objective::Survive(_) => 0f32..=duration.as_secs_f32(),
                                Objective::TimeLimit { .. } => duration.as_secs_f32()..=0f32,
                                _ => unreachable!(),
                            };
                            ui.add(BarWidget::<f32> {
                                color_background: colors::BACKGROUND_COLOR,
                                color_foreground: colors::GREEN_COLOR,
                                color_outline: colors::BACKGROUND_LIGHT_COLOR,
                                direction: egui::Direction::LeftToRight,
                                range,
                                current: Instant::now().duration_since(*started).as_secs_f32(),
                                size: egui::Vec2::new(120., 20.),
                            });
                        }
                    });
                });

            egui::containers::Area::new("info")
                .anchor(Align2::LEFT_TOP, egui::Vec2::splat(32f32))
                .show(ctx.ctx(), |ui| {
                    ui.vertical(|ui| {
                        // Dispalay any objective related info
                        match details.objective {
                            Objective::FreePlay => {
                                ui.vertical(|ui| {
                                    ui.heading("Free Play");
                                    ui.label(format!(
                                        "Time elapsed: {}",
                                        Instant::now().duration_since(*started).as_secs()
                                    ));
                                    ui.label(format!("Score: {}", **score));
                                });
                            }
                            Objective::Survive(duration) => {
                                // Countdown timer
                                let secs_left = duration.as_secs()
                                    - Instant::now().duration_since(*started).as_secs();

                                ui.vertical(|ui| {
                                    ui.heading("Survival");
                                    ui.label(format!("Time left: {}", secs_left));
                                    ui.label(format!("Score: {}", **score));
                                });
                            }
                            Objective::TimeLimit {
                                required_score,
                                duration,
                            } => {
                                // Countdown timer
                                let secs_left = duration.as_secs()
                                    - Instant::now().duration_since(*started).as_secs();

                                ui.vertical(|ui| {
                                    ui.heading("Time Trial");
                                    ui.label(format!("Time left: {}", secs_left));
                                    ui.label(format!("Score: {}/{}", **score, required_score));
                                });
                            }
                            Objective::Score(required_score) => {
                                ui.vertical(|ui| {
                                    ui.heading("Score Limit");
                                    ui.label(format!("Score: {}/{}", **score, required_score));
                                });
                            }
                        }
                    });
                });

            // Side panel info
            egui::containers::Area::new("panel")
                .anchor(Align2::RIGHT_TOP, egui::Vec2::new(-32f32, 32f32))
                //.fixed_pos(extents.right_top() + egui::Vec2::new(32f32, 0f32)) // + egui::Vec2::new(100f32, 0f32))
                .show(ctx.ctx(), |ui| {
                    ui.vertical(|ui| {
                        // Create a widget for our held piece, if we are allowed
                        if options.can_hold {
                            ui.add(PatternWidget {
                                pattern: hold.get(),
                                ..Default::default()
                            });
                            ui.heading("Hold");
                            ui.add_space(50f32);
                        }

                        // Create a widget for our next-up pattern, if we are allowed to view it
                        if options.can_peek {
                            if let Some(next_pattern) = patterns.get(next_up.get()) {
                                ui.add(PatternWidget {
                                    pattern: Some(next_pattern),
                                    ..Default::default()
                                });
                                ui.heading("Next");
                                ui.add_space(50f32);
                            }
                        }
                    });
                });
        }
    }
}
