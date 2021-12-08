use crate::assets::Pattern;
use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};
use std::collections::VecDeque;

/// Possibly contains a [`Pattern`] piece put on hold
#[derive(Default)]
pub struct Hold(Option<Pattern>);

impl Hold {
    /// Get the optional [`Pattern`] piece within
    pub fn get(&self) -> Option<&Pattern> {
        self.0.as_ref()
    }

    /// Set the [`Pattern`] piece within
    pub fn set(&mut self, pattern: Pattern) {
        self.0 = Some(pattern);
    }

    /// Clear any pieces and set to `None`
    pub fn clear(&mut self) {
        self.0 = None;
    }

    /// Set the [`Pattern`] piece within and return the old piece, if any.
    /// Returns the old optional piece.
    ///
    /// * `pattern` - the new pattern to put in the hold
    pub fn swap(&mut self, pattern: Pattern) -> Option<Pattern> {
        let ret = self.get().cloned();
        self.set(pattern);
        ret
    }
}

/// A random distribution of all game pieces. This is similar to the other
/// 4-block game and helps with reducing bad luck losses. The bag is also
/// a cool iterator that does a cool side effect lol watch out haha...
#[derive(Default, Clone)]
pub struct Bag {
    // All possible patterns that can be played
    pub(crate) patterns: Vec<Handle<Pattern>>,
    /// The queue of pieces in the bag
    pub queue: VecDeque<Handle<Pattern>>,
}

impl Bag {
    /// Create a new bag with selected `patterns`
    pub fn new(patterns: Vec<Handle<Pattern>>) -> Self {
        let mut s = Self {
            patterns,
            queue: Default::default(),
        };
        s.next();
        s
    }
}

/// The next-up piece in queue to become a [`crate::components::ActiveEntity`]
#[derive(Default, Clone)]
pub struct NextUp(Handle<Pattern>);

impl NextUp {
    pub fn set(&mut self, pattern: Handle<Pattern>) {
        self.0 = pattern;
    }
    pub fn get(&self) -> Handle<Pattern> {
        self.0.clone()
    }
}

impl Iterator for Bag {
    type Item = Handle<Pattern>;

    fn next(&mut self) -> Option<Self::Item> {
        // add more pieces if we have no more
        if self.queue.is_empty() {
            self.patterns.shuffle(&mut thread_rng());
            for pattern in &self.patterns {
                self.queue.push_back(pattern.clone());
            }
        }
        self.queue.pop_front()
    }
}
