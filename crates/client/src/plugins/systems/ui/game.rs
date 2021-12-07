use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};
use bevy_egui::egui::{self, *};
use bevy_egui::{EguiContext, EguiSettings};

use shared::{prelude::*, GameDetails};

use super::widgets::PatternWidget;

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
    //  bounds: Res<Bounds<bevy::math::Vec2>>,
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
                .map(|(_, timer, t)| {
                    if let Some(pos) =
                        camera.world_to_screen(&windows, camera_transform, t.translation)
                    {
                        let window_height =
                            windows.get_primary().map(|x| x.height()).unwrap_or(0f32);
                        let margin = 64f32 * ui_settings.scale_factor as f32;
                        let pos = egui::Vec2::new(pos.x - margin, window_height - pos.y - margin)
                            / ui_settings.scale_factor as f32;
                        let radius = 10f32;

                        // Show the placement timer and overall speed next to the current cursor
                        // TODO: make this work for arrow keys / console
                        egui::containers::Area::new("timer")
                            .interactable(false)
                            .current_pos(Pos2::new(pos.x, pos.y))
                            .show(ctx.ctx(), |ui| {
                                const STROKE_WIDTH: f32 = 1.;
                                let (_, paint) = ui.allocate_painter(
                                    egui::Vec2::splat((radius + STROKE_WIDTH) * 2f32),
                                    Sense::click(),
                                );

                                // Background shape
                                paint.add(Shape::circle_filled(
                                    Pos2::new(pos.x + radius, pos.y + radius),
                                    radius,
                                    Color32::from_rgb(46, 45, 91),
                                ));

                                // Current speed shape
                                paint.add(Shape::circle_stroke(
                                    Pos2::new(pos.x + radius, pos.y + radius),
                                    radius * step.percent(&options).unwrap_or(1.),
                                    Stroke::new(STROKE_WIDTH, Color32::RED),
                                ));

                                // Placement timer shape
                                paint.add(Shape::circle_filled(
                                    Pos2::new(pos.x + radius, pos.y + radius),
                                    radius * timer.percent(),
                                    Color32::GREEN,
                                ));
                            });
                    }
                })
                .ok();

            // let height = windows
            //     .get_primary()
            //     .map(|w| w.height())
            //     .unwrap_or_default();

            // // Score and other ui stuff
            // // Get the extents with on-screen pixel values so we can add UI stuff aligned to the gameboard.
            // // let extents = {
            // //     let min_world = bounds.world.left_bottom();
            // //     let max_world = bounds.world.right_top();

            //     // // get the min and max in pixels. We use "raw" because in egui the y axis has to be flipped, which we do later on.
            //     let trans = camera_transform.translation;
            //     // offset from camera in game units that the target position is at
            //     let pos_to_pixels = |pos: Pos2| -> Vec3 {
            //         let scalar = 1f32 / projection.scale;
            //         let unit_offset = trans - Vec3::new(pos.x, pos.y, 0f32);
            //         let mut r = unit_offset * scalar;
            //         // invert for egui
            //         r.y = height - r.y;
            //         r
            //     };

            //     let min_raw = pos_to_pixels(Pos2::new(min_world.x, min_world.y));
            //     let max_raw = pos_to_pixels(Pos2::new(max_world.x, max_world.y));

            //     let min = Pos2::new(min_raw.x, height - min_raw.y);
            //     let max = Pos2::new(max_raw.x, height - max_raw.y);

            //     egui::Rect { min, max }
            // };

            egui::containers::Area::new("score")
                .anchor(Align2::LEFT_TOP, egui::Vec2::splat(32f32))
                .show(ctx.ctx(), |ui| {
                    ui.vertical(|ui| {
                        ui.heading(format!("Score: {}", *score));
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
                            ui.add(PatternWidget::new(hold.get()).size(128f32));
                            ui.heading("Hold");
                            ui.add_space(50f32);
                        }

                        // Create a widget for our next-up pattern, if we are allowed to view it
                        if options.can_peek {
                            if let Some(next_pattern) = patterns.get(next_up.clone()) {
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
