use crate::prelude::*;
use bevy::math::Vec2;
use bevy_egui::{egui::Vec2 as EVec2, egui::*};

/// Draws an arbitrary number of tiles with color on a grid
#[derive(Default)]
pub struct PatternWidget<'a> {
    pub pattern: Option<&'a Pattern>,
    pub color: Color32,
    pub size: Option<f32>,
}
impl<'a> PatternWidget<'a> {
    pub fn new(pattern: Option<&'a Pattern>) -> Self {
        Self {
            pattern,
            size: None,
            color: pattern
                .map(|x: &Pattern| {
                    let u: [u8; 4] = x.clone().color.into();
                    Color32::from_rgb(u[0], u[1], u[2])
                })
                .unwrap_or(Color32::WHITE),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

impl<'a> Widget for PatternWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = self.size.unwrap_or_else(|| ui.available_width());
        // allocate a square
        let (rect, _) = ui.allocate_exact_size(EVec2::new(size, size), Sense::hover());
        // TODO: determine the number of "blocks" needed and the correct size to use based on the piece size
        // get the largest side of the pattern. If no block is in the hold, default to a 4x4 grid

        let grid_divisions = 4f32;
        // split the available widget space into n number of blocks where the n number is equal to the largest side
        let unit_size = rect.width() / grid_divisions;
        ui.vertical(|ui| {
            // create a grid hehe
            for x in 0..grid_divisions as usize {
                for y in 0..grid_divisions as usize {
                    let mut local_rect = rect;
                    local_rect = local_rect
                        .translate(EVec2::new(x as f32 * unit_size, y as f32 * unit_size));
                    let square_rect = Rect::from_two_pos(
                        local_rect.min,
                        local_rect.min + EVec2::new(unit_size, unit_size),
                    );
                    // create an empty grid
                    ui.painter().add(Shape::Rect {
                        rect: square_rect,
                        corner_radius: 0f32,
                        fill: Color32::TRANSPARENT,
                        stroke: Stroke::new(2f32, Color32::from_rgb(46, 45, 91)),
                    });
                    if let Some(pattern) = self.pattern {
                        // highlight if correct coords
                        if pattern.blocks.contains(&Vec2::new(x as f32, -(y as f32))) {
                            ui.painter().add(Shape::Rect {
                                rect: square_rect,
                                corner_radius: 0f32,
                                fill: self.color,
                                stroke: Stroke::new(2f32, self.color),
                            });
                        }
                    }
                }
            }
        })
        .response
    }
}
