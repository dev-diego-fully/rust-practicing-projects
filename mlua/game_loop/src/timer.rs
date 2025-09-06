//! This module provides a simple `Timer` struct for calculating the delta time
//! between game loop frames.
//!
use std::time::Instant;
type LuaNumber = mlua::Number;

/// A simple timer used to calculate delta time between frames.
pub(crate) struct Timer {
    last: Instant,
}

impl Timer {
    /// Constructs a new `Timer`, initializing the last recorded time.
    pub(crate) fn new() -> Self {
        Self {
            last: Instant::now(),
        }
    }

    /// Records the current time and returns the duration since the last turn.
    ///
    /// The delta time is returned as a `LuaNumber` (f64) in seconds.
    pub(crate) fn turn(&mut self) -> LuaNumber {
        let current = Instant::now();
        let delta_t = current.duration_since(self.last);
        self.last = current;

        delta_t.as_secs_f64()
    }
}