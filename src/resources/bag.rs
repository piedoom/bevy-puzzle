use crate::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;

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
