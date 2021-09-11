use bevy::utils::AHasher;
use std::hash::{Hash, Hasher};

pub type Score = usize;

/// The name and score of a leaderboard score holder
pub type Leader = (String, usize);

/// An always sorted collection of highest scores
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct Leaderboard {
    /// The name and score of a leaderboard score holder, sorted by largest to smallest score
    pub leaders: Vec<Leader>,
    /// The maximum number of entries that will be saved and displayed. If a score
    /// is sorted to past this `max_length`, it will be dropped
    pub max_length: usize,
}

impl Leaderboard {
    /// Add an entry to the leaderboard if it is better than the worst score already on the leaderboards
    pub fn add(&mut self, name: &str, score: usize) -> bool {
        // Obtain a hash to see if anything changes
        let mut before_hash = AHasher::default();
        self.leaders.hash(&mut before_hash);
        // Push the entry and then truncate by our max length to get the new leaderboard
        self.leaders.push((name.to_string(), score));
        self.leaders.sort_by(|a, b| b.1.cmp(&a.1));
        self.leaders.truncate(self.max_length);
        // If the hash is not the same, it has been added!
        let mut after_hash = AHasher::default();
        self.leaders.hash(&mut after_hash);
        before_hash.finish() != after_hash.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn populated_leaderboard() -> Leaderboard {
        Leaderboard {
            leaders: vec![
                ("Name1".into(), 300),
                ("Name2".into(), 200),
                ("Name3".into(), 100),
            ],
            max_length: 3,
        }
    }

    #[test]
    fn dont_add_bad_score_to_leaderboard() {
        let mut leaderboard = populated_leaderboard();
        let entry = ("Name4".to_string(), 50);
        let added = leaderboard.add(&entry.0, entry.1);
        assert_eq!(added, false);
        assert!(!leaderboard.leaders.contains(&entry));
    }

    #[test]
    fn add_new_to_leaderboard() {
        let mut leaderboard = populated_leaderboard();
        let entry = ("Name4".to_string(), 150);
        leaderboard.add(&entry.0, entry.1);
        assert_eq!(leaderboard.leaders[1], entry); // (1), entry);
    }
}
