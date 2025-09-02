//! This module defines the `Task` struct, which wraps a Lua coroutine
//! and its associated priority.
//!
use mlua::prelude::*;

/// Represents a schedulable unit of work, which is a Lua coroutine with a priority.
pub(crate) struct Task {
    /// The Lua coroutine that represents the executable task.
    coroutine: LuaThread,
    /// The priority of the task. Higher values indicate higher priority.
    priority: LuaInteger,
}

impl Task {
    /// Creates a new `Task` instance.
    pub(crate) fn new(coroutine: LuaThread, priority: LuaInteger) -> Self {
        Self {
            coroutine,
            priority,
        }
    }

    /// Returns the priority of the task.
    pub(crate) fn priority(&self) -> LuaInteger {
        self.priority
    }

    /// Checks if the task is still "alive" (resumable or running).
    pub(crate) fn is_alive(&self) -> bool {
        match self.coroutine.status() {
            LuaThreadStatus::Resumable | LuaThreadStatus::Running => true,
            LuaThreadStatus::Finished | LuaThreadStatus::Error => false,
        }
    }

    /// Resumes the coroutine if its status is `Resumable`.
    ///
    /// This method ensures that `step` is only called on valid tasks.
    pub(crate) fn resume(&mut self) {
        if matches!(self.coroutine.status(), LuaThreadStatus::Resumable) {
            self.step();
        }
    }

    /// Advances the coroutine by one step, resuming its execution.
    ///
    /// This is an internal method and does not check the coroutine's status.
    fn step(&mut self) {
        let _ = self.coroutine.resume::<()>(());
    }
}
