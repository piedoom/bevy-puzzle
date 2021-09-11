/// Includes every tile ever
pub struct Tile;

/// Marker that means the [`Tile`] is part of the game board
pub struct GameBoard;

/// Marks an active entity as unswappable. This is useful to prevent constant swapping between the hold
#[derive(Default)]
pub struct Unswappable;

// TODO: use generics instead of this (?)
pub mod tile_states {
    #[derive(Default)]
    pub struct Empty;
    #[derive(Default)]
    pub struct Full;
    /// Already scored tile that will be cleaned up
    #[derive(Default)]
    pub struct Scored;
}

pub mod tile_styles {
    #[derive(Default)]
    pub struct None;
    #[derive(Default)]
    pub struct Hover;
    #[derive(Default)]
    pub struct Invalid;
}
