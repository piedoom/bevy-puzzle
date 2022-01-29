use std::f64::consts::TAU;

use bevy_egui::egui::{self, epaint::PathShape, Color32, Pos2, Sense, Stroke, Widget};
use lyon_geom::*;

use crate::prelude::*;

pub struct TimeWidget {
    pub current_time: u64,
    pub time_limit: Option<u64>,
}

impl Widget for TimeWidget {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        ui.heading(format!(
            "{}",
            match self.time_limit {
                Some(time_limit) => time_limit - self.current_time,
                None => self.current_time,
            }
        ))
    }
}

pub struct PlacementTimerWidget {
    pub size: egui::Vec2,
    pub timer_percent: f32,
    pub timer_stroke: Stroke,
    pub speed_percent: f32,
    pub speed_stroke: Stroke,
    pub background_color: Color32,
}

impl Widget for PlacementTimerWidget {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        let speed_radius = (self.size.y - self.speed_stroke.width) / 2f32;
        let timer_radius =
            (((self.size.y - self.timer_stroke.width) / 2f32) - self.speed_stroke.width) as f64;
        let (resp, paint) = ui.allocate_painter(self.size, Sense::hover());
        let center = resp.rect.center();
        let normalized = TAU * self.timer_percent as f64;
        let normalized_speed = TAU * self.speed_percent as f64;

        let place_arc: Vec<Pos2> = Arc {
            center: Point::new(center.x as f64, center.y as f64),
            radii: vector(timer_radius as f64, timer_radius as f64),
            start_angle: Angle::zero(),
            sweep_angle: Angle::radians(normalized + 0.5), // adds a bit extra to overcompensate so feels less cheap
            x_rotation: Angle::radians(normalized),
        }
        .flattened(0.1)
        .map(|p| egui::Pos2::new(p.x as f32, p.y as f32))
        .collect();

        let speed_arc: Vec<Pos2> = Arc {
            center: Point::new(center.x as f64, center.y as f64),
            radii: vector(speed_radius as f64, speed_radius as f64),
            start_angle: Angle::zero(),
            sweep_angle: Angle::radians(normalized_speed + 0.5),
            x_rotation: Angle::zero(),
        }
        .flattened(0.1)
        .map(|p| egui::Pos2::new(p.x as f32, p.y as f32))
        .collect();

        paint.circle_filled(center, self.size.x / 2f32, self.background_color);
        paint.add(PathShape::line(place_arc, self.timer_stroke));
        paint.circle_stroke(
            center,
            speed_radius,
            Stroke::new(self.speed_stroke.width, CONTRAST_LOW_COLOR),
        );
        paint.add(PathShape::line(speed_arc, self.speed_stroke));
        resp
    }
}
