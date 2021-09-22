use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_egui::egui::{self, *};
use bevy_egui::EguiContext;

use crate::assets::*;
use crate::prelude::*;
use crate::ui::widgets::PatternWidget;

pub(crate) struct Bounds {
    pub(crate) world: egui::Rect,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            world: egui::Rect::NOTHING,
        }
    }
}

impl Bounds {
    pub fn screen(
        &self,
        camera: &Camera,
        windows: &Windows,
        camera_transform: &GlobalTransform,
    ) -> egui::Rect {
        let min_world = self.world.left_bottom();
        let max_world = self.world.right_top();

        let min_raw = camera
            .world_to_screen(
                &windows,
                camera_transform,
                Vec3::new(min_world.x, min_world.y, 0f32),
            )
            // TODO: need to account for this lol
            .unwrap();

        let max_raw = camera
            .world_to_screen(
                &windows,
                camera_transform,
                Vec3::new(max_world.x, max_world.y, 0f32),
            )
            .unwrap();

        let height = windows
            .get_primary()
            .map(|w| w.height())
            .unwrap_or_default();

        let min = Pos2::new(min_raw.x, height - min_raw.y);
        let max = Pos2::new(max_raw.x, height - max_raw.y);

        egui::Rect { min, max }
    }
}

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
    bounds: Res<Bounds>,
) {
    // get current mode
    if let GameState::Main { mode, map: _ } = state.current() {
        if let Ok((camera, camera_transform)) = camera.single() {
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

            // Score and other ui stuff
            // Get the extents with on-screen pixel values so we can add UI stuff aligned to the gameboard.
            let extents = bounds.screen(&camera, &windows, &camera_transform);

            egui::containers::Area::new("score")
                .fixed_pos(extents.left_top() + egui::Vec2::new(0f32, -32f32))
                .show(ctx.ctx(), |ui| {
                    ui.heading(format!("Score: {}", score.to_string()));
                });

            // Side panel info
            egui::containers::Area::new("panel")
                .fixed_pos(extents.right_top() + egui::Vec2::new(32f32, 0f32)) // + egui::Vec2::new(100f32, 0f32))
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
