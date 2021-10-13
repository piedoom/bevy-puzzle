use crate::prelude::*;

mod campaign;
mod input;
mod leaderboard;
mod tile;

pub use {campaign::*, input::*, leaderboard::*, tile::*};

#[derive(Default)]
/// Counts the number of piece placements in the game
pub struct Step(usize);

impl Step {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) {
        self.0 += 1;
    }

    pub fn current(&self) -> usize {
        self.0
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn percent(&self, mode: &GameMode) -> Option<f32> {
        match mode.timer_rate {
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

    /// Create a timer appropriate for this step of the game, depending on the gamemode
    pub fn create_timer(&self, mode: &GameMode) -> PlacementTimer {
        // calculate the duration of our timer based on our current step and our gamemode settings
        match mode.timer_rate {
            TimerRate::Constant(duration) => PlacementTimer::new(duration, false),
            TimerRate::Progressive {
                start_rate,
                end_rate,
                steps,
                delay,
            } => {
                // map step to range of values
                let percent = self.percent(&mode).unwrap();
                PlacementTimer::from_seconds(
                    percent.lerp(start_rate.as_secs_f32(), end_rate.as_secs_f32()),
                    false,
                )
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
                PlacementTimer::new(new_time, false)
            }
        }
    }
}
