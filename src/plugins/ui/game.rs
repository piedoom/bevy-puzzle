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
                        let offset = 32f32 * ui_settings.scale_factor as f32;
                        let pos = egui::Vec2::new(pos.x - offset, window_height - pos.y - offset)
                            / ui_settings.scale_factor as f32;

                        // Show the placement timer and overall speed next to the current cursor
                        // TODO: make this work for arrow keys / console

                        egui::containers::Area::new("timer")
                            .interactable(false)
                            .current_pos(Pos2::new(pos.x, pos.y))
                            .show(ctx.ctx(), |ui| {
                                ui.add(PlacementTimerWidget {
                                    percent: timer.percent(),
                                    size: egui::Vec2::splat(16f32),
                                    stroke: Stroke::new(3f32, GREEN_COLOR),
                                });
                            });

                        // egui::containers::Area::new("timer")
                        //     .interactable(false)
                        //     .current_pos(Pos2::new(pos.x, pos.y))
                        //     .show(ctx.ctx(), |ui| {
                        //         const STROKE_WIDTH: f32 = 1.;
                        //         let (_, paint) = ui.allocate_painter(
                        //             egui::Vec2::splat((radius + STROKE_WIDTH) * 2f32),
                        //             Sense::click(),
                        //         );

                        //         // Background shape
                        //         paint.add(Shape::circle_filled(
                        //             Pos2::new(pos.x + radius, pos.y + radius),
                        //             radius,
                        //             Color32::from_rgb(
                        //                 colors::BACKGROUND_LIGHT[0],
                        //                 colors::BACKGROUND_LIGHT[1],
                        //                 colors::BACKGROUND_LIGHT[2],
                        //             ),
                        //         ));

                        //         // Placement timer shape
                        //         paint.add(Shape::circle_filled(
                        //             Pos2::new(pos.x + radius, pos.y + radius),
                        //             radius * timer.get().percent(),
                        //             Color32::from_rgb(
                        //                 colors::GREEN[0],
                        //                 colors::GREEN[1],
                        //                 colors::GREEN[2],
                        //             ),
                        //         ));
                        //     });
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

                        // Speed
                        ui.heading("Speed");
                        ui.add(SpeedWidget {
                            size: egui::Vec2::new(300f32, 20.),
                            timer_rate: &details.options.timer_rate,
                            step: &step,
                        });
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

            // egui::containers::Area::new("speed")
            //     .anchor(Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
            //     .show(ctx.ctx(), |ui| {
            //         let window_size = windows
            //             .get_primary()
            //             .map(|window| egui::Vec2::new(window.width(), window.height()))
            //             .unwrap_or(egui::Vec2::ZERO);
            //         let window_width = window_size.x;

            //         ui.vertical(|ui| {
            //             ui.heading("Speed");
            //             ui.add(SpeedWidget {
            //                 size: egui::Vec2::new(300f32.clamp(0.0, window_width), 20.),
            //                 timer_rate: &details.options.timer_rate,
            //                 step: &step,
            //             });
            //         });
            //     });

            // Side panel info
            egui::containers::Area::new("panel")
                .anchor(Align2::RIGHT_TOP, egui::Vec2::new(-32f32, 32f32))
                //.fixed_pos(extents.right_top() + egui::Vec2::new(32f32, 0f32)) // + egui::Vec2::new(100f32, 0f32))
                .show(ctx.ctx(), |ui| {
                    ui.vertical(|ui| {
                        // Create a widget for our held piece, if we are allowed
                        if options.can_hold {
                            ui.add(PatternWidget::new(hold.get()).size(128f32));
                            ui.heading("Hold");
                            ui.add_space(50f32);
                        }

                        // Create a widget for our next-up pattern, if we are allowed to view it
                        if options.can_peek {
                            if let Some(next_pattern) = patterns.get(next_up.get()) {
                                ui.add(PatternWidget::new(Some(next_pattern)).size(128f32));
                                ui.heading("Next");
                                ui.add_space(50f32);
                            }
                        }
                    });
                });
        }
    }
}
