use bevy::math::Vec2;

pub type ActiveCoordinates = Vec<Vec2>;

#[derive(Default)]
pub struct CursorPosition {
    pub local: Vec2,
    pub global: Vec2,
}
