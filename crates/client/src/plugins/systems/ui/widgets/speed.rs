//! Widget that shows the speed of the mode timer, and how it increases over time

use bevy_egui::egui::*;
use shared::prelude::*;

pub struct SpeedWidget<'a> {
    pub mode: &'a GameMode,
    pub step: &'a Step,
    pub timer: Option<&'a PlacementTimer>,
}

impl<'a> Widget for SpeedWidget<'a> {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        // draw vertical rects representing the placement timer and the current speed
        ui.horizontal(|ui| {
            let rect = ui
                .allocate_exact_size(Vec2::new(24f32, 256f32), Sense::hover())
                .0;
            let mut speed_rect = rect.clone();
            // adjust height based on top speed
            if let Some(percent) = self.step.percent(self.mode) {
                speed_rect.set_height(rect.height() * percent);
            }

            // bg
            ui.painter().add(Shape::Rect {
                rect,
                fill: Color32::from_rgba_unmultiplied(255, 255, 255, 12),
                corner_radius: 0f32,
                stroke: Stroke::new(2f32, Color32::from_rgba_unmultiplied(255, 255, 255, 36)),
            });

            ui.painter().add(Shape::Rect {
                rect: speed_rect,
                fill: Color32::GREEN,
                corner_radius: 0f32,
                stroke: Stroke::none(),
            });

            // redundant show of placement timer
            if let Some(timer) = self.timer {
                let mut timer_rect = rect.clone();
                timer_rect.set_height(rect.height() * timer.percent());
                ui.painter().add(Shape::Rect {
                    rect: timer_rect,
                    fill: Color32::from_rgba_unmultiplied(255, 255, 255, 36),
                    corner_radius: 0f32,
                    stroke: Stroke::none(),
                });
            }
        })
        .response
    }
}
