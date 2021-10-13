use bevy::ecs::component::Component;

/// Includes every tile ever
#[derive(Default, Component)]
pub struct Tile;

/// Marker that means the [`Tile`] is part of the game board
#[derive(Component)]
pub struct GameBoard;

/// Marks an active entity as unswappable. This is useful to prevent constant swapping between the hold
#[derive(Default, Component)]
pub struct Unswappable;

// TODO: use generics instead of this (?)
pub mod tile_states {
    use super::*;
    #[derive(Default, Component)]
    pub struct Empty;
    #[derive(Default, Component)]
    pub struct Full;
    /// Already scored tile that will be cleaned up
    #[derive(Default, Component)]
    pub struct Scored;
}

pub mod tile_styles {
    use super::*;
    #[derive(Default, Component)]
    pub struct None;
    #[derive(Default, Component)]
    pub struct Hover;
    #[derive(Default, Component)]
    pub struct Invalid;
}
