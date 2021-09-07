use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32, Shape},
    EguiContext,
};

use crate::{PlacementTimer, Score};

pub(crate) fn ui(ctx: ResMut<EguiContext>, score: Res<Score>, place_timer: Res<PlacementTimer>) {
    egui::SidePanel::right("panel")
        .default_width(300f32)
        .show(ctx.ctx(), |ui| {
            ui.vertical(|ui| {
                ui.label(format!("Score: {}", score.to_string()));
                let (res, paint) = ui
                    .allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::click());
                let mut rect = res.rect;
                rect.set_width(rect.width() * place_timer.normalized());
                paint.add(egui::Shape::rect_filled(rect, 0f32, Color32::GREEN))
            });
        });
}
