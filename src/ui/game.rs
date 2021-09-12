use bevy::prelude::*;
use bevy_egui::egui::*;
use bevy_egui::EguiContext;

use crate::assets::*;
use crate::prelude::*;
use crate::ui::widgets::PatternWidget;

/// Draw the UI for our main game state
pub(crate) fn ui_main_system(
    ctx: ResMut<EguiContext>,
    score: Res<Score>,
    place_timer: Query<&PlacementTimer, With<ActiveEntity>>,
    hold: Res<Hold>,
    next_up: Res<NextUp>,
    patterns: Res<Assets<Pattern>>,
) {
    SidePanel::right("panel")
        .default_width(300f32)
        .show(ctx.ctx(), |ui| {
            ui.label(format!("Score: {}", score.to_string()));

            // Create a widget for our held piece
            ui.add(PatternWidget::new(hold.get()));

            // Create a widget for our next-up pattern
            let next_pattern = patterns.get(next_up.clone()).unwrap();
            ui.add(PatternWidget::new(Some(next_pattern)).size(80f32));

            // Visualize the piece placement timer
            let (res, paint) =
                ui.allocate_painter(ui.available_size_before_wrap_finite(), Sense::click());
            let mut rect = res.rect;

            // get the timer on the active piece to see how much time is left
            let width = place_timer
                .single()
                .map(|t| rect.width() * t.percent())
                .unwrap_or(0f32);
            rect.set_width(width);
            paint.add(Shape::rect_filled(rect, 0f32, Color32::GREEN))
        });
}
