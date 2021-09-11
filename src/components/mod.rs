pub mod tiles;

#[derive(Default)]
pub struct ActiveEntity;

use bevy::prelude::Color;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Style {
    pub outline: Color,
    pub line_width: f32,
    pub margin: f32,
}
