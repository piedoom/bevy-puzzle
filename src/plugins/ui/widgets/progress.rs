use std::cmp::Ordering;

use crate::utils::colors::*;
use bevy_egui::egui::{self, Color32, Widget};

/// Shows what level we are currently on and how many there are to go/how many there were
pub struct ProgressWidget {
    pub current_index: usize,
    pub current_completed: bool,
    pub size: egui::Vec2,
    pub length: usize,
}

impl Widget for ProgressWidget {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        let ProgressWidget {
            current_index,
            current_completed,
            size,
            length,
        } = self;

        let radius = size.y / 2f32;

        // [()=()=()=()]
        //  |--|--|--|--
        //    ^  ^  ^  ^    block_size

        // subtract margin from either of our width
        let width = size.x - (radius * 2f32);

        // Divide the width by the amount of nodes minus one to give us the block size we want
        let block_size = width / (length - 1) as f32;

        // Place a block at the center left of every block
        let (resp, paint) = ui.allocate_painter(size, egui::Sense::hover());
        for i in 0..length {
            let center = |i| {
                resp.rect.left_center()
                // add spacing back
                + (egui::Vec2::X * radius)
                // move forward by a block size
                + (egui::Vec2::X * (block_size * i as f32))
            };

            // Get colors
            let order = i.cmp(&current_index);
            // Get a specific color to fill/stroke each node based on the current position
            let (color, filled) = match order {
                Ordering::Less => (GREEN_COLOR, true),
                Ordering::Equal => match current_completed {
                    true => (GREEN_COLOR, true),
                    false => (YELLOW_COLOR, false),
                },
                Ordering::Greater => (CONTRAST_LOW_COLOR, false),
            };

            let fill_color = if filled { color } else { Color32::TRANSPARENT };
            paint.circle(
                center(i),
                radius - 1f32, // account for stroke
                fill_color,
                egui::Stroke::new(2f32, color),
            );
            if order == Ordering::Equal && !current_completed {
                paint.circle_filled(center(i), radius * 0.6f32, color);
            }

            // draw a line to connect circles in-between
            if i != length - 1 {
                paint.line_segment(
                    [
                        center(i) + (egui::Vec2::X * radius),
                        center(i + 1) - (egui::Vec2::X * radius),
                    ],
                    egui::Stroke::new(2f32, color),
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
            current_completed: false,
            length: 3,
            size: egui::Vec2::new(200f32, 32f32),
        }
    }
}
