use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_egui::egui::{self, *};
use bevy_egui::EguiContext;

use crate::assets::*;
use crate::prelude::*;
use crate::ui::widgets::PatternWidget;

/// Draw the UI for our main game state
pub(crate) fn ui_main_system(
    windows: Res<Windows>,
    camera: Query<(&Camera, &GlobalTransform)>,
    ctx: ResMut<EguiContext>,
    score: Res<Score>,
    active: Query<(Entity, &PlacementTimer, &GlobalTransform), With<ActiveEntity>>,
    hold: Res<Hold>,
    next_up: Res<NextUp>,
    patterns: Res<Assets<Pattern>>,
    state: Res<State<GameState>>,
    extents: QuerySet<(
        Query<&GlobalTransform, With<BoardTopLeftExtent>>,
        Query<&GlobalTransform, With<BoardTopRightExtent>>,
        Query<&GlobalTransform, With<BoardBottomLeftExtent>>,
        Query<&GlobalTransform, With<BoardBottomRightExtent>>,
    )>,
) {
    // get current mode
    if let GameState::Main { mode, .. } = state.current() {
        if let Ok((camera, camera_transform)) = camera.single() {
            // get the positions of the extents of the game board to align UI
            let (top_left, top_right, bottom_left, bottom_right) = {
                // get game positions
                let (mut tl, mut tr, mut bl, mut br) = {
                    (
                        extents
                            .q0()
                            .single()
                            .map(|t| t.translation)
                            .unwrap_or_default(),
                        extents
                            .q1()
                            .single()
                            .map(|t| t.translation)
                            .unwrap_or_default(),
                        extents
                            .q2()
                            .single()
                            .map(|t| t.translation)
                            .unwrap_or_default(),
                        extents
                            .q3()
                            .single()
                            .map(|t| t.translation)
                            .unwrap_or_default(),
                    )
                };

                // adjust to get corners of tiles instead of center
                tl.y += 0.5;
                tl.x -= 0.5;

                tr.y += 0.5;
                tr.x += 0.5;

                bl.y -= 0.5;
                bl.x -= 0.5;

                br.y -= 0.5;
                br.x += 0.5;

                let (mut tl, mut tr, mut bl, mut br) = (
                    camera
                        .world_to_screen(&windows, camera_transform, tl)
                        .unwrap(),
                    camera
                        .world_to_screen(&windows, camera_transform, tr)
                        .unwrap(),
                    camera
                        .world_to_screen(&windows, camera_transform, bl)
                        .unwrap(),
                    camera
                        .world_to_screen(&windows, camera_transform, br)
                        .unwrap(),
                );

                // get window height for conversion
                let height = windows.get_primary().map(|x| x.height()).unwrap_or(0f32);

                // Convert to egui coordinates (negative y)
                tl.y = -tl.y + height;
                tr.y = -tr.y + height;
                bl.y = -bl.y + height;
                br.y = -br.y + height;

                (
                    Pos2::new(tl.x, tl.y),
                    Pos2::new(tr.x, tr.y),
                    Pos2::new(bl.x, bl.y),
                    Pos2::new(br.x, br.y),
                )
            };

            // cursor
            // get active position
            active
                .single()
                .map(|(_, timer, t)| {
                    if let Some(pos) =
                        camera.world_to_screen(&windows, camera_transform, t.translation)
                    {
                        let margin = 64f32;
                        let pos = egui::Vec2::new(
                            pos.x - margin,
                            windows.get_primary().map(|x| x.height()).unwrap_or(0f32)
                                - pos.y
                                - margin,
                        );
                        egui::containers::Area::new("timer")
                            .interactable(false)
                            .current_pos(Pos2::new(pos.x, pos.y))
                            .show(ctx.ctx(), |ui| {
                                let (_, paint) = ui.allocate_painter(
                                    egui::Vec2::new(32f32, 32f32),
                                    Sense::click(),
                                );

                                paint.add(Shape::circle_filled(
                                    Pos2::new(pos.x + 16f32, pos.y + 16f32),
                                    16f32,
                                    Color32::from_rgb(46, 45, 91),
                                ));

                                paint.add(Shape::circle_filled(
                                    Pos2::new(pos.x + 16f32, pos.y + 16f32),
                                    16f32 * timer.percent(),
                                    Color32::GREEN,
                                ));
                            });
                    }
                })
                .ok();

            // Score
            egui::containers::Area::new("score")
                .fixed_pos(top_left + egui::Vec2::new(0f32, -32f32))
                .show(ctx.ctx(), |ui| {
                    ui.heading(format!("Score: {}", score.to_string()));
                });

            // Side panel info
            egui::containers::Area::new("panel")
                .fixed_pos(top_right + egui::Vec2::new(100f32, 0f32))
                .show(ctx.ctx(), |ui| {
                    ui.vertical(|ui| {
                        // Create a widget for our held piece, if we are allowed
                        if mode.can_hold {
                            ui.add(PatternWidget::new(hold.get()).size(128f32));
                            ui.heading("Hold");
                            ui.add_space(50f32);
                        }

                        // Create a widget for our next-up pattern, if we are allowed to view it
                        if mode.can_peek {
                            if let Some(next_pattern) = patterns.get(next_up.clone()) {
                                ui.add(PatternWidget::new(Some(next_pattern)).size(128f32));
                                ui.heading("Next");
                            }
                        }
                    });
                });
        }
    }
}
