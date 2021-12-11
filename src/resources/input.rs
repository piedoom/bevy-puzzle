use bevy::math::Vec2;

/// A list of all whole-number (x, y) coordinates of the [`crate::components::ActiveEntity`]'s [`crate::prelude::Tile`]s
pub type ActiveCoordinates = Vec<Vec2>;

/// The local and global game coordinates for the cursor
#[derive(Default)]
pub struct CursorPosition {
    /// The local space coordinates for the cursor
    pub local: Vec2,
    /// The global space coordinates for the cursor
    pub global: Vec2,
}

/// Which device is currently controlling where the active piece is on screen.
pub enum ActivePositionMode {
    Keyboard,
    Mouse,
}
