use std::ops::{Deref, DerefMut};

pub struct UsernameResource(String);

impl UsernameResource {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl Deref for UsernameResource {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UsernameResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
