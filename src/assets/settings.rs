use bevy::{prelude::*, reflect::TypeUuid};

use crate::{prelude::Style, resources::Leaderboard};

#[derive(serde::Deserialize, serde::Serialize, TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268fb"]
pub struct SettingsAsset {
    pub style: Style,
    pub board_size: Vec2,
    pub camera_scale: f32,
    pub leaderboard: Leaderboard,
    /// The name of the active user to insert
    pub active_name: String,
}
