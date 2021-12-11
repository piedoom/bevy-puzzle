use bevy::{core::Timer, ecs::component::Component, prelude::Color};

mod tiles;

/// Marks the parent entity of several tiles that act as the game's cursor for placing blocks
#[derive(Default, Component)]
pub struct ActiveEntity;

/// When this looping timer completes, the current [`crate::components::ActiveEntity`] will (attempt) to be placed on the gameboard
#[derive(Default, Component)]
pub struct PlacementTimer(Timer);
impl PlacementTimer {
    pub fn get(&self) -> &Timer {
        &self.0
    }
    pub fn get_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
    pub fn set(&mut self, timer: Timer) {
        self.0 = timer;
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
