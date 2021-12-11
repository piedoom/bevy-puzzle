use crate::prelude::Pattern;
use bevy::prelude::Handle;

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
