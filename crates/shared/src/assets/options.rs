//! Describes different properties about gameplay that can be loaded, saved, and applied.
use std::time::Duration;

const fn r#true() -> bool {
    true
}

/// When creating a RON description, most fields can be left out for defaults that work in most situations
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub struct GameOptions {
    /// Whether or not the active piece can be rotated
    #[serde(default = "r#true")]
    pub can_rotate: bool,
    /// Whether or not the active piece can be swapped with the [`crate::prelude::Hold`]
    #[serde(default = "r#true")]
    pub can_hold: bool,
    /// Whether or not we can see the next tile in the bag
    #[serde(default = "r#true")]
    pub can_peek: bool,
    /// [`crate::prelude::PlacementTimer`] behavior
    #[serde(default)]
    pub timer_rate: TimerRate,
    /// Optionally restrict some patterns in this mode. If [`None`], then all loaded patterns are used. This is a vector of names that can be used to find
    /// the correct patterns. This is not a Handle as that changes at runtime, and we need this data to persist.
    #[serde(default)]
    pub patterns: Option<Vec<String>>,
    pub scorer: Scorer,
}

impl Default for GameOptions {
    /// Sane defaults for options
    fn default() -> Self {
        Self {
            can_rotate: true,
            can_hold: true,
            can_peek: true,
            timer_rate: TimerRate::Constant(Duration::from_secs(3)),
            patterns: None,
            scorer: Default::default(),
        }
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

/// Dictates the period over time of the [`crate::prelude::PlacementTimer`]. This time controls how fast tiles are auto-placed
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum TimerRate {
    /// The timer is a constant rate defined as a period (e.g. 2 seconds)
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
