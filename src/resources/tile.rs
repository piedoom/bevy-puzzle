use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};
use std::collections::VecDeque;

/// Saves the handles of all textures needed for styling tiles
#[derive(Default, Clone)]
pub struct TileResources {
    pub empty: TileResource,
    pub full: TileResource,
    pub hover: TileResource,
    pub invalid: TileResource,
    pub scored: TileResource,
}

#[derive(Default, Clone)]
pub struct TileResource {
    pub texture: Handle<Texture>,
    pub material: Handle<ColorMaterial>,
}

impl TileResource {
    pub fn new(tex_mat: (Handle<Texture>, Handle<ColorMaterial>)) -> Self {
        Self {
            texture: tex_mat.0.clone(),
            material: tex_mat.1.clone(),
        }
    }
}

use bevy::core::Timer;

use crate::{assets::Pattern, prelude::GameMode};

/// The piece that is currently in a holding state and can be swapped out for the active piece.
#[derive(Default)]
pub struct Hold(Option<Pattern>);

impl Hold {
    pub fn get(&self) -> Option<&Pattern> {
        self.0.as_ref()
    }

    pub fn set(&mut self, pattern: Pattern) {
        self.0 = Some(pattern);
    }

    pub fn swap(&mut self, pattern: Pattern) -> Option<Pattern> {
        let ret = self.get().cloned();
        self.set(pattern);
        ret
    }
}

/// When this looping timer completes, the current [`crate::components::ActiveEntity`] will (attempt) to be placed on the gameboard
pub type PlacementTimer = Timer;

/// A random distribution of all game pieces. This is similar to the other 4-block game and helps with reducing bad luck losses.
/// The bag is also a cool iterator that does a cool side effect lol watch out haha...
#[derive(Default, Clone)]
pub struct Bag {
    // All possible patterns that can be played
    pub(crate) patterns: Vec<Handle<Pattern>>,
    pub queue: VecDeque<Handle<Pattern>>,
}

impl Bag {
    pub fn new(patterns: Vec<Handle<Pattern>>) -> Self {
        let mut s = Self {
            patterns,
            queue: Default::default(),
        };
        s.next();
        s
    }
}

/// The next-up piece in the queue to become an [`crate::components::ActiveEntity`]. This pattern is shown in the
/// user interface as well.
pub type NextUp = Handle<Pattern>;

impl Iterator for Bag {
    type Item = Handle<Pattern>;

    fn next(&mut self) -> Option<Self::Item> {
        // add more pieces if we have no more
        if self.queue.len() == 0 {
            self.patterns.shuffle(&mut thread_rng());
            for pattern in &self.patterns {
                self.queue.push_back(pattern.clone());
            }
        }
        self.queue.pop_front()
    }
}
