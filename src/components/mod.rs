use bevy::core::Timer;

mod tiles;

/// Marks the parent entity of several tiles that act as the game's cursor for placing blocks
#[derive(Default)]
pub struct ActiveEntity;

/// When this looping timer completes, the current [`crate::components::ActiveEntity`] will (attempt) to be placed on the gameboard
pub type PlacementTimer = Timer;

pub use tiles::*;
