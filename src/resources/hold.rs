use crate::assets::Pattern;

/// The piece that is currently in hold and can be swapped out
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
