//! Describes different properties about gameplay that can be loaded, saved, and applied.
use std::time::Duration;

use bevy::{prelude::Assets, reflect::TypeUuid};

use super::Pattern;

#[derive(
    serde::Deserialize, serde::Serialize, TypeUuid, PartialEq, Eq, Debug, Clone, Hash, Default,
)]
#[uuid = "abcdef12-3456-4fa8-adc4-78c5822268f8"]
pub struct GameMode {
    pub name: String,
    /// Whether or not the active piece can be rotated
    pub can_rotate: bool,
    /// Whether or not the active piece can be swapped with the [`crate::prelude::Hold`]
    pub can_hold: bool,
    /// Whether or not we can see the next tile in the bag
    pub can_peek: bool,
    /// [`crate::prelude::PlacementTimer`] behavior
    pub timer_rate: TimerRate,
    /// Only allow some patterns in this mode. If [`None`], then all loaded patterns are used. This is a vector of names that can be used to find
    /// the correct patterns. This is not a Handle as that changes at runtime, and we need this data to persist.
    pub patterns: Vec<String>,
    pub scorer: Scorer,
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum Scorer {
    /// Scores when a square with a diameter of `n` is completely filled. Because some piece are
    /// as small as 2x2, values less than 3 should be avoided, else they will auto-score when placed.
    Square(usize),
    /// Scores when a full line has been completed vertically or horizontally
    Line(ScoreDirection),
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum ScoreDirection {
    Vertical,
    Horizontal,
    Both,
}

impl Default for Scorer {
    fn default() -> Self {
        Self::Square(3)
    }
}

impl GameMode {
    pub fn default_name() -> &'static str {
        "default"
    }

    pub fn default_with_patterns(patterns: &Assets<Pattern>) -> Self {
        Self {
            name: "default".into(),
            patterns: patterns.iter().map(|(_, p)| p.name.clone()).collect(),
            ..Default::default()
        }
    }
}

/// Dictates the period over time of the [`crate::prelude::PlacementTimer`]
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum TimerRate {
    Constant(Duration),
    /// The timer progressively gets faster. The game starts at `start_rate`, and continues for n steps `delay`.
    /// Steps are when the [`crate::prelude::PlacementTimer`] is finished. The timer will eventually begin to interpolate to the `end_rate`
    /// after n steps, and will completely transition to the `end_rate` after n `steps`.
    Progressive {
        start_rate: Duration,
        end_rate: Duration,
        steps: usize,
        delay: usize,
    },
    /// Same as the [`TimerRate::Progressive`], but with no defined `end_rate`. Instead, the `delta` to increase each `step`
    /// is specified.
    Endless {
        start_rate: Duration,
        end_rate: Duration,
        delta: Duration,
        delay: usize,
    },
}

impl Default for TimerRate {
    fn default() -> Self {
        Self::Progressive {
            start_rate: Duration::new(3, 0),
            end_rate: Duration::new(1, 0),
            steps: 64,
            delay: 8,
        }
    }
}
