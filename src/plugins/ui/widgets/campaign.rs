use crate::utils::colors::*;
use bevy_egui::egui::{self, Color32, Widget};

/// Shows what level we are currently on and how many there are to go/how many there were
pub struct ProgressWidget {
    pub current_index: usize,
    pub completed: bool,
    pub length: usize,
    pub spacing: egui::Vec2,
    pub node_size: f32,
}

impl Widget for ProgressWidget {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        let ProgressWidget {
            current_index,
            completed,
            length,
            spacing,
            node_size,
        } = self;
        let (width, height) = (
            (node_size * length as f32) + (spacing.x * (length - 1) as f32),
            node_size + (spacing.y * 2f32),
        );
        let calc_size = egui::Vec2::new(width / length as f32, height);
        let (resp, paint) =
            ui.allocate_painter(egui::Vec2::new(width, height), egui::Sense::hover());
        let get_center = |index| -> egui::Pos2 {
            let x = (calc_size * (index + 1) as f32) - (calc_size / 2f32);
            resp.rect.left_center() + (egui::Vec2::X * x)
        };
        for i in 0..length {
            let center = get_center(i);
            let radius = node_size / 2f32;

            let (fill_color, stroke) = match i.cmp(&current_index) {
                std::cmp::Ordering::Less => {
                    // already completed level
                    (BLUE_COLOR, egui::Stroke::new(2f32, BLUE_COLOR))
                }
                std::cmp::Ordering::Equal => {
                    // current level
                    match completed {
                        true => (Color32::GREEN, egui::Stroke::new(2f32, Color32::GREEN)),
                        false => (
                            Color32::TRANSPARENT,
                            egui::Stroke::new(2f32, Color32::YELLOW),
                        ),
                    }
                }
                std::cmp::Ordering::Greater => {
                    (Color32::TRANSPARENT, egui::Stroke::new(2f32, Color32::GRAY))
                }
            };
            paint.circle(center, radius, fill_color, stroke);
            if !completed && i == current_index {
                // add a smaller inner circle to show current level
                paint.circle_filled(center, radius / 2f32, Color32::YELLOW);
            }
            // draw a line to connect circles in-between
            if i != length - 1 {
                paint.line_segment(
                    [
                        get_center(i) + (egui::Vec2::X * radius),
                        get_center(i + 1) - (egui::Vec2::X * radius),
                    ],
                    stroke,
                );
            }
        }
        resp
    }
}

impl Default for ProgressWidget {
    fn default() -> Self {
        Self {
            current_index: 0,
            completed: false,
            length: 3,
            spacing: egui::Vec2::new(32f32, 8f32),
            node_size: 16f32,
        }
    }
}
