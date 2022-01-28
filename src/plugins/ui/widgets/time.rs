use std::{f32::consts::PI, f64::consts::TAU};

use bevy_egui::egui::{self, epaint::PathShape, Color32, Pos2, Sense, Stroke, Widget};
use lyon_geom::*;

use crate::prelude::colors::{BACKGROUND_LIGHT_COLOR, YELLOW, YELLOW_COLOR};

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
    pub percent: f32,
    pub size: egui::Vec2,
    pub stroke: Stroke,
}

impl Widget for PlacementTimerWidget {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        let radius = ((self.size.x - self.stroke.width) / 2f32) as f64;
        let (resp, paint) = ui.allocate_painter(self.size, Sense::hover());
        let center = resp.rect.center();
        let normalized = TAU * self.percent as f64;
        let arc = Arc {
            center: Point::new(center.x as f64, center.y as f64),
            radii: vector(radius as f64, radius as f64),
            start_angle: Angle::radians(0.0),
            sweep_angle: Angle::radians(normalized + 0.5), // adds a bit extra to overcompensate so feels less cheap
            x_rotation: Angle::radians(normalized),
        };
        let points: Vec<Pos2> = arc
            .flattened(0.1)
            .map(|p| egui::Pos2::new(p.x as f32, p.y as f32))
            .collect();
        paint.circle_filled(center, self.size.x / 2f32, BACKGROUND_LIGHT_COLOR);
        paint.add(PathShape::line(points, self.stroke));
        resp
    }
}
