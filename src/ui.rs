use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Widget},
    EguiContext,
};

use crate::{ActiveEntity, Hold, Pattern, PlacementTimer, Score};

pub(crate) fn ui(
    ctx: ResMut<EguiContext>,
    score: Res<Score>,
    place_timer: Res<PlacementTimer>,
    hold: Res<Hold>,
) {
    egui::SidePanel::right("panel")
        .default_width(300f32)
        .show(ctx.ctx(), |ui| {
            ui.vertical(|ui| {
                ui.label(format!("Score: {}", score.to_string()));
                // hold

                ui.add(HoldWidget::new(hold.get()));

                // timer thing
                let (res, paint) = ui
                    .allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::click());
                let mut rect = res.rect;
                rect.set_width(rect.width() * place_timer.normalized());
                paint.add(egui::Shape::rect_filled(rect, 0f32, Color32::GREEN))
            });
        });
}

/// Shows the next up piece
pub struct HoldWidget<'a> {
    pattern: Option<&'a Pattern>,
    color: Color32,
    size: Option<f32>,
}
impl<'a> HoldWidget<'a> {
    pub fn new(pattern: Option<&'a Pattern>) -> Self {
        Self {
            pattern: pattern,
            size: Some(100f32),
            color: pattern
                .map(|x| {
                    Color32::from_rgb(
                        (u8::MAX as f32 * x.color.r()) as u8,
                        (u8::MAX as f32 * x.color.g()) as u8,
                        (u8::MAX as f32 * x.color.b()) as u8,
                    )
                })
                .unwrap_or(Color32::WHITE),
        }
    }
}

impl<'a> Widget for HoldWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = self.size.unwrap_or(ui.available_width());
        // allocate a square
        let (rect, _) = ui.allocate_exact_size(egui::Vec2::new(size, size), Sense::hover());
        // TODO: determine the number of "blocks" needed and the correct size to use based on the piece size
        // get the largest side of the pattern. If no block is in the hold, default to a 4x4 grid

        let grid_divisions = 4f32;
        // split the available widget space into n number of blocks where the n number is equal to the largest side
        let unit_size = rect.width() / grid_divisions;
        ui.group(|ui| {
            // create a grid hehe
            for x in 0..grid_divisions as usize {
                for y in 0..grid_divisions as usize {
                    let mut local_rect = rect.clone();
                    local_rect = local_rect
                        .translate(egui::Vec2::new(x as f32 * unit_size, y as f32 * unit_size));
                    let square_rect = egui::Rect::from_two_pos(
                        local_rect.min,
                        local_rect.min + egui::Vec2::new(unit_size, unit_size),
                    );
                    // create an empty grid
                    ui.painter().add(Shape::Rect {
                        rect: square_rect,
                        corner_radius: 1f32,
                        fill: Color32::TRANSPARENT,
                        stroke: egui::Stroke::new(
                            2f32,
                            Color32::from_rgba_unmultiplied(255, 255, 255, 80),
                        ),
                    });
                    if let Some(pattern) = self.pattern {
                        // highlight if correct coords
                        if pattern.blocks.contains(&Vec2::new(x as f32, -(y as f32))) {
                            ui.painter().add(Shape::Rect {
                                rect: square_rect,
                                corner_radius: 1f32,
                                fill: self.color,
                                stroke: Stroke::none(),
                            });
                        }
                    }
                }
            }
        })
        .response
    }
}

trait Vec2Ext {
    fn largest(&self) -> f32;
}

impl Vec2Ext for Vec2 {
    fn largest(&self) -> f32 {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}
