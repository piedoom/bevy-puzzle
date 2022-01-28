use std::ops::{Deref, DerefMut};

use bevy::{core::Timer, ecs::component::Component, prelude::Color};

mod tiles;

/// Marks the parent entity of several tiles that act as the game's cursor for placing blocks
#[derive(Default, Component)]
pub struct ActiveEntity;

/// When this looping timer completes, the current [`crate::components::ActiveEntity`] will (attempt) to be placed on the gameboard
#[derive(Default, Component)]
pub struct PlacementTimer(Timer);
impl Deref for PlacementTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PlacementTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Timer> for PlacementTimer {
    fn from(t: Timer) -> Self {
        Self(t)
    }
}

pub use tiles::*;

#[derive(Default, Component)]
pub struct TileColor(pub Color);
