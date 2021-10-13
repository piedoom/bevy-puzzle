use bevy::{core::Timer, ecs::component::Component, prelude::Color};

mod tiles;

/// Marks the parent entity of several tiles that act as the game's cursor for placing blocks
#[derive(Default, Component)]
pub struct ActiveEntity;

/// When this looping timer completes, the current [`crate::components::ActiveEntity`] will (attempt) to be placed on the gameboard
pub type PlacementTimer = Timer;

pub use tiles::*;

#[derive(Default, Component)]
pub struct TileColor(pub Color);
