//! Score for the current game state. This resets every level.
use std::ops::{Add, AddAssign, Deref, DerefMut};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ScoreRecord {
    pub score: usize,
    pub name: String,
}

#[derive(Default)]
pub struct Score(usize);

impl Add for Score {
    type Output = Score;

    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0)
    }
}

impl AddAssign<usize> for Score {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl Deref for Score {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct HighScoresResource(pub Option<Vec<ScoreRecord>>);

impl Deref for HighScoresResource {
    type Target = Option<Vec<ScoreRecord>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HighScoresResource {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        &mut self.0
    }
}
