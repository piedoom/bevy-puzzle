use bevy::reflect::TypeUuid;

#[derive(serde::Deserialize, serde::Serialize, TypeUuid)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268fb"]
pub struct UserPreferencesAsset {
    pub camera_scale: f32,
}
