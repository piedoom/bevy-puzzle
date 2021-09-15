mod input;
mod leaderboard;
pub mod mode;
mod tile;

pub use {input::*, leaderboard::*, mode::*, tile::*};

#[derive(Default)]
/// Counts the number of piece placements in the game
pub struct Step(usize);

impl Step {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) {
        self.0 += 1;
    }

    pub fn current(&self) -> usize {
        self.0
    }
}
