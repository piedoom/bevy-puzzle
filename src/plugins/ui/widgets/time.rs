use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::egui::{Label, Layout, Sense, Vec2, Widget};

pub struct TimeWidget {
    pub current_time: u64,
    pub time_limit: Option<u64>,
}

impl Widget for TimeWidget {
    fn ui(self, ui: &mut bevy_egui::egui::Ui) -> bevy_egui::egui::Response {
        ui.add(
            Label::new(format!(
                "{}",
                match self.time_limit {
                    Some(time_limit) => time_limit - self.current_time,
                    None => self.current_time,
                }
            ))
            .heading(),
        )
    }
}
