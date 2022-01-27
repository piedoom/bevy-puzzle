use bevy_egui::egui::{self, Stroke, Widget};
use std::ops::RangeInclusive;

pub struct BarWidget<T>
where
    T: std::ops::Div<Output = T> + Into<f32> + Copy,
{
    pub color_background: egui::Color32,
    pub color_foreground: egui::Color32,
    pub color_outline: egui::Color32,
    pub direction: egui::Direction,
    pub range: RangeInclusive<T>,
    pub current: T,
    pub size: egui::Vec2,
}

impl<T> BarWidget<T>
where
    T: std::ops::Div<Output = T> + Into<f32> + Copy,
{
    pub fn percent(&self) -> f32 {
        let (start, end, current): (f32, f32, f32) = (
            (*self.range.start()).into(),
            (*self.range.end()).into(),
            self.current.into(),
        );

        (current - start) / (end - start)
    }
}

impl<T> Widget for BarWidget<T>
where
    T: std::ops::Div<Output = T> + Into<f32> + Copy,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (res, paint) = ui.allocate_painter(self.size, egui::Sense::click());
        // Shape Background
        paint.add(egui::Shape::rect_filled(
            paint.clip_rect(),
            0.,
            self.color_background,
        ));
        // Shape outline
        paint.add(egui::Shape::rect_stroke(
            paint.clip_rect(),
            0.,
            Stroke::new(1., self.color_outline),
        ));
        // Determine the new rect based on the direction of the bar
        let mut rect = paint.clip_rect();
        let percent = self.percent();
        match self.direction {
            egui::Direction::LeftToRight => rect.set_right(rect.left() + (rect.width() * percent)),
            egui::Direction::RightToLeft => rect.set_left(rect.right() - (rect.width() * percent)),
            egui::Direction::TopDown => rect.set_bottom(rect.top() + (rect.height() * percent)),
            egui::Direction::BottomUp => rect.set_top(rect.bottom() - (rect.height() * percent)),
        }
        // Shape foreground
        paint.add(egui::Shape::rect_filled(rect, 0., self.color_foreground));
        res
    }
}
