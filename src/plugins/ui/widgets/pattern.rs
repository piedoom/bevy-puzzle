use crate::prelude::*;
use bevy::math::Vec2;
use bevy_egui::{
    self,
    egui::{self, Color32, Rect, Response, Sense, Ui, Widget},
};

/// Draws an arbitrary number of tiles with color on a grid
pub struct PatternWidget<'a> {
    pub pattern: Option<&'a Pattern>,
    pub width: f32,
    pub amount: usize,
    pub gap: f32,
}

impl<'a> Default for PatternWidget<'a> {
    fn default() -> Self {
        Self {
            pattern: Default::default(),
            width: 100f32,
            amount: 4,
            gap: 2f32,
        }
    }
}

impl<'a> Widget for PatternWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (resp, paint) =
            ui.allocate_painter(egui::Vec2::new(self.width, self.width), Sense::hover());

        let item_size =
            (resp.rect.width() - ((self.amount - 1) as f32 * self.gap)) / self.amount as f32;

        for x in 0..self.amount {
            for y in 0..self.amount {
                let offset = egui::Vec2::new(
                    (x as f32 * item_size) + (x as f32 * self.gap),
                    (y as f32 * item_size) + (y as f32 * self.gap),
                );

                let rect = Rect {
                    min: resp.rect.left_top() + offset,
                    max: resp.rect.left_top() + egui::Vec2::splat(item_size) + offset,
                };

                paint.rect_stroke(
                    rect.shrink(0.5f32),
                    0.,
                    egui::Stroke::new(1f32, BACKGROUND_LIGHT_COLOR),
                );
                if let Some(pattern) = self.pattern {
                    if pattern.blocks.contains(&Vec2::new(x as f32, -(y as f32))) {
                        let color: Color32 = pattern.color.into();
                        paint.rect_filled(rect, 0., color);
                    }
                }
            }
        }

        resp
    }
}
