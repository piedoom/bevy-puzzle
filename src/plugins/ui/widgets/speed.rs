use bevy_egui::egui::{self, Sense, Widget};

use crate::prelude::*;

pub struct SpeedWidget<'a> {
    pub size: egui::Vec2,
    pub timer_rate: &'a TimerRate,
    pub step: &'a Step,
}

impl<'a> Widget for SpeedWidget<'a> {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        ui.vertical(|ui| {
            let (res, paint) = ui.allocate_painter(self.size, Sense::click());
            match self.timer_rate {
                TimerRate::Constant(_) => (),
                TimerRate::Progressive { steps, .. } => {
                    let block_width = self.size.x / *steps as f32;
                    let rect = paint.clip_rect();
                    for i in 0..*steps {
                        let color = if i <= self.step.current() {
                            let percent = i as f32 / *steps as f32;
                            if percent <= 0.33 {
                                colors::GREEN
                            } else if percent > 0.33 && percent <= 0.66 {
                                colors::YELLOW
                            } else {
                                colors::RED
                            }
                        } else {
                            colors::BACKGROUND_LIGHT
                        };
                        let shrink = if i <= self.step.current() { 1.0 } else { 2.0 };

                        // Background shape
                        paint.add(egui::Shape::rect_filled(
                            egui::Rect::from_min_max(
                                egui::Pos2::new(
                                    rect.min.x + (i as f32 * block_width),
                                    rect.min.y - self.size.y,
                                ),
                                egui::Pos2::new(
                                    rect.min.x + ((i + 1) as f32 * block_width),
                                    rect.max.y,
                                ),
                            )
                            .shrink(shrink),
                            0.,
                            egui::Color32::from_rgb_array(color),
                        ));
                    }
                }
                TimerRate::Endless { .. } => (),
            }
        })
        .response
    }
}
