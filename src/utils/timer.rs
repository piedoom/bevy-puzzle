use std::time::Duration;

use bevy::utils::Instant;

/// A generic timer that can be used within a component or resource
pub struct Timer {
    created: Instant,
    /// Instant describes when timer will go off
    alarm: Instant,
}

impl From<Duration> for Timer {
    fn from(offset: Duration) -> Self {
        Self {
            created: Instant::now(),
            alarm: Instant::now() + offset,
        }
    }
}

impl Timer {
    /// Poll the timer to see if it is time to go off yet
    pub fn done(&self) -> bool {
        Instant::now() >= self.alarm
    }

    pub fn remaining(&self) -> Option<Duration> {
        let remaining = Instant::now() - self.alarm;
        if remaining > Duration::ZERO {
            Some(remaining)
        } else {
            None
        }
    }

    pub fn duration(&self) -> Duration {
        self.alarm - self.created
    }

    // Return a value from 0.0 to 1.0 where 1.0 is fully complete
    pub fn normalized(&self) -> f32 {
        let now = Instant::now();
        let current_duration = now - self.created;

        (current_duration.as_secs_f32() / self.duration().as_secs_f32()).clamp(0f32, 1f32)
    }

    // Reset the timer in a way that ensures the time period is as stable as possible. This
    // should be used only with a system that runs every frame
    pub fn cycle(&mut self) {
        // we add plus the alarm instead of the current time which will help with gradual drift
        let alarm = self.alarm + self.duration();
        self.created = Instant::now();
        self.alarm = alarm
    }

    // Reset the timer based off of the time this fn was called
    pub fn reset(&mut self) {
        // we add plus the alarm instead of the current time which will help with gradual drift
        let alarm = Instant::now() + self.duration();
        self.created = Instant::now();
        self.alarm = alarm
    }
}

/// Forcibly places tile when the timer has run out then resets
pub struct PlacementTimer(Timer);

impl PlacementTimer {
    pub fn done(&mut self) -> bool {
        let done = self.0.done();
        if done {
            self.0.cycle();
        }
        done
    }
    pub fn normalized(&self) -> f32 {
        self.0.normalized()
    }
    pub fn reset(&mut self) {
        self.0.reset();
    }
}

impl From<Duration> for PlacementTimer {
    fn from(duration: Duration) -> Self {
        Self(Timer::from(duration))
    }
}
