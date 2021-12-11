use crate::assets::Pattern;

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
