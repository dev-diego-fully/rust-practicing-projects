//! This module provides a simple `FIFOTaskList`, which implements a
//! First-In, First-Out queueing strategy for tasks.
//!
use std::collections::VecDeque;

use crate::tasks::Task;

use super::TaskList;

/// A task list that uses a FIFO (First-In, First-Out) strategy.
///
/// It uses a `VecDeque` for efficient addition and removal of tasks from
/// the ends of the list.
pub(crate) struct FIFOTaskList {
    /// The underlying queue of tasks.
    tasks: VecDeque<Task>
}

impl TaskList for FIFOTaskList {

    type That = Self;

    /// Creates a new, empty `FIFOTaskList`.
    fn new() -> Self::That {
        Self {
            tasks: VecDeque::new()
        }
    }
     
    /// Removes and returns the oldest task from the front of the queue.
    ///
    /// Returns `None` if the queue is empty.
    fn peek(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Adds a new task to the back of the queue.
    fn add(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    /// Checks if the task list is empty.
    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}