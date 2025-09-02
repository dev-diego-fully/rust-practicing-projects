//! This module contains the core Rust implementation of the cooperative scheduler.
//! It defines the generic `LuaScheduler` struct and its main logic for
//! managing and executing tasks.
//!
use mlua::prelude::*;

use crate::{task_list::TaskList, tasks::Task};

/// A generic scheduler that executes tasks managed by a `TaskList`.
///
/// The scheduler takes ownership of a `TaskList` and provides methods
/// for running, stepping, and adding new tasks.
pub(crate) struct LuaScheduler<Tasks: TaskList + 'static> {
    /// The collection of tasks managed by the scheduler.
    tasks: Tasks,
    /// The number of steps the scheduler has executed.
    life_time: usize
}

impl<Tasks: TaskList + 'static> LuaScheduler<Tasks> {

    /// Creates a new `LuaScheduler` instance with a specific `TaskList` implementation.
    pub(super) fn new(tasks: Tasks) -> Self {
        Self {
            tasks,
            life_time: 0
        }
    }

    /// Checks if there are any tasks left in the scheduler's list.
    pub(super) fn has_tasks(&self) -> bool {
        !self.tasks.is_empty()
    }

    /// Runs the scheduler until all tasks are completed.
    ///
    /// This method repeatedly calls `step` until the task list is empty.
    pub(super) fn run(&mut self) {
        while self.has_tasks() {
            self.step();
        }
    }

    /// Executes the scheduler for a specified number of steps.
    ///
    /// The loop continues until the step count is reached or the task list
    /// becomes empty.
    pub(super) fn steps(&mut self, count: LuaInteger) {
        (0..count).for_each(|_| self.step());
    }

    /// Adds a new Lua task to the scheduler's list.
    ///
    /// The task is created with a given coroutine and priority.
    pub(super) fn add_task(&mut self, coroutine: LuaThread, priority: LuaInteger) {
        self.tasks.add(Task::new(coroutine, priority));
    }
}

impl<Tasks: TaskList + 'static> LuaScheduler<Tasks> {
    /// Executes a single step of the scheduler.
    ///
    /// A task is peeked from the list, resumed, and if it's still alive
    /// after the step, it's added back to the list. The scheduler's lifetime
    /// is incremented.
    fn step(&mut self) {
        let mut task = match self.tasks.peek() {
            Some(t) => t,
            None => return
        };

        task.resume();
        self.life_time += 1;

        if task.is_alive() {
            self.tasks.add(task);
        }
    }
}