use bevy_egui::egui::Color32;

use super::from_rgb_array;

pub const BACKGROUND: [u8; 3] = [0x1b, 0x19, 0x20];
pub const BACKGROUND_LIGHT: [u8; 3] = [0x2E, 0x2D, 0x5B];
pub const CONTRAST_LOW: [u8; 3] = [0x51, 0x50, 0x87];
pub const CONTRAST_HIGH: [u8; 3] = [0xC8, 0xC8, 0xDE];
pub const RED: [u8; 3] = [0xEB, 0x47, 0x6F];
pub const ORANGE: [u8; 3] = [0xFA, 0x67, 0x38];
pub const YELLOW: [u8; 3] = [0xFF, 0xD2, 0x33];
pub const GREEN: [u8; 3] = [0x49, 0xE9, 0x89];
pub const LIME: [u8; 3] = [0xC5, 0xF5, 0x3D];
pub const LIGHT_BLUE: [u8; 3] = [0x3D, 0xFF, 0xF5];
pub const BLUE: [u8; 3] = [0x33, 0x91, 0xFF];
pub const INDIGO: [u8; 3] = [0x5D, 0x59, 0xFF];
pub const PURPLE: [u8; 3] = [0x78, 0x3D, 0xF5];

pub const BACKGROUND_COLOR: Color32 = from_rgb_array(BACKGROUND);
pub const BACKGROUND_LIGHT_COLOR: Color32 = from_rgb_array(BACKGROUND_LIGHT);
pub const CONTRAST_LOW_COLOR: Color32 = from_rgb_array(CONTRAST_LOW);
pub const CONTRAST_HIGH_COLOR: Color32 = from_rgb_array(CONTRAST_HIGH);
pub const RED_COLOR: Color32 = from_rgb_array(RED);
pub const ORANGE_COLOR: Color32 = from_rgb_array(ORANGE);
pub const YELLOW_COLOR: Color32 = from_rgb_array(YELLOW);
pub const GREEN_COLOR: Color32 = from_rgb_array(GREEN);
pub const LIME_COLOR: Color32 = from_rgb_array(LIME);
pub const LIGHT_BLUE_COLOR: Color32 = from_rgb_array(LIGHT_BLUE);
pub const BLUE_COLOR: Color32 = from_rgb_array(BLUE);
pub const INDIGO_COLOR: Color32 = from_rgb_array(INDIGO);
pub const PURPLE_COLOR: Color32 = from_rgb_array(PURPLE);
