use bevy::core::Timer;

use crate::prelude::*;

mod input;
mod leaderboard;
mod tile;
pub use {input::*, leaderboard::*, tile::*};

#[derive(Default)]
/// Counts the number of piece placements, and is used to progress various
/// elements of the game.
///
/// `.0` - the `usize` representing the current step
pub struct Step(usize);

impl Step {
    /// Creates a new `Step`, starting at `0`.
    pub fn new() -> Self {
        Self(0)
    }

    /// Advances the step counter
    pub fn next(&mut self) {
        self.0 += 1;
    }

    /// Gets the current step
    pub fn current(&self) -> usize {
        self.0
    }

    /// Resets the step counter
    pub fn reset(&mut self) {
        self.0 = 0;
    }

    /// Returns an optional normalized completion of the counter based on the
    /// completion strategy defined in [`GameOptions`]. If `None`, the strategy
    /// is not calculatable, such as for [`TimerRate::Endless`].
    ///
    /// * `options` - The current [`GameOptions`]
    pub fn percent(&self, options: &GameOptions) -> Option<f32> {
        match options.timer_rate {
            // A constant timer gives us a constant percentage finished (1f32)
            TimerRate::Constant(_) => Some(1f32),
            TimerRate::Progressive { steps, delay, .. } => {
                // map step to range of values
                Some(if self.0 <= delay {
                    0f32
                } else {
                    ((self.0 - delay) as f32 / steps as f32).clamp(0f32, 1f32)
                })
            }
            TimerRate::Endless { .. } => None,
        }
    }

    /// Returns a [`PlacementTimer`] appropriate for this step of the game,
    /// depending on the gamemode. This is important as the time can vary
    /// throughout the course of the game depending on the step we are at.
    ///
    /// * `options` - The current [`GameOptions`]
    pub fn create_timer(&self, options: &GameOptions) -> PlacementTimer {
        // calculate the duration of our timer based on our current step and our gamemode settings
        match options.timer_rate {
            TimerRate::Constant(duration) => Timer::new(duration, false).into(),
            TimerRate::Progressive {
                start_rate,
                end_rate,
                ..
            } => {
                // map step to range of values
                let percent = self.percent(options).unwrap();
                Timer::from_seconds(
                    percent.lerp(start_rate.as_secs_f32(), end_rate.as_secs_f32()),
                    false,
                )
                .into()
            }
            TimerRate::Endless {
                start_rate,
                end_rate,
                delta,
                delay,
            } => {
                let reduced_rate = delta * (self.0 - delay).clamp(0, usize::MAX) as u32;
                let mut new_time = start_rate - reduced_rate;
                if new_time < end_rate {
                    new_time = end_rate
                }
                Timer::new(new_time, false).into()
            }
        }
    }
}
